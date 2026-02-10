//! Index service implementation.

use thiserror::Error;
use tracing::info;

/// Index service error.
#[derive(Debug, Error)]
pub enum IndexError {
    #[error("Index operation failed: {0}")]
    Operation(String),
}

/// Index service.
pub struct IndexService;

impl IndexService {
    /// Create a new Index service.
    pub fn new() -> Self {
        Self
    }

    /// Initialize indexing.
    pub fn init(&self) {
        info!("Initializing index service");
    }
}

impl Default for IndexService {
    fn default() -> Self {
        Self::new()
    }
}
