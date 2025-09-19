use crate::error::{HoverShellError, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResult {
    pub host: String,
    pub ip: Option<String>,
    pub packets_sent: u32,
    pub packets_received: u32,
    pub packet_loss: f32,
    pub min_time: Option<f64>,
    pub max_time: Option<f64>,
    pub avg_time: Option<f64>,
    pub times: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortScanResult {
    pub host: String,
    pub port: u16,
    pub is_open: bool,
    pub response_time: Option<f64>,
    pub service: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<String>,
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: String,
    pub response_time: f64,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsLookupResult {
    pub hostname: String,
    pub ip_addresses: Vec<String>,
    pub aliases: Vec<String>,
    pub query_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteResult {
    pub host: String,
    pub hops: Vec<TracerouteHop>,
    pub total_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteHop {
    pub hop_number: u8,
    pub ip: Option<String>,
    pub hostname: Option<String>,
    pub times: Vec<f64>,
    pub is_final: bool,
}

pub struct NetworkTools {
    default_timeout: Duration,
    user_agent: String,
}

impl NetworkTools {
    pub fn new() -> Self {
        Self {
            default_timeout: Duration::from_secs(10),
            user_agent: "HoverShell/1.0".to_string(),
        }
    }

    /// Ping a host
    pub async fn ping(&self, host: &str, count: Option<u32>) -> Result<PingResult> {
        let count = count.unwrap_or(4);
        let mut result = PingResult {
            host: host.to_string(),
            ip: None,
            packets_sent: count,
            packets_received: 0,
            packet_loss: 0.0,
            min_time: None,
            max_time: None,
            avg_time: None,
            times: Vec::new(),
        };

        // Use system ping command
        let output = tokio::process::Command::new("ping")
            .args(&["-c", &count.to_string(), host])
            .output()
            .await
            .map_err(|e| HoverShellError::Network(format!("Failed to run ping: {}", e)))?;

        if !output.status.success() {
            return Err(HoverShellError::Network("Ping command failed".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        self.parse_ping_output(&output_str, &mut result)?;

        info!("Pinged {}: {} packets sent, {} received", host, result.packets_sent, result.packets_received);
        Ok(result)
    }

    /// Scan ports on a host
    pub async fn scan_ports(&self, host: &str, ports: &[u16], timeout_ms: Option<u64>) -> Result<Vec<PortScanResult>> {
        let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(1000));
        let mut results = Vec::new();

        for &port in ports {
            let start_time = std::time::Instant::now();
            let is_open = self.check_port_open(host, port, timeout_duration).await;
            let response_time = start_time.elapsed().as_secs_f64();

            let service = if is_open {
                self.guess_service(port)
            } else {
                None
            };

            results.push(PortScanResult {
                host: host.to_string(),
                port,
                is_open,
                response_time: if is_open { Some(response_time) } else { None },
                service,
            });
        }

        info!("Scanned {} ports on {}", ports.len(), host);
        Ok(results)
    }

    /// Check if a port is open
    async fn check_port_open(&self, host: &str, port: u16, timeout_duration: Duration) -> bool {
        let addr = format!("{}:{}", host, port);
        
        match timeout(timeout_duration, TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => true,
            Ok(Err(_)) => false,
            Err(_) => false, // Timeout
        }
    }

    /// Guess service name for common ports
    fn guess_service(&self, port: u16) -> Option<String> {
        match port {
            22 => Some("SSH".to_string()),
            23 => Some("Telnet".to_string()),
            25 => Some("SMTP".to_string()),
            53 => Some("DNS".to_string()),
            80 => Some("HTTP".to_string()),
            110 => Some("POP3".to_string()),
            143 => Some("IMAP".to_string()),
            443 => Some("HTTPS".to_string()),
            993 => Some("IMAPS".to_string()),
            995 => Some("POP3S".to_string()),
            3389 => Some("RDP".to_string()),
            5432 => Some("PostgreSQL".to_string()),
            3306 => Some("MySQL".to_string()),
            6379 => Some("Redis".to_string()),
            27017 => Some("MongoDB".to_string()),
            _ => None,
        }
    }

    /// Make HTTP request
    pub async fn http_request(&self, request: &HttpRequest) -> Result<HttpResponse> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(request.timeout.unwrap_or(30)))
            .user_agent(&self.user_agent)
            .build()
            .map_err(|e| HoverShellError::Network(format!("Failed to create HTTP client: {}", e)))?;

        let start_time = std::time::Instant::now();
        
        let mut req_builder = match request.method.to_uppercase().as_str() {
            "GET" => client.get(&request.url),
            "POST" => client.post(&request.url),
            "PUT" => client.put(&request.url),
            "DELETE" => client.delete(&request.url),
            "PATCH" => client.patch(&request.url),
            "HEAD" => client.head(&request.url),
            _ => return Err(HoverShellError::Network(format!("Unsupported HTTP method: {}", request.method))),
        };

        // Add headers
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body for POST/PUT/PATCH
        if let Some(body) = &request.body {
            if matches!(request.method.to_uppercase().as_str(), "POST" | "PUT" | "PATCH") {
                req_builder = req_builder.body(body.clone());
            }
        }

        let response = req_builder.send().await
            .map_err(|e| HoverShellError::Network(format!("HTTP request failed: {}", e)))?;

        let status_code = response.status().as_u16();
        let status_text = response.status().canonical_reason().unwrap_or("Unknown").to_string();
        
        let mut headers = std::collections::HashMap::new();
        for (key, value) in response.headers() {
            headers.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
        }

        let body = response.text().await
            .map_err(|e| HoverShellError::Network(format!("Failed to read response body: {}", e)))?;

        let response_time = start_time.elapsed().as_secs_f64();
        let size = body.len();

        info!("HTTP {} {}: {} {} ({}ms)", request.method, request.url, status_code, status_text, (response_time * 1000.0) as u64);

        Ok(HttpResponse {
            status_code,
            status_text,
            headers,
            body,
            response_time,
            size,
        })
    }

    /// Download a file
    pub async fn download_file(&self, url: &str, output_path: &str) -> Result<usize> {
        let client = reqwest::Client::builder()
            .timeout(self.default_timeout)
            .user_agent(&self.user_agent)
            .build()
            .map_err(|e| HoverShellError::Network(format!("Failed to create HTTP client: {}", e)))?;

        let response = client.get(url).send().await
            .map_err(|e| HoverShellError::Network(format!("Download failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(HoverShellError::Network(format!("Download failed with status: {}", response.status())));
        }

        let bytes = response.bytes().await
            .map_err(|e| HoverShellError::Network(format!("Failed to read response: {}", e)))?;

        tokio::fs::write(output_path, &bytes).await
            .map_err(|e| HoverShellError::Network(format!("Failed to write file: {}", e)))?;

        info!("Downloaded {} bytes to {}", bytes.len(), output_path);
        Ok(bytes.len())
    }

    /// Perform DNS lookup
    pub async fn dns_lookup(&self, hostname: &str) -> Result<DnsLookupResult> {
        let start_time = std::time::Instant::now();
        
        // Use system nslookup command
        let output = tokio::process::Command::new("nslookup")
            .arg(hostname)
            .output()
            .await
            .map_err(|e| HoverShellError::Network(format!("Failed to run nslookup: {}", e)))?;

        let query_time = start_time.elapsed().as_secs_f64();
        let output_str = String::from_utf8_lossy(&output.stdout);

        let mut result = DnsLookupResult {
            hostname: hostname.to_string(),
            ip_addresses: Vec::new(),
            aliases: Vec::new(),
            query_time,
        };

        self.parse_nslookup_output(&output_str, &mut result)?;

        info!("DNS lookup for {}: {} IPs found", hostname, result.ip_addresses.len());
        Ok(result)
    }

    /// Perform traceroute
    pub async fn traceroute(&self, host: &str, max_hops: Option<u8>) -> Result<TracerouteResult> {
        let max_hops = max_hops.unwrap_or(30);
        let start_time = std::time::Instant::now();

        // Use system traceroute command
        let output = tokio::process::Command::new("traceroute")
            .args(&["-m", &max_hops.to_string(), host])
            .output()
            .await
            .map_err(|e| HoverShellError::Network(format!("Failed to run traceroute: {}", e)))?;

        let total_time = start_time.elapsed().as_secs_f64();
        let output_str = String::from_utf8_lossy(&output.stdout);

        let mut result = TracerouteResult {
            host: host.to_string(),
            hops: Vec::new(),
            total_time,
        };

        self.parse_traceroute_output(&output_str, &mut result)?;

        info!("Traceroute to {}: {} hops", host, result.hops.len());
        Ok(result)
    }

    /// Check if a host is reachable
    pub async fn is_reachable(&self, host: &str, timeout_ms: Option<u64>) -> Result<bool> {
        let timeout_duration = Duration::from_millis(timeout_ms.unwrap_or(5000));
        
        // Try to connect to port 80 (HTTP) first
        if self.check_port_open(host, 80, timeout_duration).await {
            return Ok(true);
        }

        // Try to connect to port 443 (HTTPS)
        if self.check_port_open(host, 443, timeout_duration).await {
            return Ok(true);
        }

        // Try ping as last resort
        match self.ping(host, Some(1)).await {
            Ok(result) => Ok(result.packets_received > 0),
            Err(_) => Ok(false),
        }
    }

    /// Get local IP address
    pub async fn get_local_ip(&self) -> Result<String> {
        // Try to connect to a remote address to determine local IP
        let socket = TcpStream::connect("8.8.8.8:80").await
            .map_err(|e| HoverShellError::Network(format!("Failed to determine local IP: {}", e)))?;
        
        let local_addr = socket.local_addr()
            .map_err(|e| HoverShellError::Network(format!("Failed to get local address: {}", e)))?;
        
        Ok(local_addr.ip().to_string())
    }

    /// Parse ping output
    fn parse_ping_output(&self, output: &str, result: &mut PingResult) -> Result<()> {
        for line in output.lines() {
            if line.contains("PING") && line.contains("(") {
                // Extract IP address
                if let Some(start) = line.find('(') {
                    if let Some(end) = line[start..].find(')') {
                        result.ip = Some(line[start+1..start+end].to_string());
                    }
                }
            } else if line.contains("packets transmitted") {
                // Parse packet statistics
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    if let Ok(sent) = parts[0].parse::<u32>() {
                        result.packets_sent = sent;
                    }
                    if let Ok(received) = parts[3].parse::<u32>() {
                        result.packets_received = received;
                    }
                }
            } else if line.contains("packet loss") {
                // Parse packet loss percentage
                if let Some(start) = line.find('%') {
                    if let Some(space_pos) = line[..start].rfind(' ') {
                        if let Ok(loss) = line[space_pos+1..start].parse::<f32>() {
                            result.packet_loss = loss;
                        }
                    }
                }
            } else if line.contains("min/avg/max") {
                // Parse timing statistics
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() >= 2 {
                    let times_str = parts[1].trim();
                    let times: Vec<&str> = times_str.split('/').collect();
                    if times.len() >= 3 {
                        if let Ok(min) = times[0].parse::<f64>() {
                            result.min_time = Some(min);
                        }
                        if let Ok(avg) = times[1].parse::<f64>() {
                            result.avg_time = Some(avg);
                        }
                        if let Ok(max) = times[2].parse::<f64>() {
                            result.max_time = Some(max);
                        }
                    }
                }
            } else if line.contains("time=") {
                // Parse individual ping times
                if let Some(start) = line.find("time=") {
                    if let Some(end) = line[start..].find(' ') {
                        if let Ok(time) = line[start+5..start+end].parse::<f64>() {
                            result.times.push(time);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Parse nslookup output
    fn parse_nslookup_output(&self, output: &str, result: &mut DnsLookupResult) -> Result<()> {
        let mut in_address_section = false;

        for line in output.lines() {
            if line.contains("Name:") {
                in_address_section = true;
            } else if line.contains("Address:") && in_address_section {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    result.ip_addresses.push(parts[1].to_string());
                }
            } else if line.contains("Aliases:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    result.aliases.push(parts[1].to_string());
                }
            }
        }

        Ok(())
    }

    /// Parse traceroute output
    fn parse_traceroute_output(&self, output: &str, result: &mut TracerouteResult) -> Result<()> {
        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(hop_num) = parts[0].parse::<u8>() {
                    let mut hop = TracerouteHop {
                        hop_number: hop_num,
                        ip: None,
                        hostname: None,
                        times: Vec::new(),
                        is_final: false,
                    };

                    // Parse IP and hostname
                    if parts.len() >= 2 {
                        let addr_part = parts[1];
                        if addr_part.contains('(') && addr_part.contains(')') {
                            // Format: hostname (ip)
                            let hostname_end = addr_part.find('(').unwrap();
                            hop.hostname = Some(addr_part[..hostname_end].to_string());
                            
                            let ip_start = addr_part.find('(').unwrap() + 1;
                            let ip_end = addr_part.find(')').unwrap();
                            hop.ip = Some(addr_part[ip_start..ip_end].to_string());
                        } else {
                            // Just IP address
                            hop.ip = Some(addr_part.to_string());
                        }
                    }

                    // Parse timing information
                    for part in &parts[2..] {
                        if part.contains("ms") {
                            if let Ok(time) = part.replace("ms", "").parse::<f64>() {
                                hop.times.push(time);
                            }
                        }
                    }

                    // Check if this is the final hop
                    hop.is_final = hop.ip.as_ref().map_or(false, |ip| ip == &result.host);

                    result.hops.push(hop);
                }
            }
        }

        Ok(())
    }
}

impl Default for NetworkTools {
    fn default() -> Self {
        Self::new()
    }
}