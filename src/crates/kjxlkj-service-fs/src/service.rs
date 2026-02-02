//! Filesystem service.

use std::path::PathBuf;
use tokio::sync::mpsc;

/// Filesystem service.
pub struct FsService {
    /// Root path.
    root: PathBuf,
}

impl FsService {
    /// Creates a new filesystem service.
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Returns the root path.
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// Runs the service.
    pub async fn run(self) {
        // Service loop
    }
}
