//! Application configuration

use std::env;
use std::net::IpAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVar(String),
    #[error("Invalid port: {0}")]
    InvalidPort(String),
    #[error("Invalid boolean: {0}")]
    InvalidBool(String),
    #[error("Invalid number: {0}")]
    InvalidNumber(String),
    #[error("Invalid IP address: {0}")]
    InvalidIp(String),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_host: String,
    pub bind_port: u16,
    pub live_ice_bind_ip: String,
    pub live_ice_udp_port: u16,
    pub live_ice_public_ips: Vec<String>,
    pub live_ice_lan_ips: Vec<String>,
    pub live_trusted_proxy_ips: Vec<IpAddr>,
    pub database_url: String,
    pub seaweedfs_s3_endpoint: String,
    pub seaweedfs_s3_region: String,
    pub seaweedfs_s3_bucket: String,
    pub seaweedfs_s3_access_key: String,
    pub seaweedfs_s3_secret_key: String,
    pub seaweedfs_s3_path_style: bool,
    pub media_upload_max_bytes: usize,
    pub site_icon_upload_max_bytes: usize,
    pub setup_code: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            bind_host: env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            bind_port: parse_port("BIND_PORT", "8080")?,
            live_ice_bind_ip: env::var("LIVE_ICE_BIND_IP")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            live_ice_udp_port: parse_port("LIVE_ICE_UDP_PORT", "8189")?,
            live_ice_public_ips: parse_csv("LIVE_ICE_PUBLIC_IPS"),
            live_ice_lan_ips: parse_csv("LIVE_ICE_LAN_IPS"),
            live_trusted_proxy_ips: parse_ip_csv("LIVE_TRUSTED_PROXY_IPS")?,
            database_url: required_var("DATABASE_URL")?,
            seaweedfs_s3_endpoint: required_var("SEAWEEDFS_S3_ENDPOINT")?,
            seaweedfs_s3_region: required_var("SEAWEEDFS_S3_REGION")?,
            seaweedfs_s3_bucket: required_var("SEAWEEDFS_S3_BUCKET")?,
            seaweedfs_s3_access_key: required_var("SEAWEEDFS_S3_ACCESS_KEY")?,
            seaweedfs_s3_secret_key: required_var("SEAWEEDFS_S3_SECRET_KEY")?,
            seaweedfs_s3_path_style: parse_bool("SEAWEEDFS_S3_PATH_STYLE", "true")?,
            media_upload_max_bytes: parse_usize("MEDIA_UPLOAD_MAX_BYTES", "536870912")?,
            site_icon_upload_max_bytes: parse_usize("SITE_ICON_UPLOAD_MAX_BYTES", "2097152")?,
            setup_code: env::var("SETUP_CODE")
                .ok()
                .filter(|value| !value.is_empty()),
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.bind_host, self.bind_port)
    }

    pub fn live_ice_addr(&self) -> String {
        format!("{}:{}", self.live_ice_bind_ip, self.live_ice_udp_port)
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

fn parse_usize(name: &str, default: &str) -> Result<usize, ConfigError> {
    env::var(name)
        .unwrap_or_else(|_| default.to_string())
        .parse::<usize>()
        .map_err(|_| ConfigError::InvalidNumber(format!("{name} must be a positive integer")))
}

fn parse_csv(name: &str) -> Vec<String> {
    env::var(name)
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect()
}

fn parse_ip_csv(name: &str) -> Result<Vec<IpAddr>, ConfigError> {
    parse_csv(name)
        .into_iter()
        .map(|value| {
            value
                .parse()
                .map_err(|_| ConfigError::InvalidIp(format!("{name} contains {value}")))
        })
        .collect()
}
