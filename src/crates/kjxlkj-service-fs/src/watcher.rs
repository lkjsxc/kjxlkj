//! Filesystem watcher.

use std::path::PathBuf;

/// Filesystem watcher.
pub struct FsWatcher {
    /// Paths being watched.
    paths: Vec<PathBuf>,
}

impl FsWatcher {
    /// Creates a new watcher.
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    /// Adds a path to watch.
    pub fn watch(&mut self, path: PathBuf) {
        self.paths.push(path);
    }

    /// Removes a path from watching.
    pub fn unwatch(&mut self, path: &PathBuf) {
        self.paths.retain(|p| p != path);
    }
}

impl Default for FsWatcher {
    fn default() -> Self {
        Self::new()
    }
}
