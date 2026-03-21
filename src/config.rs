use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub content_root: PathBuf,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let host_value =
            env::var("BIND_HOST").unwrap_or_else(|_| IpAddr::V4(Ipv4Addr::UNSPECIFIED).to_string());
        let bind_host = host_value
            .parse::<IpAddr>()
            .map_err(|_| AppError::InvalidHost { value: host_value })?;

        let port_value = env::var("BIND_PORT").unwrap_or_else(|_| "8080".to_owned());
        let bind_port = port_value
            .parse::<u16>()
            .map_err(|_| AppError::InvalidPort { value: port_value })?;

        let database_url = env::var("DATABASE_URL").map_err(|_| AppError::MissingEnv {
            key: "DATABASE_URL",
        })?;

        let content_root_value =
            env::var("CONTENT_ROOT").unwrap_or_else(|_| "./data/content".to_owned());
        if content_root_value.trim().is_empty() {
            return Err(AppError::InvalidContentRoot);
        }

        Ok(Self {
            bind_addr: SocketAddr::new(bind_host, bind_port),
            database_url,
            content_root: PathBuf::from(content_root_value),
        })
    }
}
