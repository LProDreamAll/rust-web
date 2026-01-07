use serde::{Deserialize, Serialize};

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default = "default_max_payload_size")]
    pub max_payload_size: usize,
    #[serde(default = "default_request_timeout")]
    pub request_timeout_secs: u64,
}

fn default_max_payload_size() -> usize {
    536_870_912 // 512MB
}

fn default_request_timeout() -> u64 {
    1800 // 30 minutes
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            log_dir: None,
            log_level: None,
            max_payload_size: default_max_payload_size(),
            request_timeout_secs: default_request_timeout(),
        }
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn log_dir(mut self, dir: impl Into<String>) -> Self {
        self.log_dir = Some(dir.into());
        self
    }

    pub fn log_level(mut self, level: impl Into<String>) -> Self {
        self.log_level = Some(level.into());
        self
    }

    pub fn max_payload_size(mut self, size: usize) -> Self {
        self.max_payload_size = size;
        self
    }

    pub fn request_timeout_secs(mut self, secs: u64) -> Self {
        self.request_timeout_secs = secs;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("Port must be greater than 0".to_string());
        }

        if self.host.is_empty() {
            return Err("Host cannot be empty".to_string());
        }

        if self.max_payload_size == 0 {
            return Err("Max payload size must be greater than 0".to_string());
        }

        Ok(())
    }

    /// Load from JSON file
    pub fn from_json_file(path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
    }

    /// Save to JSON file
    pub fn to_json_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        std::fs::write(path, content).map_err(|e| format!("Failed to write config file: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.max_payload_size, 536_870_912);
        assert_eq!(config.request_timeout_secs, 1800);
        assert!(config.log_dir.is_none());
        assert!(config.log_level.is_none());
    }

    #[test]
    fn test_server_config_builder() {
        let config = ServerConfig::new()
            .host("127.0.0.1")
            .port(8080)
            .log_dir("/var/log")
            .log_level("debug")
            .max_payload_size(1024 * 1024)
            .request_timeout_secs(60);

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.log_dir, Some("/var/log".to_string()));
        assert_eq!(config.log_level, Some("debug".to_string()));
        assert_eq!(config.max_payload_size, 1024 * 1024);
        assert_eq!(config.request_timeout_secs, 60);
    }

    #[test]
    fn test_addr() {
        let config = ServerConfig::new().host("localhost").port(9000);
        assert_eq!(config.addr(), "localhost:9000");
    }

    #[test]
    fn test_validate() {
        let config = ServerConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config = ServerConfig::new().port(0);
        assert!(invalid_config.validate().is_err());

        let invalid_config = ServerConfig::new().host("");
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let config = ServerConfig::new()
            .host("0.0.0.0")
            .port(3000)
            .log_level("info");

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ServerConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.host, deserialized.host);
        assert_eq!(config.port, deserialized.port);
        assert_eq!(config.log_level, deserialized.log_level);
    }
}
