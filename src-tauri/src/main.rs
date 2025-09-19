use hovershell::{app::HoverShellApp, error::HoverShellError};
use log::{error, info};
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tokio::sync::RwLock;

type AppState = Arc<RwLock<HoverShellApp>>;

#[tokio::main]
async fn main() -> Result<(), HoverShellError> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting HoverShell v{}", env!("CARGO_PKG_VERSION"));

    // Initialize the application state
    let app_state = Arc::new(RwLock::new(HoverShellApp::new().await?));

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            hovershell::commands::toggle_window,
            hovershell::commands::show_window,
            hovershell::commands::hide_window,
            hovershell::commands::get_config,
            hovershell::commands::set_config,
            hovershell::commands::get_providers,
            hovershell::commands::add_provider,
            hovershell::commands::remove_provider,
            hovershell::commands::set_default_provider,
            hovershell::commands::execute_command,
            hovershell::commands::get_terminal_state,
            hovershell::commands::send_terminal_input,
            hovershell::commands::get_plugin_list,
            hovershell::commands::load_plugin,
            hovershell::commands::unload_plugin,
            hovershell::commands::register_hotkey,
            hovershell::commands::unregister_hotkey,
            hovershell::commands::get_menu_items,
            hovershell::commands::update_menu,
            hovershell::commands::show_notification,
            hovershell::commands::get_system_info,
            hovershell::commands::get_workspace_info,
            hovershell::commands::save_workspace_config,
            hovershell::commands::get_theme_list,
            hovershell::commands::apply_theme,
            hovershell::commands::export_config,
            hovershell::commands::import_config,
            // File Operations
            hovershell::commands::list_directory,
            hovershell::commands::copy_file,
            hovershell::commands::move_file,
            hovershell::commands::delete_file,
            hovershell::commands::find_files,
            hovershell::commands::search_in_files,
            hovershell::commands::get_directory_stats,
            hovershell::commands::create_directory,
            hovershell::commands::create_file_with_content,
            hovershell::commands::read_file_content,
            hovershell::commands::write_file_content,
            // Git Operations
            hovershell::commands::git_status,
            hovershell::commands::git_branches,
            hovershell::commands::git_commits,
            hovershell::commands::git_diff,
            hovershell::commands::git_staged_diff,
            hovershell::commands::git_add_files,
            hovershell::commands::git_commit,
            hovershell::commands::git_create_branch,
            hovershell::commands::git_checkout_branch,
            hovershell::commands::git_pull,
            hovershell::commands::git_push,
            // System Monitoring
            hovershell::commands::get_system_info_detailed,
            hovershell::commands::get_processes,
            hovershell::commands::get_process_by_pid,
            hovershell::commands::kill_process_by_pid,
            hovershell::commands::get_disk_info,
            hovershell::commands::get_network_interfaces,
            hovershell::commands::get_network_connections,
            hovershell::commands::get_top_processes_by_cpu,
            hovershell::commands::get_top_processes_by_memory,
            // Text Processing
            hovershell::commands::grep_text,
            hovershell::commands::sort_text,
            hovershell::commands::sed_text,
            hovershell::commands::awk_text,
            hovershell::commands::wc_text,
            hovershell::commands::uniq_text,
            hovershell::commands::cut_text,
            hovershell::commands::join_text,
            hovershell::commands::text_to_uppercase,
            hovershell::commands::text_to_lowercase,
            hovershell::commands::text_capitalize,
            hovershell::commands::text_reverse,
            hovershell::commands::text_truncate,
            hovershell::commands::text_trim,
            hovershell::commands::text_replace,
            // Network Tools
            hovershell::commands::ping_host,
            hovershell::commands::scan_ports,
            hovershell::commands::http_request,
            hovershell::commands::download_file,
            hovershell::commands::dns_lookup,
            hovershell::commands::traceroute_host,
            hovershell::commands::is_host_reachable,
            hovershell::commands::get_local_ip,
            // Database Tools
            hovershell::commands::add_database_connection,
            hovershell::commands::remove_database_connection,
            hovershell::commands::get_database_connections,
            hovershell::commands::test_database_connection,
            hovershell::commands::execute_database_query,
            hovershell::commands::get_database_info,
            hovershell::commands::get_database_tables,
            hovershell::commands::get_database_table_schema,
            // Docker Tools
            hovershell::commands::is_docker_available,
            hovershell::commands::is_docker_compose_available,
            hovershell::commands::get_docker_system_info,
            hovershell::commands::list_docker_containers,
            hovershell::commands::start_docker_container,
            hovershell::commands::stop_docker_container,
            hovershell::commands::remove_docker_container,
            hovershell::commands::get_docker_container_logs,
            hovershell::commands::list_docker_images,
            hovershell::commands::pull_docker_image,
            hovershell::commands::remove_docker_image,
            hovershell::commands::list_docker_volumes,
            hovershell::commands::list_docker_networks,
            hovershell::commands::run_docker_container,
            hovershell::commands::exec_docker_command,
            hovershell::commands::docker_compose_up,
            hovershell::commands::docker_compose_down,
            hovershell::commands::docker_compose_ps,
            hovershell::commands::get_docker_system_usage,
            // Package Manager Tools
            hovershell::commands::check_available_package_managers,
            hovershell::commands::install_package_with_manager,
            hovershell::commands::uninstall_package_with_manager,
            hovershell::commands::list_installed_packages_with_manager,
            hovershell::commands::search_packages_with_manager,
            hovershell::commands::update_packages_with_manager,
            hovershell::commands::get_package_info_with_manager,
            hovershell::commands::check_outdated_packages_with_manager,
            hovershell::commands::init_project_with_manager,
        ])
        .setup(|app| {
            let app_handle = app.handle();
            let state: State<AppState> = app.state();
            
            // Initialize the application
            tauri::async_runtime::spawn(async move {
                if let Ok(mut app) = state.write().await {
                    if let Err(e) = app.initialize(app_handle).await {
                        error!("Failed to initialize application: {}", e);
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Prevent closing, just hide the window
                    api.prevent_close();
                    if let Some(window) = event.window().get_webview_window() {
                        let _ = window.hide();
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}