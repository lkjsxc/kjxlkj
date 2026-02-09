//! Git service: executes git commands.

use std::path::PathBuf;
use std::process::Command;

use tokio::sync::{broadcast, mpsc};

use kjxlkj_core_types::ServiceResponse;

/// Git service for repository operations.
pub struct GitService {
    response_tx: mpsc::Sender<ServiceResponse>,
    repo_root: Option<PathBuf>,
}

impl GitService {
    pub fn new(response_tx: mpsc::Sender<ServiceResponse>) -> Self {
        Self {
            response_tx,
            repo_root: None,
        }
    }

    /// Detect git repository root.
    pub fn detect_repo(&mut self) -> Option<PathBuf> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .ok()?;

        if output.status.success() {
            let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path = PathBuf::from(root);
            self.repo_root = Some(path.clone());
            Some(path)
        } else {
            None
        }
    }

    /// Get git status.
    pub fn status(&self) -> Result<String, String> {
        let output = Command::new("git")
            .args(["status", "--porcelain=v1"])
            .output()
            .map_err(|e| format!("git status: {e}"))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Get git diff for a file.
    pub fn diff(&self, path: &str) -> Result<String, String> {
        let output = Command::new("git")
            .args(["diff", "--unified=3", "--", path])
            .output()
            .map_err(|e| format!("git diff: {e}"))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Run the service loop.
    pub async fn run(self, mut quit_rx: broadcast::Receiver<()>) {
        loop {
            tokio::select! {
                _ = quit_rx.recv() => break,
                _ = tokio::time::sleep(
                    std::time::Duration::from_secs(3600)
                ) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_git_service() {
        let (tx, _rx) = mpsc::channel(256);
        let _svc = GitService::new(tx);
    }
}
