//! Application configuration

use std::env;
use thiserror::Error;
use url::Url;

/// Configuration error
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVar(String),
    #[error("Invalid port: {0}")]
    InvalidPort(String),
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub bind_host: String,
    pub bind_port: u16,
    pub database_url: String,
    pub public_base_url: Option<String>,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        let bind_host = env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let bind_port = env::var("BIND_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidPort("BIND_PORT must be valid port".to_string()))?;

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVar("DATABASE_URL".to_string()))?;
        let public_base_url = env::var("PUBLIC_BASE_URL")
            .ok()
            .and_then(|value| normalize_public_base_url(&value));

        Ok(Self {
            bind_host,
            bind_port,
            database_url,
            public_base_url,
        })
    }

    /// Get bind address as string
    pub fn bind_addr(&self) -> String {
        format!(
            "{host}:{port}",
            host = self.bind_host,
            port = self.bind_port
        )
    }
}

fn normalize_public_base_url(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut url = Url::parse(trimmed).ok()?;
    if !matches!(url.scheme(), "http" | "https") {
        return None;
    }
    if url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
    {
        return None;
    }
    url.set_path("");
    Some(url.as_str().trim_end_matches('/').to_string())
}
