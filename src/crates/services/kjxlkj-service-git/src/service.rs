//! Git service implementation.

use std::path::PathBuf;
use thiserror::Error;
use tracing::info;

/// Git service error.
#[derive(Debug, Error)]
pub enum GitError {
    #[error("Not a git repository")]
    NotRepo,
    #[error("Git operation failed: {0}")]
    Operation(String),
}

/// Git service.
pub struct GitService;

impl GitService {
    /// Create a new Git service.
    pub fn new() -> Self {
        Self
    }

    /// Check if path is in a git repository.
    pub fn is_repo(&self, _path: &PathBuf) -> bool {
        info!("Checking git repo status");
        false // Placeholder.
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}
