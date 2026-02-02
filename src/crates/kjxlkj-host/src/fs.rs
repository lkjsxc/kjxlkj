//! Filesystem operations.

use std::path::{Path, PathBuf};
use tokio::fs;

/// Filesystem abstraction.
#[derive(Debug, Default)]
pub struct FileSystem;

impl FileSystem {
    /// Creates a new filesystem instance.
    pub fn new() -> Self {
        Self
    }

    /// Reads a file to string.
    pub async fn read_to_string(&self, path: impl AsRef<Path>) -> std::io::Result<String> {
        fs::read_to_string(path).await
    }

    /// Reads a file to bytes.
    pub async fn read(&self, path: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
        fs::read(path).await
    }

    /// Writes a string to a file.
    pub async fn write(&self, path: impl AsRef<Path>, contents: &str) -> std::io::Result<()> {
        fs::write(path, contents).await
    }

    /// Checks if a path exists.
    pub async fn exists(&self, path: impl AsRef<Path>) -> bool {
        fs::try_exists(path).await.unwrap_or(false)
    }

    /// Checks if a path is a file.
    pub async fn is_file(&self, path: impl AsRef<Path>) -> bool {
        fs::metadata(path)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    }

    /// Checks if a path is a directory.
    pub async fn is_dir(&self, path: impl AsRef<Path>) -> bool {
        fs::metadata(path)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    }

    /// Lists directory contents.
    pub async fn read_dir(&self, path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
        let mut entries = fs::read_dir(path).await?;
        let mut result = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            result.push(entry.path());
        }
        Ok(result)
    }

    /// Creates a directory (and parents).
    pub async fn create_dir_all(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        fs::create_dir_all(path).await
    }

    /// Removes a file.
    pub async fn remove_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        fs::remove_file(path).await
    }

    /// Renames/moves a file.
    pub async fn rename(
        &self,
        from: impl AsRef<Path>,
        to: impl AsRef<Path>,
    ) -> std::io::Result<()> {
        fs::rename(from, to).await
    }

    /// Returns the current working directory.
    pub fn current_dir(&self) -> std::io::Result<PathBuf> {
        std::env::current_dir()
    }

    /// Canonicalizes a path.
    pub async fn canonicalize(&self, path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
        fs::canonicalize(path).await
    }
}
