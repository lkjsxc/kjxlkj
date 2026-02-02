//! Filesystem service.

use crate::ops;
use anyhow::Result;
use std::path::{Path, PathBuf};

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

    /// Saves content to a file.
    pub async fn save(&self, path: impl AsRef<Path>, content: &str) -> Result<()> {
        ops::write_file(path, content).await
    }

    /// Loads content from a file.
    pub async fn load(&self, path: impl AsRef<Path>) -> Result<String> {
        ops::read_file(path).await
    }

    /// Runs the service.
    pub async fn run(self) {
        // Service loop - future message handling
    }
}
