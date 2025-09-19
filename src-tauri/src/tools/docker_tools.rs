use crate::error::{HoverShellError, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use tokio::process::Command as AsyncCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub state: String,
    pub created: String,
    pub ports: Vec<String>,
    pub command: String,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerImage {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
    pub created: String,
    pub virtual_size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerVolume {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub created: String,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerNetwork {
    pub id: String,
    pub name: String,
    pub driver: String,
    pub scope: String,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerComposeService {
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<String>,
    pub environment: HashMap<String, String>,
    pub volumes: Vec<String>,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerComposeProject {
    pub name: String,
    pub services: Vec<DockerComposeService>,
    pub status: String,
    pub created: String,
}

pub struct DockerManager {
    docker_path: String,
    compose_path: String,
}

impl DockerManager {
    pub fn new() -> Self {
        Self {
            docker_path: "docker".to_string(),
            compose_path: "docker-compose".to_string(),
        }
    }

    /// Check if Docker is available
    pub async fn is_docker_available(&self) -> bool {
        let output = AsyncCommand::new(&self.docker_path)
            .arg("--version")
            .output()
            .await;

        match output {
            Ok(result) => result.status.success(),
            Err(_) => false,
        }
    }

    /// Check if Docker Compose is available
    pub async fn is_compose_available(&self) -> bool {
        let output = AsyncCommand::new(&self.compose_path)
            .arg("--version")
            .output()
            .await;

        match output {
            Ok(result) => result.status.success(),
            Err(_) => false,
        }
    }

    /// Get Docker system information
    pub async fn get_system_info(&self) -> Result<HashMap<String, String>> {
        let output = AsyncCommand::new(&self.docker_path)
            .arg("system")
            .arg("info")
            .arg("--format")
            .arg("{{.Key}}: {{.Value}}")
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to get system info: {}", e)))?;

        if !output.status.success() {
            return Err(HoverShellError::Docker("Failed to get Docker system info".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut info = HashMap::new();

        for line in output_str.lines() {
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..].trim().to_string();
                info.insert(key, value);
            }
        }

        info!("Retrieved Docker system information");
        Ok(info)
    }

    /// List all containers
    pub async fn list_containers(&self, all: bool) -> Result<Vec<DockerContainer>> {
        let mut args = vec!["ps"];
        if all {
            args.push("-a");
        }
        args.extend(&["--format", "table {{.ID}}\t{{.Names}}\t{{.Image}}\t{{.Status}}\t{{.State}}\t{{.CreatedAt}}\t{{.Ports}}\t{{.Command}}\t{{.Size}}"]);

        let output = AsyncCommand::new(&self.docker_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to list containers: {}", e)))?;

        if !output.status.success() {
            return Err(HoverShellError::Docker("Failed to list containers".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut containers = Vec::new();

        for line in output_str.lines().skip(1) { // Skip header
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 9 {
                containers.push(DockerContainer {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    image: parts[2].to_string(),
                    status: parts[3].to_string(),
                    state: parts[4].to_string(),
                    created: parts[5].to_string(),
                    ports: parts[6].split(',').map(|s| s.trim().to_string()).collect(),
                    command: parts[7].to_string(),
                    size: if parts[8].is_empty() { None } else { Some(parts[8].to_string()) },
                });
            }
        }

        info!("Listed {} containers", containers.len());
        Ok(containers)
    }

    /// Start a container
    pub async fn start_container(&self, container_id: &str) -> Result<()> {
        let output = AsyncCommand::new(&self.docker_path)
            .arg("start")
            .arg(container_id)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to start container: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to start container {}: {}", container_id, error_msg)));
        }

        info!("Started container: {}", container_id);
        Ok(())
    }

    /// Stop a container
    pub async fn stop_container(&self, container_id: &str, timeout: Option<u32>) -> Result<()> {
        let mut args = vec!["stop"];
        if let Some(timeout_secs) = timeout {
            args.extend(&["--time", &timeout_secs.to_string()]);
        }
        args.push(container_id);

        let output = AsyncCommand::new(&self.docker_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to stop container: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to stop container {}: {}", container_id, error_msg)));
        }

        info!("Stopped container: {}", container_id);
        Ok(())
    }

    /// Remove a container
    pub async fn remove_container(&self, container_id: &str, force: bool) -> Result<()> {
        let mut args = vec!["rm"];
        if force {
            args.push("-f");
        }
        args.push(container_id);

        let output = AsyncCommand::new(&self.docker_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to remove container: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to remove container {}: {}", container_id, error_msg)));
        }

        info!("Removed container: {}", container_id);
        Ok(())
    }

    /// Get container logs
    pub async fn get_container_logs(&self, container_id: &str, tail: Option<usize>, follow: bool) -> Result<String> {
        let mut args = vec!["logs"];
        if let Some(tail_count) = tail {
            args.extend(&["--tail", &tail_count.to_string()]);
        }
        if follow {
            args.push("--follow");
        }
        args.push(container_id);

        let output = AsyncCommand::new(&self.docker_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to get container logs: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to get logs for container {}: {}", container_id, error_msg)));
        }

        let logs = String::from_utf8_lossy(&output.stdout).to_string();
        info!("Retrieved logs for container: {}", container_id);
        Ok(logs)
    }

    /// List all images
    pub async fn list_images(&self, all: bool) -> Result<Vec<DockerImage>> {
        let mut args = vec!["images"];
        if all {
            args.push("-a");
        }
        args.extend(&["--format", "table {{.ID}}\t{{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedAt}}\t{{.VirtualSize}}"]);

        let output = AsyncCommand::new(&self.docker_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to list images: {}", e)))?;

        if !output.status.success() {
            return Err(HoverShellError::Docker("Failed to list images".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut images = Vec::new();

        for line in output_str.lines().skip(1) { // Skip header
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 6 {
                images.push(DockerImage {
                    id: parts[0].to_string(),
                    repository: parts[1].to_string(),
                    tag: parts[2].to_string(),
                    size: parts[3].to_string(),
                    created: parts[4].to_string(),
                    virtual_size: if parts[5].is_empty() { None } else { Some(parts[5].to_string()) },
                });
            }
        }

        info!("Listed {} images", images.len());
        Ok(images)
    }

    /// Pull an image
    pub async fn pull_image(&self, image_name: &str) -> Result<()> {
        let output = AsyncCommand::new(&self.docker_path)
            .arg("pull")
            .arg(image_name)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to pull image: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to pull image {}: {}", image_name, error_msg)));
        }

        info!("Pulled image: {}", image_name);
        Ok(())
    }

    /// Remove an image
    pub async fn remove_image(&self, image_id: &str, force: bool) -> Result<()> {
        let mut args = vec!["rmi"];
        if force {
            args.push("-f");
        }
        args.push(image_id);

        let output = AsyncCommand::new(&self.docker_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to remove image: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to remove image {}: {}", image_id, error_msg)));
        }

        info!("Removed image: {}", image_id);
        Ok(())
    }

    /// List volumes
    pub async fn list_volumes(&self) -> Result<Vec<DockerVolume>> {
        let output = AsyncCommand::new(&self.docker_path)
            .args(&["volume", "ls", "--format", "table {{.Name}}\t{{.Driver}}\t{{.Mountpoint}}\t{{.CreatedAt}}\t{{.Size}}"])
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to list volumes: {}", e)))?;

        if !output.status.success() {
            return Err(HoverShellError::Docker("Failed to list volumes".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut volumes = Vec::new();

        for line in output_str.lines().skip(1) { // Skip header
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                volumes.push(DockerVolume {
                    name: parts[0].to_string(),
                    driver: parts[1].to_string(),
                    mountpoint: parts[2].to_string(),
                    created: parts[3].to_string(),
                    size: if parts[4].is_empty() { None } else { Some(parts[4].to_string()) },
                });
            }
        }

        info!("Listed {} volumes", volumes.len());
        Ok(volumes)
    }

    /// List networks
    pub async fn list_networks(&self) -> Result<Vec<DockerNetwork>> {
        let output = AsyncCommand::new(&self.docker_path)
            .args(&["network", "ls", "--format", "table {{.ID}}\t{{.Name}}\t{{.Driver}}\t{{.Scope}}\t{{.CreatedAt}}"])
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to list networks: {}", e)))?;

        if !output.status.success() {
            return Err(HoverShellError::Docker("Failed to list networks".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut networks = Vec::new();

        for line in output_str.lines().skip(1) { // Skip header
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                networks.push(DockerNetwork {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    driver: parts[2].to_string(),
                    scope: parts[3].to_string(),
                    created: parts[4].to_string(),
                });
            }
        }

        info!("Listed {} networks", networks.len());
        Ok(networks)
    }

    /// Run a container
    pub async fn run_container(&self, image: &str, command: Option<&str>, options: &RunOptions) -> Result<String> {
        let mut args = vec!["run"];

        // Add options
        if options.detached {
            args.push("-d");
        }
        if options.interactive {
            args.push("-i");
        }
        if options.tty {
            args.push("-t");
        }
        if let Some(name) = &options.name {
            args.extend(&["--name", name]);
        }
        if let Some(port) = &options.port {
            args.extend(&["-p", port]);
        }
        for volume in &options.volumes {
            args.extend(&["-v", volume]);
        }
        for env in &options.environment {
            args.extend(&["-e", env]);
        }
        if let Some(workdir) = &options.workdir {
            args.extend(&["-w", workdir]);
        }

        args.push(image);

        if let Some(cmd) = command {
            args.push(cmd);
        }

        let output = AsyncCommand::new(&self.docker_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to run container: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to run container: {}", error_msg)));
        }

        let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        info!("Started container: {} (ID: {})", image, container_id);
        Ok(container_id)
    }

    /// Execute command in running container
    pub async fn exec_command(&self, container_id: &str, command: &str, interactive: bool) -> Result<String> {
        let mut args = vec!["exec"];
        if interactive {
            args.extend(&["-i", "-t"]);
        }
        args.push(container_id);
        args.push(command);

        let output = AsyncCommand::new(&self.docker_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to execute command: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to execute command in container {}: {}", container_id, error_msg)));
        }

        let result = String::from_utf8_lossy(&output.stdout).to_string();
        info!("Executed command '{}' in container: {}", command, container_id);
        Ok(result)
    }

    /// Docker Compose operations
    pub async fn compose_up(&self, project_path: &str, services: Option<Vec<&str>>) -> Result<()> {
        let mut args = vec!["-f", project_path, "up", "-d"];
        if let Some(service_list) = services {
            args.extend(service_list);
        }

        let output = AsyncCommand::new(&self.compose_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to start compose services: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to start compose services: {}", error_msg)));
        }

        info!("Started Docker Compose services");
        Ok(())
    }

    pub async fn compose_down(&self, project_path: &str) -> Result<()> {
        let output = AsyncCommand::new(&self.compose_path)
            .args(&["-f", project_path, "down"])
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to stop compose services: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::Docker(format!("Failed to stop compose services: {}", error_msg)));
        }

        info!("Stopped Docker Compose services");
        Ok(())
    }

    pub async fn compose_ps(&self, project_path: &str) -> Result<Vec<DockerComposeService>> {
        let output = AsyncCommand::new(&self.compose_path)
            .args(&["-f", project_path, "ps", "--format", "json"])
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to list compose services: {}", e)))?;

        if !output.status.success() {
            return Err(HoverShellError::Docker("Failed to list compose services".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut services = Vec::new();

        for line in output_str.lines() {
            if let Ok(service) = serde_json::from_str::<DockerComposeService>(line) {
                services.push(service);
            }
        }

        info!("Listed {} compose services", services.len());
        Ok(services)
    }

    /// Get Docker system usage
    pub async fn get_system_usage(&self) -> Result<HashMap<String, String>> {
        let output = AsyncCommand::new(&self.docker_path)
            .args(&["system", "df", "--format", "{{.Type}}: {{.Size}}"])
            .output()
            .await
            .map_err(|e| HoverShellError::Docker(format!("Failed to get system usage: {}", e)))?;

        if !output.status.success() {
            return Err(HoverShellError::Docker("Failed to get system usage".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut usage = HashMap::new();

        for line in output_str.lines() {
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..].trim().to_string();
                usage.insert(key, value);
            }
        }

        info!("Retrieved Docker system usage");
        Ok(usage)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOptions {
    pub detached: bool,
    pub interactive: bool,
    pub tty: bool,
    pub name: Option<String>,
    pub port: Option<String>,
    pub volumes: Vec<String>,
    pub environment: Vec<String>,
    pub workdir: Option<String>,
}

impl Default for DockerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            detached: false,
            interactive: false,
            tty: false,
            name: None,
            port: None,
            volumes: Vec::new(),
            environment: Vec::new(),
            workdir: None,
        }
    }
}