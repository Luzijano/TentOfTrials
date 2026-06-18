use serde::Serialize;
use std::sync::OnceLock;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

static STARTED_AT: OnceLock<Instant> = OnceLock::new();

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub commit: String,
    pub uptime_seconds: u64,
    pub features: Vec<String>,
}

pub fn health_response(features: impl IntoIterator<Item = String>) -> HealthResponse {
    let started = STARTED_AT.get_or_init(Instant::now);
    HealthResponse {
        status: "ok".to_string(),
        version: crate::VERSION.to_string(),
        commit: std::env::var("GIT_COMMIT")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "unknown".to_string()),
        uptime_seconds: started.elapsed().as_secs(),
        features: features.into_iter().collect(),
    }
}

pub fn health_json(features: impl IntoIterator<Item = String>) -> serde_json::Result<String> {
    serde_json::to_string(&health_response(features))
}

pub fn health_http_response(features: impl IntoIterator<Item = String>) -> serde_json::Result<String> {
    let body = health_json(features)?;
    Ok(format!(
        "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncache-control: no-store\r\ncontent-length: {}\r\n\r\n{}",
        body.len(),
        body
    ))
}

pub async fn serve_health_endpoint(bind_addr: String, features: Vec<String>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&bind_addr).await?;
    tracing::info!(bind_addr = %bind_addr, "health endpoint listening");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let features = features.clone();
        tokio::spawn(async move {
            let mut buffer = [0_u8; 1024];
            let read = socket.read(&mut buffer).await.unwrap_or(0);
            let request = String::from_utf8_lossy(&buffer[..read]);
            let first_line = request.lines().next().unwrap_or_default();
            let response = if first_line.starts_with("GET /health ") {
                health_http_response(features).unwrap_or_else(|_| {
                    "HTTP/1.1 500 Internal Server Error\r\ncontent-length: 0\r\n\r\n".to_string()
                })
            } else {
                "HTTP/1.1 404 Not Found\r\ncontent-length: 0\r\n\r\n".to_string()
            };
            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.shutdown().await;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn health_json_has_required_shape_and_no_secret_dump() {
        let json = health_json(vec!["registry".to_string(), "messaging".to_string()]).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["status"], "ok");
        assert_eq!(value["version"], crate::VERSION);
        assert!(value["commit"].as_str().is_some());
        assert!(value["uptime_seconds"].as_u64().is_some());
        assert_eq!(value["features"].as_array().unwrap().len(), 2);
        assert!(value.get("env").is_none());
        assert!(value.get("secrets").is_none());
    }

    #[test]
    fn health_http_response_is_json() {
        let response = health_http_response(vec!["health-endpoint".to_string()]).unwrap();
        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.contains("content-type: application/json"));
        assert!(response.contains("\"status\":\"ok\""));
        assert!(response.contains("\"commit\""));
        assert!(response.contains("\"uptime_seconds\""));
    }
}
