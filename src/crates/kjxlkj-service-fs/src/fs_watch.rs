//! File watch events and watcher management.

use serde::{Deserialize, Serialize};

/// A file system event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FsEvent {
    Created(String),
    Modified(String),
    Deleted(String),
    Renamed(String, String),
}

/// Manages watched paths for file system events.
#[derive(Debug, Default)]
pub struct FsWatcher {
    watched_paths: Vec<String>,
}

impl FsWatcher {
    pub fn new() -> Self {
        Self {
            watched_paths: Vec::new(),
        }
    }

    /// Add a path to watch.
    pub fn add_watch(&mut self, path: &str) {
        if !self.watched_paths.contains(&path.to_string()) {
            tracing::debug!("watching: {}", path);
            self.watched_paths.push(path.to_string());
        }
    }

    /// Remove a watched path.
    pub fn remove_watch(&mut self, path: &str) {
        self.watched_paths.retain(|p| p != path);
    }

    /// List currently watched paths.
    pub fn watched(&self) -> &[String] {
        &self.watched_paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_remove() {
        let mut w = FsWatcher::new();
        w.add_watch("/tmp/a");
        w.add_watch("/tmp/b");
        assert_eq!(w.watched().len(), 2);
        w.remove_watch("/tmp/a");
        assert_eq!(w.watched().len(), 1);
        assert_eq!(w.watched()[0], "/tmp/b");
    }

    #[test]
    fn no_duplicates() {
        let mut w = FsWatcher::new();
        w.add_watch("/x");
        w.add_watch("/x");
        assert_eq!(w.watched().len(), 1);
    }

    #[test]
    fn event_variants() {
        let e = FsEvent::Renamed("old.rs".into(), "new.rs".into());
        assert_eq!(e, FsEvent::Renamed("old.rs".into(), "new.rs".into()));
    }
}
