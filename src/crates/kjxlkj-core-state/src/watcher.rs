//! File watcher state management.
//!
//! Tracks external file system changes.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Event type from file system watcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchEvent {
    /// File was created.
    Created,
    /// File was modified.
    Modified,
    /// File was deleted.
    Deleted,
    /// File was renamed.
    Renamed,
    /// Metadata changed (permissions, etc).
    Metadata,
}

/// A pending file event.
#[derive(Debug, Clone)]
pub struct PendingEvent {
    /// The path that changed.
    pub path: PathBuf,
    /// What type of change.
    pub event: WatchEvent,
    /// When the event was received.
    pub received: Instant,
}

impl PendingEvent {
    /// Creates a new pending event.
    pub fn new(path: PathBuf, event: WatchEvent) -> Self {
        Self {
            path,
            event,
            received: Instant::now(),
        }
    }

    /// Checks if event is older than given duration.
    pub fn is_stale(&self, max_age: Duration) -> bool {
        self.received.elapsed() > max_age
    }
}

/// Configuration for file watching.
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// Whether watching is enabled.
    pub enabled: bool,
    /// Debounce duration for events.
    pub debounce: Duration,
    /// Whether to watch recursively.
    pub recursive: bool,
    /// Whether to ignore hidden files.
    pub ignore_hidden: bool,
    /// Maximum number of pending events.
    pub max_pending: usize,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            debounce: Duration::from_millis(100),
            recursive: true,
            ignore_hidden: true,
            max_pending: 1000,
        }
    }
}

/// Manages file watching for a directory.
#[derive(Debug)]
pub struct WatchState {
    /// Root directory being watched.
    root: PathBuf,
    /// Configuration.
    config: WatchConfig,
    /// Pending events waiting for debounce.
    pending: Vec<PendingEvent>,
    /// Ignored paths.
    ignored: Vec<PathBuf>,
}

impl WatchState {
    /// Creates a new watch state.
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            config: WatchConfig::default(),
            pending: Vec::new(),
            ignored: Vec::new(),
        }
    }

    /// Creates with custom config.
    pub fn with_config(root: PathBuf, config: WatchConfig) -> Self {
        Self {
            root,
            config,
            pending: Vec::new(),
            ignored: Vec::new(),
        }
    }

    /// Gets the root path.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Gets the configuration.
    pub fn config(&self) -> &WatchConfig {
        &self.config
    }

    /// Adds an ignored path.
    pub fn add_ignored(&mut self, path: PathBuf) {
        if !self.ignored.contains(&path) {
            self.ignored.push(path);
        }
    }

    /// Checks if a path is ignored.
    pub fn is_ignored(&self, path: &Path) -> bool {
        self.ignored.iter().any(|p| path.starts_with(p))
    }

    /// Records a file event.
    pub fn record_event(&mut self, path: PathBuf, event: WatchEvent) {
        if !self.config.enabled {
            return;
        }

        if self.is_ignored(&path) {
            return;
        }

        // Remove old events for same path
        self.pending.retain(|e| e.path != path);

        // Add new event
        self.pending.push(PendingEvent::new(path, event));

        // Trim if too many
        while self.pending.len() > self.config.max_pending {
            self.pending.remove(0);
        }
    }

    /// Gets ready events (past debounce period).
    pub fn drain_ready(&mut self) -> Vec<PendingEvent> {
        let debounce = self.config.debounce;
        let (ready, pending): (Vec<_>, Vec<_>) = self
            .pending
            .drain(..)
            .partition(|e| e.received.elapsed() >= debounce);
        self.pending = pending;
        ready
    }

    /// Clears all pending events.
    pub fn clear(&mut self) {
        self.pending.clear();
    }

    /// Gets number of pending events.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

/// Manages multiple watch states.
#[derive(Debug, Default)]
pub struct WatchManager {
    /// Active watchers by root path.
    watchers: HashMap<PathBuf, WatchState>,
}

impl WatchManager {
    /// Creates a new watch manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts watching a directory.
    pub fn watch(&mut self, root: PathBuf) {
        if !self.watchers.contains_key(&root) {
            self.watchers.insert(root.clone(), WatchState::new(root));
        }
    }

    /// Stops watching a directory.
    pub fn unwatch(&mut self, root: &Path) {
        self.watchers.remove(root);
    }

    /// Gets a watcher for a path.
    pub fn get(&self, root: &Path) -> Option<&WatchState> {
        self.watchers.get(root)
    }

    /// Gets a mutable watcher for a path.
    pub fn get_mut(&mut self, root: &Path) -> Option<&mut WatchState> {
        self.watchers.get_mut(root)
    }

    /// Gets all watched roots.
    pub fn roots(&self) -> impl Iterator<Item = &Path> {
        self.watchers.keys().map(|p| p.as_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_event() {
        assert_eq!(WatchEvent::Created, WatchEvent::Created);
        assert_ne!(WatchEvent::Created, WatchEvent::Modified);
    }

    #[test]
    fn test_watch_config_default() {
        let config = WatchConfig::default();
        assert!(config.enabled);
        assert!(config.recursive);
        assert!(config.ignore_hidden);
        assert_eq!(config.max_pending, 1000);
    }

    #[test]
    fn test_watch_state_new() {
        let state = WatchState::new(PathBuf::from("/test"));
        assert_eq!(state.root(), Path::new("/test"));
        assert!(state.config().enabled);
    }

    #[test]
    fn test_watch_state_record() {
        let mut state = WatchState::new(PathBuf::from("/test"));
        state.record_event(PathBuf::from("/test/file.txt"), WatchEvent::Modified);
        assert_eq!(state.pending_count(), 1);
    }

    #[test]
    fn test_watch_state_ignored() {
        let mut state = WatchState::new(PathBuf::from("/test"));
        state.add_ignored(PathBuf::from("/test/node_modules"));
        assert!(state.is_ignored(Path::new("/test/node_modules/foo.js")));
        assert!(!state.is_ignored(Path::new("/test/src/main.rs")));
    }

    #[test]
    fn test_watch_manager() {
        let mut manager = WatchManager::new();
        manager.watch(PathBuf::from("/project1"));
        manager.watch(PathBuf::from("/project2"));
        assert!(manager.get(Path::new("/project1")).is_some());
        manager.unwatch(Path::new("/project1"));
        assert!(manager.get(Path::new("/project1")).is_none());
    }
}
