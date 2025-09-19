use crate::{
    config::Config,
    error::{HoverShellError, Result},
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub id: String,
    pub title: String,
    pub working_directory: String,
    pub shell: String,
    pub is_active: bool,
    pub output: String,
    pub process_id: Option<u32>,
}

pub struct TerminalManager {
    sessions: HashMap<String, TerminalSession>,
    active_session: Option<String>,
    output_buffer: HashMap<String, Vec<String>>,
}

impl TerminalManager {
    pub async fn new() -> Result<Self> {
        info!("Initializing terminal manager");
        
        Ok(Self {
            sessions: HashMap::new(),
            active_session: None,
            output_buffer: HashMap::new(),
        })
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down terminal manager");
        
        // Terminate all active sessions
        for (id, session) in self.sessions.iter() {
            if let Some(pid) = session.process_id {
                if let Err(e) = self.terminate_process(pid).await {
                    error!("Failed to terminate process {} for session {}: {}", pid, id, e);
                }
            }
        }
        
        self.sessions.clear();
        self.active_session = None;
        self.output_buffer.clear();
        
        Ok(())
    }

    pub async fn initialize(&mut self, config: &Config) -> Result<()> {
        // Create initial session
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = TerminalSession {
            id: session_id.clone(),
            title: "Terminal".to_string(),
            working_directory: std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            shell: config.terminal.shell.clone(),
            is_active: true,
            output: String::new(),
            process_id: None,
        };

        self.sessions.insert(session_id.clone(), session);
        self.active_session = Some(session_id);
        
        info!("Terminal manager initialized with default session");
        Ok(())
    }

    pub async fn create_session(&mut self, title: Option<String>, working_directory: Option<String>) -> Result<String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let title = title.unwrap_or_else(|| format!("Terminal {}", self.sessions.len() + 1));
        let working_directory = working_directory.unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string()
        });

        let session = TerminalSession {
            id: session_id.clone(),
            title,
            working_directory,
            shell: "/bin/zsh".to_string(), // TODO: Get from config
            is_active: false,
            output: String::new(),
            process_id: None,
        };

        self.sessions.insert(session_id.clone(), session);
        info!("Created new terminal session: {}", session_id);
        
        Ok(session_id)
    }

    pub async fn close_session(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.sessions.remove(session_id) {
            if let Some(pid) = session.process_id {
                self.terminate_process(pid).await?;
            }
            
            self.output_buffer.remove(session_id);
            
            if self.active_session.as_ref() == Some(session_id) {
                self.active_session = self.sessions.keys().next().cloned();
            }
            
            info!("Closed terminal session: {}", session_id);
        }
        
        Ok(())
    }

    pub async fn set_active_session(&mut self, session_id: &str) -> Result<()> {
        if self.sessions.contains_key(session_id) {
            // Deactivate current session
            if let Some(current_id) = &self.active_session {
                if let Some(session) = self.sessions.get_mut(current_id) {
                    session.is_active = false;
                }
            }
            
            // Activate new session
            if let Some(session) = self.sessions.get_mut(session_id) {
                session.is_active = true;
                self.active_session = Some(session_id.to_string());
            }
            
            info!("Set active session: {}", session_id);
        } else {
            return Err(HoverShellError::Terminal(format!("Session not found: {}", session_id)));
        }
        
        Ok(())
    }

    pub async fn send_input(&mut self, session_id: &str, input: &str) -> Result<()> {
        if let Some(session) = self.sessions.get(session_id) {
            // Execute the command and update output buffer
            self.execute_command(session_id, input).await?;
            info!("Sent input to session {}: {}", session_id, input);
        } else {
            return Err(HoverShellError::Terminal(format!("Session not found: {}", session_id)));
        }
        
        Ok(())
    }

    pub async fn get_output(&self, session_id: &str) -> Result<String> {
        if let Some(lines) = self.output_buffer.get(session_id) {
            Ok(lines.join("\n"))
        } else {
            Ok(String::new())
        }
    }

    pub async fn get_state(&self) -> Vec<crate::commands::TerminalState> {
        self.sessions.values().map(|session| {
            crate::commands::TerminalState {
                id: session.id.clone(),
                title: session.title.clone(),
                working_directory: session.working_directory.clone(),
                is_active: session.is_active,
                output: self.output_buffer.get(&session.id)
                    .map(|lines| lines.join("\n"))
                    .unwrap_or_default(),
            }
        }).collect()
    }

    pub async fn get_session(&self, session_id: &str) -> Option<&TerminalSession> {
        self.sessions.get(session_id)
    }

    pub async fn get_active_session(&self) -> Option<&TerminalSession> {
        self.active_session.as_ref().and_then(|id| self.sessions.get(id))
    }

    pub async fn get_session_list(&self) -> Vec<&TerminalSession> {
        self.sessions.values().collect()
    }

    pub async fn execute_command(&mut self, session_id: &str, command: &str) -> Result<String> {
        if let Some(session) = self.sessions.get(session_id) {
            use std::process::{Command, Stdio};
            use tokio::io::{AsyncBufReadExt, BufReader};
            
            // Start shell process
            let mut child = Command::new(&session.shell)
                .current_dir(&session.working_directory)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| HoverShellError::Terminal(format!("Failed to start shell: {}", e)))?;
            
            // Send command to stdin
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin.write_all(command.as_bytes())
                    .map_err(|e| HoverShellError::Terminal(format!("Failed to write to stdin: {}", e)))?;
                stdin.write_all(b"\n")
                    .map_err(|e| HoverShellError::Terminal(format!("Failed to write newline: {}", e)))?;
            }
            
            // Wait for command to complete
            let output = child.wait_with_output()
                .map_err(|e| HoverShellError::Terminal(format!("Failed to wait for command: {}", e)))?;
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // Add to output buffer
            let output_lines = self.output_buffer.entry(session_id.to_string()).or_insert_with(Vec::new);
            output_lines.push(format!("$ {}", command));
            
            if !stdout.is_empty() {
                for line in stdout.lines() {
                    output_lines.push(line.to_string());
                }
            }
            
            if !stderr.is_empty() {
                for line in stderr.lines() {
                    output_lines.push(format!("error: {}", line));
                }
            }
            
            info!("Executed command in session {}: {}", session_id, command);
            Ok(stdout.to_string())
        } else {
            Err(HoverShellError::Terminal(format!("Session not found: {}", session_id)))
        }
    }

    pub async fn clear_output(&mut self, session_id: &str) -> Result<()> {
        self.output_buffer.remove(session_id);
        
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.output.clear();
        }
        
        info!("Cleared output for session: {}", session_id);
        Ok(())
    }

    pub async fn resize_terminal(&mut self, session_id: &str, width: u16, height: u16) -> Result<()> {
        if let Some(session) = self.sessions.get(session_id) {
            if let Some(pid) = session.process_id {
                // TODO: Implement terminal resize
                // This would involve sending SIGWINCH to the process
                info!("Resizing terminal {} to {}x{}", session_id, width, height);
            }
        } else {
            return Err(HoverShellError::Terminal(format!("Session not found: {}", session_id)));
        }
        
        Ok(())
    }

    async fn terminate_process(&self, pid: u32) -> Result<()> {
        // TODO: Implement process termination
        // This would involve sending SIGTERM or SIGKILL to the process
        info!("Terminating process: {}", pid);
        Ok(())
    }

    pub async fn start_shell_process(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            // TODO: Implement shell process startup
            // This would involve:
            // 1. Starting the shell process with proper environment
            // 2. Setting up stdin/stdout/stderr pipes
            // 3. Storing the process ID
            // 4. Starting background task to read output
            
            info!("Starting shell process for session: {}", session_id);
            // session.process_id = Some(process_id);
        } else {
            return Err(HoverShellError::Terminal(format!("Session not found: {}", session_id)));
        }
        
        Ok(())
    }

    pub async fn stop_shell_process(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            if let Some(pid) = session.process_id {
                self.terminate_process(pid).await?;
                session.process_id = None;
            }
            
            info!("Stopped shell process for session: {}", session_id);
        } else {
            return Err(HoverShellError::Terminal(format!("Session not found: {}", session_id)));
        }
        
        Ok(())
    }
}