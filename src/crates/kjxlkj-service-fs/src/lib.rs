//! Filesystem IO/watch service.

use anyhow::Result;
use std::path::Path;

/// FS service handle.
pub struct FsService;

impl FsService {
    /// Create a new FS service.
    pub fn new() -> Self {
        Self
    }

    /// Read a file to string.
    pub async fn read_file(path: &Path) -> Result<String> {
        Ok(tokio::fs::read_to_string(path).await?)
    }

    /// Write a file.
    pub async fn write_file(path: &Path, content: &str) -> Result<()> {
        tokio::fs::write(path, content).await?;
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

    #[tokio::test]
    async fn test_read_write() {
        let dir = std::env::temp_dir();
        let path = dir.join("kjxlkj_test_file.txt");
        FsService::write_file(&path, "hello").await.unwrap();
        let content = FsService::read_file(&path).await.unwrap();
        assert_eq!(content, "hello");
        let _ = std::fs::remove_file(&path);
    }
}
