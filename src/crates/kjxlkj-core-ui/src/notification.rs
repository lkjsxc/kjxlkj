//! Notification system types.
//!
//! Implements notifications as specified in `/docs/spec/features/ui/notifications.md`.

use std::time::{Duration, Instant};

/// Notification severity/type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationKind {
    /// Informational message.
    #[default]
    Info,
    /// Warning message.
    Warning,
    /// Error message.
    Error,
    /// Action required.
    Action,
    /// Success message.
    Success,
    /// Debug/trace message.
    Debug,
}

impl NotificationKind {
    /// Get default duration for this kind.
    pub fn default_duration(&self) -> Option<Duration> {
        match self {
            Self::Info | Self::Success => Some(Duration::from_secs(3)),
            Self::Warning => Some(Duration::from_secs(5)),
            Self::Error | Self::Action => None, // Persistent
            Self::Debug => Some(Duration::from_secs(2)),
        }
    }

    /// Get icon for this kind.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Info => "",
            Self::Warning => "",
            Self::Error => "",
            Self::Action => "",
            Self::Success => "",
            Self::Debug => "",
        }
    }

    /// Get fallback icon (ASCII).
    pub fn icon_ascii(&self) -> &'static str {
        match self {
            Self::Info => "[i]",
            Self::Warning => "[!]",
            Self::Error => "[x]",
            Self::Action => "[?]",
            Self::Success => "[✓]",
            Self::Debug => "[d]",
        }
    }
}

/// A notification message.
#[derive(Debug, Clone)]
pub struct Notification {
    /// Unique ID.
    pub id: u32,
    /// Notification kind.
    pub kind: NotificationKind,
    /// Message content.
    pub message: String,
    /// When the notification was created.
    pub created: Instant,
    /// Duration before auto-dismiss (None = persistent).
    pub duration: Option<Duration>,
    /// Whether this has been read/acknowledged.
    pub read: bool,
    /// Source of the notification.
    pub source: Option<String>,
}

impl Notification {
    /// Create a new notification.
    pub fn new(id: u32, kind: NotificationKind, message: impl Into<String>) -> Self {
        Self {
            id,
            kind,
            message: message.into(),
            created: Instant::now(),
            duration: kind.default_duration(),
            read: false,
            source: None,
        }
    }

    /// Create an info notification.
    pub fn info(id: u32, message: impl Into<String>) -> Self {
        Self::new(id, NotificationKind::Info, message)
    }

    /// Create a warning notification.
    pub fn warning(id: u32, message: impl Into<String>) -> Self {
        Self::new(id, NotificationKind::Warning, message)
    }

    /// Create an error notification.
    pub fn error(id: u32, message: impl Into<String>) -> Self {
        Self::new(id, NotificationKind::Error, message)
    }

    /// Set a custom duration.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Make persistent (no auto-dismiss).
    pub fn persistent(mut self) -> Self {
        self.duration = None;
        self
    }

    /// Set source.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Check if notification has expired.
    pub fn is_expired(&self) -> bool {
        if let Some(duration) = self.duration {
            self.created.elapsed() >= duration
        } else {
            false
        }
    }

    /// Mark as read.
    pub fn mark_read(&mut self) {
        self.read = true;
    }
}

/// Notification manager.
#[derive(Debug, Default)]
pub struct NotificationManager {
    /// Active notifications.
    notifications: Vec<Notification>,
    /// History of dismissed notifications.
    history: Vec<Notification>,
    /// Next notification ID.
    next_id: u32,
    /// Maximum history size.
    max_history: usize,
}

impl NotificationManager {
    /// Create a new notification manager.
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            history: Vec::new(),
            next_id: 1,
            max_history: 100,
        }
    }

    /// Add a notification.
    pub fn notify(&mut self, kind: NotificationKind, message: impl Into<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.notifications.push(Notification::new(id, kind, message));
        id
    }

    /// Add an info notification.
    pub fn info(&mut self, message: impl Into<String>) -> u32 {
        self.notify(NotificationKind::Info, message)
    }

    /// Add a warning notification.
    pub fn warning(&mut self, message: impl Into<String>) -> u32 {
        self.notify(NotificationKind::Warning, message)
    }

    /// Add an error notification.
    pub fn error(&mut self, message: impl Into<String>) -> u32 {
        self.notify(NotificationKind::Error, message)
    }

    /// Dismiss a notification by ID.
    pub fn dismiss(&mut self, id: u32) -> bool {
        if let Some(pos) = self.notifications.iter().position(|n| n.id == id) {
            let mut notification = self.notifications.remove(pos);
            notification.mark_read();
            self.add_to_history(notification);
            true
        } else {
            false
        }
    }

    /// Dismiss all notifications.
    pub fn dismiss_all(&mut self) {
        let notifications: Vec<_> = self.notifications.drain(..).collect();
        for mut n in notifications {
            n.mark_read();
            self.add_to_history(n);
        }
    }

    /// Remove expired notifications.
    pub fn cleanup_expired(&mut self) {
        let expired: Vec<_> = self
            .notifications
            .iter()
            .filter(|n| n.is_expired())
            .map(|n| n.id)
            .collect();
        for id in expired {
            self.dismiss(id);
        }
    }

    /// Get active notifications.
    pub fn active(&self) -> &[Notification] {
        &self.notifications
    }

    /// Get notification history.
    pub fn history(&self) -> &[Notification] {
        &self.history
    }

    /// Get count of active notifications.
    pub fn count(&self) -> usize {
        self.notifications.len()
    }

    /// Check if there are any active notifications.
    pub fn has_notifications(&self) -> bool {
        !self.notifications.is_empty()
    }

    /// Clear history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    fn add_to_history(&mut self, notification: Notification) {
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push(notification);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_kind_default() {
        assert_eq!(NotificationKind::default(), NotificationKind::Info);
    }

    #[test]
    fn test_notification_kind_duration() {
        assert!(NotificationKind::Info.default_duration().is_some());
        assert!(NotificationKind::Error.default_duration().is_none());
    }

    #[test]
    fn test_notification_kind_icon() {
        // Icons exist - may be nerd font Unicode characters
        let _ = NotificationKind::Error.icon();
        assert!(!NotificationKind::Error.icon_ascii().is_empty());
    }

    #[test]
    fn test_notification_new() {
        let n = Notification::new(1, NotificationKind::Info, "Hello");
        assert_eq!(n.id, 1);
        assert_eq!(n.message, "Hello");
        assert!(!n.read);
    }

    #[test]
    fn test_notification_info() {
        let n = Notification::info(1, "Test");
        assert_eq!(n.kind, NotificationKind::Info);
    }

    #[test]
    fn test_notification_warning() {
        let n = Notification::warning(1, "Test");
        assert_eq!(n.kind, NotificationKind::Warning);
    }

    #[test]
    fn test_notification_error() {
        let n = Notification::error(1, "Test");
        assert_eq!(n.kind, NotificationKind::Error);
    }

    #[test]
    fn test_notification_persistent() {
        let n = Notification::info(1, "Test").persistent();
        assert!(n.duration.is_none());
    }

    #[test]
    fn test_notification_with_source() {
        let n = Notification::info(1, "Test").with_source("LSP");
        assert_eq!(n.source, Some("LSP".to_string()));
    }

    #[test]
    fn test_notification_mark_read() {
        let mut n = Notification::info(1, "Test");
        assert!(!n.read);
        n.mark_read();
        assert!(n.read);
    }

    #[test]
    fn test_notification_manager_new() {
        let mgr = NotificationManager::new();
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_notification_manager_notify() {
        let mut mgr = NotificationManager::new();
        let id = mgr.notify(NotificationKind::Info, "Test");
        assert_eq!(mgr.count(), 1);
        assert_eq!(id, 1);
    }

    #[test]
    fn test_notification_manager_info() {
        let mut mgr = NotificationManager::new();
        mgr.info("Test");
        assert!(mgr.has_notifications());
    }

    #[test]
    fn test_notification_manager_dismiss() {
        let mut mgr = NotificationManager::new();
        let id = mgr.info("Test");
        assert!(mgr.dismiss(id));
        assert!(!mgr.has_notifications());
        assert_eq!(mgr.history().len(), 1);
    }

    #[test]
    fn test_notification_manager_dismiss_nonexistent() {
        let mut mgr = NotificationManager::new();
        assert!(!mgr.dismiss(999));
    }

    #[test]
    fn test_notification_manager_dismiss_all() {
        let mut mgr = NotificationManager::new();
        mgr.info("Test1");
        mgr.info("Test2");
        mgr.dismiss_all();
        assert!(!mgr.has_notifications());
        assert_eq!(mgr.history().len(), 2);
    }

    #[test]
    fn test_notification_manager_clear_history() {
        let mut mgr = NotificationManager::new();
        let id = mgr.info("Test");
        mgr.dismiss(id);
        mgr.clear_history();
        assert!(mgr.history().is_empty());
    }

    #[test]
    fn test_notification_manager_active() {
        let mut mgr = NotificationManager::new();
        mgr.error("Error message");
        let active = mgr.active();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].kind, NotificationKind::Error);
    }
}
