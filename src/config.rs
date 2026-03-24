//! Application configuration

use std::env;
use thiserror::Error;

/// Configuration error
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVar(String),
    #[error("Invalid port: {0}")]
    InvalidPort(String),
    #[error("Invalid timeout: {0}")]
    InvalidTimeout(String),
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub bind_host: String,
    pub bind_port: u16,
    pub data_root: String,
    pub database_url: String,
    pub admin_token: String,
    pub session_timeout_minutes: u32,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        let bind_host = env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let bind_port = env::var("BIND_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidPort("BIND_PORT must be valid port".to_string()))?;

        let data_root = env::var("DATA_ROOT").unwrap_or_else(|_| "/app/data".to_string());

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVar("DATABASE_URL".to_string()))?;

        let admin_token = env::var("ADMIN_TOKEN")
            .map_err(|_| ConfigError::MissingVar("ADMIN_TOKEN".to_string()))?;

        let session_timeout_minutes = env::var("SESSION_TIMEOUT_MINUTES")
            .unwrap_or_else(|_| "1440".to_string())
            .parse::<u32>()
            .map_err(|_| {
                ConfigError::InvalidTimeout("SESSION_TIMEOUT_MINUTES must be a number".to_string())
            })?
            .clamp(5, 10080);

        Ok(Self {
            bind_host,
            bind_port,
            data_root,
            database_url,
            admin_token,
            session_timeout_minutes,
        })
    }

    /// Get bind address as string
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.bind_host, self.bind_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_clamping() {
        env::set_var("DATABASE_URL", "postgres://test");
        env::set_var("ADMIN_TOKEN", "test-token");
        env::set_var("SESSION_TIMEOUT_MINUTES", "1");

        let config = Config::from_env().unwrap();
        assert_eq!(config.session_timeout_minutes, 5);

        env::set_var("SESSION_TIMEOUT_MINUTES", "99999");
        let config = Config::from_env().unwrap();
        assert_eq!(config.session_timeout_minutes, 10080);

        env::remove_var("SESSION_TIMEOUT_MINUTES");
    }
}
