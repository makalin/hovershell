use crate::{
    config::{Config, ProviderConfig},
    error::{HoverShellError, Result},
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub content: String,
    pub usage: Option<UsageInfo>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInfo {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub cost: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub trait AIProvider: Send + Sync {
    async fn execute(&self, prompt: &str, context: Option<&str>) -> Result<ProviderResponse>;
    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<ProviderResponse>;
    async fn stream(&self, prompt: &str, context: Option<&str>) -> Result<Box<dyn futures_util::Stream<Item = Result<String>> + Unpin>>;
    fn get_info(&self) -> ProviderInfo;
}

#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub capabilities: Vec<String>,
    pub max_tokens: Option<u32>,
    pub supports_streaming: bool,
    pub supports_chat: bool,
}

pub struct ProviderManager {
    providers: HashMap<String, Box<dyn AIProvider>>,
    default_provider: Option<String>,
}

impl ProviderManager {
    pub async fn new() -> Result<Self> {
        info!("Initializing provider manager");
        
        Ok(Self {
            providers: HashMap::new(),
            default_provider: None,
        })
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down provider manager");
        self.providers.clear();
        self.default_provider = None;
        Ok(())
    }

    pub async fn load_from_config(&mut self, config: &Config) -> Result<()> {
        for provider_config in &config.providers {
            if provider_config.enabled {
                self.add_provider_from_config(provider_config).await?;
            }
        }
        
        // Set default provider
        if let Some(default) = config.get_default_provider() {
            self.set_default_provider(&default.id).await?;
        }
        
        Ok(())
    }

    pub async fn add_provider_from_config(&mut self, config: &ProviderConfig) -> Result<()> {
        let provider: Box<dyn AIProvider> = match config.provider_type.as_str() {
            "openai" => Box::new(OpenAIProvider::new(config)?),
            "anthropic" => Box::new(AnthropicProvider::new(config)?),
            "ollama" => Box::new(OllamaProvider::new(config)?),
            "cohere" => Box::new(CohereProvider::new(config)?),
            _ => return Err(HoverShellError::Provider(format!("Unknown provider type: {}", config.provider_type))),
        };

        self.providers.insert(config.id.clone(), provider);
        info!("Added provider: {}", config.id);
        Ok(())
    }

    pub async fn execute(&self, prompt: &str) -> Result<String> {
        let provider = self.get_default_provider()?;
        let response = provider.execute(prompt, None).await?;
        Ok(response.content)
    }

    pub async fn execute_with_provider(&self, prompt: &str, provider_id: &str) -> Result<String> {
        let provider = self.providers.get(provider_id)
            .ok_or_else(|| HoverShellError::Provider(format!("Provider not found: {}", provider_id)))?;
        
        let response = provider.execute(prompt, None).await?;
        Ok(response.content)
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>, provider_id: Option<&str>) -> Result<String> {
        let provider = if let Some(id) = provider_id {
            self.providers.get(id)
                .ok_or_else(|| HoverShellError::Provider(format!("Provider not found: {}", id)))?
        } else {
            self.get_default_provider()?
        };

        let response = provider.chat(messages).await?;
        Ok(response.content)
    }

    pub async fn stream(&self, prompt: &str, provider_id: Option<&str>) -> Result<Box<dyn futures_util::Stream<Item = Result<String>> + Unpin>> {
        let provider = if let Some(id) = provider_id {
            self.providers.get(id)
                .ok_or_else(|| HoverShellError::Provider(format!("Provider not found: {}", id)))?
        } else {
            self.get_default_provider()?
        };

        provider.stream(prompt, None).await
    }

    pub fn get_default_provider(&self) -> Result<&Box<dyn AIProvider>> {
        if let Some(default_id) = &self.default_provider {
            self.providers.get(default_id)
                .ok_or_else(|| HoverShellError::Provider(format!("Default provider not found: {}", default_id)))
        } else {
            Err(HoverShellError::Provider("No default provider set".to_string()))
        }
    }

    pub async fn set_default_provider(&mut self, provider_id: &str) -> Result<()> {
        if self.providers.contains_key(provider_id) {
            self.default_provider = Some(provider_id.to_string());
            info!("Set default provider: {}", provider_id);
            Ok(())
        } else {
            Err(HoverShellError::Provider(format!("Provider not found: {}", provider_id)))
        }
    }

    pub fn get_provider_list(&self) -> Vec<ProviderInfo> {
        self.providers.values().map(|p| p.get_info()).collect()
    }

    pub fn get_provider_info(&self, provider_id: &str) -> Option<ProviderInfo> {
        self.providers.get(provider_id).map(|p| p.get_info())
    }
}

// OpenAI Provider Implementation
pub struct OpenAIProvider {
    config: ProviderConfig,
    client: reqwest::Client,
}

impl OpenAIProvider {
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self {
            config: config.clone(),
            client,
        })
    }
}

#[async_trait::async_trait]
impl AIProvider for OpenAIProvider {
    async fn execute(&self, prompt: &str, context: Option<&str>) -> Result<ProviderResponse> {
        let url = format!("{}/v1/completions", self.config.base_url.as_deref().unwrap_or("https://api.openai.com"));
        
        let mut body = serde_json::json!({
            "model": self.config.model.as_deref().unwrap_or("gpt-3.5-turbo"),
            "prompt": prompt,
            "max_tokens": 1000,
            "temperature": 0.7
        });

        if let Some(ctx) = context {
            body["prompt"] = serde_json::json!(format!("Context: {}\n\nPrompt: {}", ctx, prompt));
        }

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key.as_deref().unwrap_or("")))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let content = result["choices"][0]["text"].as_str()
            .unwrap_or("")
            .to_string();

        Ok(ProviderResponse {
            content,
            usage: None,
            metadata: HashMap::new(),
        })
    }

    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<ProviderResponse> {
        let url = format!("{}/v1/chat/completions", self.config.base_url.as_deref().unwrap_or("https://api.openai.com"));
        
        let body = serde_json::json!({
            "model": self.config.model.as_deref().unwrap_or("gpt-3.5-turbo"),
            "messages": messages,
            "max_tokens": 1000,
            "temperature": 0.7
        });

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key.as_deref().unwrap_or("")))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let content = result["choices"][0]["message"]["content"].as_str()
            .unwrap_or("")
            .to_string();

        Ok(ProviderResponse {
            content,
            usage: None,
            metadata: HashMap::new(),
        })
    }

    async fn stream(&self, prompt: &str, context: Option<&str>) -> Result<Box<dyn futures_util::Stream<Item = Result<String>> + Unpin>> {
        // TODO: Implement streaming
        Err(HoverShellError::Provider("Streaming not implemented".to_string()))
    }

    fn get_info(&self) -> ProviderInfo {
        ProviderInfo {
            id: self.config.id.clone(),
            name: self.config.name.clone(),
            provider_type: "openai".to_string(),
            capabilities: vec!["text", "chat", "streaming".to_string()],
            max_tokens: Some(4096),
            supports_streaming: true,
            supports_chat: true,
        }
    }
}

// Anthropic Provider Implementation
pub struct AnthropicProvider {
    config: ProviderConfig,
    client: reqwest::Client,
}

impl AnthropicProvider {
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self {
            config: config.clone(),
            client,
        })
    }
}

#[async_trait::async_trait]
impl AIProvider for AnthropicProvider {
    async fn execute(&self, prompt: &str, context: Option<&str>) -> Result<ProviderResponse> {
        let url = format!("{}/v1/messages", self.config.base_url.as_deref().unwrap_or("https://api.anthropic.com"));
        
        let mut body = serde_json::json!({
            "model": self.config.model.as_deref().unwrap_or("claude-3-sonnet-20240229"),
            "max_tokens": 1000,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        if let Some(ctx) = context {
            body["messages"][0]["content"] = serde_json::json!(format!("Context: {}\n\nPrompt: {}", ctx, prompt));
        }

        let response = self.client
            .post(&url)
            .header("x-api-key", self.config.api_key.as_deref().unwrap_or(""))
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let content = result["content"][0]["text"].as_str()
            .unwrap_or("")
            .to_string();

        Ok(ProviderResponse {
            content,
            usage: None,
            metadata: HashMap::new(),
        })
    }

    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<ProviderResponse> {
        let url = format!("{}/v1/messages", self.config.base_url.as_deref().unwrap_or("https://api.anthropic.com"));
        
        let body = serde_json::json!({
            "model": self.config.model.as_deref().unwrap_or("claude-3-sonnet-20240229"),
            "max_tokens": 1000,
            "messages": messages
        });

        let response = self.client
            .post(&url)
            .header("x-api-key", self.config.api_key.as_deref().unwrap_or(""))
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let content = result["content"][0]["text"].as_str()
            .unwrap_or("")
            .to_string();

        Ok(ProviderResponse {
            content,
            usage: None,
            metadata: HashMap::new(),
        })
    }

    async fn stream(&self, prompt: &str, context: Option<&str>) -> Result<Box<dyn futures_util::Stream<Item = Result<String>> + Unpin>> {
        // TODO: Implement Anthropic streaming
        Err(HoverShellError::Provider("Anthropic streaming not implemented".to_string()))
    }

    fn get_info(&self) -> ProviderInfo {
        ProviderInfo {
            id: self.config.id.clone(),
            name: self.config.name.clone(),
            provider_type: "anthropic".to_string(),
            capabilities: vec!["text", "chat".to_string()],
            max_tokens: Some(100000),
            supports_streaming: false,
            supports_chat: true,
        }
    }
}

// Ollama Provider Implementation
pub struct OllamaProvider {
    config: ProviderConfig,
    client: reqwest::Client,
}

impl OllamaProvider {
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self {
            config: config.clone(),
            client,
        })
    }
}

#[async_trait::async_trait]
impl AIProvider for OllamaProvider {
    async fn execute(&self, prompt: &str, context: Option<&str>) -> Result<ProviderResponse> {
        let url = format!("{}/api/generate", self.config.base_url.as_deref().unwrap_or("http://127.0.0.1:11434"));
        
        let body = serde_json::json!({
            "model": self.config.model.as_deref().unwrap_or("llama3.1:8b"),
            "prompt": prompt,
            "stream": false
        });

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let content = result["response"].as_str()
            .unwrap_or("")
            .to_string();

        Ok(ProviderResponse {
            content,
            usage: None,
            metadata: HashMap::new(),
        })
    }

    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<ProviderResponse> {
        let url = format!("{}/api/chat", self.config.base_url.as_deref().unwrap_or("http://127.0.0.1:11434"));
        
        let body = serde_json::json!({
            "model": self.config.model.as_deref().unwrap_or("llama3.1:8b"),
            "messages": messages,
            "stream": false
        });

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let content = result["message"]["content"].as_str()
            .unwrap_or("")
            .to_string();

        Ok(ProviderResponse {
            content,
            usage: None,
            metadata: HashMap::new(),
        })
    }

    async fn stream(&self, prompt: &str, context: Option<&str>) -> Result<Box<dyn futures_util::Stream<Item = Result<String>> + Unpin>> {
        // TODO: Implement Ollama streaming
        Err(HoverShellError::Provider("Ollama streaming not implemented".to_string()))
    }

    fn get_info(&self) -> ProviderInfo {
        ProviderInfo {
            id: self.config.id.clone(),
            name: self.config.name.clone(),
            provider_type: "ollama".to_string(),
            capabilities: vec!["text", "chat", "streaming".to_string()],
            max_tokens: Some(8192),
            supports_streaming: true,
            supports_chat: true,
        }
    }
}

// Cohere Provider Implementation
pub struct CohereProvider {
    config: ProviderConfig,
    client: reqwest::Client,
}

impl CohereProvider {
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self {
            config: config.clone(),
            client,
        })
    }
}

#[async_trait::async_trait]
impl AIProvider for CohereProvider {
    async fn execute(&self, prompt: &str, context: Option<&str>) -> Result<ProviderResponse> {
        let url = format!("{}/v1/generate", self.config.base_url.as_deref().unwrap_or("https://api.cohere.ai"));
        
        let mut body = serde_json::json!({
            "model": self.config.model.as_deref().unwrap_or("command"),
            "prompt": prompt,
            "max_tokens": 1000,
            "temperature": 0.7
        });

        if let Some(ctx) = context {
            body["prompt"] = serde_json::json!(format!("Context: {}\n\nPrompt: {}", ctx, prompt));
        }

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key.as_deref().unwrap_or("")))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let content = result["generations"][0]["text"].as_str()
            .unwrap_or("")
            .to_string();

        Ok(ProviderResponse {
            content,
            usage: None,
            metadata: HashMap::new(),
        })
    }

    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<ProviderResponse> {
        let url = format!("{}/v1/chat", self.config.base_url.as_deref().unwrap_or("https://api.cohere.ai"));
        
        let body = serde_json::json!({
            "model": self.config.model.as_deref().unwrap_or("command"),
            "chat_history": messages,
            "message": messages.last().map(|m| &m.content).unwrap_or(""),
            "max_tokens": 1000,
            "temperature": 0.7
        });

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key.as_deref().unwrap_or("")))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let content = result["text"].as_str()
            .unwrap_or("")
            .to_string();

        Ok(ProviderResponse {
            content,
            usage: None,
            metadata: HashMap::new(),
        })
    }

    async fn stream(&self, prompt: &str, context: Option<&str>) -> Result<Box<dyn futures_util::Stream<Item = Result<String>> + Unpin>> {
        // TODO: Implement Cohere streaming
        Err(HoverShellError::Provider("Cohere streaming not implemented".to_string()))
    }

    fn get_info(&self) -> ProviderInfo {
        ProviderInfo {
            id: self.config.id.clone(),
            name: self.config.name.clone(),
            provider_type: "cohere".to_string(),
            capabilities: vec!["text", "chat".to_string()],
            max_tokens: Some(2048),
            supports_streaming: false,
            supports_chat: true,
        }
    }
}