use std::path::PathBuf;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub bind_host: String,
    pub bind_port: u16,
    pub data_root: PathBuf,
    pub admin_token: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let bind_host = std::env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_owned());
        let bind_port = std::env::var("BIND_PORT")
            .unwrap_or_else(|_| "8080".to_owned())
            .parse::<u16>()
            .map_err(|_| AppError::Config("BIND_PORT must be a valid u16".to_owned()))?;
        let data_root =
            PathBuf::from(std::env::var("DATA_ROOT").unwrap_or_else(|_| "data".to_owned()));
        let admin_token =
            std::env::var("ADMIN_TOKEN").unwrap_or_else(|_| "local-dev-token".to_owned());
        if admin_token.trim().is_empty() {
            return Err(AppError::Config("ADMIN_TOKEN must not be empty".to_owned()));
        }
        Ok(Self {
            bind_host,
            bind_port,
            data_root,
            admin_token,
        })
    }
}
