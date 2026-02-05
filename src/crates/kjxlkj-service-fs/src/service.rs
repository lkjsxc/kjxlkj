//! Filesystem service implementation.

use std::path::Path;
use tokio::fs;

/// Filesystem service for file operations.
pub struct FsService {
    running: bool,
}

impl FsService {
    /// Create a new filesystem service.
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Start the service.
    pub async fn start(&mut self) {
        self.running = true;
        tracing::info!("Filesystem service started");
    }

    /// Stop the service.
    pub async fn stop(&mut self) {
        self.running = false;
        tracing::info!("Filesystem service stopped");
    }

    /// Check if the service is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Read a file.
    pub async fn read_file(&self, path: &Path) -> std::io::Result<String> {
        fs::read_to_string(path).await
    }

    /// Write a file.
    pub async fn write_file(&self, path: &Path, content: &str) -> std::io::Result<()> {
        fs::write(path, content).await
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

    #[tokio::test]
    async fn test_fs_service() {
        let mut svc = FsService::new();
        assert!(!svc.is_running());
        svc.start().await;
        assert!(svc.is_running());
    }
}
