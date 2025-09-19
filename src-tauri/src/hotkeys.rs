use crate::{
    config::{Config, TriggersConfig},
    error::{HoverShellError, Result},
};
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, GlobalShortcutManager};
use tokio::sync::RwLock;

pub struct HotkeyManager {
    registered_hotkeys: HashMap<String, String>,
    app_handle: Option<AppHandle>,
}

impl HotkeyManager {
    pub async fn new() -> Result<Self> {
        info!("Initializing hotkey manager");
        
        Ok(Self {
            registered_hotkeys: HashMap::new(),
            app_handle: None,
        })
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down hotkey manager");
        
        if let Some(app_handle) = &self.app_handle {
            let mut manager = app_handle.global_shortcut_manager();
            
            for hotkey in self.registered_hotkeys.keys() {
                if let Err(e) = manager.unregister(hotkey) {
                    error!("Failed to unregister hotkey {}: {}", hotkey, e);
                }
            }
        }
        
        self.registered_hotkeys.clear();
        self.app_handle = None;
        
        Ok(())
    }

    pub async fn register_default_hotkeys(&mut self, app_handle: &AppHandle, config: &Config) -> Result<()> {
        self.app_handle = Some(app_handle.clone());
        
        let triggers = &config.triggers;
        
        // Register toggle hotkey
        self.register(app_handle, &triggers.hotkeys.toggle, "toggle_window").await?;
        
        // Register paste and run hotkey
        self.register(app_handle, &triggers.hotkeys.paste_run, "paste_run").await?;
        
        // Register quick hide hotkey
        self.register(app_handle, &triggers.hotkeys.quick_hide, "quick_hide").await?;
        
        // Register tab management hotkeys
        self.register(app_handle, &triggers.hotkeys.new_tab, "new_tab").await?;
        self.register(app_handle, &triggers.hotkeys.close_tab, "close_tab").await?;
        self.register(app_handle, &triggers.hotkeys.next_tab, "next_tab").await?;
        self.register(app_handle, &triggers.hotkeys.prev_tab, "prev_tab").await?;
        
        info!("Registered default hotkeys");
        Ok(())
    }

    pub async fn register(&mut self, app_handle: &AppHandle, hotkey: &str, callback: &str) -> Result<()> {
        let mut manager = app_handle.global_shortcut_manager();
        
        // Convert hotkey string to proper format
        let normalized_hotkey = self.normalize_hotkey(hotkey)?;
        
        // Register the hotkey
        manager.register(&normalized_hotkey, move || {
            // TODO: Implement hotkey callback handling
            info!("Hotkey triggered: {} -> {}", normalized_hotkey, callback);
            
            // Emit event to frontend
            if let Some(app_handle) = app_handle.get_webview_window("main") {
                let _ = app_handle.emit("hotkey-triggered", serde_json::json!({
                    "hotkey": normalized_hotkey,
                    "callback": callback
                }));
            }
        }).map_err(|e| HoverShellError::Hotkey(format!("Failed to register hotkey {}: {}", hotkey, e)))?;
        
        self.registered_hotkeys.insert(normalized_hotkey.clone(), callback.to_string());
        info!("Registered hotkey: {} -> {}", normalized_hotkey, callback);
        
        Ok(())
    }

    pub async fn unregister(&mut self, hotkey: &str) -> Result<()> {
        let normalized_hotkey = self.normalize_hotkey(hotkey)?;
        
        if let Some(app_handle) = &self.app_handle {
            let mut manager = app_handle.global_shortcut_manager();
            
            manager.unregister(&normalized_hotkey)
                .map_err(|e| HoverShellError::Hotkey(format!("Failed to unregister hotkey {}: {}", hotkey, e)))?;
        }
        
        self.registered_hotkeys.remove(&normalized_hotkey);
        info!("Unregistered hotkey: {}", normalized_hotkey);
        
        Ok(())
    }

    pub async fn is_registered(&self, hotkey: &str) -> Result<bool> {
        let normalized_hotkey = self.normalize_hotkey(hotkey)?;
        Ok(self.registered_hotkeys.contains_key(&normalized_hotkey))
    }

    pub async fn get_registered_hotkeys(&self) -> HashMap<String, String> {
        self.registered_hotkeys.clone()
    }

    pub async fn update_hotkey(&mut self, old_hotkey: &str, new_hotkey: &str, callback: &str) -> Result<()> {
        // Unregister old hotkey
        if self.is_registered(old_hotkey).await? {
            self.unregister(old_hotkey).await?;
        }
        
        // Register new hotkey
        if let Some(app_handle) = &self.app_handle {
            self.register(app_handle, new_hotkey, callback).await?;
        }
        
        Ok(())
    }

    fn normalize_hotkey(&self, hotkey: &str) -> Result<String> {
        // Convert common hotkey formats to Tauri format
        let normalized = hotkey
            .to_lowercase()
            .replace("cmd", "CommandOrControl")
            .replace("ctrl", "CommandOrControl")
            .replace("alt", "Alt")
            .replace("shift", "Shift")
            .replace("meta", "Meta")
            .replace("super", "Super")
            .replace("+", "+")
            .replace(" ", "+");
        
        // Validate hotkey format
        if normalized.is_empty() {
            return Err(HoverShellError::Hotkey("Empty hotkey".to_string()));
        }
        
        Ok(normalized)
    }

    pub async fn handle_hotkey_event(&self, hotkey: &str) -> Result<()> {
        if let Some(callback) = self.registered_hotkeys.get(hotkey) {
            match callback.as_str() {
                "toggle_window" => {
                    // TODO: Implement toggle window logic
                    info!("Toggle window hotkey triggered");
                }
                "paste_run" => {
                    // TODO: Implement paste and run logic
                    info!("Paste and run hotkey triggered");
                }
                "quick_hide" => {
                    // TODO: Implement quick hide logic
                    info!("Quick hide hotkey triggered");
                }
                "new_tab" => {
                    // TODO: Implement new tab logic
                    info!("New tab hotkey triggered");
                }
                "close_tab" => {
                    // TODO: Implement close tab logic
                    info!("Close tab hotkey triggered");
                }
                "next_tab" => {
                    // TODO: Implement next tab logic
                    info!("Next tab hotkey triggered");
                }
                "prev_tab" => {
                    // TODO: Implement previous tab logic
                    info!("Previous tab hotkey triggered");
                }
                _ => {
                    info!("Unknown hotkey callback: {}", callback);
                }
            }
        }
        
        Ok(())
    }

    pub async fn register_custom_hotkey(&mut self, app_handle: &AppHandle, hotkey: &str, callback: &str) -> Result<()> {
        self.register(app_handle, hotkey, callback).await
    }

    pub async fn unregister_custom_hotkey(&mut self, hotkey: &str) -> Result<()> {
        self.unregister(hotkey).await
    }

    pub async fn get_hotkey_info(&self, hotkey: &str) -> Option<String> {
        let normalized_hotkey = self.normalize_hotkey(hotkey).ok()?;
        self.registered_hotkeys.get(&normalized_hotkey).cloned()
    }

    pub async fn validate_hotkey(&self, hotkey: &str) -> Result<bool> {
        let normalized = self.normalize_hotkey(hotkey)?;
        
        // Check if hotkey is already registered
        if self.registered_hotkeys.contains_key(&normalized) {
            return Ok(false);
        }
        
        // TODO: Add more validation logic
        // - Check for conflicts with system hotkeys
        // - Validate hotkey format
        // - Check for reserved hotkeys
        
        Ok(true)
    }

    pub async fn get_available_hotkeys(&self) -> Vec<String> {
        // TODO: Return list of available hotkey combinations
        vec![
            "CommandOrControl+`".to_string(),
            "Alt+`".to_string(),
            "CommandOrControl+Enter".to_string(),
            "Escape".to_string(),
            "CommandOrControl+T".to_string(),
            "CommandOrControl+W".to_string(),
            "CommandOrControl+Shift+]".to_string(),
            "CommandOrControl+Shift+[".to_string(),
        ]
    }
}