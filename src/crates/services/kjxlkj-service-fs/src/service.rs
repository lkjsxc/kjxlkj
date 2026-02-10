//! FS service implementation.

use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;
use tracing::info;

/// FS service error.
#[derive(Debug, Error)]
pub enum FsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Path not found: {0}")]
    NotFound(PathBuf),
}

/// FS service.
pub struct FsService;

impl FsService {
    /// Create a new FS service.
    pub fn new() -> Self {
        Self
    }

    /// Read file contents.
    pub async fn read_file(&self, path: &PathBuf) -> Result<String, FsError> {
        info!(?path, "Reading file");
        let content = fs::read_to_string(path).await?;
        Ok(content)
    }

    /// Write file contents.
    pub async fn write_file(&self, path: &PathBuf, content: &str) -> Result<(), FsError> {
        info!(?path, "Writing file");
        fs::write(path, content).await?;
        Ok(())
    }

    /// Check if path exists.
    pub async fn exists(&self, path: &PathBuf) -> bool {
        fs::metadata(path).await.is_ok()
    }

    /// List directory.
    pub async fn list_dir(&self, path: &PathBuf) -> Result<Vec<PathBuf>, FsError> {
        info!(?path, "Listing directory");
        let mut entries = Vec::new();
        let mut dir = fs::read_dir(path).await?;
        while let Some(entry) = dir.next_entry().await? {
            entries.push(entry.path());
        }
        Ok(entries)
    }
}

impl Default for FsService {
    fn default() -> Self {
        Self::new()
    }
}
