use crate::error::{HoverShellError, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::process::Command as AsyncCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageManager {
    NPM,
    Yarn,
    PNPM,
    Pip,
    Pipenv,
    Poetry,
    Cargo,
    Brew,
    Apt,
    Yum,
    Pacman,
    Snap,
    Flatpak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub installed: bool,
    pub latest_version: Option<String>,
    pub outdated: bool,
    pub dependencies: Vec<String>,
    pub size: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManagerInfo {
    pub manager: PackageManager,
    pub version: String,
    pub available: bool,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallOptions {
    pub global: bool,
    pub dev: bool,
    pub exact: bool,
    pub force: bool,
    pub save: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub name: String,
    pub version: String,
    pub description: String,
    pub downloads: Option<u64>,
    pub stars: Option<u32>,
    pub homepage: Option<String>,
}

pub struct PackageManagerTools {
    managers: HashMap<PackageManager, String>,
}

impl PackageManagerTools {
    pub fn new() -> Self {
        let mut managers = HashMap::new();
        
        // Common package manager commands
        managers.insert(PackageManager::NPM, "npm".to_string());
        managers.insert(PackageManager::Yarn, "yarn".to_string());
        managers.insert(PackageManager::PNPM, "pnpm".to_string());
        managers.insert(PackageManager::Pip, "pip".to_string());
        managers.insert(PackageManager::Pipenv, "pipenv".to_string());
        managers.insert(PackageManager::Poetry, "poetry".to_string());
        managers.insert(PackageManager::Cargo, "cargo".to_string());
        managers.insert(PackageManager::Brew, "brew".to_string());
        managers.insert(PackageManager::Apt, "apt".to_string());
        managers.insert(PackageManager::Yum, "yum".to_string());
        managers.insert(PackageManager::Pacman, "pacman".to_string());
        managers.insert(PackageManager::Snap, "snap".to_string());
        managers.insert(PackageManager::Flatpak, "flatpak".to_string());

        Self { managers }
    }

    /// Check which package managers are available
    pub async fn check_available_managers(&self) -> Vec<PackageManagerInfo> {
        let mut available = Vec::new();

        for (manager, command) in &self.managers {
            let version = self.get_manager_version(command).await.unwrap_or_default();
            let available = self.is_manager_available(command).await;
            let path = if available {
                self.get_manager_path(command).await
            } else {
                None
            };

            available.push(PackageManagerInfo {
                manager: manager.clone(),
                version,
                available,
                path,
            });
        }

        available
    }

    /// Install a package
    pub async fn install_package(&self, manager: &PackageManager, package: &str, options: &InstallOptions) -> Result<()> {
        let command = self.managers.get(manager)
            .ok_or_else(|| HoverShellError::PackageManager(format!("Package manager {:?} not supported", manager)))?;

        let mut args = self.build_install_args(manager, package, options)?;

        let output = AsyncCommand::new(command)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::PackageManager(format!("Failed to install package: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::PackageManager(format!("Failed to install package '{}': {}", package, error_msg)));
        }

        info!("Installed package '{}' using {:?}", package, manager);
        Ok(())
    }

    /// Uninstall a package
    pub async fn uninstall_package(&self, manager: &PackageManager, package: &str, global: bool) -> Result<()> {
        let command = self.managers.get(manager)
            .ok_or_else(|| HoverShellError::PackageManager(format!("Package manager {:?} not supported", manager)))?;

        let mut args = self.build_uninstall_args(manager, package, global)?;

        let output = AsyncCommand::new(command)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::PackageManager(format!("Failed to uninstall package: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::PackageManager(format!("Failed to uninstall package '{}': {}", package, error_msg)));
        }

        info!("Uninstalled package '{}' using {:?}", package, manager);
        Ok(())
    }

    /// List installed packages
    pub async fn list_installed_packages(&self, manager: &PackageManager, global: bool) -> Result<Vec<Package>> {
        let command = self.managers.get(manager)
            .ok_or_else(|| HoverShellError::PackageManager(format!("Package manager {:?} not supported", manager)))?;

        let args = self.build_list_args(manager, global)?;

        let output = AsyncCommand::new(command)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::PackageManager(format!("Failed to list packages: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::PackageManager(format!("Failed to list packages: {}", error_msg)));
        }

        let packages = self.parse_package_list(manager, &String::from_utf8_lossy(&output.stdout))?;
        info!("Listed {} packages using {:?}", packages.len(), manager);
        Ok(packages)
    }

    /// Search for packages
    pub async fn search_packages(&self, manager: &PackageManager, query: &str, limit: Option<usize>) -> Result<Vec<SearchResult>> {
        let command = self.managers.get(manager)
            .ok_or_else(|| HoverShellError::PackageManager(format!("Package manager {:?} not supported", manager)))?;

        let args = self.build_search_args(manager, query, limit)?;

        let output = AsyncCommand::new(command)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::PackageManager(format!("Failed to search packages: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::PackageManager(format!("Failed to search packages: {}", error_msg)));
        }

        let results = self.parse_search_results(manager, &String::from_utf8_lossy(&output.stdout))?;
        info!("Found {} packages for query '{}' using {:?}", results.len(), query, manager);
        Ok(results)
    }

    /// Update packages
    pub async fn update_packages(&self, manager: &PackageManager, packages: Option<Vec<&str>>) -> Result<()> {
        let command = self.managers.get(manager)
            .ok_or_else(|| HoverShellError::PackageManager(format!("Package manager {:?} not supported", manager)))?;

        let args = self.build_update_args(manager, packages)?;

        let output = AsyncCommand::new(command)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::PackageManager(format!("Failed to update packages: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::PackageManager(format!("Failed to update packages: {}", error_msg)));
        }

        info!("Updated packages using {:?}", manager);
        Ok(())
    }

    /// Get package information
    pub async fn get_package_info(&self, manager: &PackageManager, package: &str) -> Result<Package> {
        let command = self.managers.get(manager)
            .ok_or_else(|| HoverShellError::PackageManager(format!("Package manager {:?} not supported", manager)))?;

        let args = self.build_info_args(manager, package)?;

        let output = AsyncCommand::new(command)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::PackageManager(format!("Failed to get package info: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::PackageManager(format!("Failed to get info for package '{}': {}", package, error_msg)));
        }

        let package_info = self.parse_package_info(manager, &String::from_utf8_lossy(&output.stdout))?;
        info!("Retrieved info for package '{}' using {:?}", package, manager);
        Ok(package_info)
    }

    /// Check for outdated packages
    pub async fn check_outdated_packages(&self, manager: &PackageManager, global: bool) -> Result<Vec<Package>> {
        let command = self.managers.get(manager)
            .ok_or_else(|| HoverShellError::PackageManager(format!("Package manager {:?} not supported", manager)))?;

        let args = self.build_outdated_args(manager, global)?;

        let output = AsyncCommand::new(command)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::PackageManager(format!("Failed to check outdated packages: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::PackageManager(format!("Failed to check outdated packages: {}", error_msg)));
        }

        let packages = self.parse_outdated_packages(manager, &String::from_utf8_lossy(&output.stdout))?;
        info!("Found {} outdated packages using {:?}", packages.len(), manager);
        Ok(packages)
    }

    /// Initialize a new project
    pub async fn init_project(&self, manager: &PackageManager, project_path: &str, project_name: Option<&str>) -> Result<()> {
        let command = self.managers.get(manager)
            .ok_or_else(|| HoverShellError::PackageManager(format!("Package manager {:?} not supported", manager)))?;

        let args = self.build_init_args(manager, project_name)?;

        let output = AsyncCommand::new(command)
            .current_dir(project_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::PackageManager(format!("Failed to initialize project: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::PackageManager(format!("Failed to initialize project: {}", error_msg)));
        }

        info!("Initialized project using {:?} in {}", manager, project_path);
        Ok(())
    }

    /// Build install arguments based on package manager
    fn build_install_args(&self, manager: &PackageManager, package: &str, options: &InstallOptions) -> Result<Vec<String>> {
        let mut args = Vec::new();

        match manager {
            PackageManager::NPM => {
                args.push("install".to_string());
                if options.global {
                    args.push("-g".to_string());
                }
                if options.dev {
                    args.push("--save-dev".to_string());
                }
                if options.exact {
                    args.push("--save-exact".to_string());
                }
                if options.force {
                    args.push("--force".to_string());
                }
                args.push(package.to_string());
            }
            PackageManager::Yarn => {
                if options.global {
                    args.push("global".to_string());
                }
                args.push("add".to_string());
                if options.dev {
                    args.push("--dev".to_string());
                }
                if options.exact {
                    args.push("--exact".to_string());
                }
                args.push(package.to_string());
            }
            PackageManager::PNPM => {
                if options.global {
                    args.push("add".to_string());
                    args.push("-g".to_string());
                } else {
                    args.push("add".to_string());
                }
                if options.dev {
                    args.push("--save-dev".to_string());
                }
                args.push(package.to_string());
            }
            PackageManager::Pip => {
                args.push("install".to_string());
                if options.global {
                    args.push("--user".to_string());
                }
                if options.force {
                    args.push("--force-reinstall".to_string());
                }
                args.push(package.to_string());
            }
            PackageManager::Cargo => {
                args.push("install".to_string());
                args.push(package.to_string());
            }
            PackageManager::Brew => {
                args.push("install".to_string());
                args.push(package.to_string());
            }
            _ => {
                return Err(HoverShellError::PackageManager(format!("Install not supported for {:?}", manager)));
            }
        }

        Ok(args)
    }

    /// Build uninstall arguments based on package manager
    fn build_uninstall_args(&self, manager: &PackageManager, package: &str, global: bool) -> Result<Vec<String>> {
        let mut args = Vec::new();

        match manager {
            PackageManager::NPM => {
                args.push("uninstall".to_string());
                if global {
                    args.push("-g".to_string());
                }
                args.push(package.to_string());
            }
            PackageManager::Yarn => {
                if global {
                    args.push("global".to_string());
                }
                args.push("remove".to_string());
                args.push(package.to_string());
            }
            PackageManager::PNPM => {
                args.push("remove".to_string());
                if global {
                    args.push("-g".to_string());
                }
                args.push(package.to_string());
            }
            PackageManager::Pip => {
                args.push("uninstall".to_string());
                args.push("-y".to_string());
                args.push(package.to_string());
            }
            PackageManager::Cargo => {
                args.push("uninstall".to_string());
                args.push(package.to_string());
            }
            PackageManager::Brew => {
                args.push("uninstall".to_string());
                args.push(package.to_string());
            }
            _ => {
                return Err(HoverShellError::PackageManager(format!("Uninstall not supported for {:?}", manager)));
            }
        }

        Ok(args)
    }

    /// Build list arguments based on package manager
    fn build_list_args(&self, manager: &PackageManager, global: bool) -> Result<Vec<String>> {
        let mut args = Vec::new();

        match manager {
            PackageManager::NPM => {
                args.push("list".to_string());
                if global {
                    args.push("-g".to_string());
                }
                args.push("--depth=0".to_string());
            }
            PackageManager::Yarn => {
                if global {
                    args.push("global".to_string());
                }
                args.push("list".to_string());
            }
            PackageManager::PNPM => {
                args.push("list".to_string());
                if global {
                    args.push("-g".to_string());
                }
            }
            PackageManager::Pip => {
                args.push("list".to_string());
            }
            PackageManager::Cargo => {
                args.push("install".to_string());
                args.push("--list".to_string());
            }
            PackageManager::Brew => {
                args.push("list".to_string());
            }
            _ => {
                return Err(HoverShellError::PackageManager(format!("List not supported for {:?}", manager)));
            }
        }

        Ok(args)
    }

    /// Build search arguments based on package manager
    fn build_search_args(&self, manager: &PackageManager, query: &str, limit: Option<usize>) -> Result<Vec<String>> {
        let mut args = Vec::new();

        match manager {
            PackageManager::NPM => {
                args.push("search".to_string());
                if let Some(limit_count) = limit {
                    args.push(format!("--limit={}", limit_count));
                }
                args.push(query.to_string());
            }
            PackageManager::Yarn => {
                args.push("search".to_string());
                args.push(query.to_string());
            }
            PackageManager::PNPM => {
                args.push("search".to_string());
                args.push(query.to_string());
            }
            PackageManager::Pip => {
                args.push("search".to_string());
                args.push(query.to_string());
            }
            PackageManager::Cargo => {
                args.push("search".to_string());
                args.push(query.to_string());
            }
            PackageManager::Brew => {
                args.push("search".to_string());
                args.push(query.to_string());
            }
            _ => {
                return Err(HoverShellError::PackageManager(format!("Search not supported for {:?}", manager)));
            }
        }

        Ok(args)
    }

    /// Build update arguments based on package manager
    fn build_update_args(&self, manager: &PackageManager, packages: Option<Vec<&str>>) -> Result<Vec<String>> {
        let mut args = Vec::new();

        match manager {
            PackageManager::NPM => {
                args.push("update".to_string());
                if let Some(package_list) = packages {
                    args.extend(package_list.iter().map(|s| s.to_string()));
                }
            }
            PackageManager::Yarn => {
                args.push("upgrade".to_string());
                if let Some(package_list) = packages {
                    args.extend(package_list.iter().map(|s| s.to_string()));
                }
            }
            PackageManager::PNPM => {
                args.push("update".to_string());
                if let Some(package_list) = packages {
                    args.extend(package_list.iter().map(|s| s.to_string()));
                }
            }
            PackageManager::Pip => {
                args.push("install".to_string());
                args.push("--upgrade".to_string());
                if let Some(package_list) = packages {
                    args.extend(package_list.iter().map(|s| s.to_string()));
                }
            }
            PackageManager::Cargo => {
                args.push("update".to_string());
            }
            PackageManager::Brew => {
                args.push("upgrade".to_string());
                if let Some(package_list) = packages {
                    args.extend(package_list.iter().map(|s| s.to_string()));
                }
            }
            _ => {
                return Err(HoverShellError::PackageManager(format!("Update not supported for {:?}", manager)));
            }
        }

        Ok(args)
    }

    /// Build info arguments based on package manager
    fn build_info_args(&self, manager: &PackageManager, package: &str) -> Result<Vec<String>> {
        let mut args = Vec::new();

        match manager {
            PackageManager::NPM => {
                args.push("info".to_string());
                args.push(package.to_string());
            }
            PackageManager::Yarn => {
                args.push("info".to_string());
                args.push(package.to_string());
            }
            PackageManager::PNPM => {
                args.push("info".to_string());
                args.push(package.to_string());
            }
            PackageManager::Pip => {
                args.push("show".to_string());
                args.push(package.to_string());
            }
            PackageManager::Cargo => {
                args.push("search".to_string());
                args.push(package.to_string());
            }
            PackageManager::Brew => {
                args.push("info".to_string());
                args.push(package.to_string());
            }
            _ => {
                return Err(HoverShellError::PackageManager(format!("Info not supported for {:?}", manager)));
            }
        }

        Ok(args)
    }

    /// Build outdated arguments based on package manager
    fn build_outdated_args(&self, manager: &PackageManager, global: bool) -> Result<Vec<String>> {
        let mut args = Vec::new();

        match manager {
            PackageManager::NPM => {
                args.push("outdated".to_string());
                if global {
                    args.push("-g".to_string());
                }
            }
            PackageManager::Yarn => {
                args.push("outdated".to_string());
            }
            PackageManager::PNPM => {
                args.push("outdated".to_string());
                if global {
                    args.push("-g".to_string());
                }
            }
            PackageManager::Pip => {
                args.push("list".to_string());
                args.push("--outdated".to_string());
            }
            PackageManager::Brew => {
                args.push("outdated".to_string());
            }
            _ => {
                return Err(HoverShellError::PackageManager(format!("Outdated not supported for {:?}", manager)));
            }
        }

        Ok(args)
    }

    /// Build init arguments based on package manager
    fn build_init_args(&self, manager: &PackageManager, project_name: Option<&str>) -> Result<Vec<String>> {
        let mut args = Vec::new();

        match manager {
            PackageManager::NPM => {
                args.push("init".to_string());
                args.push("-y".to_string());
            }
            PackageManager::Yarn => {
                args.push("init".to_string());
                if let Some(name) = project_name {
                    args.push("-y".to_string());
                }
            }
            PackageManager::PNPM => {
                args.push("init".to_string());
            }
            PackageManager::Pipenv => {
                args.push("install".to_string());
            }
            PackageManager::Poetry => {
                args.push("init".to_string());
                if let Some(name) = project_name {
                    args.push("--name".to_string());
                    args.push(name.to_string());
                }
            }
            PackageManager::Cargo => {
                args.push("init".to_string());
                if let Some(name) = project_name {
                    args.push("--name".to_string());
                    args.push(name.to_string());
                }
            }
            _ => {
                return Err(HoverShellError::PackageManager(format!("Init not supported for {:?}", manager)));
            }
        }

        Ok(args)
    }

    /// Check if a package manager is available
    async fn is_manager_available(&self, command: &str) -> bool {
        let output = AsyncCommand::new(command)
            .arg("--version")
            .output()
            .await;

        match output {
            Ok(result) => result.status.success(),
            Err(_) => false,
        }
    }

    /// Get package manager version
    async fn get_manager_version(&self, command: &str) -> Option<String> {
        let output = AsyncCommand::new(command)
            .arg("--version")
            .output()
            .await
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }

    /// Get package manager path
    async fn get_manager_path(&self, command: &str) -> Option<String> {
        let output = AsyncCommand::new("which")
            .arg(command)
            .output()
            .await
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }

    /// Parse package list output
    fn parse_package_list(&self, manager: &PackageManager, output: &str) -> Result<Vec<Package>> {
        // TODO: Implement parsing for different package managers
        // This is a simplified implementation
        let mut packages = Vec::new();

        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            // Simple parsing - would need to be more sophisticated for each manager
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                packages.push(Package {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: None,
                    installed: true,
                    latest_version: None,
                    outdated: false,
                    dependencies: Vec::new(),
                    size: None,
                    homepage: None,
                    repository: None,
                });
            }
        }

        Ok(packages)
    }

    /// Parse search results
    fn parse_search_results(&self, manager: &PackageManager, output: &str) -> Result<Vec<SearchResult>> {
        // TODO: Implement parsing for different package managers
        let mut results = Vec::new();

        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            // Simple parsing - would need to be more sophisticated for each manager
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                results.push(SearchResult {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: parts.get(2).unwrap_or(&"").to_string(),
                    downloads: None,
                    stars: None,
                    homepage: None,
                });
            }
        }

        Ok(results)
    }

    /// Parse package info
    fn parse_package_info(&self, manager: &PackageManager, output: &str) -> Result<Package> {
        // TODO: Implement parsing for different package managers
        // This is a simplified implementation
        Ok(Package {
            name: "example".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Example package".to_string()),
            installed: false,
            latest_version: Some("1.0.0".to_string()),
            outdated: false,
            dependencies: Vec::new(),
            size: Some("1MB".to_string()),
            homepage: Some("https://example.com".to_string()),
            repository: Some("https://github.com/example/example".to_string()),
        })
    }

    /// Parse outdated packages
    fn parse_outdated_packages(&self, manager: &PackageManager, output: &str) -> Result<Vec<Package>> {
        // TODO: Implement parsing for different package managers
        let mut packages = Vec::new();

        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            // Simple parsing - would need to be more sophisticated for each manager
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                packages.push(Package {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: None,
                    installed: true,
                    latest_version: Some(parts[2].to_string()),
                    outdated: true,
                    dependencies: Vec::new(),
                    size: None,
                    homepage: None,
                    repository: None,
                });
            }
        }

        Ok(packages)
    }
}

impl Default for PackageManagerTools {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            global: false,
            dev: false,
            exact: false,
            force: false,
            save: true,
        }
    }
}