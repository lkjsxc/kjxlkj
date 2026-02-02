//! Git service.

use crate::GitRepo;
use std::path::PathBuf;

/// Git service.
pub struct GitService {
    /// Current repository.
    repo: Option<GitRepo>,
}

impl GitService {
    /// Creates a new git service.
    pub fn new() -> Self {
        Self { repo: None }
    }

    /// Opens a repository.
    pub fn open(&mut self, path: PathBuf) {
        self.repo = GitRepo::open(path);
    }

    /// Returns the current repository.
    pub fn repo(&self) -> Option<&GitRepo> {
        self.repo.as_ref()
    }

    /// Runs the service.
    pub async fn run(self) {
        // Service loop
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}
