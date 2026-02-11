//! Git service implementation.

use std::path::PathBuf;
use thiserror::Error;
use tracing::info;

/// Git service error.
#[derive(Debug, Error)]
pub enum GitError {
    #[error("Not a git repository")]
    NotRepo,
    #[error("Git operation failed: {0}")]
    Operation(String),
}

/// File status in git.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    /// Untracked file.
    Untracked,
    /// Modified but unstaged.
    Modified,
    /// Staged for commit.
    Staged,
    /// Both staged and modified.
    StagedModified,
    /// Deleted.
    Deleted,
    /// Renamed.
    Renamed,
    /// Clean (committed).
    Clean,
    /// File ignored by .gitignore.
    Ignored,
}

/// Hunk information for changed lines.
#[derive(Debug, Clone)]
pub struct Hunk {
    /// Start line (1-indexed).
    pub start: usize,
    /// Number of lines.
    pub count: usize,
    /// Type of change.
    pub kind: HunkKind,
}

/// Type of hunk change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunkKind {
    Added,
    Removed,
    Modified,
}

/// Git service.
pub struct GitService {
    /// Cached repo root for current session.
    repo_root: Option<PathBuf>,
}

impl GitService {
    /// Create a new Git service.
    pub fn new() -> Self {
        Self { repo_root: None }
    }

    /// Check if path is in a git repository.
    pub fn is_repo(&self, path: &PathBuf) -> bool {
        info!("Checking git repo status for {:?}", path);
        // Walk up the path looking for .git
        let mut current = path.as_path();
        loop {
            if current.join(".git").exists() {
                return true;
            }
            match current.parent() {
                Some(parent) => current = parent,
                None => return false,
            }
        }
    }

    /// Get the repository root.
    pub fn repo_root(&mut self, path: &PathBuf) -> Option<PathBuf> {
        let mut current = path.clone();
        loop {
            if current.join(".git").exists() {
                self.repo_root = Some(current.clone());
                return Some(current);
            }
            if !current.pop() {
                return None;
            }
        }
    }

    /// Get file status (stub - would use git2 in real impl).
    pub fn file_status(&self, _path: &PathBuf) -> FileStatus {
        // Placeholder - real impl would use git2
        FileStatus::Clean
    }

    /// Get changed hunks for a file (stub).
    pub fn get_hunks(&self, _path: &PathBuf) -> Vec<Hunk> {
        // Placeholder - real impl would diff working tree
        Vec::new()
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_service_creation() {
        let _service = GitService::new();
    }

    #[test]
    fn test_file_status_enum() {
        assert_ne!(FileStatus::Modified, FileStatus::Staged);
        assert_eq!(FileStatus::Clean, FileStatus::Clean);
    }

    #[test]
    fn test_hunk_kind_enum() {
        let hunk = Hunk {
            start: 10,
            count: 5,
            kind: HunkKind::Added,
        };
        assert_eq!(hunk.start, 10);
        assert_eq!(hunk.count, 5);
        assert_eq!(hunk.kind, HunkKind::Added);
    }
}
