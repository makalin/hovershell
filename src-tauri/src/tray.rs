use crate::{
    config::Config,
    error::{HoverShellError, Result},
};
use log::{error, info};
use serde_json::Value;
use std::sync::Arc;
use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tokio::sync::RwLock;

pub struct TrayManager {
    tray_menu: Option<SystemTrayMenu>,
    app_handle: Option<AppHandle>,
}

impl TrayManager {
    pub async fn new() -> Result<Self> {
        info!("Initializing tray manager");
        
        Ok(Self {
            tray_menu: None,
            app_handle: None,
        })
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down tray manager");
        self.tray_menu = None;
        self.app_handle = None;
        Ok(())
    }

    pub async fn initialize(&mut self, app_handle: &AppHandle) -> Result<()> {
        self.app_handle = Some(app_handle.clone());
        
        // Create system tray menu
        let tray_menu = self.create_tray_menu().await?;
        
        // Set up tray event handler
        app_handle.listen("system-tray-event", |event| {
            if let Some(payload) = event.payload() {
                if let Ok(tray_event) = serde_json::from_str::<SystemTrayEvent>(payload) {
                    Self::handle_tray_event(tray_event);
                }
            }
        });
        
        self.tray_menu = Some(tray_menu);
        info!("Tray manager initialized");
        
        Ok(())
    }

    async fn create_tray_menu(&self) -> Result<SystemTrayMenu> {
        let menu = SystemTrayMenu::new()
            .add_item(SystemTrayMenuItem::new("Show HoverShell", "show"))
            .add_item(SystemTrayMenuItem::new("Hide HoverShell", "hide"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(SystemTrayMenuItem::new("New Terminal", "new_terminal"))
            .add_item(SystemTrayMenuItem::new("Settings", "settings"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(SystemTrayMenuItem::new("About", "about"))
            .add_item(SystemTrayMenuItem::new("Quit", "quit"));

        Ok(menu)
    }

    fn handle_tray_event(event: SystemTrayEvent) {
        match event {
            SystemTrayEvent::LeftClick { .. } => {
                info!("Tray left click");
                // TODO: Implement toggle window on left click
            }
            SystemTrayEvent::RightClick { .. } => {
                info!("Tray right click");
                // TODO: Show context menu
            }
            SystemTrayEvent::DoubleClick { .. } => {
                info!("Tray double click");
                // TODO: Implement double click action
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                info!("Tray menu item clicked: {}", id);
                Self::handle_menu_item_click(id);
            }
            _ => {}
        }
    }

    fn handle_menu_item_click(id: String) {
        match id.as_str() {
            "show" => {
                info!("Show HoverShell menu item clicked");
                // TODO: Show window
            }
            "hide" => {
                info!("Hide HoverShell menu item clicked");
                // TODO: Hide window
            }
            "new_terminal" => {
                info!("New Terminal menu item clicked");
                // TODO: Create new terminal session
            }
            "settings" => {
                info!("Settings menu item clicked");
                // TODO: Open settings
            }
            "about" => {
                info!("About menu item clicked");
                // TODO: Show about dialog
            }
            "quit" => {
                info!("Quit menu item clicked");
                // TODO: Quit application
            }
            _ => {
                info!("Unknown menu item clicked: {}", id);
            }
        }
    }

    pub async fn get_menu_items(&self) -> Vec<Value> {
        // TODO: Return current menu items as JSON
        vec![
            serde_json::json!({
                "id": "show",
                "label": "Show HoverShell",
                "enabled": true
            }),
            serde_json::json!({
                "id": "hide",
                "label": "Hide HoverShell",
                "enabled": true
            }),
            serde_json::json!({
                "id": "separator1",
                "type": "separator"
            }),
            serde_json::json!({
                "id": "new_terminal",
                "label": "New Terminal",
                "enabled": true
            }),
            serde_json::json!({
                "id": "settings",
                "label": "Settings",
                "enabled": true
            }),
            serde_json::json!({
                "id": "separator2",
                "type": "separator"
            }),
            serde_json::json!({
                "id": "about",
                "label": "About",
                "enabled": true
            }),
            serde_json::json!({
                "id": "quit",
                "label": "Quit",
                "enabled": true
            }),
        ]
    }

    pub async fn update_menu(&mut self, items: Vec<Value>) -> Result<()> {
        // TODO: Update tray menu with new items
        info!("Updating tray menu with {} items", items.len());
        
        // Recreate menu with new items
        let mut menu = SystemTrayMenu::new();
        
        for item in items {
            if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
                if let Some(label) = item.get("label").and_then(|v| v.as_str()) {
                    let enabled = item.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
                    
                    if id == "separator" || id.starts_with("separator") {
                        menu = menu.add_native_item(SystemTrayMenuItem::Separator);
                    } else {
                        let menu_item = SystemTrayMenuItem::new(label, id);
                        if !enabled {
                            // TODO: Handle disabled menu items
                        }
                        menu = menu.add_item(menu_item);
                    }
                }
            }
        }
        
        self.tray_menu = Some(menu);
        Ok(())
    }

    pub async fn add_menu_item(&mut self, id: &str, label: &str, after: Option<&str>) -> Result<()> {
        // TODO: Add new menu item
        info!("Adding menu item: {} -> {}", id, label);
        Ok(())
    }

    pub async fn remove_menu_item(&mut self, id: &str) -> Result<()> {
        // TODO: Remove menu item
        info!("Removing menu item: {}", id);
        Ok(())
    }

    pub async fn update_menu_item(&mut self, id: &str, label: Option<&str>, enabled: Option<bool>) -> Result<()> {
        // TODO: Update menu item
        info!("Updating menu item: {}", id);
        Ok(())
    }

    pub async fn set_tooltip(&mut self, tooltip: &str) -> Result<()> {
        // TODO: Set tray tooltip
        info!("Setting tray tooltip: {}", tooltip);
        Ok(())
    }

    pub async fn set_icon(&mut self, icon_path: &str) -> Result<()> {
        // TODO: Set tray icon
        info!("Setting tray icon: {}", icon_path);
        Ok(())
    }

    pub async fn show_notification(&self, title: &str, body: &str) -> Result<()> {
        if let Some(app_handle) = &self.app_handle {
            tauri::api::notification::Notification::new(&app_handle.config().tauri.bundle.identifier)
                .title(title)
                .body(body)
                .show()
                .map_err(|e| HoverShellError::Tray(format!("Failed to show notification: {}", e)))?;
        }
        Ok(())
    }

    pub async fn update_status(&mut self, status: &str) -> Result<()> {
        // TODO: Update tray status
        info!("Updating tray status: {}", status);
        Ok(())
    }

    pub async fn get_current_menu(&self) -> Option<&SystemTrayMenu> {
        self.tray_menu.as_ref()
    }

    pub async fn refresh_menu(&mut self) -> Result<()> {
        // TODO: Refresh tray menu
        info!("Refreshing tray menu");
        Ok(())
    }

    pub async fn set_menu_visibility(&mut self, visible: bool) -> Result<()> {
        // TODO: Set menu visibility
        info!("Setting menu visibility: {}", visible);
        Ok(())
    }

    pub async fn handle_tray_click(&self, button: u8) -> Result<()> {
        match button {
            1 => {
                // Left click - toggle window
                info!("Tray left click - toggling window");
                // TODO: Implement toggle window
            }
            2 => {
                // Right click - show context menu
                info!("Tray right click - showing context menu");
                // TODO: Show context menu
            }
            3 => {
                // Middle click
                info!("Tray middle click");
                // TODO: Implement middle click action
            }
            _ => {
                info!("Unknown tray click button: {}", button);
            }
        }
        Ok(())
    }
}