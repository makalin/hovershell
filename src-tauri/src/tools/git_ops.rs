use crate::error::{HoverShellError, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{Command, Stdio};
use tokio::process::Command as AsyncCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub branch: String,
    pub is_clean: bool,
    pub staged_files: Vec<GitFileStatus>,
    pub unstaged_files: Vec<GitFileStatus>,
    pub untracked_files: Vec<String>,
    pub ahead: usize,
    pub behind: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitFileStatus {
    pub path: String,
    pub status: String,
    pub staged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommit {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub message: String,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitBranch {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub last_commit: String,
    pub last_commit_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitDiff {
    pub file_path: String,
    pub changes: Vec<DiffChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffChange {
    pub line_type: String, // "+", "-", " "
    pub line_number: usize,
    pub content: String,
}

pub struct GitOperations {
    repo_path: String,
}

impl GitOperations {
    pub fn new(repo_path: &str) -> Self {
        Self {
            repo_path: repo_path.to_string(),
        }
    }

    /// Check if the current directory is a git repository
    pub async fn is_git_repo(&self) -> bool {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--git-dir")
            .current_dir(&self.repo_path)
            .output();

        match output {
            Ok(result) => result.status.success(),
            Err(_) => false,
        }
    }

    /// Get current git status
    pub async fn get_status(&self) -> Result<GitStatus> {
        if !self.is_git_repo().await {
            return Err(HoverShellError::Git("Not a git repository".to_string()));
        }

        let branch = self.get_current_branch().await?;
        let status_output = self.run_git_command(&["status", "--porcelain"]).await?;
        let ahead_behind = self.get_ahead_behind().await?;

        let mut staged_files = Vec::new();
        let mut unstaged_files = Vec::new();
        let mut untracked_files = Vec::new();

        for line in status_output.lines() {
            if line.len() < 3 {
                continue;
            }

            let status = &line[0..2];
            let path = &line[3..];

            match status {
                "??" => untracked_files.push(path.to_string()),
                "A " | "M " | "D " | "R " | "C " => {
                    staged_files.push(GitFileStatus {
                        path: path.to_string(),
                        status: status.to_string(),
                        staged: true,
                    });
                }
                " M" | " D" | " A" | " R" | " C" => {
                    unstaged_files.push(GitFileStatus {
                        path: path.to_string(),
                        status: status.to_string(),
                        staged: false,
                    });
                }
                "MM" | "MD" | "AM" | "AD" | "RM" | "RD" | "CM" | "CD" => {
                    // Both staged and unstaged changes
                    staged_files.push(GitFileStatus {
                        path: path.to_string(),
                        status: format!("{} (staged)", &status[0..1]),
                        staged: true,
                    });
                    unstaged_files.push(GitFileStatus {
                        path: path.to_string(),
                        status: format!("{} (unstaged)", &status[1..2]),
                        staged: false,
                    });
                }
                _ => {}
            }
        }

        let is_clean = staged_files.is_empty() && unstaged_files.is_empty() && untracked_files.is_empty();

        Ok(GitStatus {
            branch,
            is_clean,
            staged_files,
            unstaged_files,
            untracked_files,
            ahead: ahead_behind.0,
            behind: ahead_behind.1,
        })
    }

    /// Get current branch name
    pub async fn get_current_branch(&self) -> Result<String> {
        let output = self.run_git_command(&["branch", "--show-current"]).await?;
        Ok(output.trim().to_string())
    }

    /// Get list of all branches
    pub async fn get_branches(&self) -> Result<Vec<GitBranch>> {
        let local_output = self.run_git_command(&["branch", "--format=%(refname:short)|%(objectname)|%(committerdate:iso)"]).await?;
        let remote_output = self.run_git_command(&["branch", "-r", "--format=%(refname:short)|%(objectname)|%(committerdate:iso)"]).await?;
        
        let current_branch = self.get_current_branch().await?;
        let mut branches = Vec::new();

        // Parse local branches
        for line in local_output.lines() {
            if line.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let hash = parts[1].to_string();
                let date = parts[2].to_string();
                
                branches.push(GitBranch {
                    name: name.clone(),
                    is_current: name == current_branch,
                    is_remote: false,
                    last_commit: hash,
                    last_commit_date: date,
                });
            }
        }

        // Parse remote branches
        for line in remote_output.lines() {
            if line.is_empty() || line.contains("origin/HEAD") {
                continue;
            }
            
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let hash = parts[1].to_string();
                let date = parts[2].to_string();
                
                branches.push(GitBranch {
                    name: name.clone(),
                    is_current: false,
                    is_remote: true,
                    last_commit: hash,
                    last_commit_date: date,
                });
            }
        }

        Ok(branches)
    }

    /// Get commit history
    pub async fn get_commits(&self, limit: Option<usize>) -> Result<Vec<GitCommit>> {
        let limit = limit.unwrap_or(20);
        let format = "%H|%an|%ae|%ad|%s";
        let date_format = "--date=iso";
        
        let output = self.run_git_command(&[
            "log",
            &format!("--max-count={}", limit),
            &format!("--pretty=format:{}", format),
            date_format,
        ]).await?;

        let mut commits = Vec::new();
        
        for line in output.lines() {
            if line.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 5 {
                let hash = parts[0].to_string();
                let author = parts[1].to_string();
                let email = parts[2].to_string();
                let date = parts[3].to_string();
                let message = parts[4].to_string();
                
                // Get stats for this commit
                let stats = self.get_commit_stats(&hash).await?;
                
                commits.push(GitCommit {
                    short_hash: hash[..8].to_string(),
                    hash,
                    author,
                    email,
                    date,
                    message,
                    files_changed: stats.0,
                    insertions: stats.1,
                    deletions: stats.2,
                });
            }
        }

        Ok(commits)
    }

    /// Get diff for a specific file or all changes
    pub async fn get_diff(&self, file_path: Option<&str>) -> Result<Vec<GitDiff>> {
        let args = if let Some(path) = file_path {
            vec!["diff", "--", path]
        } else {
            vec!["diff"]
        };

        let output = self.run_git_command(&args).await?;
        self.parse_diff_output(&output)
    }

    /// Get staged diff
    pub async fn get_staged_diff(&self) -> Result<Vec<GitDiff>> {
        let output = self.run_git_command(&["diff", "--staged"]).await?;
        self.parse_diff_output(&output)
    }

    /// Add files to staging
    pub async fn add_files(&self, files: &[String]) -> Result<()> {
        let mut args = vec!["add"];
        for file in files {
            args.push(file);
        }
        
        self.run_git_command(&args).await?;
        info!("Added {} files to staging", files.len());
        Ok(())
    }

    /// Commit staged changes
    pub async fn commit(&self, message: &str) -> Result<String> {
        let output = self.run_git_command(&["commit", "-m", message]).await?;
        info!("Committed changes: {}", message);
        Ok(output)
    }

    /// Create a new branch
    pub async fn create_branch(&self, branch_name: &str, checkout: bool) -> Result<()> {
        let args = if checkout {
            vec!["checkout", "-b", branch_name]
        } else {
            vec!["branch", branch_name]
        };
        
        self.run_git_command(&args).await?;
        info!("Created branch: {}", branch_name);
        Ok(())
    }

    /// Switch to a branch
    pub async fn checkout_branch(&self, branch_name: &str) -> Result<()> {
        self.run_git_command(&["checkout", branch_name]).await?;
        info!("Switched to branch: {}", branch_name);
        Ok(())
    }

    /// Pull latest changes
    pub async fn pull(&self, branch: Option<&str>) -> Result<String> {
        let args = if let Some(branch) = branch {
            vec!["pull", "origin", branch]
        } else {
            vec!["pull"]
        };
        
        let output = self.run_git_command(&args).await?;
        info!("Pulled latest changes");
        Ok(output)
    }

    /// Push changes to remote
    pub async fn push(&self, branch: Option<&str>, upstream: bool) -> Result<String> {
        let args = if upstream {
            if let Some(branch) = branch {
                vec!["push", "--set-upstream", "origin", branch]
            } else {
                vec!["push", "--set-upstream", "origin"]
            }
        } else {
            if let Some(branch) = branch {
                vec!["push", "origin", branch]
            } else {
                vec!["push"]
            }
        };
        
        let output = self.run_git_command(&args).await?;
        info!("Pushed changes to remote");
        Ok(output)
    }

    /// Get ahead/behind count
    async fn get_ahead_behind(&self) -> Result<(usize, usize)> {
        let current_branch = self.get_current_branch().await?;
        let output = self.run_git_command(&[
            "rev-list",
            "--left-right",
            "--count",
            &format!("origin/{}...HEAD", current_branch),
        ]).await?;

        let parts: Vec<&str> = output.trim().split('\t').collect();
        if parts.len() == 2 {
            let behind = parts[0].parse::<usize>().unwrap_or(0);
            let ahead = parts[1].parse::<usize>().unwrap_or(0);
            Ok((ahead, behind))
        } else {
            Ok((0, 0))
        }
    }

    /// Get commit statistics
    async fn get_commit_stats(&self, commit_hash: &str) -> Result<(usize, usize, usize)> {
        let output = self.run_git_command(&[
            "show",
            "--stat",
            "--format=",
            commit_hash,
        ]).await?;

        let mut files_changed = 0;
        let mut insertions = 0;
        let mut deletions = 0;

        for line in output.lines() {
            if line.contains("files changed") {
                let parts: Vec<&str> = line.split(',').collect();
                if let Some(files_part) = parts.get(0) {
                    if let Some(num) = files_part.split_whitespace().next() {
                        files_changed = num.parse::<usize>().unwrap_or(0);
                    }
                }
                
                if let Some(insertions_part) = parts.get(1) {
                    if let Some(num) = insertions_part.split_whitespace().find(|s| s.parse::<usize>().is_ok()) {
                        insertions = num.parse::<usize>().unwrap_or(0);
                    }
                }
                
                if let Some(deletions_part) = parts.get(2) {
                    if let Some(num) = deletions_part.split_whitespace().find(|s| s.parse::<usize>().is_ok()) {
                        deletions = num.parse::<usize>().unwrap_or(0);
                    }
                }
            }
        }

        Ok((files_changed, insertions, deletions))
    }

    /// Parse diff output into structured format
    fn parse_diff_output(&self, output: &str) -> Result<Vec<GitDiff>> {
        let mut diffs = Vec::new();
        let mut current_diff: Option<GitDiff> = None;

        for line in output.lines() {
            if line.starts_with("diff --git") {
                // Save previous diff if exists
                if let Some(diff) = current_diff.take() {
                    diffs.push(diff);
                }

                // Extract file path from diff header
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let file_path = parts[3].to_string();
                    current_diff = Some(GitDiff {
                        file_path,
                        changes: Vec::new(),
                    });
                }
            } else if let Some(ref mut diff) = current_diff {
                if line.starts_with("@@") {
                    // Skip hunk headers for now
                    continue;
                } else if line.starts_with('+') || line.starts_with('-') || line.starts_with(' ') {
                    let line_type = line.chars().next().unwrap().to_string();
                    let content = line[1..].to_string();
                    
                    diff.changes.push(DiffChange {
                        line_type,
                        line_number: diff.changes.len() + 1,
                        content,
                    });
                }
            }
        }

        // Add the last diff
        if let Some(diff) = current_diff {
            diffs.push(diff);
        }

        Ok(diffs)
    }

    /// Run git command and return output
    async fn run_git_command(&self, args: &[&str]) -> Result<String> {
        let output = AsyncCommand::new("git")
            .args(args)
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| HoverShellError::Git(format!("Failed to run git command: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Git(format!("Git command failed: {}", error_msg)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}