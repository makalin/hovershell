use crate::error::{HoverShellError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub position: String,
    pub height: String,
    pub blur: u8,
    pub opacity: f32,
    pub font: String,
    pub theme: String,
    pub font_size: u16,
    pub line_height: f32,
    pub padding: u16,
    pub border_radius: u16,
    pub shadow: bool,
    pub animations: bool,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            position: "top".to_string(),
            height: "45vh".to_string(),
            blur: 18,
            opacity: 0.92,
            font: "JetBrainsMono Nerd Font".to_string(),
            theme: "tokyo-night".to_string(),
            font_size: 14,
            line_height: 1.4,
            padding: 16,
            border_radius: 8,
            shadow: true,
            animations: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub toggle: String,
    pub paste_run: String,
    pub quick_hide: String,
    pub new_tab: String,
    pub close_tab: String,
    pub next_tab: String,
    pub prev_tab: String,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            toggle: "alt+`".to_string(),
            paste_run: "cmd+enter".to_string(),
            quick_hide: "esc".to_string(),
            new_tab: "cmd+t".to_string(),
            close_tab: "cmd+w".to_string(),
            next_tab: "cmd+shift+]".to_string(),
            prev_tab: "cmd+shift+[".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeConfig {
    pub reveal: bool,
    pub dwell_ms: u64,
    pub scroll_resize: bool,
    pub sensitivity: f32,
}

impl Default for EdgeConfig {
    fn default() -> Self {
        Self {
            reveal: true,
            dwell_ms: 450,
            scroll_resize: true,
            sensitivity: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggersConfig {
    pub hotkeys: HotkeyConfig,
    pub edges: EdgeConfig,
    pub wheel_reveal: bool,
    pub menu_bar_click: bool,
}

impl Default for TriggersConfig {
    fn default() -> Self {
        Self {
            hotkeys: HotkeyConfig::default(),
            edges: EdgeConfig::default(),
            wheel_reveal: true,
            menu_bar_click: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub api_key: Option<String>,
    pub default: bool,
    pub enabled: bool,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub shell: String,
    pub working_directory: Option<String>,
    pub environment: std::collections::HashMap<String, String>,
    pub scrollback_lines: usize,
    pub cursor_blink: bool,
    pub cursor_style: String,
    pub bell_sound: bool,
    pub auto_close: bool,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            shell: "/bin/zsh".to_string(),
            working_directory: None,
            environment: std::collections::HashMap::new(),
            scrollback_lines: 10000,
            cursor_blink: true,
            cursor_style: "block".to_string(),
            bell_sound: true,
            auto_close: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub auto_load: bool,
    pub config: serde_json::Value,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_load: true,
            config: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ui: UIConfig,
    pub triggers: TriggersConfig,
    pub providers: Vec<ProviderConfig>,
    pub terminal: TerminalConfig,
    pub plugins: std::collections::HashMap<String, PluginConfig>,
    pub workspace_rules: Vec<WorkspaceRule>,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRule {
    pub name: String,
    pub pattern: String,
    pub profile: String,
    pub auto_switch: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub keychain_storage: bool,
    pub sandbox_providers: bool,
    pub minimal_scopes: bool,
    pub auto_lock: bool,
    pub lock_timeout: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            keychain_storage: true,
            sandbox_providers: true,
            minimal_scopes: true,
            auto_lock: false,
            lock_timeout: 300, // 5 minutes
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: UIConfig::default(),
            triggers: TriggersConfig::default(),
            providers: vec![],
            terminal: TerminalConfig::default(),
            plugins: std::collections::HashMap::new(),
            workspace_rules: vec![],
            security: SecurityConfig::default(),
        }
    }
}

impl Config {
    pub fn config_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".hovershell")
    }

    pub fn config_file() -> PathBuf {
        Self::config_dir().join("config.yaml")
    }

    pub async fn load() -> Result<Self> {
        let config_path = Self::config_file();
        
        if !config_path.exists() {
            let config = Self::default();
            config.save().await?;
            return Ok(config);
        }

        let content = tokio::fs::read_to_string(&config_path).await?;
        let config: Config = serde_yaml::from_str(&content)
            .map_err(|e| HoverShellError::Parse(format!("Failed to parse config: {}", e)))?;

        Ok(config)
    }

    pub async fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir();
        tokio::fs::create_dir_all(&config_dir).await?;

        let config_path = Self::config_file();
        let content = serde_yaml::to_string(self)
            .map_err(|e| HoverShellError::Serialization(format!("Failed to serialize config: {}", e)))?;

        tokio::fs::write(&config_path, content).await?;
        Ok(())
    }

    pub fn get_provider(&self, id: &str) -> Option<&ProviderConfig> {
        self.providers.iter().find(|p| p.id == id)
    }

    pub fn get_default_provider(&self) -> Option<&ProviderConfig> {
        self.providers.iter().find(|p| p.default && p.enabled)
    }

    pub fn add_provider(&mut self, provider: ProviderConfig) {
        // Remove existing provider with same ID
        self.providers.retain(|p| p.id != provider.id);
        self.providers.push(provider);
    }

    pub fn remove_provider(&mut self, id: &str) -> bool {
        let initial_len = self.providers.len();
        self.providers.retain(|p| p.id != id);
        self.providers.len() < initial_len
    }

    pub fn set_default_provider(&mut self, id: &str) -> Result<()> {
        // Clear all default flags
        for provider in &mut self.providers {
            provider.default = false;
        }

        // Set the specified provider as default
        if let Some(provider) = self.providers.iter_mut().find(|p| p.id == id) {
            provider.default = true;
            Ok(())
        } else {
            Err(HoverShellError::Config(format!("Provider '{}' not found", id)))
        }
    }
}