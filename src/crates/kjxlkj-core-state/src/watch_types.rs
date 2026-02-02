//! File watcher types and configuration.

use std::path::PathBuf;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_event() {
        assert_eq!(WatchEvent::Created, WatchEvent::Created);
        assert_ne!(WatchEvent::Created, WatchEvent::Modified);
    }

    #[test]
    fn test_pending_event() {
        let event = PendingEvent::new(PathBuf::from("/test.txt"), WatchEvent::Modified);
        assert_eq!(event.path, PathBuf::from("/test.txt"));
        assert_eq!(event.event, WatchEvent::Modified);
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
    fn test_pending_event_stale() {
        let event = PendingEvent::new(PathBuf::from("/test.txt"), WatchEvent::Modified);
        assert!(!event.is_stale(Duration::from_secs(10)));
    }
}
