//! Git service implementation.

use crate::types::{DiffHunk, GitCommand, GitNotification, GitSign, StatusEntry};
use std::collections::HashMap;
use std::path::PathBuf;

/// Git service managing repository state.
pub struct GitService {
    /// Cached file status entries.
    status_cache: Vec<StatusEntry>,
    /// Cached hunks per file path.
    hunk_cache: HashMap<PathBuf, Vec<DiffHunk>>,
    /// Whether git binary is available.
    git_available: bool,
    /// Repository root path.
    root: Option<PathBuf>,
}

impl GitService {
    /// Create a new git service.
    pub fn new() -> Self {
        Self {
            status_cache: Vec::new(),
            hunk_cache: HashMap::new(),
            git_available: true,
            root: None,
        }
    }

    /// Set the repository root directory.
    pub fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root);
    }

    /// Get the repository root.
    pub fn root(&self) -> Option<&PathBuf> {
        self.root.as_ref()
    }

    /// Check if git is available.
    pub fn is_available(&self) -> bool {
        self.git_available
    }

    /// Mark git as unavailable (binary not found).
    pub fn set_unavailable(&mut self) {
        self.git_available = false;
    }

    /// Handle a git command and produce a notification.
    pub fn handle_command(&mut self, cmd: &GitCommand) -> GitNotification {
        if !self.git_available {
            return GitNotification::Error("git binary not found".to_string());
        }

        match cmd {
            GitCommand::RefreshStatus => {
                // In production: spawn `git status --porcelain=v2`
                tracing::debug!("git status refresh");
                GitNotification::StatusUpdated(self.status_cache.clone())
            }
            GitCommand::ComputeHunks { file } => {
                tracing::debug!(?file, "computing hunks");
                let hunks = self.hunk_cache.get(file).cloned().unwrap_or_default();
                GitNotification::HunksUpdated {
                    file: file.clone(),
                    hunks,
                }
            }
            GitCommand::Blame { file } => {
                tracing::debug!(?file, "blame");
                GitNotification::BlameResult {
                    file: file.clone(),
                    entries: Vec::new(),
                }
            }
            GitCommand::StageHunk { file, hunk_index } => {
                tracing::debug!(?file, hunk_index, "stage hunk");
                GitNotification::StatusUpdated(self.status_cache.clone())
            }
            GitCommand::ResetHunk { file, hunk_index } => {
                tracing::debug!(?file, hunk_index, "reset hunk");
                GitNotification::HunksUpdated {
                    file: file.clone(),
                    hunks: self.hunk_cache.get(file).cloned().unwrap_or_default(),
                }
            }
            GitCommand::Diff { file } => {
                tracing::debug!(?file, "diff");
                GitNotification::DiffContent {
                    file: file.clone(),
                    content: String::new(),
                }
            }
        }
    }

    /// Get the cached status entries.
    pub fn cached_status(&self) -> &[StatusEntry] {
        &self.status_cache
    }

    /// Get cached hunks for a file.
    pub fn cached_hunks(&self, file: &PathBuf) -> Option<&[DiffHunk]> {
        self.hunk_cache.get(file).map(|v| v.as_slice())
    }

    /// Inject status entries (for testing or manual refresh).
    pub fn set_status(&mut self, entries: Vec<StatusEntry>) {
        self.status_cache = entries;
    }

    /// Inject hunks for a file (for testing or diff parse result).
    pub fn set_hunks(&mut self, file: PathBuf, hunks: Vec<DiffHunk>) {
        self.hunk_cache.insert(file, hunks);
    }

    /// Get the sign for a specific line in a file.
    pub fn sign_for_line(&self, file: &PathBuf, line: usize) -> Option<GitSign> {
        let hunks = self.hunk_cache.get(file)?;
        for hunk in hunks {
            let end = hunk.start_line + hunk.count;
            if line >= hunk.start_line && line < end {
                return Some(hunk.sign);
            }
        }
        None
    }

    /// Navigate to the next hunk relative to cursor line.
    pub fn next_hunk(&self, file: &PathBuf, current_line: usize) -> Option<usize> {
        let hunks = self.hunk_cache.get(file)?;
        for hunk in hunks {
            if hunk.start_line > current_line {
                return Some(hunk.start_line);
            }
        }
        None
    }

    /// Navigate to the previous hunk relative to cursor line.
    pub fn prev_hunk(&self, file: &PathBuf, current_line: usize) -> Option<usize> {
        let hunks = self.hunk_cache.get(file)?;
        for hunk in hunks.iter().rev() {
            if hunk.start_line < current_line {
                return Some(hunk.start_line);
            }
        }
        None
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}
