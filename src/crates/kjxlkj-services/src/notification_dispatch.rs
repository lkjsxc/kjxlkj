//! Notification dispatch system.

use serde::{Deserialize, Serialize};

/// Notification severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    Debug,
    Info,
    Warning,
    Error,
}

/// Source of a notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotifySource {
    Editor,
    Lsp,
    Plugin,
    Git,
    System,
}

/// A single notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: u64,
    pub severity: Severity,
    pub source: NotifySource,
    pub message: String,
    pub timestamp_ms: u64,
    pub auto_dismiss_ms: Option<u64>,
}

/// Notification dispatcher.
#[derive(Debug)]
pub struct Dispatcher {
    pub notifications: Vec<Notification>,
    pub max_visible: usize,
    next_id: u64,
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self { notifications: Vec::new(), max_visible: 5, next_id: 1 }
    }
}

impl Dispatcher {
    pub fn new(max_visible: usize) -> Self {
        Self { notifications: Vec::new(), max_visible, next_id: 1 }
    }

    /// Add a notification, returning its id.
    pub fn add(
        &mut self,
        severity: Severity,
        source: NotifySource,
        message: &str,
        timestamp_ms: u64,
        auto_dismiss_ms: Option<u64>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.notifications.push(Notification {
            id,
            severity,
            source,
            message: message.to_string(),
            timestamp_ms,
            auto_dismiss_ms,
        });
        id
    }

    /// Dismiss a notification by id.
    pub fn dismiss(&mut self, id: u64) {
        self.notifications.retain(|n| n.id != id);
    }

    /// Dismiss all notifications from a source.
    pub fn dismiss_source(&mut self, source: NotifySource) {
        self.notifications.retain(|n| n.source != source);
    }

    /// Garbage-collect auto-dismissed notifications older than `now_ms`.
    pub fn gc(&mut self, now_ms: u64) {
        self.notifications.retain(|n| {
            if let Some(dismiss_after) = n.auto_dismiss_ms {
                n.timestamp_ms + dismiss_after > now_ms
            } else {
                true
            }
        });
    }

    /// Format a notification for display.
    pub fn format_notification(n: &Notification) -> String {
        let sev = match n.severity {
            Severity::Debug => "DBG",
            Severity::Info => "INF",
            Severity::Warning => "WRN",
            Severity::Error => "ERR",
        };
        let src = match n.source {
            NotifySource::Editor => "editor",
            NotifySource::Lsp => "lsp",
            NotifySource::Plugin => "plugin",
            NotifySource::Git => "git",
            NotifySource::System => "system",
        };
        format!("[{}][{}] {}", sev, src, n.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_dismiss() {
        let mut d = Dispatcher::new(5);
        let id = d.add(Severity::Info, NotifySource::Editor, "hello", 1000, None);
        assert_eq!(d.notifications.len(), 1);
        d.dismiss(id);
        assert!(d.notifications.is_empty());
    }

    #[test]
    fn gc_auto_dismiss() {
        let mut d = Dispatcher::new(5);
        d.add(Severity::Info, NotifySource::Lsp, "temp", 1000, Some(500));
        d.gc(1400); // not yet expired
        assert_eq!(d.notifications.len(), 1);
        d.gc(1600); // expired
        assert!(d.notifications.is_empty());
    }

    #[test]
    fn format() {
        let n = Notification {
            id: 1,
            severity: Severity::Error,
            source: NotifySource::System,
            message: "disk full".into(),
            timestamp_ms: 0,
            auto_dismiss_ms: None,
        };
        assert_eq!(Dispatcher::format_notification(&n), "[ERR][system] disk full");
    }

    #[test]
    fn severity_ordering() {
        assert!(Severity::Debug < Severity::Info);
        assert!(Severity::Warning < Severity::Error);
    }
}
