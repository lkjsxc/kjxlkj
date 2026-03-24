use std::path::PathBuf;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub bind_host: String,
    pub bind_port: u16,
    pub data_root: PathBuf,
    pub database_url: String,
    pub admin_token: String,
    pub session_timeout_minutes: Option<i32>,
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
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| AppError::Config("DATABASE_URL must be set".to_owned()))?;
        if database_url.trim().is_empty() {
            return Err(AppError::Config(
                "DATABASE_URL must not be empty".to_owned(),
            ));
        }
        let admin_token =
            std::env::var("ADMIN_TOKEN").unwrap_or_else(|_| "local-dev-token".to_owned());
        if admin_token.trim().is_empty() {
            return Err(AppError::Config("ADMIN_TOKEN must not be empty".to_owned()));
        }
        let session_timeout_minutes = std::env::var("SESSION_TIMEOUT_MINUTES")
            .ok()
            .map(|raw| {
                raw.parse::<i32>().map_err(|_| {
                    AppError::Config("SESSION_TIMEOUT_MINUTES must be a valid i32".to_owned())
                })
            })
            .transpose()?;
        Ok(Self {
            bind_host,
            bind_port,
            data_root,
            database_url,
            admin_token,
            session_timeout_minutes,
        })
    }
}
