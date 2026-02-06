//! Filesystem watch service — monitors file changes.

use std::path::PathBuf;

/// A filesystem change event.
#[derive(Debug, Clone)]
pub struct FsEvent {
    pub path: PathBuf,
    pub kind: FsEventKind,
}

/// The kind of filesystem change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsEventKind {
    Created,
    Modified,
    Deleted,
    Renamed,
}

/// Service that watches the filesystem for changes and notifies the editor.
pub struct FsWatchService {
    watch_roots: Vec<PathBuf>,
    running: bool,
}

impl FsWatchService {
    pub fn new() -> Self {
        Self {
            watch_roots: Vec::new(),
            running: false,
        }
    }

    /// Add a directory to the watch set.
    pub fn watch(&mut self, path: PathBuf) {
        tracing::debug!(path = %path.display(), "adding watch root");
        self.watch_roots.push(path);
    }

    /// Remove a directory from the watch set.
    pub fn unwatch(&mut self, path: &std::path::Path) {
        self.watch_roots.retain(|p| p != path);
    }

    /// Start the filesystem watcher.
    pub async fn start(&mut self) -> anyhow::Result<()> {
        tracing::info!(roots = self.watch_roots.len(), "starting fs watcher");
        self.running = true;
        Ok(())
    }

    /// Stop the filesystem watcher.
    pub async fn stop(&mut self) -> anyhow::Result<()> {
        tracing::info!("stopping fs watcher");
        self.running = false;
        Ok(())
    }

    /// Whether the watcher is currently running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for FsWatchService {
    fn default() -> Self {
        Self::new()
    }
}
