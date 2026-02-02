//! Git repository operations.

use std::path::{Path, PathBuf};
use std::process::Command;

/// File status in Git.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitStatus {
    /// Untracked file.
    Untracked,
    /// Unmodified (clean).
    Unmodified,
    /// Modified.
    Modified,
    /// Staged for commit.
    Staged,
    /// Both modified and staged.
    StagedModified,
    /// Deleted.
    Deleted,
    /// Renamed.
    Renamed,
    /// Conflicted.
    Conflicted,
    /// Ignored.
    Ignored,
}

/// Git repository wrapper.
#[derive(Debug)]
pub struct GitRepo {
    /// Repository root path.
    root: PathBuf,
}

impl GitRepo {
    /// Opens a repository at the given path.
    pub fn open(path: impl AsRef<Path>) -> Option<Self> {
        let path = path.as_ref();

        // Find .git directory
        let mut current = path.to_path_buf();
        loop {
            if current.join(".git").exists() {
                return Some(Self { root: current });
            }
            if !current.pop() {
                return None;
            }
        }
    }

    /// Returns the repository root.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Gets the current branch name.
    pub fn current_branch(&self) -> Option<String> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .current_dir(&self.root)
            .output()
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }

    /// Gets the status of a file.
    pub fn file_status(&self, path: impl AsRef<Path>) -> GitStatus {
        let path = path.as_ref();
        let relative = path.strip_prefix(&self.root).unwrap_or(path);

        let output = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .arg(relative)
            .current_dir(&self.root)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let status = String::from_utf8_lossy(&out.stdout);
                Self::parse_status_line(status.trim())
            }
            _ => GitStatus::Unmodified,
        }
    }

    /// Parses a porcelain status line.
    fn parse_status_line(line: &str) -> GitStatus {
        if line.is_empty() {
            return GitStatus::Unmodified;
        }

        let bytes = line.as_bytes();
        if bytes.len() < 2 {
            return GitStatus::Unmodified;
        }

        let index = bytes[0];
        let worktree = bytes[1];

        match (index, worktree) {
            (b'?', b'?') => GitStatus::Untracked,
            (b'!', b'!') => GitStatus::Ignored,
            (b'U', _) | (_, b'U') | (b'A', b'A') | (b'D', b'D') => GitStatus::Conflicted,
            (b'M', b' ') | (b'A', b' ') => GitStatus::Staged,
            (b' ', b'M') => GitStatus::Modified,
            (b'M', b'M') => GitStatus::StagedModified,
            (b'D', _) | (_, b'D') => GitStatus::Deleted,
            (b'R', _) => GitStatus::Renamed,
            _ => GitStatus::Unmodified,
        }
    }

    /// Gets file diff against HEAD.
    pub fn file_diff(&self, path: impl AsRef<Path>) -> Option<String> {
        let path = path.as_ref();
        let relative = path.strip_prefix(&self.root).unwrap_or(path);

        let output = Command::new("git")
            .arg("diff")
            .arg("HEAD")
            .arg("--")
            .arg(relative)
            .current_dir(&self.root)
            .output()
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            None
        }
    }
}
