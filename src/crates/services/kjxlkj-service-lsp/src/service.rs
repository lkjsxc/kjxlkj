//! LSP service implementation.

use std::collections::HashMap;
use thiserror::Error;
use tracing::info;

/// LSP service error.
#[derive(Debug, Error)]
pub enum LspError {
    #[error("LSP operation failed: {0}")]
    Operation(String),
    #[error("Server not running")]
    NotRunning,
    #[error("Server not found for filetype: {0}")]
    NoServer(String),
}

/// LSP server state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerState {
    /// Server not started.
    Stopped,
    /// Server starting/initializing.
    Starting,
    /// Server running and ready.
    Running,
    /// Server shutting down.
    ShuttingDown,
}

/// Configuration for a language server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Command to spawn the server.
    pub cmd: Vec<String>,
    /// File extensions this server handles.
    pub filetypes: Vec<String>,
    /// Root markers for project detection.
    pub root_markers: Vec<String>,
}

impl ServerConfig {
    /// Create a basic server config.
    pub fn new(cmd: Vec<String>, filetypes: Vec<String>) -> Self {
        Self {
            cmd,
            filetypes,
            root_markers: vec![".git".to_string()],
        }
    }
}

/// LSP service managing multiple language servers.
pub struct LspService {
    /// Server configurations by filetype.
    configs: HashMap<String, ServerConfig>,
    /// Server states by filetype.
    states: HashMap<String, ServerState>,
}

impl LspService {
    /// Create a new LSP service with default configurations.
    pub fn new() -> Self {
        let mut configs = HashMap::new();
        
        // Built-in defaults
        configs.insert(
            "rust".to_string(),
            ServerConfig::new(
                vec!["rust-analyzer".to_string()],
                vec!["rs".to_string()],
            ),
        );
        configs.insert(
            "typescript".to_string(),
            ServerConfig::new(
                vec!["typescript-language-server".to_string(), "--stdio".to_string()],
                vec!["ts".to_string(), "tsx".to_string(), "js".to_string(), "jsx".to_string()],
            ),
        );
        configs.insert(
            "python".to_string(),
            ServerConfig::new(
                vec!["pylsp".to_string()],
                vec!["py".to_string()],
            ),
        );
        
        Self {
            configs,
            states: HashMap::new(),
        }
    }

    /// Initialize the LSP service.
    pub fn init(&self) {
        info!("Initializing LSP service with {} server configs", self.configs.len());
    }

    /// Get server state for a filetype.
    pub fn server_state(&self, filetype: &str) -> ServerState {
        self.states.get(filetype).cloned().unwrap_or(ServerState::Stopped)
    }

    /// Start a server for a filetype.
    pub fn start_server(&mut self, filetype: &str) -> Result<(), LspError> {
        if !self.configs.contains_key(filetype) {
            return Err(LspError::NoServer(filetype.to_string()));
        }
        info!("Starting LSP server for {}", filetype);
        self.states.insert(filetype.to_string(), ServerState::Running);
        Ok(())
    }

    /// Stop a server for a filetype.
    pub fn stop_server(&mut self, filetype: &str) -> Result<(), LspError> {
        self.states.insert(filetype.to_string(), ServerState::Stopped);
        Ok(())
    }

    /// Find server config for a file extension.
    pub fn find_server_for_extension(&self, ext: &str) -> Option<&ServerConfig> {
        self.configs.values().find(|c| c.filetypes.contains(&ext.to_string()))
    }

    /// Get number of configured servers.
    pub fn server_count(&self) -> usize {
        self.configs.len()
    }
}

impl Default for LspService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_service_creation() {
        let service = LspService::new();
        assert!(service.server_count() >= 3);
    }

    #[test]
    fn test_find_server_for_rust() {
        let service = LspService::new();
        let config = service.find_server_for_extension("rs");
        assert!(config.is_some());
        assert!(config.unwrap().cmd.contains(&"rust-analyzer".to_string()));
    }

    #[test]
    fn test_server_lifecycle() {
        let mut service = LspService::new();
        assert_eq!(service.server_state("rust"), ServerState::Stopped);
        
        service.start_server("rust").unwrap();
        assert_eq!(service.server_state("rust"), ServerState::Running);
        
        service.stop_server("rust").unwrap();
        assert_eq!(service.server_state("rust"), ServerState::Stopped);
    }

    #[test]
    fn test_unknown_filetype_error() {
        let mut service = LspService::new();
        let result = service.start_server("unknown");
        assert!(matches!(result, Err(LspError::NoServer(_))));
    }
}
