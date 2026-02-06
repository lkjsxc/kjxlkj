//! Git service — repository status, diff, blame, hunk tracking.

use std::path::PathBuf;

/// Status of a file relative to git.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus { Untracked, Modified, Added, Deleted, Renamed, Unmodified }

/// A summary of the git repository state.
#[derive(Debug, Clone, Default)]
pub struct RepoStatus {
    pub branch: Option<String>,
    pub ahead: usize,
    pub behind: usize,
    pub staged: usize,
    pub unstaged: usize,
    pub untracked: usize,
}

/// A file change entry from `git status`.
#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub status: FileStatus,
    pub old_path: Option<String>,
}

/// A diff hunk showing changed lines.
#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub kind: HunkKind,
}

/// Kind of change in a hunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunkKind { Added, Removed, Changed }

/// A blame entry for a single line.
#[derive(Debug, Clone)]
pub struct BlameLine {
    pub commit: String,
    pub author: String,
    pub date: String,
    pub line_number: usize,
}

/// Gutter sign for modified lines (sign column indicators).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitSign { Added, Modified, Removed }

/// Git integration service.
pub struct GitService {
    repo_root: Option<PathBuf>,
}

impl GitService {
    pub fn new() -> Self { Self { repo_root: None } }
    pub fn set_root(&mut self, root: PathBuf) { self.repo_root = Some(root); }

    /// Query the current repository status.
    pub async fn status(&self) -> anyhow::Result<RepoStatus> {
        let root = self.repo_root.as_ref()
            .ok_or_else(|| anyhow::anyhow!("no git repository configured"))?;
        let mut status = RepoStatus::default();
        let head_path = root.join(".git/HEAD");
        if let Ok(contents) = std::fs::read_to_string(&head_path) {
            let contents = contents.trim();
            if let Some(branch) = contents.strip_prefix("ref: refs/heads/") {
                status.branch = Some(branch.to_string());
            } else {
                status.branch = Some(contents[..8.min(contents.len())].to_string());
            }
        }
        Ok(status)
    }

    pub async fn file_status(&self, _path: &std::path::Path) -> anyhow::Result<FileStatus> {
        Ok(FileStatus::Unmodified)
    }

    /// Get changed files in the working tree (stub).
    pub async fn changed_files(&self) -> anyhow::Result<Vec<FileChange>> {
        Ok(Vec::new())
    }

    /// Get diff hunks for a specific file (stub).
    pub async fn diff_hunks(&self, _path: &std::path::Path) -> anyhow::Result<Vec<DiffHunk>> {
        Ok(Vec::new())
    }

    /// Get blame for a specific file (stub).
    pub async fn blame(&self, _path: &std::path::Path) -> anyhow::Result<Vec<BlameLine>> {
        Ok(Vec::new())
    }

    /// Compute gutter signs for a buffer's lines (stub).
    pub fn gutter_signs(&self, _line_count: usize) -> Vec<Option<GitSign>> {
        Vec::new()
    }
}

impl Default for GitService { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_status_variants() {
        assert_ne!(FileStatus::Modified, FileStatus::Unmodified);
        assert_eq!(FileStatus::Added, FileStatus::Added);
    }

    #[test]
    fn repo_status_default() {
        let rs = RepoStatus::default();
        assert!(rs.branch.is_none());
        assert_eq!(rs.ahead, 0);
    }

    #[test]
    fn diff_hunk_kind() {
        let h = DiffHunk { old_start: 1, old_count: 3, new_start: 1, new_count: 5, kind: HunkKind::Changed };
        assert_eq!(h.kind, HunkKind::Changed);
    }

    #[test]
    fn blame_line_creation() {
        let bl = BlameLine {
            commit: "abc1234".into(), author: "test".into(),
            date: "2024-01-01".into(), line_number: 42,
        };
        assert_eq!(bl.line_number, 42);
    }

    #[test]
    fn file_change_with_rename() {
        let fc = FileChange {
            path: "new.rs".into(), status: FileStatus::Renamed,
            old_path: Some("old.rs".into()),
        };
        assert_eq!(fc.old_path.as_deref(), Some("old.rs"));
    }

    #[test]
    fn git_sign_variants() {
        assert_ne!(GitSign::Added, GitSign::Removed);
        assert_eq!(GitSign::Modified, GitSign::Modified);
    }

    #[test]
    fn service_default() {
        let svc = GitService::new();
        assert!(svc.repo_root.is_none());
    }
}
