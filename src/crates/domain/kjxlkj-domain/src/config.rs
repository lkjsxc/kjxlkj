/// Runtime configuration types per /docs/spec/architecture/configuration.md
use serde::{Deserialize, Serialize};

/// Top-level config loaded from data/config.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub websocket: WebsocketConfig,
    pub editor: EditorConfig,
    pub search: SearchConfig,
    pub automation: AutomationProviderConfig,
    pub agent: AgentConfig,
    pub storage: StorageConfig,
    pub features: FeaturesConfig,
    pub security: SecurityConfig,
    pub health: HealthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub default_level: String,
    pub json: bool,
    pub request_log: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub static_dir: String,
    pub request_timeout_ms: u64,
    pub max_request_body_mb: u64,
    pub cors_allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub app_name: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_ms: u64,
    pub idle_timeout_ms: u64,
    pub statement_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketConfig {
    pub heartbeat_interval_ms: u64,
    pub client_timeout_ms: u64,
    pub replay_batch_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub autosave_debounce_ms: u64,
    pub conflict_retry_limit: u32,
    pub compact_menu_breakpoint_px: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub default_mode: String,
    pub embedding_provider: String,
    pub embedding_model: String,
    pub vector_dimensions: u32,
    pub semantic_enabled: bool,
    pub reindex_batch_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationProviderConfig {
    pub default_provider_kind: String,
    pub base_url: String,
    pub model: String,
    pub timeout_ms: u64,
    pub max_tokens: u32,
    pub temperature: f64,
    pub fallback_models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub mode: String,
    pub prompt_path: String,
    pub memory_store_path: String,
    pub retain_full_conversation_logs: bool,
    pub loop_delay_ms: u64,
    pub idle_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub attachments_dir: String,
    pub backups_dir: String,
    pub max_attachment_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub dashboard_enabled: bool,
    pub librarian_enabled: bool,
    pub saved_views_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub secure_cookies: bool,
    pub same_site: String,
    pub csrf_header: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    pub healthz_path: String,
    pub readyz_path: String,
}

impl AppConfig {
    /// Load config from file path
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("read config: {e}"))?;
        serde_json::from_str(&content).map_err(|e| format!("parse config: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let workspace_root = std::path::Path::new(manifest_dir)
            .ancestors()
            .nth(4)
            .expect("workspace root");
        let path = workspace_root.join("data/config.json");
        let config = AppConfig::load_from_file(path.to_str().unwrap())
            .expect("config should load");
        assert_eq!(config.agent.name, "kjxlkj-agent");
        assert!(!config.agent.retain_full_conversation_logs);
        assert_eq!(config.server.bind_addr, "0.0.0.0:8080");
    }
}
