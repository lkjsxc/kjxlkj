//! Filesystem service for kjxlkj editor.
//!
//! Provides async file operations.

use kjxlkj_services::{Service, ServiceMessage};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::fs;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// Filesystem service.
pub struct FsService {
    /// Service name.
    name: String,
}

impl FsService {
    /// Create a new filesystem service.
    pub fn new() -> Self {
        Self {
            name: "fs".to_string(),
        }
    }

    /// Read a file asynchronously.
    pub async fn read_file(path: &PathBuf) -> Result<String, std::io::Error> {
        fs::read_to_string(path).await
    }

    /// Write a file asynchronously.
    pub async fn write_file(path: &PathBuf, content: &str) -> Result<(), std::io::Error> {
        fs::write(path, content).await
    }

    /// Check if file exists.
    pub async fn exists(path: &PathBuf) -> bool {
        fs::metadata(path).await.is_ok()
    }

    /// Create directory.
    pub async fn create_dir(path: &PathBuf) -> Result<(), std::io::Error> {
        fs::create_dir_all(path).await
    }

    /// List directory contents.
    pub async fn read_dir(path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
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

impl Service for FsService {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(
        self: Box<Self>,
        mut rx: mpsc::Receiver<ServiceMessage>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            info!("Filesystem service started");

            while let Some(msg) = rx.recv().await {
                match msg {
                    ServiceMessage::Shutdown => {
                        info!("Filesystem service shutting down");
                        break;
                    }
                    ServiceMessage::Custom(cmd) => {
                        debug!(%cmd, "Received command");
                        // Handle fs commands
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs_service_new() {
        let service = FsService::new();
        assert_eq!(service.name(), "fs");
    }
}
