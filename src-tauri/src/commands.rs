use crate::{
    app::HoverShellApp,
    config::{Config, ProviderConfig},
    error::{HoverShellError, Result},
    tools::*,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, State};
use tokio::sync::RwLock;

type AppState = Arc<RwLock<HoverShellApp>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalState {
    pub id: String,
    pub title: String,
    pub working_directory: String,
    pub is_active: bool,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub version: String,
    pub memory: u64,
    pub cpu_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub path: String,
    pub name: String,
    pub git_branch: Option<String>,
    pub git_status: Option<String>,
    pub file_count: usize,
    pub language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub colors: serde_json::Value,
}

#[tauri::command]
pub async fn toggle_window(app_handle: AppHandle) -> Result<()> {
    if let Some(window) = app_handle.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            window.hide().map_err(|e| HoverShellError::UI(e.to_string()))?;
        } else {
            window.show().map_err(|e| HoverShellError::UI(e.to_string()))?;
            window.set_focus().map_err(|e| HoverShellError::UI(e.to_string()))?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn show_window(app_handle: AppHandle) -> Result<()> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.show().map_err(|e| HoverShellError::UI(e.to_string()))?;
        window.set_focus().map_err(|e| HoverShellError::UI(e.to_string()))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn hide_window(app_handle: AppHandle) -> Result<()> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.hide().map_err(|e| HoverShellError::UI(e.to_string()))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Config> {
    let app = state.read().await;
    let config = app.config.read().await;
    Ok(config.clone())
}

#[tauri::command]
pub async fn set_config(state: State<'_, AppState>, config: Config) -> Result<()> {
    let app = state.read().await;
    let mut app_config = app.config.write().await;
    *app_config = config;
    app_config.save().await?;
    Ok(())
}

#[tauri::command]
pub async fn get_providers(state: State<'_, AppState>) -> Result<Vec<ProviderConfig>> {
    let app = state.read().await;
    let config = app.config.read().await;
    Ok(config.providers.clone())
}

#[tauri::command]
pub async fn add_provider(state: State<'_, AppState>, provider: ProviderConfig) -> Result<()> {
    let app = state.read().await;
    let mut config = app.config.write().await;
    config.add_provider(provider);
    config.save().await?;
    Ok(())
}

#[tauri::command]
pub async fn remove_provider(state: State<'_, AppState>, id: String) -> Result<bool> {
    let app = state.read().await;
    let mut config = app.config.write().await;
    let removed = config.remove_provider(&id);
    config.save().await?;
    Ok(removed)
}

#[tauri::command]
pub async fn set_default_provider(state: State<'_, AppState>, id: String) -> Result<()> {
    let app = state.read().await;
    let mut config = app.config.write().await;
    config.set_default_provider(&id)?;
    config.save().await?;
    Ok(())
}

#[tauri::command]
pub async fn execute_command(
    state: State<'_, AppState>,
    command: String,
    provider_id: Option<String>,
) -> Result<String> {
    let app = state.read().await;
    let terminal = app.terminal.read().await;
    let providers = app.providers.read().await;
    
    let result = if let Some(pid) = provider_id {
        // Execute with specific provider
        providers.execute_with_provider(&command, &pid).await?
    } else {
        // Execute with default provider
        providers.execute(&command).await?
    };
    
    Ok(result)
}

#[tauri::command]
pub async fn get_terminal_state(state: State<'_, AppState>) -> Result<Vec<TerminalState>> {
    let app = state.read().await;
    let terminal = app.terminal.read().await;
    Ok(terminal.get_state().await)
}

#[tauri::command]
pub async fn send_terminal_input(
    state: State<'_, AppState>,
    terminal_id: String,
    input: String,
) -> Result<()> {
    let app = state.read().await;
    let mut terminal = app.terminal.write().await;
    terminal.send_input(&terminal_id, &input).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_plugin_list(state: State<'_, AppState>) -> Result<Vec<String>> {
    let app = state.read().await;
    let core = app.core.read().await;
    Ok(core.get_plugin_list().await)
}

#[tauri::command]
pub async fn load_plugin(state: State<'_, AppState>, plugin_path: String) -> Result<()> {
    let app = state.read().await;
    let mut core = app.core.write().await;
    core.load_plugin(&plugin_path).await?;
    Ok(())
}

#[tauri::command]
pub async fn unload_plugin(state: State<'_, AppState>, plugin_id: String) -> Result<()> {
    let app = state.read().await;
    let mut core = app.core.write().await;
    core.unload_plugin(&plugin_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn register_hotkey(
    state: State<'_, AppState>,
    app_handle: AppHandle,
    hotkey: String,
    callback: String,
) -> Result<()> {
    let app = state.read().await;
    let mut hotkeys = app.hotkeys.write().await;
    hotkeys.register(&app_handle, &hotkey, &callback).await?;
    Ok(())
}

#[tauri::command]
pub async fn unregister_hotkey(state: State<'_, AppState>, hotkey: String) -> Result<()> {
    let app = state.read().await;
    let mut hotkeys = app.hotkeys.write().await;
    hotkeys.unregister(&hotkey).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_menu_items(state: State<'_, AppState>) -> Result<Vec<serde_json::Value>> {
    let app = state.read().await;
    let tray = app.tray.read().await;
    Ok(tray.get_menu_items().await)
}

#[tauri::command]
pub async fn update_menu(state: State<'_, AppState>, items: Vec<serde_json::Value>) -> Result<()> {
    let app = state.read().await;
    let mut tray = app.tray.write().await;
    tray.update_menu(items).await?;
    Ok(())
}

#[tauri::command]
pub async fn show_notification(
    app_handle: AppHandle,
    title: String,
    body: String,
) -> Result<()> {
    tauri::api::notification::Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title(&title)
        .body(&body)
        .show()
        .map_err(|e| HoverShellError::UI(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo> {
    let info = SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        version: "1.0.0".to_string(), // TODO: Get actual version
        memory: 0, // TODO: Get actual memory
        cpu_count: num_cpus::get(),
    };
    Ok(info)
}

#[tauri::command]
pub async fn get_workspace_info(workspace_path: String) -> Result<WorkspaceInfo> {
    let path = std::path::Path::new(&workspace_path);
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    // TODO: Implement git status detection
    let git_branch = None;
    let git_status = None;
    
    // TODO: Implement file count and language detection
    let file_count = 0;
    let language = None;

    Ok(WorkspaceInfo {
        path: workspace_path,
        name,
        git_branch,
        git_status,
        file_count,
        language,
    })
}

#[tauri::command]
pub async fn save_workspace_config(
    state: State<'_, AppState>,
    workspace_path: String,
    config: serde_json::Value,
) -> Result<()> {
    let app = state.read().await;
    let mut core = app.core.write().await;
    core.save_workspace_config(&workspace_path, config).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_theme_list() -> Result<Vec<ThemeInfo>> {
    let themes = vec![
        ThemeInfo {
            name: "tokyo-night".to_string(),
            display_name: "Tokyo Night".to_string(),
            description: "Dark theme inspired by Tokyo's night sky".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "dracula".to_string(),
            display_name: "Dracula".to_string(),
            description: "Dark theme with vibrant colors".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "light".to_string(),
            display_name: "Light".to_string(),
            description: "Clean light theme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "monokai".to_string(),
            display_name: "Monokai".to_string(),
            description: "Classic Monokai color scheme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "nord".to_string(),
            display_name: "Nord".to_string(),
            description: "Arctic-inspired color palette".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "gruvbox".to_string(),
            display_name: "Gruvbox".to_string(),
            description: "Retro groove color scheme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "one-dark".to_string(),
            display_name: "One Dark".to_string(),
            description: "Atom's One Dark theme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "solarized-dark".to_string(),
            display_name: "Solarized Dark".to_string(),
            description: "Solarized dark color scheme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "solarized-light".to_string(),
            display_name: "Solarized Light".to_string(),
            description: "Solarized light color scheme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "catppuccin-mocha".to_string(),
            display_name: "Catppuccin Mocha".to_string(),
            description: "Soothing pastel theme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "catppuccin-latte".to_string(),
            display_name: "Catppuccin Latte".to_string(),
            description: "Light pastel theme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "material-dark".to_string(),
            display_name: "Material Dark".to_string(),
            description: "Google Material Design dark theme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "github-dark".to_string(),
            display_name: "GitHub Dark".to_string(),
            description: "GitHub's dark theme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
        ThemeInfo {
            name: "github-light".to_string(),
            display_name: "GitHub Light".to_string(),
            description: "GitHub's light theme".to_string(),
            colors: serde_json::Value::Object(serde_json::Map::new()),
        },
    ];
    Ok(themes)
}

#[tauri::command]
pub async fn apply_theme(state: State<'_, AppState>, theme_name: String) -> Result<()> {
    let app = state.read().await;
    let mut config = app.config.write().await;
    config.ui.theme = theme_name;
    config.save().await?;
    Ok(())
}

#[tauri::command]
pub async fn export_config(state: State<'_, AppState>, file_path: String) -> Result<()> {
    let app = state.read().await;
    let config = app.config.read().await;
    let content = serde_yaml::to_string(&*config)
        .map_err(|e| HoverShellError::Serialization(e.to_string()))?;
    tokio::fs::write(&file_path, content).await?;
    Ok(())
}

#[tauri::command]
pub async fn import_config(state: State<'_, AppState>, file_path: String) -> Result<()> {
    let content = tokio::fs::read_to_string(&file_path).await?;
    let config: Config = serde_yaml::from_str(&content)
        .map_err(|e| HoverShellError::Parse(e.to_string()))?;
    
    let app = state.read().await;
    let mut app_config = app.config.write().await;
    *app_config = config;
    app_config.save().await?;
    Ok(())
}

// File Operations Commands
#[tauri::command]
pub async fn list_directory(path: String, recursive: bool) -> Result<Vec<FileInfo>> {
    let file_ops = FileOperations::new();
    file_ops.list_directory(&path, recursive).await
}

#[tauri::command]
pub async fn copy_file(source: String, destination: String, recursive: bool) -> Result<()> {
    let file_ops = FileOperations::new();
    file_ops.copy(&source, &destination, recursive).await
}

#[tauri::command]
pub async fn move_file(source: String, destination: String) -> Result<()> {
    let file_ops = FileOperations::new();
    file_ops.move_file(&source, &destination).await
}

#[tauri::command]
pub async fn delete_file(path: String, recursive: bool) -> Result<()> {
    let file_ops = FileOperations::new();
    file_ops.delete(&path, recursive).await
}

#[tauri::command]
pub async fn find_files(directory: String, pattern: String, case_sensitive: bool) -> Result<Vec<FileInfo>> {
    let file_ops = FileOperations::new();
    file_ops.find_files(&directory, &pattern, case_sensitive).await
}

#[tauri::command]
pub async fn search_in_files(directory: String, query: String, file_pattern: Option<String>, case_sensitive: bool) -> Result<Vec<SearchResult>> {
    let file_ops = FileOperations::new();
    file_ops.search_in_files(&directory, &query, file_pattern.as_deref(), case_sensitive).await
}

#[tauri::command]
pub async fn get_directory_stats(directory: String) -> Result<DirectoryStats> {
    let file_ops = FileOperations::new();
    file_ops.get_directory_stats(&directory).await
}

#[tauri::command]
pub async fn create_directory(path: String, parents: bool) -> Result<()> {
    let file_ops = FileOperations::new();
    file_ops.create_directory(&path, parents).await
}

#[tauri::command]
pub async fn create_file_with_content(path: String, content: String) -> Result<()> {
    let file_ops = FileOperations::new();
    file_ops.create_file(&path, &content).await
}

#[tauri::command]
pub async fn read_file_content(path: String) -> Result<String> {
    let file_ops = FileOperations::new();
    file_ops.read_file(&path).await
}

#[tauri::command]
pub async fn write_file_content(path: String, content: String, append: bool) -> Result<()> {
    let file_ops = FileOperations::new();
    file_ops.write_file(&path, &content, append).await
}

// Git Operations Commands
#[tauri::command]
pub async fn git_status(repo_path: String) -> Result<GitStatus> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.get_status().await
}

#[tauri::command]
pub async fn git_branches(repo_path: String) -> Result<Vec<GitBranch>> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.get_branches().await
}

#[tauri::command]
pub async fn git_commits(repo_path: String, limit: Option<usize>) -> Result<Vec<GitCommit>> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.get_commits(limit).await
}

#[tauri::command]
pub async fn git_diff(repo_path: String, file_path: Option<String>) -> Result<Vec<GitDiff>> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.get_diff(file_path.as_deref()).await
}

#[tauri::command]
pub async fn git_staged_diff(repo_path: String) -> Result<Vec<GitDiff>> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.get_staged_diff().await
}

#[tauri::command]
pub async fn git_add_files(repo_path: String, files: Vec<String>) -> Result<()> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.add_files(&files).await
}

#[tauri::command]
pub async fn git_commit(repo_path: String, message: String) -> Result<String> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.commit(&message).await
}

#[tauri::command]
pub async fn git_create_branch(repo_path: String, branch_name: String, checkout: bool) -> Result<()> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.create_branch(&branch_name, checkout).await
}

#[tauri::command]
pub async fn git_checkout_branch(repo_path: String, branch_name: String) -> Result<()> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.checkout_branch(&branch_name).await
}

#[tauri::command]
pub async fn git_pull(repo_path: String, branch: Option<String>) -> Result<String> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.pull(branch.as_deref()).await
}

#[tauri::command]
pub async fn git_push(repo_path: String, branch: Option<String>, upstream: bool) -> Result<String> {
    let git_ops = GitOperations::new(&repo_path);
    git_ops.push(branch.as_deref(), upstream).await
}

// System Monitoring Commands
#[tauri::command]
pub async fn get_system_info_detailed() -> Result<SystemInfo> {
    let mut monitor = SystemMonitor::new();
    monitor.get_system_info()
}

#[tauri::command]
pub async fn get_processes(limit: Option<usize>) -> Result<Vec<ProcessInfo>> {
    let mut monitor = SystemMonitor::new();
    monitor.get_processes(limit)
}

#[tauri::command]
pub async fn get_process_by_pid(pid: u32) -> Result<Option<ProcessInfo>> {
    let mut monitor = SystemMonitor::new();
    monitor.get_process(pid)
}

#[tauri::command]
pub async fn kill_process_by_pid(pid: u32, signal: Option<i32>) -> Result<()> {
    let monitor = SystemMonitor::new();
    monitor.kill_process(pid, signal)
}

#[tauri::command]
pub async fn get_disk_info() -> Result<Vec<DiskInfo>> {
    let mut monitor = SystemMonitor::new();
    monitor.get_disk_info()
}

#[tauri::command]
pub async fn get_network_interfaces() -> Result<Vec<NetworkInterface>> {
    let mut monitor = SystemMonitor::new();
    monitor.get_network_interfaces()
}

#[tauri::command]
pub async fn get_network_connections() -> Result<Vec<NetworkConnection>> {
    let monitor = SystemMonitor::new();
    monitor.get_network_connections()
}

#[tauri::command]
pub async fn get_top_processes_by_cpu(limit: usize) -> Result<Vec<ProcessInfo>> {
    let mut monitor = SystemMonitor::new();
    monitor.get_top_processes_by_cpu(limit)
}

#[tauri::command]
pub async fn get_top_processes_by_memory(limit: usize) -> Result<Vec<ProcessInfo>> {
    let mut monitor = SystemMonitor::new();
    monitor.get_top_processes_by_memory(limit)
}

// Text Processing Commands
#[tauri::command]
pub async fn grep_text(pattern: String, files: Vec<String>, options: GrepOptions) -> Result<Vec<GrepResult>> {
    let processor = TextProcessor::new();
    processor.grep(&pattern, &files, &options).await
}

#[tauri::command]
pub async fn sort_text(input: String, options: SortOptions) -> Result<String> {
    let processor = TextProcessor::new();
    processor.sort(&input, &options).await
}

#[tauri::command]
pub async fn sed_text(input: String, pattern: String, replacement: String, options: SedOptions) -> Result<String> {
    let processor = TextProcessor::new();
    processor.sed(&input, &pattern, &replacement, &options).await
}

#[tauri::command]
pub async fn awk_text(input: String, script: String, options: AwkOptions) -> Result<String> {
    let processor = TextProcessor::new();
    processor.awk(&input, &script, &options).await
}

#[tauri::command]
pub async fn wc_text(input: String) -> Result<WcResult> {
    let processor = TextProcessor::new();
    processor.wc(&input).await
}

#[tauri::command]
pub async fn uniq_text(input: String, case_insensitive: bool) -> Result<String> {
    let processor = TextProcessor::new();
    processor.uniq(&input, case_insensitive).await
}

#[tauri::command]
pub async fn cut_text(input: String, delimiter: String, fields: Vec<usize>) -> Result<String> {
    let processor = TextProcessor::new();
    processor.cut(&input, &delimiter, &fields).await
}

#[tauri::command]
pub async fn join_text(input: String, delimiter: String) -> Result<String> {
    let processor = TextProcessor::new();
    processor.join(&input, &delimiter).await
}

#[tauri::command]
pub async fn text_to_uppercase(input: String) -> Result<String> {
    let processor = TextProcessor::new();
    processor.to_uppercase(&input).await
}

#[tauri::command]
pub async fn text_to_lowercase(input: String) -> Result<String> {
    let processor = TextProcessor::new();
    processor.to_lowercase(&input).await
}

#[tauri::command]
pub async fn text_capitalize(input: String) -> Result<String> {
    let processor = TextProcessor::new();
    processor.capitalize(&input).await
}

#[tauri::command]
pub async fn text_reverse(input: String) -> Result<String> {
    let processor = TextProcessor::new();
    processor.reverse(&input).await
}

#[tauri::command]
pub async fn text_truncate(input: String, length: usize, suffix: Option<String>) -> Result<String> {
    let processor = TextProcessor::new();
    processor.truncate(&input, length, suffix.as_deref()).await
}

#[tauri::command]
pub async fn text_trim(input: String) -> Result<String> {
    let processor = TextProcessor::new();
    processor.trim(&input).await
}

#[tauri::command]
pub async fn text_replace(input: String, from: String, to: String) -> Result<String> {
    let processor = TextProcessor::new();
    processor.replace(&input, &from, &to).await
}

// Network Tools Commands
#[tauri::command]
pub async fn ping_host(host: String, count: Option<u32>) -> Result<PingResult> {
    let network_tools = NetworkTools::new();
    network_tools.ping(&host, count).await
}

#[tauri::command]
pub async fn scan_ports(host: String, ports: Vec<u16>, timeout_ms: Option<u64>) -> Result<Vec<PortScanResult>> {
    let network_tools = NetworkTools::new();
    network_tools.scan_ports(&host, &ports, timeout_ms).await
}

#[tauri::command]
pub async fn http_request(request: HttpRequest) -> Result<HttpResponse> {
    let network_tools = NetworkTools::new();
    network_tools.http_request(&request).await
}

#[tauri::command]
pub async fn download_file(url: String, output_path: String) -> Result<usize> {
    let network_tools = NetworkTools::new();
    network_tools.download_file(&url, &output_path).await
}

#[tauri::command]
pub async fn dns_lookup(hostname: String) -> Result<DnsLookupResult> {
    let network_tools = NetworkTools::new();
    network_tools.dns_lookup(&hostname).await
}

#[tauri::command]
pub async fn traceroute_host(host: String, max_hops: Option<u8>) -> Result<TracerouteResult> {
    let network_tools = NetworkTools::new();
    network_tools.traceroute(&host, max_hops).await
}

#[tauri::command]
pub async fn is_host_reachable(host: String, timeout_ms: Option<u64>) -> Result<bool> {
    let network_tools = NetworkTools::new();
    network_tools.is_reachable(&host, timeout_ms).await
}

#[tauri::command]
pub async fn get_local_ip() -> Result<String> {
    let network_tools = NetworkTools::new();
    network_tools.get_local_ip().await
}

// Database Tools Commands
#[tauri::command]
pub async fn add_database_connection(connection: DatabaseConnection) -> Result<()> {
    let mut manager = DatabaseManager::new();
    manager.add_connection(connection)
}

#[tauri::command]
pub async fn remove_database_connection(connection_id: String) -> Result<()> {
    let mut manager = DatabaseManager::new();
    manager.remove_connection(&connection_id)
}

#[tauri::command]
pub async fn get_database_connections() -> Result<Vec<DatabaseConnection>> {
    let manager = DatabaseManager::new();
    Ok(manager.get_connections().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn test_database_connection(connection_id: String) -> Result<bool> {
    let manager = DatabaseManager::new();
    manager.test_connection(&connection_id).await
}

#[tauri::command]
pub async fn execute_database_query(connection_id: String, query: String) -> Result<QueryResult> {
    let manager = DatabaseManager::new();
    manager.execute_query(&connection_id, &query).await
}

#[tauri::command]
pub async fn get_database_info(connection_id: String) -> Result<DatabaseInfo> {
    let manager = DatabaseManager::new();
    manager.get_database_info(&connection_id).await
}

#[tauri::command]
pub async fn get_database_tables(connection_id: String) -> Result<Vec<TableInfo>> {
    let manager = DatabaseManager::new();
    manager.get_tables(&connection_id).await
}

#[tauri::command]
pub async fn get_database_table_schema(connection_id: String, table_name: String) -> Result<TableInfo> {
    let manager = DatabaseManager::new();
    manager.get_table_schema(&connection_id, &table_name).await
}

// Docker Tools Commands
#[tauri::command]
pub async fn is_docker_available() -> Result<bool> {
    let docker_manager = DockerManager::new();
    Ok(docker_manager.is_docker_available().await)
}

#[tauri::command]
pub async fn is_docker_compose_available() -> Result<bool> {
    let docker_manager = DockerManager::new();
    Ok(docker_manager.is_compose_available().await)
}

#[tauri::command]
pub async fn get_docker_system_info() -> Result<std::collections::HashMap<String, String>> {
    let docker_manager = DockerManager::new();
    docker_manager.get_system_info().await
}

#[tauri::command]
pub async fn list_docker_containers(all: bool) -> Result<Vec<DockerContainer>> {
    let docker_manager = DockerManager::new();
    docker_manager.list_containers(all).await
}

#[tauri::command]
pub async fn start_docker_container(container_id: String) -> Result<()> {
    let docker_manager = DockerManager::new();
    docker_manager.start_container(&container_id).await
}

#[tauri::command]
pub async fn stop_docker_container(container_id: String, timeout: Option<u32>) -> Result<()> {
    let docker_manager = DockerManager::new();
    docker_manager.stop_container(&container_id, timeout).await
}

#[tauri::command]
pub async fn remove_docker_container(container_id: String, force: bool) -> Result<()> {
    let docker_manager = DockerManager::new();
    docker_manager.remove_container(&container_id, force).await
}

#[tauri::command]
pub async fn get_docker_container_logs(container_id: String, tail: Option<usize>, follow: bool) -> Result<String> {
    let docker_manager = DockerManager::new();
    docker_manager.get_container_logs(&container_id, tail, follow).await
}

#[tauri::command]
pub async fn list_docker_images(all: bool) -> Result<Vec<DockerImage>> {
    let docker_manager = DockerManager::new();
    docker_manager.list_images(all).await
}

#[tauri::command]
pub async fn pull_docker_image(image_name: String) -> Result<()> {
    let docker_manager = DockerManager::new();
    docker_manager.pull_image(&image_name).await
}

#[tauri::command]
pub async fn remove_docker_image(image_id: String, force: bool) -> Result<()> {
    let docker_manager = DockerManager::new();
    docker_manager.remove_image(&image_id, force).await
}

#[tauri::command]
pub async fn list_docker_volumes() -> Result<Vec<DockerVolume>> {
    let docker_manager = DockerManager::new();
    docker_manager.list_volumes().await
}

#[tauri::command]
pub async fn list_docker_networks() -> Result<Vec<DockerNetwork>> {
    let docker_manager = DockerManager::new();
    docker_manager.list_networks().await
}

#[tauri::command]
pub async fn run_docker_container(image: String, command: Option<String>, options: RunOptions) -> Result<String> {
    let docker_manager = DockerManager::new();
    docker_manager.run_container(&image, command.as_deref(), &options).await
}

#[tauri::command]
pub async fn exec_docker_command(container_id: String, command: String, interactive: bool) -> Result<String> {
    let docker_manager = DockerManager::new();
    docker_manager.exec_command(&container_id, &command, interactive).await
}

#[tauri::command]
pub async fn docker_compose_up(project_path: String, services: Option<Vec<String>>) -> Result<()> {
    let docker_manager = DockerManager::new();
    let service_refs: Option<Vec<&str>> = services.as_ref().map(|s| s.iter().map(|s| s.as_str()).collect());
    docker_manager.compose_up(&project_path, service_refs.as_deref()).await
}

#[tauri::command]
pub async fn docker_compose_down(project_path: String) -> Result<()> {
    let docker_manager = DockerManager::new();
    docker_manager.compose_down(&project_path).await
}

#[tauri::command]
pub async fn docker_compose_ps(project_path: String) -> Result<Vec<DockerComposeService>> {
    let docker_manager = DockerManager::new();
    docker_manager.compose_ps(&project_path).await
}

#[tauri::command]
pub async fn get_docker_system_usage() -> Result<std::collections::HashMap<String, String>> {
    let docker_manager = DockerManager::new();
    docker_manager.get_system_usage().await
}

// Package Manager Commands
#[tauri::command]
pub async fn check_available_package_managers() -> Result<Vec<PackageManagerInfo>> {
    let package_tools = PackageManagerTools::new();
    Ok(package_tools.check_available_managers().await)
}

#[tauri::command]
pub async fn install_package_with_manager(manager: PackageManager, package: String, options: InstallOptions) -> Result<()> {
    let package_tools = PackageManagerTools::new();
    package_tools.install_package(&manager, &package, &options).await
}

#[tauri::command]
pub async fn uninstall_package_with_manager(manager: PackageManager, package: String, global: bool) -> Result<()> {
    let package_tools = PackageManagerTools::new();
    package_tools.uninstall_package(&manager, &package, global).await
}

#[tauri::command]
pub async fn list_installed_packages_with_manager(manager: PackageManager, global: bool) -> Result<Vec<Package>> {
    let package_tools = PackageManagerTools::new();
    package_tools.list_installed_packages(&manager, global).await
}

#[tauri::command]
pub async fn search_packages_with_manager(manager: PackageManager, query: String, limit: Option<usize>) -> Result<Vec<SearchResult>> {
    let package_tools = PackageManagerTools::new();
    package_tools.search_packages(&manager, &query, limit).await
}

#[tauri::command]
pub async fn update_packages_with_manager(manager: PackageManager, packages: Option<Vec<String>>) -> Result<()> {
    let package_tools = PackageManagerTools::new();
    let package_refs: Option<Vec<&str>> = packages.as_ref().map(|s| s.iter().map(|s| s.as_str()).collect());
    package_tools.update_packages(&manager, package_refs.as_deref()).await
}

#[tauri::command]
pub async fn get_package_info_with_manager(manager: PackageManager, package: String) -> Result<Package> {
    let package_tools = PackageManagerTools::new();
    package_tools.get_package_info(&manager, &package).await
}

#[tauri::command]
pub async fn check_outdated_packages_with_manager(manager: PackageManager, global: bool) -> Result<Vec<Package>> {
    let package_tools = PackageManagerTools::new();
    package_tools.check_outdated_packages(&manager, global).await
}

#[tauri::command]
pub async fn init_project_with_manager(manager: PackageManager, project_path: String, project_name: Option<String>) -> Result<()> {
    let package_tools = PackageManagerTools::new();
    package_tools.init_project(&manager, &project_path, project_name.as_deref()).await
}