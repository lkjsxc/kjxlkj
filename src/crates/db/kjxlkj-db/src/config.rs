//! Database configuration

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub app_name: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_ms: u64,
    pub idle_timeout_ms: u64,
    pub statement_timeout_ms: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            app_name: "kjxlkj".to_string(),
            max_connections: 20,
            min_connections: 2,
            connect_timeout_ms: 5000,
            idle_timeout_ms: 30000,
            statement_timeout_ms: 15000,
        }
    }
}
