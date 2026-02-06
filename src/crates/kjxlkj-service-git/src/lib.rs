//! Git service — repository status, diff, blame.

use std::path::PathBuf;

/// Status of a file relative to git.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Untracked,
    Modified,
    Added,
    Deleted,
    Renamed,
    Unmodified,
}

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

/// Git integration service.
pub struct GitService {
    repo_root: Option<PathBuf>,
}

impl GitService {
    pub fn new() -> Self {
        Self { repo_root: None }
    }

    /// Set the repository root directory.
    pub fn set_root(&mut self, root: PathBuf) {
        self.repo_root = Some(root);
    }

    /// Query the current repository status.
    pub async fn status(&self) -> anyhow::Result<RepoStatus> {
        let _root = self.repo_root.as_ref().ok_or_else(|| {
            anyhow::anyhow!("no git repository configured")
        })?;
        tracing::debug!("querying git status");
        Ok(RepoStatus::default())
    }

    /// Get the file status for a specific path.
    pub async fn file_status(&self, _path: &std::path::Path) -> anyhow::Result<FileStatus> {
        Ok(FileStatus::Unmodified)
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}
