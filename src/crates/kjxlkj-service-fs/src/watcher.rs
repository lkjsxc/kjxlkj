//! File watcher.

use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use crate::events::FsEvent;

/// Configuration for file watching.
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// Whether to watch recursively.
    pub recursive: bool,
    /// Debounce delay in milliseconds.
    pub debounce_ms: u64,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            recursive: true,
            debounce_ms: 100,
        }
    }
}

/// File watcher service.
pub struct FileWatcher {
    /// Watched paths.
    paths: Vec<PathBuf>,
    /// Configuration.
    config: WatchConfig,
    /// Event sender.
    tx: mpsc::Sender<FsEvent>,
}

impl FileWatcher {
    /// Creates a new file watcher.
    pub fn new(tx: mpsc::Sender<FsEvent>) -> Self {
        Self {
            paths: Vec::new(),
            config: WatchConfig::default(),
            tx,
        }
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: WatchConfig) -> Self {
        self.config = config;
        self
    }

    /// Watches a path.
    pub fn watch(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let path = path.as_ref().to_path_buf();
        if !self.paths.contains(&path) {
            self.paths.push(path);
        }
        Ok(())
    }

    /// Unwatches a path.
    pub fn unwatch(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        self.paths.retain(|p| p != path);
    }

    /// Returns watched paths.
    pub fn watched_paths(&self) -> &[PathBuf] {
        &self.paths
    }

    /// Sends an event.
    async fn send_event(&self, event: FsEvent) {
        let _ = self.tx.send(event).await;
    }
}
