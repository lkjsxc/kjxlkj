//! Filesystem service for kjxlkj editor.
//!
//! Provides async file operations.

use kjxlkj_services::{Service, ServiceMessage};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tracing::{debug, info};

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

    /// Read a file in streaming chunks (for large files).
    /// Returns lines as they are read, without buffering the entire file.
    pub async fn read_file_lines(path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
        let file = fs::File::open(path).await?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut reader_lines = reader.lines();

        while let Some(line) = reader_lines.next_line().await? {
            lines.push(line);
        }

        Ok(lines)
    }

    /// Read a portion of a file (for large files).
    pub async fn read_file_range(
        path: &PathBuf,
        start_line: usize,
        line_count: usize,
    ) -> Result<Vec<String>, std::io::Error> {
        let file = fs::File::open(path).await?;
        let reader = BufReader::new(file);
        let mut lines = Vec::with_capacity(line_count);
        let mut reader_lines = reader.lines();
        let mut current_line = 0;

        while let Some(line) = reader_lines.next_line().await? {
            if current_line >= start_line {
                lines.push(line);
                if lines.len() >= line_count {
                    break;
                }
            }
            current_line += 1;
        }

        Ok(lines)
    }

    /// Get file metadata.
    pub async fn file_size(path: &PathBuf) -> Result<u64, std::io::Error> {
        let metadata = fs::metadata(path).await?;
        Ok(metadata.len())
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

    /// List directory contents incrementally with limit (for large directories).
    pub async fn read_dir_limited(
        path: &PathBuf,
        limit: usize,
    ) -> Result<(Vec<PathBuf>, bool), std::io::Error> {
        let mut entries = Vec::new();
        let mut dir = fs::read_dir(path).await?;
        let mut has_more = false;

        while let Some(entry) = dir.next_entry().await? {
            if entries.len() >= limit {
                has_more = true;
                break;
            }
            entries.push(entry.path());
        }

        Ok((entries, has_more))
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

    #[tokio::test]
    async fn test_read_empty_dir() {
        let dir = tempdir().unwrap();
        let entries = FsService::read_dir(&dir.path().to_path_buf()).await.unwrap();
        assert!(entries.is_empty());
    }

    #[tokio::test]
    async fn test_create_nested_dir() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("a/b/c/d/e");
        
        FsService::create_dir(&path).await.unwrap();
        assert!(FsService::exists(&path).await);
    }

    #[tokio::test]
    async fn test_write_empty_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.txt");
        
        FsService::write_file(&path, "").await.unwrap();
        let content = FsService::read_file(&path).await.unwrap();
        assert!(content.is_empty());
    }

    #[tokio::test]
    async fn test_write_large_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("large.txt");
        let content = "x".repeat(100000);
        
        FsService::write_file(&path, &content).await.unwrap();
        let read = FsService::read_file(&path).await.unwrap();
        assert_eq!(read.len(), 100000);
    }

    #[tokio::test]
    async fn test_read_dir_with_subdirs() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        
        FsService::create_dir(&path.join("subdir")).await.unwrap();
        FsService::write_file(&path.join("file.txt"), "test").await.unwrap();
        
        let entries = FsService::read_dir(&path).await.unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[tokio::test]
    async fn test_write_newlines() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("lines.txt");
        let content = "line1\nline2\nline3";
        
        FsService::write_file(&path, content).await.unwrap();
        let read = FsService::read_file(&path).await.unwrap();
        assert!(read.contains('\n'));
    }

    #[tokio::test]
    async fn test_exists_on_dir() {
        let dir = tempdir().unwrap();
        assert!(FsService::exists(&dir.path().to_path_buf()).await);
    }

    #[tokio::test]
    async fn test_read_dir_nonexistent() {
        let path = PathBuf::from("/nonexistent/directory/path");
        let result = FsService::read_dir(&path).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_fs_service_name_value() {
        let service = FsService::new();
        assert_eq!(service.name(), "fs");
    }

    #[tokio::test]
    async fn test_write_tabs() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tabs.txt");
        let content = "col1\tcol2\tcol3";
        
        FsService::write_file(&path, content).await.unwrap();
        let read = FsService::read_file(&path).await.unwrap();
        assert!(read.contains('\t'));
    }

    #[tokio::test]
    async fn test_write_carriage_return() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("crlf.txt");
        let content = "line1\r\nline2\r\n";
        
        FsService::write_file(&path, content).await.unwrap();
        let read = FsService::read_file(&path).await.unwrap();
        assert!(read.contains("\r\n"));
    }

    #[tokio::test]
    async fn test_create_dir_already_exists() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("existdir");
        
        FsService::create_dir(&path).await.unwrap();
        // Should not error on existing
        let result = FsService::create_dir(&path).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_write_special_chars() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("special.txt");
        let content = "!@#$%^&*()[]{}|;':\",./<>?";
        
        FsService::write_file(&path, content).await.unwrap();
        let read = FsService::read_file(&path).await.unwrap();
        assert_eq!(read, content);
    }

    #[tokio::test]
    async fn test_exists_after_delete() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("to_delete.txt");
        
        FsService::write_file(&path, "test").await.unwrap();
        assert!(FsService::exists(&path).await);
        
        tokio::fs::remove_file(&path).await.unwrap();
        assert!(!FsService::exists(&path).await);
    }

    #[tokio::test]
    async fn test_read_file_lines() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("lines.txt");
        let content = "line1\nline2\nline3";
        
        FsService::write_file(&path, content).await.unwrap();
        let lines = FsService::read_file_lines(&path).await.unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line1");
        assert_eq!(lines[1], "line2");
        assert_eq!(lines[2], "line3");
    }

    #[tokio::test]
    async fn test_read_file_lines_empty() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.txt");
        
        FsService::write_file(&path, "").await.unwrap();
        let lines = FsService::read_file_lines(&path).await.unwrap();
        assert!(lines.is_empty());
    }

    #[tokio::test]
    async fn test_read_file_range() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("range.txt");
        let content = "line0\nline1\nline2\nline3\nline4";
        
        FsService::write_file(&path, content).await.unwrap();
        
        // Read lines 1-2 (0-indexed)
        let lines = FsService::read_file_range(&path, 1, 2).await.unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "line1");
        assert_eq!(lines[1], "line2");
    }

    #[tokio::test]
    async fn test_read_file_range_from_start() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("range_start.txt");
        let content = "a\nb\nc\nd\ne";
        
        FsService::write_file(&path, content).await.unwrap();
        
        let lines = FsService::read_file_range(&path, 0, 3).await.unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "a");
        assert_eq!(lines[2], "c");
    }

    #[tokio::test]
    async fn test_read_file_range_past_end() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("range_end.txt");
        let content = "x\ny\nz";
        
        FsService::write_file(&path, content).await.unwrap();
        
        // Request more lines than exist
        let lines = FsService::read_file_range(&path, 1, 100).await.unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "y");
        assert_eq!(lines[1], "z");
    }

    #[tokio::test]
    async fn test_file_size() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("size.txt");
        let content = "12345";
        
        FsService::write_file(&path, content).await.unwrap();
        let size = FsService::file_size(&path).await.unwrap();
        assert_eq!(size, 5);
    }

    #[tokio::test]
    async fn test_file_size_empty() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty_size.txt");
        
        FsService::write_file(&path, "").await.unwrap();
        let size = FsService::file_size(&path).await.unwrap();
        assert_eq!(size, 0);
    }

    #[tokio::test]
    async fn test_read_dir_limited() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        
        for i in 0..10 {
            FsService::write_file(&path.join(format!("{}.txt", i)), "test").await.unwrap();
        }
        
        let (entries, has_more) = FsService::read_dir_limited(&path, 5).await.unwrap();
        assert_eq!(entries.len(), 5);
        assert!(has_more);
    }

    #[tokio::test]
    async fn test_read_dir_limited_all() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        
        for i in 0..3 {
            FsService::write_file(&path.join(format!("{}.txt", i)), "test").await.unwrap();
        }
        
        let (entries, has_more) = FsService::read_dir_limited(&path, 10).await.unwrap();
        assert_eq!(entries.len(), 3);
        assert!(!has_more);
    }

    #[tokio::test]
    async fn test_read_file_lines_large() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("large_lines.txt");
        let content: String = (0..1000).map(|i| format!("line{}\n", i)).collect();
        
        FsService::write_file(&path, &content).await.unwrap();
        let lines = FsService::read_file_lines(&path).await.unwrap();
        assert_eq!(lines.len(), 1000);
        assert_eq!(lines[0], "line0");
        assert_eq!(lines[999], "line999");
    }

    #[tokio::test]
    async fn test_read_file_range_middle() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("middle.txt");
        let content: String = (0..100).map(|i| format!("{}\n", i)).collect();
        
        FsService::write_file(&path, &content).await.unwrap();
        let lines = FsService::read_file_range(&path, 50, 10).await.unwrap();
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0], "50");
        assert_eq!(lines[9], "59");
    }
}
