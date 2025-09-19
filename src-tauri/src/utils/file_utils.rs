use crate::error::{HoverShellError, Result};
use log::{error, info};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

pub async fn read_file(path: &Path) -> Result<String> {
    tokio::fs::read_to_string(path).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))
}

pub async fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(path, content).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    Ok(())
}

pub async fn read_file_bytes(path: &Path) -> Result<Vec<u8>> {
    tokio::fs::read(path).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))
}

pub async fn write_file_bytes(path: &Path, content: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(path, content).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    Ok(())
}

pub async fn file_exists(path: &Path) -> bool {
    tokio::fs::metadata(path).await.is_ok()
}

pub async fn is_file(path: &Path) -> bool {
    tokio::fs::metadata(path).await
        .map(|m| m.is_file())
        .unwrap_or(false)
}

pub async fn is_directory(path: &Path) -> bool {
    tokio::fs::metadata(path).await
        .map(|m| m.is_dir())
        .unwrap_or(false)
}

pub async fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = tokio::fs::metadata(path).await?;
    Ok(metadata.len())
}

pub async fn get_file_modified_time(path: &Path) -> Result<std::time::SystemTime> {
    let metadata = tokio::fs::metadata(path).await?;
    metadata.modified()
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))
}

pub async fn create_directory(path: &Path) -> Result<()> {
    tokio::fs::create_dir_all(path).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    Ok(())
}

pub async fn remove_file(path: &Path) -> Result<()> {
    tokio::fs::remove_file(path).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    Ok(())
}

pub async fn remove_directory(path: &Path) -> Result<()> {
    tokio::fs::remove_dir_all(path).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    Ok(())
}

pub async fn list_directory(path: &Path) -> Result<Vec<PathBuf>> {
    let mut entries = tokio::fs::read_dir(path).await?;
    let mut paths = Vec::new();
    
    while let Some(entry) = entries.next_entry().await? {
        paths.push(entry.path());
    }
    
    Ok(paths)
}

pub async fn list_directory_contents(path: &Path) -> Result<Vec<DirectoryEntry>> {
    let mut entries = tokio::fs::read_dir(path).await?;
    let mut contents = Vec::new();
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let metadata = entry.metadata().await?;
        
        contents.push(DirectoryEntry {
            path,
            name: entry.file_name().to_string_lossy().to_string(),
            is_file: metadata.is_file(),
            is_directory: metadata.is_dir(),
            size: metadata.len(),
            modified: metadata.modified().unwrap_or_default(),
        });
    }
    
    Ok(contents)
}

pub async fn copy_file(src: &Path, dst: &Path) -> Result<()> {
    if let Some(parent) = dst.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::copy(src, dst).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    Ok(())
}

pub async fn move_file(src: &Path, dst: &Path) -> Result<()> {
    if let Some(parent) = dst.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::rename(src, dst).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    Ok(())
}

pub async fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

pub async fn get_file_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}

pub async fn get_file_stem(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|s| s.to_string())
}

pub async fn get_parent_directory(path: &Path) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}

pub async fn get_absolute_path(path: &Path) -> Result<PathBuf> {
    tokio::fs::canonicalize(path).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))
}

pub async fn find_files_with_extension(dir: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    
    while let Some(current_dir) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&current_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            
            if metadata.is_dir() {
                stack.push(path);
            } else if metadata.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().to_lowercase() == extension.to_lowercase() {
                        results.push(path);
                    }
                }
            }
        }
    }
    
    Ok(results)
}

pub async fn find_files_by_name(dir: &Path, name: &str) -> Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    
    while let Some(current_dir) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&current_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            
            if metadata.is_dir() {
                stack.push(path.clone());
            }
            
            if let Some(file_name) = path.file_name() {
                if file_name.to_string_lossy().to_lowercase().contains(&name.to_lowercase()) {
                    results.push(path);
                }
            }
        }
    }
    
    Ok(results)
}

pub async fn get_directory_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;
    let mut stack = vec![path.to_path_buf()];
    
    while let Some(current_path) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&current_path).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            
            if metadata.is_dir() {
                stack.push(path);
            } else {
                total_size += metadata.len();
            }
        }
    }
    
    Ok(total_size)
}

pub async fn get_file_hash(path: &Path) -> Result<String> {
    use sha2::{Sha256, Digest};
    
    let content = read_file_bytes(path).await?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    Ok(format!("{:x}", hasher.finalize()))
}

pub async fn compare_files(file1: &Path, file2: &Path) -> Result<bool> {
    let hash1 = get_file_hash(file1).await?;
    let hash2 = get_file_hash(file2).await?;
    Ok(hash1 == hash2)
}

pub async fn create_temp_file() -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("hovershell_{}", uuid::Uuid::new_v4()));
    tokio::fs::File::create(&temp_file).await?;
    Ok(temp_file)
}

pub async fn create_temp_directory() -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join(format!("hovershell_{}", uuid::Uuid::new_v4()));
    tokio::fs::create_dir_all(&temp_path).await?;
    Ok(temp_path)
}

pub async fn cleanup_temp_files(pattern: &str) -> Result<()> {
    let temp_dir = std::env::temp_dir();
    let mut entries = tokio::fs::read_dir(&temp_dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if let Some(name) = path.file_name() {
            if name.to_string_lossy().starts_with(pattern) {
                if path.is_file() {
                    let _ = tokio::fs::remove_file(&path).await;
                } else if path.is_dir() {
                    let _ = tokio::fs::remove_dir_all(&path).await;
                }
            }
        }
    }
    
    Ok(())
}

pub async fn watch_directory(path: &Path, callback: impl Fn(PathBuf) + Send + Sync + 'static) -> Result<()> {
    use notify::{Watcher, RecursiveMode, Event, EventKind};
    
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    
    let mut watcher = notify::recommended_watcher(tx)
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    
    watcher.watch(path, RecursiveMode::Recursive)
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    
    tokio::spawn(async move {
        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => {
                    if let EventKind::Modify(_) = event.kind {
                        for path in event.paths {
                            callback(path);
                        }
                    }
                }
                Err(e) => {
                    error!("Directory watch error: {}", e);
                }
            }
        }
    });
    
    Ok(())
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_file: bool,
    pub is_directory: bool,
    pub size: u64,
    pub modified: std::time::SystemTime,
}

impl DirectoryEntry {
    pub fn get_extension(&self) -> Option<String> {
        self.path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }
    
    pub fn get_size_formatted(&self) -> String {
        crate::utils::format_bytes(self.size)
    }
    
    pub fn get_modified_formatted(&self) -> String {
        chrono::DateTime::<chrono::Local>::from(self.modified)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}