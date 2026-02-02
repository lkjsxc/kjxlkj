//! File operations.

#![allow(dead_code)]

use anyhow::Result;
use std::path::Path;
use tokio::fs;

/// Reads a file.
pub async fn read_file(path: impl AsRef<Path>) -> Result<String> {
    let content = fs::read_to_string(path).await?;
    Ok(content)
}

/// Writes a file.
pub async fn write_file(path: impl AsRef<Path>, content: &str) -> Result<()> {
    fs::write(path, content).await?;
    Ok(())
}

/// Checks if a file exists.
pub async fn exists(path: impl AsRef<Path>) -> bool {
    fs::metadata(path).await.is_ok()
}

/// Creates a directory.
pub async fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(path).await?;
    Ok(())
}
