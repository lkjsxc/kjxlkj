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

    #[test]
    fn fs_read_nonexistent_file_fails() {
        let path = Path::new("/nonexistent/path/to/file.txt");
        let result = FsService::read_file(path);
        assert!(result.is_err());
    }

    #[test]
    fn fs_exists_returns_false_for_nonexistent() {
        let path = Path::new("/nonexistent/path/to/file.txt");
        assert!(!FsService::exists(path));
    }

    #[test]
    fn fs_exists_returns_true_for_existing() {
        let dir = std::env::temp_dir();
        let path = dir.join("kjxlkj_exists_test.txt");
        FsService::write_file(&path, "exists").unwrap();
        assert!(FsService::exists(&path));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn fs_default_impl() {
        let _svc = FsService::default();
    }

    #[test]
    fn fs_write_empty_content() {
        let dir = std::env::temp_dir();
        let path = dir.join("kjxlkj_empty_test.txt");
        FsService::write_file(&path, "").unwrap();
        let content = FsService::read_file(&path).unwrap();
        assert!(content.is_empty());
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn fs_write_multiline() {
        let dir = std::env::temp_dir();
        let path = dir.join("kjxlkj_multiline_test.txt");
        FsService::write_file(&path, "line1\nline2\nline3").unwrap();
        let content = FsService::read_file(&path).unwrap();
        assert!(content.contains("line1"));
        assert!(content.contains("line2"));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn fs_overwrite_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("kjxlkj_overwrite_test.txt");
        FsService::write_file(&path, "original").unwrap();
        FsService::write_file(&path, "overwritten").unwrap();
        let content = FsService::read_file(&path).unwrap();
        assert_eq!(content, "overwritten");
        std::fs::remove_file(&path).ok();
    }
}
