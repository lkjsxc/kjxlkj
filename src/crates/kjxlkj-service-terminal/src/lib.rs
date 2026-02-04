//! Terminal/PTY service for `:!` commands.

use anyhow::Result;
use tokio::process::Command;

/// Terminal service handle.
pub struct TerminalService;

impl TerminalService {
    /// Create a new Terminal service.
    pub fn new() -> Self {
        Self
    }

    /// Execute a shell command and return output.
    pub async fn execute(cmd: &str) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(stdout.to_string())
        } else {
            Ok(format!("{}{}", stdout, stderr))
        }
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute() {
        let output = TerminalService::execute("echo hello").await.unwrap();
        assert!(output.contains("hello"));
    }
}
