//! Application configuration

use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVar(String),
    #[error("Invalid port: {0}")]
    InvalidPort(String),
    #[error("Invalid boolean: {0}")]
    InvalidBool(String),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_host: String,
    pub bind_port: u16,
    pub database_url: String,
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_path_style: bool,
    pub setup_code: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            bind_host: env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            bind_port: parse_port("BIND_PORT", "8080")?,
            database_url: required_var("DATABASE_URL")?,
            s3_endpoint: required_var("S3_ENDPOINT")?,
            s3_region: required_var("S3_REGION")?,
            s3_bucket: required_var("S3_BUCKET")?,
            s3_access_key: required_var("S3_ACCESS_KEY")?,
            s3_secret_key: required_var("S3_SECRET_KEY")?,
            s3_path_style: parse_bool("S3_PATH_STYLE", "true")?,
            setup_code: env::var("SETUP_CODE")
                .ok()
                .filter(|value| !value.is_empty()),
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.bind_host, self.bind_port)
    }
}

fn required_var(name: &str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingVar(name.to_string()))
}

fn parse_port(name: &str, default: &str) -> Result<u16, ConfigError> {
    env::var(name)
        .unwrap_or_else(|_| default.to_string())
        .parse::<u16>()
        .map_err(|_| ConfigError::InvalidPort(format!("{name} must be valid port")))
}

fn parse_bool(name: &str, default: &str) -> Result<bool, ConfigError> {
    env::var(name)
        .unwrap_or_else(|_| default.to_string())
        .parse::<bool>()
        .map_err(|_| ConfigError::InvalidBool(format!("{name} must be true or false")))
}
