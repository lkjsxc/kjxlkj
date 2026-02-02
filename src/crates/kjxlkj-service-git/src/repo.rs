//! Git repository.

use std::path::PathBuf;

/// Git repository wrapper.
pub struct GitRepo {
    /// Repository root.
    root: PathBuf,
}

impl GitRepo {
    /// Opens a repository.
    pub fn open(root: PathBuf) -> Option<Self> {
        if root.join(".git").exists() {
            Some(Self { root })
        } else {
            None
        }
    }

    /// Returns the root path.
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// Returns the current branch.
    pub fn branch(&self) -> Option<String> {
        None
    }

    /// Returns the status.
    pub fn status(&self) -> Vec<(PathBuf, FileStatus)> {
        Vec::new()
    }
}

/// File status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    /// Modified.
    Modified,
    /// Added.
    Added,
    /// Deleted.
    Deleted,
    /// Renamed.
    Renamed,
    /// Untracked.
    Untracked,
}
