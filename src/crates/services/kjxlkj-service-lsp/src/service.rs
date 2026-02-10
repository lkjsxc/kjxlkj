//! LSP service implementation.

use thiserror::Error;
use tracing::info;

/// LSP service error.
#[derive(Debug, Error)]
pub enum LspError {
    #[error("LSP operation failed: {0}")]
    Operation(String),
    #[error("Server not running")]
    NotRunning,
}

/// LSP service.
pub struct LspService;

impl LspService {
    /// Create a new LSP service.
    pub fn new() -> Self {
        Self
    }

    /// Initialize LSP.
    pub fn init(&self) {
        info!("Initializing LSP service");
    }
}

impl Default for LspService {
    fn default() -> Self {
        Self::new()
    }
}
