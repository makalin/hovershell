use crate::error::{HoverShellError, Result};
use log::{error, info};
use std::collections::HashMap;
use std::time::Duration;

pub async fn make_http_request(
    url: &str,
    method: &str,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> Result<HttpResponse> {
    let client = reqwest::Client::new();
    let mut request = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        "HEAD" => client.head(url),
        _ => return Err(HoverShellError::Network(format!("Unsupported HTTP method: {}", method))),
    };
    
    if let Some(headers_map) = headers {
        for (key, value) in headers_map {
            request = request.header(&key, &value);
        }
    }
    
    if let Some(body_data) = body {
        request = request.body(body_data);
    }
    
    let response = request.send().await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    let status = response.status();
    let headers: HashMap<String, String> = response.headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    
    let body = response.text().await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    Ok(HttpResponse {
        status: status.as_u16(),
        headers,
        body,
    })
}

pub async fn make_http_request_with_timeout(
    url: &str,
    method: &str,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
    timeout: Duration,
) -> Result<HttpResponse> {
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    let mut request = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        "HEAD" => client.head(url),
        _ => return Err(HoverShellError::Network(format!("Unsupported HTTP method: {}", method))),
    };
    
    if let Some(headers_map) = headers {
        for (key, value) in headers_map {
            request = request.header(&key, &value);
        }
    }
    
    if let Some(body_data) = body {
        request = request.body(body_data);
    }
    
    let response = request.send().await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    let status = response.status();
    let headers: HashMap<String, String> = response.headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    
    let body = response.text().await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    Ok(HttpResponse {
        status: status.as_u16(),
        headers,
        body,
    })
}

pub async fn download_file(url: &str, file_path: &std::path::Path) -> Result<()> {
    let response = reqwest::get(url).await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    if !response.status().is_success() {
        return Err(HoverShellError::Network(format!("HTTP error: {}", response.status())));
    }
    
    let bytes = response.bytes().await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    tokio::fs::write(file_path, bytes).await
        .map_err(|e| HoverShellError::FileSystem(e.to_string()))?;
    
    Ok(())
}

pub async fn check_url_availability(url: &str) -> Result<bool> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    match client.head(url).send().await {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Ok(false),
    }
}

pub async fn ping_host(host: &str) -> Result<PingResult> {
    // TODO: Implement actual ping functionality for macOS
    // This would involve using system ping command or raw sockets
    Ok(PingResult {
        host: host.to_string(),
        success: false,
        latency: None,
        packet_loss: 100.0,
    })
}

pub async fn resolve_dns(hostname: &str) -> Result<Vec<String>> {
    // TODO: Implement DNS resolution for macOS
    // This would involve using system DNS resolver
    Ok(vec![])
}

pub async fn get_public_ip() -> Result<String> {
    let response = reqwest::get("https://api.ipify.org").await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    let ip = response.text().await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    Ok(ip.trim().to_string())
}

pub async fn get_local_ip() -> Result<String> {
    // TODO: Implement local IP detection for macOS
    Ok("127.0.0.1".to_string())
}

pub async fn test_connection(host: &str, port: u16) -> Result<bool> {
    use tokio::net::TcpStream;
    use std::time::Duration;
    
    match tokio::time::timeout(
        Duration::from_secs(5),
        TcpStream::connect(format!("{}:{}", host, port))
    ).await {
        Ok(Ok(_)) => Ok(true),
        Ok(Err(_)) => Ok(false),
        Err(_) => Ok(false),
    }
}

pub async fn get_network_interfaces() -> Result<Vec<NetworkInterface>> {
    // TODO: Implement network interface detection for macOS
    Ok(vec![])
}

pub async fn get_network_usage() -> Result<NetworkUsage> {
    // TODO: Implement network usage monitoring for macOS
    Ok(NetworkUsage {
        bytes_sent: 0,
        bytes_received: 0,
        packets_sent: 0,
        packets_received: 0,
    })
}

pub async fn create_websocket_connection(url: &str) -> Result<WebSocketConnection> {
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
    use futures_util::{SinkExt, StreamExt};
    
    let (ws_stream, _) = connect_async(url).await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    let (mut write, mut read) = ws_stream.split();
    
    Ok(WebSocketConnection {
        write,
        read,
        url: url.to_string(),
    })
}

pub async fn send_websocket_message(
    connection: &mut WebSocketConnection,
    message: &str,
) -> Result<()> {
    use futures_util::SinkExt;
    use tokio_tungstenite::tungstenite::protocol::Message;
    
    connection.write.send(Message::Text(message.to_string())).await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    Ok(())
}

pub async fn receive_websocket_message(
    connection: &mut WebSocketConnection,
) -> Result<Option<String>> {
    use futures_util::StreamExt;
    use tokio_tungstenite::tungstenite::protocol::Message;
    
    if let Some(msg) = connection.read.next().await {
        match msg {
            Ok(Message::Text(text)) => Ok(Some(text)),
            Ok(Message::Close(_)) => Ok(None),
            Ok(_) => Ok(None),
            Err(e) => Err(HoverShellError::Network(e.to_string())),
        }
    } else {
        Ok(None)
    }
}

pub async fn close_websocket_connection(connection: &mut WebSocketConnection) -> Result<()> {
    use futures_util::SinkExt;
    use tokio_tungstenite::tungstenite::protocol::Message;
    
    connection.write.send(Message::Close(None)).await
        .map_err(|e| HoverShellError::Network(e.to_string()))?;
    
    Ok(())
}

pub fn validate_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

pub fn extract_domain(url: &str) -> Option<String> {
    url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
}

pub fn extract_path(url: &str) -> Option<String> {
    url::Url::parse(url)
        .ok()
        .map(|u| u.path().to_string())
}

pub fn extract_query_params(url: &str) -> HashMap<String, String> {
    url::Url::parse(url)
        .ok()
        .map(|u| {
            u.query_pairs()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

pub fn build_url(base: &str, path: &str, params: Option<HashMap<String, String>>) -> Result<String> {
    let mut url = url::Url::parse(base)
        .map_err(|e| HoverShellError::Network(format!("Invalid base URL: {}", e)))?;
    
    url.set_path(path);
    
    if let Some(params_map) = params {
        let mut query_pairs = url.query_pairs_mut();
        for (key, value) in params_map {
            query_pairs.append_pair(&key, &value);
        }
    }
    
    Ok(url.to_string())
}

pub fn is_local_url(url: &str) -> bool {
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return host == "localhost" || host == "127.0.0.1" || host.starts_with("192.168.") || host.starts_with("10.");
        }
    }
    false
}

pub fn is_https_url(url: &str) -> bool {
    url::Url::parse(url)
        .map(|u| u.scheme() == "https")
        .unwrap_or(false)
}

pub fn get_url_scheme(url: &str) -> Option<String> {
    url::Url::parse(url)
        .ok()
        .map(|u| u.scheme().to_string())
}

pub fn get_url_port(url: &str) -> Option<u16> {
    url::Url::parse(url)
        .ok()
        .and_then(|u| u.port())
}

pub fn normalize_url(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url)
        .map_err(|e| HoverShellError::Network(format!("Invalid URL: {}", e)))?;
    
    Ok(parsed.to_string())
}

pub fn create_http_headers(content_type: Option<&str>, authorization: Option<&str>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    
    if let Some(ct) = content_type {
        headers.insert("Content-Type".to_string(), ct.to_string());
    }
    
    if let Some(auth) = authorization {
        headers.insert("Authorization".to_string(), auth.to_string());
    }
    
    headers.insert("User-Agent".to_string(), "HoverShell/1.0".to_string());
    
    headers
}

pub fn create_bearer_token(token: &str) -> String {
    format!("Bearer {}", token)
}

pub fn create_basic_auth(username: &str, password: &str) -> String {
    let credentials = format!("{}:{}", username, password);
    let encoded = base64::encode(credentials);
    format!("Basic {}", encoded)
}

pub async fn check_internet_connectivity() -> Result<bool> {
    check_url_availability("https://www.google.com").await
}

pub async fn check_dns_resolution() -> Result<bool> {
    // TODO: Implement DNS resolution check
    Ok(true)
}

pub async fn get_network_speed() -> Result<NetworkSpeed> {
    // TODO: Implement network speed test
    Ok(NetworkSpeed {
        download_speed: 0.0,
        upload_speed: 0.0,
        latency: 0.0,
    })
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct PingResult {
    pub host: String,
    pub success: bool,
    pub latency: Option<f64>,
    pub packet_loss: f64,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_address: String,
    pub mac_address: String,
    pub status: String,
    pub speed: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkUsage {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

#[derive(Debug, Clone)]
pub struct WebSocketConnection {
    pub write: futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::protocol::Message>,
    pub read: futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct NetworkSpeed {
    pub download_speed: f64,
    pub upload_speed: f64,
    pub latency: f64,
}