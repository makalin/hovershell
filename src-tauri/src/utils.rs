use crate::error::{HoverShellError, Result};
use log::{error, info};
use std::path::PathBuf;
use std::process::Command;

pub mod file_utils;
pub mod system_utils;
pub mod crypto_utils;
pub mod network_utils;

pub use file_utils::*;
pub use system_utils::*;
pub use crypto_utils::*;
pub use network_utils::*;

pub fn get_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".hovershell")
}

pub fn get_plugins_dir() -> PathBuf {
    get_config_dir().join("plugins")
}

pub fn get_themes_dir() -> PathBuf {
    get_config_dir().join("themes")
}

pub fn get_logs_dir() -> PathBuf {
    get_config_dir().join("logs")
}

pub async fn ensure_directories() -> Result<()> {
    let dirs = vec![
        get_config_dir(),
        get_plugins_dir(),
        get_themes_dir(),
        get_logs_dir(),
    ];

    for dir in dirs {
        if !dir.exists() {
            tokio::fs::create_dir_all(&dir).await?;
            info!("Created directory: {:?}", dir);
        }
    }

    Ok(())
}

pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn get_app_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

pub fn get_app_description() -> String {
    env!("CARGO_PKG_DESCRIPTION").to_string()
}

pub fn get_app_authors() -> Vec<String> {
    env!("CARGO_PKG_AUTHORS")
        .split(':')
        .map(|s| s.to_string())
        .collect()
}

pub fn get_app_repository() -> String {
    env!("CARGO_PKG_REPOSITORY").to_string()
}

pub fn get_app_license() -> String {
    env!("CARGO_PKG_LICENSE").to_string()
}

pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}h {}m {}s", seconds / 3600, (seconds % 3600) / 60, seconds % 60)
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

pub fn is_valid_hotkey(hotkey: &str) -> bool {
    // Basic validation for hotkey format
    !hotkey.is_empty() && hotkey.len() < 100
}

pub fn parse_hotkey(hotkey: &str) -> Result<Vec<String>> {
    let parts: Vec<String> = hotkey
        .split('+')
        .map(|s| s.trim().to_lowercase())
        .collect();

    if parts.is_empty() {
        return Err(HoverShellError::Parse("Empty hotkey".to_string()));
    }

    // Validate hotkey parts
    for part in &parts {
        match part.as_str() {
            "cmd" | "ctrl" | "alt" | "shift" | "meta" | "super" => {}
            key if key.len() == 1 && key.chars().next().unwrap().is_ascii_alphanumeric() => {}
            "space" | "enter" | "return" | "tab" | "escape" | "esc" | "backspace" | "delete" => {}
            "up" | "down" | "left" | "right" => {}
            "f1" | "f2" | "f3" | "f4" | "f5" | "f6" | "f7" | "f8" | "f9" | "f10" | "f11" | "f12" => {}
            _ => {
                return Err(HoverShellError::Parse(format!("Invalid hotkey part: {}", part)));
            }
        }
    }

    Ok(parts)
}

pub fn normalize_hotkey(hotkey: &str) -> Result<String> {
    let parts = parse_hotkey(hotkey)?;
    Ok(parts.join("+"))
}

pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        family: std::env::consts::FAMILY.to_string(),
        version: get_os_version(),
        hostname: get_hostname(),
        username: get_username(),
        home_dir: get_home_directory(),
        temp_dir: get_temp_directory(),
    }
}

pub fn get_os_version() -> String {
    // TODO: Implement OS version detection
    "Unknown".to_string()
}

pub fn get_hostname() -> String {
    hostname::get()
        .unwrap_or_else(|_| "unknown".into())
        .to_string_lossy()
        .to_string()
}

pub fn get_username() -> String {
    whoami::username()
}

pub fn get_home_directory() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

pub fn get_temp_directory() -> PathBuf {
    std::env::temp_dir()
}

pub fn get_current_working_directory() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn set_current_working_directory(path: &PathBuf) -> Result<()> {
    std::env::set_current_dir(path)
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    Ok(())
}

pub fn get_environment_variable(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

pub fn set_environment_variable(key: &str, value: &str) -> Result<()> {
    std::env::set_var(key, value);
    Ok(())
}

pub fn get_all_environment_variables() -> std::collections::HashMap<String, String> {
    std::env::vars().collect()
}

pub fn execute_command(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| HoverShellError::Core(e.to_string()))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(HoverShellError::Core(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

pub fn execute_command_async(command: &str, args: &[&str]) -> Result<tokio::process::Command> {
    let mut cmd = tokio::process::Command::new(command);
    cmd.args(args);
    Ok(cmd)
}

pub fn is_process_running(pid: u32) -> bool {
    // TODO: Implement process running check
    false
}

pub fn kill_process(pid: u32) -> Result<()> {
    // TODO: Implement process termination
    Ok(())
}

pub fn get_process_list() -> Result<Vec<ProcessInfo>> {
    // TODO: Implement process list retrieval
    Ok(vec![])
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub family: String,
    pub version: String,
    pub hostname: String,
    pub username: String,
    pub home_dir: PathBuf,
    pub temp_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub command: String,
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub status: String,
}

pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn hash_string(input: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn validate_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

pub fn extract_domain(url: &str) -> Option<String> {
    url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
}

pub fn is_local_url(url: &str) -> bool {
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return host == "localhost" || host == "127.0.0.1" || host.starts_with("192.168.") || host.starts_with("10.");
        }
    }
    false
}

pub fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn get_timestamp_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

pub fn format_timestamp(timestamp: u64) -> String {
    chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_default()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

pub fn parse_timestamp(timestamp_str: &str) -> Result<u64> {
    chrono::NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| HoverShellError::Parse(e.to_string()))
        .map(|dt| dt.timestamp() as u64)
}

pub fn is_valid_email(email: &str) -> bool {
    // Basic email validation
    email.contains('@') && email.contains('.') && email.len() > 5
}

pub fn is_valid_json(json_str: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(json_str).is_ok()
}

pub fn is_valid_yaml(yaml_str: &str) -> bool {
    serde_yaml::from_str::<serde_yaml::Value>(yaml_str).is_ok()
}

pub fn is_valid_toml(toml_str: &str) -> bool {
    toml::from_str::<toml::Value>(toml_str).is_ok()
}

pub fn deep_merge_json(base: &mut serde_json::Value, other: serde_json::Value) {
    match (base, other) {
        (serde_json::Value::Object(base_map), serde_json::Value::Object(other_map)) => {
            for (key, value) in other_map {
                match base_map.get_mut(&key) {
                    Some(base_value) => {
                        deep_merge_json(base_value, value);
                    }
                    None => {
                        base_map.insert(key, value);
                    }
                }
            }
        }
        (base_value, other_value) => {
            *base_value = other_value;
        }
    }
}

pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}

pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap_or(c));
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

pub fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap_or(c));
    }
    
    result
}