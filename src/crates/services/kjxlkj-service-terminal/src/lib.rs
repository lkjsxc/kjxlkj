#![forbid(unsafe_code)]

use anyhow::Context;
use tokio::process::Command;

pub async fn run_shell(command: &str) -> anyhow::Result<String> {
    let output = Command::new("sh")
        .arg("-lc")
        .arg(command)
        .output()
        .await
        .context("spawn shell")?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

