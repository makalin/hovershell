use crate::error::{HoverShellError, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use sysinfo::{System, Process, Pid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub command: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub memory_percent: f32,
    pub status: String,
    pub start_time: u64,
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub memory_percent: f32,
    pub total_swap: u64,
    pub used_swap: u64,
    pub free_swap: u64,
    pub swap_percent: f32,
    pub cpu_count: usize,
    pub cpu_usage: f32,
    pub load_average: (f64, f64, f64),
    pub uptime: u64,
    pub boot_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub mac_address: Option<String>,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub packets_received: u64,
    pub packets_sent: u64,
    pub is_up: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub local_address: String,
    pub remote_address: String,
    pub local_port: u16,
    pub remote_port: u16,
    pub protocol: String,
    pub state: String,
    pub process_name: Option<String>,
    pub process_pid: Option<u32>,
}

pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self { system }
    }

    /// Refresh system information
    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    /// Get system information
    pub fn get_system_info(&mut self) -> Result<SystemInfo> {
        self.system.refresh_memory();
        self.system.refresh_cpu();

        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let free_memory = self.system.free_memory();
        let memory_percent = (used_memory as f32 / total_memory as f32) * 100.0;

        let total_swap = self.system.total_swap();
        let used_swap = self.system.used_swap();
        let free_swap = self.system.free_swap();
        let swap_percent = if total_swap > 0 {
            (used_swap as f32 / total_swap as f32) * 100.0
        } else {
            0.0
        };

        let cpu_count = self.system.cpus().len();
        let cpu_usage = self.system.global_cpu_info().cpu_usage();

        // Get load average (Unix-like systems)
        let load_average = self.get_load_average()?;
        
        // Get uptime and boot time
        let uptime = self.system.uptime();
        let boot_time = self.system.boot_time();

        Ok(SystemInfo {
            total_memory,
            used_memory,
            free_memory,
            memory_percent,
            total_swap,
            used_swap,
            free_swap,
            swap_percent,
            cpu_count,
            cpu_usage,
            load_average,
            uptime,
            boot_time,
        })
    }

    /// Get list of running processes
    pub fn get_processes(&mut self, limit: Option<usize>) -> Result<Vec<ProcessInfo>> {
        self.system.refresh_processes();

        let mut processes: Vec<ProcessInfo> = self.system
            .processes()
            .iter()
            .map(|(pid, process)| {
                ProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string(),
                    command: process.cmd().join(" "),
                    cpu_usage: process.cpu_usage(),
                    memory_usage: process.memory(),
                    memory_percent: process.memory() as f32 / self.system.total_memory() as f32 * 100.0,
                    status: format!("{:?}", process.status()),
                    start_time: process.start_time(),
                    user: None, // TODO: Get user information
                }
            })
            .collect();

        // Sort by CPU usage (descending)
        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));

        // Apply limit if specified
        if let Some(limit) = limit {
            processes.truncate(limit);
        }

        info!("Retrieved {} processes", processes.len());
        Ok(processes)
    }

    /// Get process by PID
    pub fn get_process(&mut self, pid: u32) -> Result<Option<ProcessInfo>> {
        self.system.refresh_processes();

        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            Ok(Some(ProcessInfo {
                pid,
                name: process.name().to_string(),
                command: process.cmd().join(" "),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                memory_percent: process.memory() as f32 / self.system.total_memory() as f32 * 100.0,
                status: format!("{:?}", process.status()),
                start_time: process.start_time(),
                user: None,
            }))
        } else {
            Ok(None)
        }
    }

    /// Kill a process
    pub fn kill_process(&self, pid: u32, signal: Option<i32>) -> Result<()> {
        let signal = signal.unwrap_or(15); // SIGTERM by default
        
        let output = Command::new("kill")
            .arg(&format!("-{}", signal))
            .arg(&pid.to_string())
            .output()
            .map_err(|e| HoverShellError::System(format!("Failed to kill process: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(HoverShellError::System(format!("Failed to kill process {}: {}", pid, error_msg)));
        }

        info!("Killed process {} with signal {}", pid, signal);
        Ok(())
    }

    /// Get disk information
    pub fn get_disk_info(&mut self) -> Result<Vec<DiskInfo>> {
        self.system.refresh_disks();

        let disks: Vec<DiskInfo> = self.system
            .disks()
            .iter()
            .map(|disk| {
                let total_space = disk.total_space();
                let available_space = disk.available_space();
                let used_space = total_space - available_space;
                let usage_percent = (used_space as f32 / total_space as f32) * 100.0;

                DiskInfo {
                    name: disk.name().to_string_lossy().to_string(),
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    file_system: String::from_utf8_lossy(disk.file_system()).to_string(),
                    total_space,
                    used_space,
                    free_space: available_space,
                    usage_percent,
                }
            })
            .collect();

        info!("Retrieved {} disk information entries", disks.len());
        Ok(disks)
    }

    /// Get network interfaces
    pub fn get_network_interfaces(&mut self) -> Result<Vec<NetworkInterface>> {
        self.system.refresh_networks();

        let interfaces: Vec<NetworkInterface> = self.system
            .networks()
            .iter()
            .map(|(name, network)| {
                NetworkInterface {
                    name: name.clone(),
                    ip_addresses: Vec::new(), // TODO: Extract IP addresses
                    mac_address: None, // TODO: Extract MAC address
                    bytes_received: network.received(),
                    bytes_sent: network.transmitted(),
                    packets_received: 0, // TODO: Get packet counts
                    packets_sent: 0,
                    is_up: true, // TODO: Check interface status
                }
            })
            .collect();

        info!("Retrieved {} network interfaces", interfaces.len());
        Ok(interfaces)
    }

    /// Get network connections
    pub fn get_network_connections(&self) -> Result<Vec<NetworkConnection>> {
        // Use netstat or ss command to get network connections
        let output = Command::new("netstat")
            .args(&["-tuln"])
            .output()
            .map_err(|e| HoverShellError::System(format!("Failed to get network connections: {}", e)))?;

        if !output.status.success() {
            return Err(HoverShellError::System("Failed to get network connections".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut connections = Vec::new();

        for line in output_str.lines() {
            if line.starts_with("tcp") || line.starts_with("udp") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let protocol = parts[0].to_string();
                    let local_address = parts[3].to_string();
                    let state = if parts.len() > 5 { parts[5].to_string() } else { "UNKNOWN".to_string() };

                    // Parse local address (format: ip:port)
                    let local_parts: Vec<&str> = local_address.split(':').collect();
                    let local_ip = local_parts[0].to_string();
                    let local_port = local_parts.get(1).unwrap_or(&"0").parse::<u16>().unwrap_or(0);

                    connections.push(NetworkConnection {
                        local_address: local_ip,
                        remote_address: "0.0.0.0".to_string(),
                        local_port,
                        remote_port: 0,
                        protocol,
                        state,
                        process_name: None,
                        process_pid: None,
                    });
                }
            }
        }

        info!("Retrieved {} network connections", connections.len());
        Ok(connections)
    }

    /// Get system load average
    fn get_load_average(&self) -> Result<(f64, f64, f64)> {
        // Try to read from /proc/loadavg on Linux
        if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 3 {
                let load1 = parts[0].parse::<f64>().unwrap_or(0.0);
                let load5 = parts[1].parse::<f64>().unwrap_or(0.0);
                let load15 = parts[2].parse::<f64>().unwrap_or(0.0);
                return Ok((load1, load5, load15));
            }
        }

        // Fallback: use uptime command
        let output = Command::new("uptime")
            .output()
            .map_err(|e| HoverShellError::System(format!("Failed to get load average: {}", e)))?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse uptime output: "load average: 0.52, 0.58, 0.59"
            if let Some(load_part) = output_str.split("load average:").nth(1) {
                let load_parts: Vec<&str> = load_part.split(',').collect();
                if load_parts.len() >= 3 {
                    let load1 = load_parts[0].trim().parse::<f64>().unwrap_or(0.0);
                    let load5 = load_parts[1].trim().parse::<f64>().unwrap_or(0.0);
                    let load15 = load_parts[2].trim().parse::<f64>().unwrap_or(0.0);
                    return Ok((load1, load5, load15));
                }
            }
        }

        Ok((0.0, 0.0, 0.0))
    }

    /// Get top processes by CPU usage
    pub fn get_top_processes_by_cpu(&mut self, limit: usize) -> Result<Vec<ProcessInfo>> {
        self.get_processes(Some(limit))
    }

    /// Get top processes by memory usage
    pub fn get_top_processes_by_memory(&mut self, limit: usize) -> Result<Vec<ProcessInfo>> {
        self.system.refresh_processes();

        let mut processes: Vec<ProcessInfo> = self.system
            .processes()
            .iter()
            .map(|(pid, process)| {
                ProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string(),
                    command: process.cmd().join(" "),
                    cpu_usage: process.cpu_usage(),
                    memory_usage: process.memory(),
                    memory_percent: process.memory() as f32 / self.system.total_memory() as f32 * 100.0,
                    status: format!("{:?}", process.status()),
                    start_time: process.start_time(),
                    user: None,
                }
            })
            .collect();

        // Sort by memory usage (descending)
        processes.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
        processes.truncate(limit);

        info!("Retrieved top {} processes by memory usage", processes.len());
        Ok(processes)
    }

    /// Format bytes to human readable format
    pub fn format_bytes(&self, bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }

    /// Format uptime to human readable format
    pub fn format_uptime(&self, seconds: u64) -> String {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;

        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, minutes, secs)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, secs)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, secs)
        } else {
            format!("{}s", secs)
        }
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}