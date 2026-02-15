use serde::Deserialize;

/// Runtime configuration model per /docs/spec/architecture/configuration.md.
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub websocket: WebsocketConfig,
    pub editor: EditorConfig,
    pub automation: AutomationConfig,
    pub storage: StorageConfig,
    pub features: FeaturesConfig,
    pub security: SecurityConfig,
    pub health: HealthConfig,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub default_level: String,
    pub json: bool,
    pub request_log: bool,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub static_dir: String,
    pub request_timeout_ms: u64,
    pub max_request_body_mb: usize,
    pub cors_allowed_origins: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub app_name: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_ms: u64,
    pub idle_timeout_ms: u64,
    pub statement_timeout_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct WebsocketConfig {
    pub heartbeat_interval_ms: u64,
    pub client_timeout_ms: u64,
    pub replay_batch_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct EditorConfig {
    pub autosave_debounce_ms: u64,
    pub conflict_retry_limit: u32,
}

#[derive(Debug, Deserialize)]
pub struct AutomationConfig {
    pub default_provider_kind: String,
    pub base_url: String,
    pub model: String,
    pub timeout_ms: u64,
    pub max_tokens: u32,
    pub temperature: f64,
    pub fallback_models: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    pub attachments_dir: String,
    pub backups_dir: String,
    pub max_attachment_mb: u64,
}

#[derive(Debug, Deserialize)]
pub struct FeaturesConfig {
    pub dashboard_enabled: bool,
    pub librarian_enabled: bool,
    pub saved_views_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub secure_cookies: bool,
    pub same_site: String,
    pub csrf_header: String,
}

#[derive(Debug, Deserialize)]
pub struct HealthConfig {
    pub healthz_path: String,
    pub readyz_path: String,
}

impl AppConfig {
    /// Load config from file per /docs/spec/architecture/configuration.md.
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }
}
