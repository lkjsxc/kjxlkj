//! Git service for kjxlkj editor.
//!
//! Provides git integration via subprocess.

use kjxlkj_services::{Service, ServiceMessage};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::process::Command;
use tokio::sync::mpsc;
use tracing::{debug, info};

/// Git status of a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitStatus {
    /// File is untracked.
    Untracked,
    /// File is modified.
    Modified,
    /// File is staged.
    Staged,
    /// File is unchanged.
    Unchanged,
    /// File is ignored.
    Ignored,
}

/// Git service.
pub struct GitService {
    /// Service name.
    name: String,
    /// Repository root.
    #[allow(dead_code)]
    repo_root: Option<PathBuf>,
}

impl GitService {
    /// Create a new git service.
    pub fn new() -> Self {
        Self {
            name: "git".to_string(),
            repo_root: None,
        }
    }

    /// Find git repository root.
    pub async fn find_repo_root(path: &PathBuf) -> Option<PathBuf> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(path)
            .output()
            .await
            .ok()?;

        if output.status.success() {
            let root = String::from_utf8(output.stdout).ok()?;
            Some(PathBuf::from(root.trim()))
        } else {
            None
        }
    }

    /// Get current branch name.
    pub async fn current_branch(repo_root: &PathBuf) -> Option<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(repo_root)
            .output()
            .await
            .ok()?;

        if output.status.success() {
            let branch = String::from_utf8(output.stdout).ok()?;
            Some(branch.trim().to_string())
        } else {
            None
        }
    }

    /// Get file status.
    pub async fn file_status(repo_root: &PathBuf, file: &PathBuf) -> Option<GitStatus> {
        let output = Command::new("git")
            .args(["status", "--porcelain", "--"])
            .arg(file)
            .current_dir(repo_root)
            .output()
            .await
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let status = String::from_utf8(output.stdout).ok()?;
        let status = status.trim();

        if status.is_empty() {
            return Some(GitStatus::Unchanged);
        }

        let first = status.chars().next()?;
        let second = status.chars().nth(1)?;

        match (first, second) {
            ('?', '?') => Some(GitStatus::Untracked),
            ('!', '!') => Some(GitStatus::Ignored),
            (_, 'M') | (_, 'D') | (_, 'A') => Some(GitStatus::Modified),
            ('M', _) | ('A', _) | ('D', _) | ('R', _) => Some(GitStatus::Staged),
            _ => Some(GitStatus::Unchanged),
        }
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for GitService {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(
        self: Box<Self>,
        mut rx: mpsc::Receiver<ServiceMessage>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            info!("Git service started");

            while let Some(msg) = rx.recv().await {
                match msg {
                    ServiceMessage::Shutdown => {
                        info!("Git service shutting down");
                        break;
                    }
                    ServiceMessage::Custom(cmd) => {
                        debug!(%cmd, "Received command");
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_service_new() {
        let service = GitService::new();
        assert_eq!(service.name(), "git");
    }

    #[test]
    fn test_git_status_variants() {
        assert_eq!(GitStatus::Untracked, GitStatus::Untracked);
        assert_ne!(GitStatus::Modified, GitStatus::Staged);
    }

    #[test]
    fn test_git_service_default() {
        let service = GitService::default();
        assert_eq!(service.name(), "git");
    }

    #[test]
    fn test_git_status_equality() {
        assert_eq!(GitStatus::Modified, GitStatus::Modified);
        assert_eq!(GitStatus::Staged, GitStatus::Staged);
        assert_eq!(GitStatus::Unchanged, GitStatus::Unchanged);
        assert_eq!(GitStatus::Ignored, GitStatus::Ignored);
    }

    #[test]
    fn test_git_status_clone() {
        let status = GitStatus::Modified;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_git_status_debug() {
        let status = GitStatus::Untracked;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Untracked"));
    }

    #[test]
    fn test_git_status_modified_debug() {
        let status = GitStatus::Modified;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Modified"));
    }

    #[test]
    fn test_git_status_staged_debug() {
        let status = GitStatus::Staged;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Staged"));
    }

    #[test]
    fn test_git_status_unchanged_debug() {
        let status = GitStatus::Unchanged;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Unchanged"));
    }

    #[test]
    fn test_git_status_ignored_debug() {
        let status = GitStatus::Ignored;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Ignored"));
    }

    #[test]
    fn test_git_service_name() {
        let service = GitService::new();
        assert!(!service.name().is_empty());
    }

    #[test]
    fn test_git_status_all_variants_clone() {
        let variants = [
            GitStatus::Untracked,
            GitStatus::Modified,
            GitStatus::Staged,
            GitStatus::Unchanged,
            GitStatus::Ignored,
        ];
        for v in variants {
            let cloned = v.clone();
            assert_eq!(v, cloned);
        }
    }

    #[test]
    fn test_git_status_eq_hash() {
        // Can't hash GitStatus since it doesn't derive Hash, but we can test Eq
        assert_eq!(GitStatus::Modified, GitStatus::Modified);
        assert_ne!(GitStatus::Modified, GitStatus::Staged);
        assert_ne!(GitStatus::Untracked, GitStatus::Ignored);
        assert_ne!(GitStatus::Unchanged, GitStatus::Modified);
    }

    #[test]
    fn test_git_service_new_name() {
        let service = GitService::new();
        assert_eq!(service.name(), "git");
    }

    #[test]
    fn test_git_status_all_inequality() {
        let variants = [
            GitStatus::Untracked,
            GitStatus::Modified,
            GitStatus::Staged,
            GitStatus::Unchanged,
            GitStatus::Ignored,
        ];
        for (i, a) in variants.iter().enumerate() {
            for (j, b) in variants.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_git_status_size() {
        // GitStatus is an enum and should be small
        assert!(std::mem::size_of::<GitStatus>() <= 8);
    }

    #[tokio::test]
    async fn test_find_repo_root_nonexistent() {
        let path = PathBuf::from("/nonexistent/path");
        let result = GitService::find_repo_root(&path).await;
        // Should be None for non-existent path
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_current_branch_nonexistent() {
        let path = PathBuf::from("/nonexistent/path");
        let result = GitService::current_branch(&path).await;
        // Should be None for non-existent path
        assert!(result.is_none());
    }

    #[test]
    fn test_git_status_untracked_clone() {
        let status = GitStatus::Untracked;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_git_status_modified_clone() {
        let status = GitStatus::Modified;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_git_service_name_is_git() {
        let service = GitService::new();
        assert_eq!(service.name, "git");
    }

    #[test]
    fn test_git_service_repo_root_none_initially() {
        let service = GitService::new();
        assert!(service.repo_root.is_none());
    }

    #[test]
    fn test_git_status_ignored_clone() {
        let status = GitStatus::Ignored;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_git_status_staged_clone() {
        let status = GitStatus::Staged;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }
}
