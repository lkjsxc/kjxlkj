//! Filesystem IO/watch service.

use anyhow::Result;
use std::path::Path;

/// FS service for file operations.
pub struct FsService {
    // Future: manage async file operations
}

impl FsService {
    /// Create a new FS service.
    pub fn new() -> Self {
        Self {}
    }

    /// Read file contents.
    pub fn read_file(path: &Path) -> Result<String> {
        let content = std::fs::read_to_string(path)?;
        Ok(content)
    }

    /// Write file contents.
    pub fn write_file(path: &Path, content: &str) -> Result<()> {
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Check if file exists.
    pub fn exists(path: &Path) -> bool {
        path.exists()
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

    #[test]
    fn fs_service_creation() {
        let _svc = FsService::new();
    }

    #[test]
    fn fs_read_write() {
        let dir = std::env::temp_dir();
        let path = dir.join("kjxlkj_test_file.txt");
        FsService::write_file(&path, "test content").unwrap();
        let content = FsService::read_file(&path).unwrap();
        assert_eq!(content, "test content");
        std::fs::remove_file(&path).ok();
    }
}
