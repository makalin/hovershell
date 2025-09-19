use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum HoverShellError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("UI error: {0}")]
    UI(String),

    #[error("Hotkey error: {0}")]
    Hotkey(String),

    #[error("Tray error: {0}")]
    Tray(String),

    #[error("Core error: {0}")]
    Core(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Security error: {0}")]
    Security(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<std::io::Error> for HoverShellError {
    fn from(err: std::io::Error) -> Self {
        HoverShellError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for HoverShellError {
    fn from(err: serde_json::Error) -> Self {
        HoverShellError::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for HoverShellError {
    fn from(err: toml::de::Error) -> Self {
        HoverShellError::Parse(err.to_string())
    }
}

impl From<toml::ser::Error> for HoverShellError {
    fn from(err: toml::ser::Error) -> Self {
        HoverShellError::Serialization(err.to_string())
    }
}

impl From<yaml_rust::ScanError> for HoverShellError {
    fn from(err: yaml_rust::ScanError) -> Self {
        HoverShellError::Parse(err.to_string())
    }
}

impl From<reqwest::Error> for HoverShellError {
    fn from(err: reqwest::Error) -> Self {
        HoverShellError::Network(err.to_string())
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for HoverShellError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        HoverShellError::Network(err.to_string())
    }
}

impl From<keyring::Error> for HoverShellError {
    fn from(err: keyring::Error) -> Self {
        HoverShellError::Security(err.to_string())
    }
}

impl From<anyhow::Error> for HoverShellError {
    fn from(err: anyhow::Error) -> Self {
        HoverShellError::Unknown(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, HoverShellError>;