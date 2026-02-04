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
    use tempfile::tempdir;

    #[test]
    fn test_fs_service_new() {
        let service = FsService::new();
        assert_eq!(service.name(), "fs");
    }

    #[test]
    fn test_fs_service_default() {
        let service = FsService::default();
        assert_eq!(service.name(), "fs");
    }

    #[tokio::test]
    async fn test_read_write_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        
        FsService::write_file(&path, "hello").await.unwrap();
        let content = FsService::read_file(&path).await.unwrap();
        assert_eq!(content, "hello");
    }

    #[tokio::test]
    async fn test_exists() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("exists.txt");
        
        assert!(!FsService::exists(&path).await);
        FsService::write_file(&path, "test").await.unwrap();
        assert!(FsService::exists(&path).await);
    }

    #[tokio::test]
    async fn test_create_dir() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("subdir/nested");
        
        FsService::create_dir(&path).await.unwrap();
        assert!(FsService::exists(&path).await);
    }

    #[tokio::test]
    async fn test_read_dir() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        
        FsService::write_file(&path.join("a.txt"), "a").await.unwrap();
        FsService::write_file(&path.join("b.txt"), "b").await.unwrap();
        
        let entries = FsService::read_dir(&path).await.unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[tokio::test]
    async fn test_read_nonexistent_file() {
        let path = PathBuf::from("/nonexistent/path/file.txt");
        let result = FsService::read_file(&path).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_write_with_utf8() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("unicode.txt");
        let content = "Hello 世界 🌍";
        
        FsService::write_file(&path, content).await.unwrap();
        let read = FsService::read_file(&path).await.unwrap();
        assert_eq!(read, content);
    }

    #[tokio::test]
    async fn test_overwrite_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("overwrite.txt");
        
        FsService::write_file(&path, "first").await.unwrap();
        FsService::write_file(&path, "second").await.unwrap();
        
        let content = FsService::read_file(&path).await.unwrap();
        assert_eq!(content, "second");
    }
}
