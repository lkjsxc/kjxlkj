//! Configuration loading

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub static_dir: String,
    pub request_timeout_ms: u64,
    pub max_request_body_mb: u64,
    pub cors_allowed_origins: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:8080".to_string(),
            static_dir: "./static".to_string(),
            request_timeout_ms: 15000,
            max_request_body_mb: 16,
            cors_allowed_origins: vec!["http://127.0.0.1:8080".to_string()],
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub default_level: String,
    pub json: bool,
    pub request_log: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            default_level: "info".to_string(),
            json: true,
            request_log: true,
        }
    }
}

/// Full application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
    pub server: ServerConfig,
    pub database: kjxlkj_db::DatabaseConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            logging: LoggingConfig::default(),
            server: ServerConfig::default(),
            database: kjxlkj_db::DatabaseConfig::default(),
        }
    }
}

/// Load configuration from file
pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = Path::new("./data/config.json");
    
    if config_path.exists() {
        let content = std::fs::read_to_string(config_path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        // Use defaults
        Ok(AppConfig::default())
    }
}
