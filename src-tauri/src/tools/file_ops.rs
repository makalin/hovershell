use crate::error::{HoverShellError, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub permissions: String,
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_path: String,
    pub line_number: usize,
    pub line_content: String,
    pub match_start: usize,
    pub match_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
    pub largest_file: Option<String>,
    pub file_types: HashMap<String, usize>,
}

pub struct FileOperations {
    // Configuration for file operations
    max_file_size: u64,
    exclude_patterns: Vec<String>,
    include_hidden: bool,
}

impl FileOperations {
    pub fn new() -> Self {
        Self {
            max_file_size: 100 * 1024 * 1024, // 100MB default
            exclude_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                ".DS_Store".to_string(),
                "target".to_string(),
            ],
            include_hidden: false,
        }
    }

    /// List files and directories in a given path
    pub async fn list_directory(&self, path: &str, recursive: bool) -> Result<Vec<FileInfo>> {
        let path = Path::new(path);
        
        if !path.exists() {
            return Err(HoverShellError::File(format!("Path does not exist: {}", path.display())));
        }

        if !path.is_dir() {
            return Err(HoverShellError::File(format!("Path is not a directory: {}", path.display())));
        }

        let mut files = Vec::new();

        if recursive {
            for entry in WalkDir::new(path)
                .max_depth(10) // Prevent infinite recursion
                .into_iter()
                .filter_entry(|e| self.should_include_entry(e))
            {
                let entry = entry.map_err(|e| HoverShellError::File(e.to_string()))?;
                if let Some(file_info) = self.get_file_info(&entry.path()).await? {
                    files.push(file_info);
                }
            }
        } else {
            let mut entries = async_fs::read_dir(path).await?;
            while let Some(entry) = entries.next_entry().await? {
                if let Some(file_info) = self.get_file_info(&entry.path()).await? {
                    files.push(file_info);
                }
            }
        }

        // Sort by name
        files.sort_by(|a, b| a.name.cmp(&b.name));
        
        info!("Listed {} items in directory: {}", files.len(), path.display());
        Ok(files)
    }

    /// Get detailed information about a file or directory
    pub async fn get_file_info(&self, path: &Path) -> Result<Option<FileInfo>> {
        if !path.exists() {
            return Ok(None);
        }

        let metadata = async_fs::metadata(path).await?;
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Check if we should exclude this file
        if !self.include_hidden && name.starts_with('.') {
            return Ok(None);
        }

        for pattern in &self.exclude_patterns {
            if name.contains(pattern) {
                return Ok(None);
            }
        }

        let permissions = format!("{:o}", metadata.permissions().mode());
        
        let modified = metadata.modified()
            .ok()
            .and_then(|t| chrono::DateTime::from_timestamp(
                t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64, 0
            ));

        let created = metadata.created()
            .ok()
            .and_then(|t| chrono::DateTime::from_timestamp(
                t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64, 0
            ));

        Ok(Some(FileInfo {
            path: path.to_string_lossy().to_string(),
            name,
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            is_symlink: metadata.file_type().is_symlink(),
            permissions,
            modified,
            created,
        }))
    }

    /// Copy files or directories
    pub async fn copy(&self, source: &str, destination: &str, recursive: bool) -> Result<()> {
        let source_path = Path::new(source);
        let dest_path = Path::new(destination);

        if !source_path.exists() {
            return Err(HoverShellError::File(format!("Source does not exist: {}", source)));
        }

        if source_path.is_file() {
            // Copy single file
            async_fs::copy(source_path, dest_path).await?;
            info!("Copied file: {} -> {}", source, destination);
        } else if source_path.is_dir() {
            if recursive {
                self.copy_directory_recursive(source_path, dest_path).await?;
                info!("Copied directory recursively: {} -> {}", source, destination);
            } else {
                return Err(HoverShellError::File("Cannot copy directory without recursive flag".to_string()));
            }
        }

        Ok(())
    }

    /// Move/rename files or directories
    pub async fn move_file(&self, source: &str, destination: &str) -> Result<()> {
        let source_path = Path::new(source);
        let dest_path = Path::new(destination);

        if !source_path.exists() {
            return Err(HoverShellError::File(format!("Source does not exist: {}", source)));
        }

        async_fs::rename(source_path, dest_path).await?;
        info!("Moved: {} -> {}", source, destination);
        Ok(())
    }

    /// Delete files or directories
    pub async fn delete(&self, path: &str, recursive: bool) -> Result<()> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(HoverShellError::File(format!("Path does not exist: {}", path.display())));
        }

        if path.is_file() {
            async_fs::remove_file(path).await?;
            info!("Deleted file: {}", path.display());
        } else if path.is_dir() {
            if recursive {
                async_fs::remove_dir_all(path).await?;
                info!("Deleted directory recursively: {}", path.display());
            } else {
                async_fs::remove_dir(path).await?;
                info!("Deleted directory: {}", path.display());
            }
        }

        Ok(())
    }

    /// Search for files by name pattern
    pub async fn find_files(&self, directory: &str, pattern: &str, case_sensitive: bool) -> Result<Vec<FileInfo>> {
        let directory = Path::new(directory);
        let mut results = Vec::new();

        let regex_pattern = if case_sensitive {
            regex::Regex::new(pattern)?
        } else {
            regex::Regex::new(&format!("(?i){}", pattern))?
        };

        for entry in WalkDir::new(directory)
            .max_depth(20)
            .into_iter()
            .filter_entry(|e| self.should_include_entry(e))
        {
            let entry = entry.map_err(|e| HoverShellError::File(e.to_string()))?;
            let path = entry.path();
            
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if regex_pattern.is_match(file_name) {
                    if let Some(file_info) = self.get_file_info(path).await? {
                        results.push(file_info);
                    }
                }
            }
        }

        info!("Found {} files matching pattern '{}'", results.len(), pattern);
        Ok(results)
    }

    /// Search for text content in files
    pub async fn search_in_files(&self, directory: &str, query: &str, file_pattern: Option<&str>, case_sensitive: bool) -> Result<Vec<SearchResult>> {
        let directory = Path::new(directory);
        let mut results = Vec::new();

        let query_regex = if case_sensitive {
            regex::Regex::new(query)?
        } else {
            regex::Regex::new(&format!("(?i){}", query))?
        };

        let file_regex = if let Some(pattern) = file_pattern {
            Some(if case_sensitive {
                regex::Regex::new(pattern)?
            } else {
                regex::Regex::new(&format!("(?i){}", pattern))?
            })
        } else {
            None
        };

        for entry in WalkDir::new(directory)
            .max_depth(20)
            .into_iter()
            .filter_entry(|e| self.should_include_entry(e))
        {
            let entry = entry.map_err(|e| HoverShellError::File(e.to_string()))?;
            let path = entry.path();

            if path.is_file() {
                // Check file pattern if specified
                if let Some(ref file_regex) = file_regex {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if !file_regex.is_match(file_name) {
                            continue;
                        }
                    }
                }

                // Skip large files
                if let Ok(metadata) = async_fs::metadata(path).await {
                    if metadata.len() > self.max_file_size {
                        continue;
                    }
                }

                // Search in file content
                if let Ok(content) = async_fs::read_to_string(path).await {
                    for (line_num, line) in content.lines().enumerate() {
                        if let Some(mat) = query_regex.find(line) {
                            results.push(SearchResult {
                                file_path: path.to_string_lossy().to_string(),
                                line_number: line_num + 1,
                                line_content: line.to_string(),
                                match_start: mat.start(),
                                match_end: mat.end(),
                            });
                        }
                    }
                }
            }
        }

        info!("Found {} matches for query '{}'", results.len(), query);
        Ok(results)
    }

    /// Get directory statistics
    pub async fn get_directory_stats(&self, directory: &str) -> Result<DirectoryStats> {
        let directory = Path::new(directory);
        let mut stats = DirectoryStats {
            total_files: 0,
            total_dirs: 0,
            total_size: 0,
            largest_file: None,
            file_types: HashMap::new(),
        };

        let mut largest_size = 0u64;

        for entry in WalkDir::new(directory)
            .max_depth(20)
            .into_iter()
            .filter_entry(|e| self.should_include_entry(e))
        {
            let entry = entry.map_err(|e| HoverShellError::File(e.to_string()))?;
            let path = entry.path();

            if path.is_file() {
                stats.total_files += 1;
                
                if let Ok(metadata) = async_fs::metadata(path).await {
                    let size = metadata.len();
                    stats.total_size += size;
                    
                    if size > largest_size {
                        largest_size = size;
                        stats.largest_file = Some(path.to_string_lossy().to_string());
                    }

                    // Count file types
                    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                        *stats.file_types.entry(extension.to_string()).or_insert(0) += 1;
                    }
                }
            } else if path.is_dir() {
                stats.total_dirs += 1;
            }
        }

        info!("Directory stats for {}: {} files, {} dirs, {} bytes", 
              directory.display(), stats.total_files, stats.total_dirs, stats.total_size);
        Ok(stats)
    }

    /// Create directory
    pub async fn create_directory(&self, path: &str, parents: bool) -> Result<()> {
        let path = Path::new(path);
        
        if parents {
            async_fs::create_dir_all(path).await?;
        } else {
            async_fs::create_dir(path).await?;
        }
        
        info!("Created directory: {}", path.display());
        Ok(())
    }

    /// Create file with content
    pub async fn create_file(&self, path: &str, content: &str) -> Result<()> {
        let path = Path::new(path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            async_fs::create_dir_all(parent).await?;
        }
        
        async_fs::write(path, content).await?;
        info!("Created file: {}", path.display());
        Ok(())
    }

    /// Read file content
    pub async fn read_file(&self, path: &str) -> Result<String> {
        let path = Path::new(path);
        
        if !path.exists() {
            return Err(HoverShellError::File(format!("File does not exist: {}", path.display())));
        }

        if !path.is_file() {
            return Err(HoverShellError::File(format!("Path is not a file: {}", path.display())));
        }

        let content = async_fs::read_to_string(path).await?;
        info!("Read file: {} ({} bytes)", path.display(), content.len());
        Ok(content)
    }

    /// Write content to file
    pub async fn write_file(&self, path: &str, content: &str, append: bool) -> Result<()> {
        let path = Path::new(path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            async_fs::create_dir_all(parent).await?;
        }

        if append {
            async_fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .await?
                .write_all(content.as_bytes())
                .await?;
        } else {
            async_fs::write(path, content).await?;
        }
        
        info!("Wrote to file: {}", path.display());
        Ok(())
    }

    /// Get file size in human readable format
    pub fn format_file_size(&self, size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }

    /// Check if entry should be included based on filters
    fn should_include_entry(&self, entry: &walkdir::DirEntry) -> bool {
        let path = entry.path();
        
        // Skip hidden files if not including them
        if !self.include_hidden {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') {
                    return false;
                }
            }
        }

        // Skip excluded patterns
        for pattern in &self.exclude_patterns {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.contains(pattern) {
                    return false;
                }
            }
        }

        true
    }

    /// Copy directory recursively
    async fn copy_directory_recursive(&self, source: &Path, dest: &Path) -> Result<()> {
        async_fs::create_dir_all(dest).await?;

        let mut entries = async_fs::read_dir(source).await?;
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            let dest_path = dest.join(entry.file_name());

            if entry_path.is_file() {
                async_fs::copy(&entry_path, &dest_path).await?;
            } else if entry_path.is_dir() {
                self.copy_directory_recursive(&entry_path, &dest_path).await?;
            }
        }

        Ok(())
    }
}

impl Default for FileOperations {
    fn default() -> Self {
        Self::new()
    }
}