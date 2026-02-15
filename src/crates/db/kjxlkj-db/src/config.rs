//! Runtime configuration loading per /docs/spec/architecture/configuration.md.

use serde::Deserialize;

/// Top-level config from data/config.json.
#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub default_level: String,
    pub json: bool,
    pub request_log: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub static_dir: String,
    pub request_timeout_ms: u64,
    pub max_request_body_mb: usize,
    pub cors_allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub app_name: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_ms: u64,
    pub idle_timeout_ms: u64,
    pub statement_timeout_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebsocketConfig {
    pub heartbeat_interval_ms: u64,
    pub client_timeout_ms: u64,
    pub replay_batch_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditorConfig {
    pub autosave_debounce_ms: u64,
    pub conflict_retry_limit: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AutomationConfig {
    pub default_provider_kind: String,
    pub base_url: String,
    pub model: String,
    pub timeout_ms: u64,
    pub max_tokens: u32,
    pub temperature: f64,
    pub fallback_models: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub attachments_dir: String,
    pub backups_dir: String,
    pub max_attachment_mb: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeaturesConfig {
    pub dashboard_enabled: bool,
    pub librarian_enabled: bool,
    pub saved_views_enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    pub secure_cookies: bool,
    pub same_site: String,
    pub csrf_header: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthConfig {
    pub healthz_path: String,
    pub readyz_path: String,
}

impl AppConfig {
    /// Load config from a JSON file path.
    pub fn load(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config at {path}: {e}"))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Invalid JSON config at {path}: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let json = r#"{
            "logging": {"default_level":"info","json":true,"request_log":true},
            "server": {"bind_addr":"0.0.0.0:8080","static_dir":"./static","request_timeout_ms":15000,"max_request_body_mb":16,"cors_allowed_origins":["http://127.0.0.1:8080"]},
            "database": {"app_name":"kjxlkj","max_connections":20,"min_connections":2,"connect_timeout_ms":5000,"idle_timeout_ms":30000,"statement_timeout_ms":15000},
            "websocket": {"heartbeat_interval_ms":10000,"client_timeout_ms":30000,"replay_batch_size":200},
            "editor": {"autosave_debounce_ms":800,"conflict_retry_limit":2},
            "automation": {"default_provider_kind":"lmstudio","base_url":"http://127.0.0.1:1234/v1","model":"local-model","timeout_ms":30000,"max_tokens":2048,"temperature":0.1,"fallback_models":[]},
            "storage": {"attachments_dir":"./data/attachments","backups_dir":"./data/backups","max_attachment_mb":500},
            "features": {"dashboard_enabled":false,"librarian_enabled":true,"saved_views_enabled":true},
            "security": {"secure_cookies":false,"same_site":"lax","csrf_header":"X-CSRF-Token"},
            "health": {"healthz_path":"/api/healthz","readyz_path":"/api/readyz"}
        }"#;
        let config: AppConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.server.bind_addr, "0.0.0.0:8080");
        assert_eq!(config.database.max_connections, 20);
        assert!(config.features.librarian_enabled);
    }
}
