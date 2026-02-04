//! Filesystem IO/watch service.
//!
//! This crate provides file system operations.

use anyhow::Result;
use std::path::Path;
use tokio::fs;

/// File system service state.
pub struct FsService {
    /// Whether the service is active.
    active: bool,
}

impl FsService {
    /// Create a new FS service.
    pub fn new() -> Self {
        Self { active: false }
    }

    /// Start the service.
    pub fn start(&mut self) {
        self.active = true;
    }

    /// Stop the service.
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Check if active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Read a file asynchronously.
    pub async fn read_file(path: &Path) -> Result<String> {
        let content = fs::read_to_string(path).await?;
        Ok(content)
    }

    /// Write a file asynchronously.
    pub async fn write_file(path: &Path, content: &str) -> Result<()> {
        fs::write(path, content).await?;
        Ok(())
    }

    /// Read a file synchronously (for startup).
    pub fn read_file_sync(path: &Path) -> Result<String> {
        let content = std::fs::read_to_string(path)?;
        Ok(content)
    }

    /// Write a file synchronously.
    pub fn write_file_sync(path: &Path, content: &str) -> Result<()> {
        std::fs::write(path, content)?;
        Ok(())
    }
}

impl Default for FsService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn fs_service_lifecycle() {
        let mut svc = FsService::new();
        assert!(!svc.is_active());
        svc.start();
        assert!(svc.is_active());
    }

    #[test]
    fn read_write_sync() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "test content").unwrap();

        let content = FsService::read_file_sync(file.path()).unwrap();
        assert_eq!(content, "test content");
    }

    #[tokio::test]
    async fn read_write_async() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        FsService::write_file(&path, "async content").await.unwrap();
        let content = FsService::read_file(&path).await.unwrap();
        assert_eq!(content, "async content");
    }
}
