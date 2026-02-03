#![forbid(unsafe_code)]

use anyhow::Context;

pub async fn read_to_string(path: &str) -> anyhow::Result<String> {
    tokio::fs::read_to_string(path).await.context("read file")
}

pub async fn write_string(path: &str, contents: &str) -> anyhow::Result<()> {
    tokio::fs::write(path, contents).await.context("write file")?;
    Ok(())
}

