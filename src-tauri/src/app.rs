use crate::{
    config::Config,
    core::Core,
    error::HoverShellError,
    hotkeys::HotkeyManager,
    providers::ProviderManager,
    terminal::TerminalManager,
    tray::TrayManager,
    ui::UIManager,
};
use log::{error, info};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

pub struct HoverShellApp {
    pub config: Arc<RwLock<Config>>,
    pub core: Arc<RwLock<Core>>,
    pub providers: Arc<RwLock<ProviderManager>>,
    pub terminal: Arc<RwLock<TerminalManager>>,
    pub ui: Arc<RwLock<UIManager>>,
    pub hotkeys: Arc<RwLock<HotkeyManager>>,
    pub tray: Arc<RwLock<TrayManager>>,
}

impl HoverShellApp {
    pub async fn new() -> Result<Self, HoverShellError> {
        info!("Initializing HoverShell application");

        // Load configuration
        let config = Arc::new(RwLock::new(Config::load().await?));
        info!("Configuration loaded");

        // Initialize core
        let core = Arc::new(RwLock::new(Core::new().await?));
        info!("Core initialized");

        // Initialize providers
        let providers = Arc::new(RwLock::new(ProviderManager::new().await?));
        info!("Provider manager initialized");

        // Initialize terminal
        let terminal = Arc::new(RwLock::new(TerminalManager::new().await?));
        info!("Terminal manager initialized");

        // Initialize UI
        let ui = Arc::new(RwLock::new(UIManager::new().await?));
        info!("UI manager initialized");

        // Initialize hotkeys
        let hotkeys = Arc::new(RwLock::new(HotkeyManager::new().await?));
        info!("Hotkey manager initialized");

        // Initialize tray
        let tray = Arc::new(RwLock::new(TrayManager::new().await?));
        info!("Tray manager initialized");

        Ok(Self {
            config,
            core,
            providers,
            terminal,
            ui,
            hotkeys,
            tray,
        })
    }

    pub async fn initialize(&mut self, app_handle: AppHandle) -> Result<(), HoverShellError> {
        info!("Starting application initialization");

        // Initialize tray icon
        {
            let mut tray = self.tray.write().await;
            tray.initialize(&app_handle).await?;
        }

        // Register global hotkeys
        {
            let config = self.config.read().await;
            let mut hotkeys = self.hotkeys.write().await;
            hotkeys.register_default_hotkeys(&app_handle, &config).await?;
        }

        // Initialize providers from config
        {
            let config = self.config.read().await;
            let mut providers = self.providers.write().await;
            providers.load_from_config(&config).await?;
        }

        // Initialize terminal with default shell
        {
            let config = self.config.read().await;
            let mut terminal = self.terminal.write().await;
            terminal.initialize(&config).await?;
        }

        // Initialize UI
        {
            let config = self.config.read().await;
            let mut ui = self.ui.write().await;
            ui.initialize(&config).await?;
        }

        info!("Application initialization completed successfully");
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), HoverShellError> {
        info!("Shutting down HoverShell application");

        // Save configuration
        {
            let config = self.config.read().await;
            config.save().await?;
        }

        // Shutdown components in reverse order
        if let Err(e) = self.tray.write().await.shutdown().await {
            error!("Error shutting down tray manager: {}", e);
        }

        if let Err(e) = self.hotkeys.write().await.shutdown().await {
            error!("Error shutting down hotkey manager: {}", e);
        }

        if let Err(e) = self.terminal.write().await.shutdown().await {
            error!("Error shutting down terminal manager: {}", e);
        }

        if let Err(e) = self.providers.write().await.shutdown().await {
            error!("Error shutting down provider manager: {}", e);
        }

        if let Err(e) = self.ui.write().await.shutdown().await {
            error!("Error shutting down UI manager: {}", e);
        }

        if let Err(e) = self.core.write().await.shutdown().await {
            error!("Error shutting down core: {}", e);
        }

        info!("HoverShell application shutdown completed");
        Ok(())
    }
}