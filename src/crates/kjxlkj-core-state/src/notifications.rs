//! Notification system per /docs/spec/features/ui/notifications.md.

/// Notification severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyLevel {
    Info,
    Warn,
    Error,
    Debug,
}

/// A notification message.
#[derive(Debug, Clone)]
pub struct Notification {
    /// Message text.
    pub message: String,
    /// Severity level.
    pub level: NotifyLevel,
    /// Auto-dismiss timeout in ms (0 = manual).
    pub timeout: u64,
    /// Creation timestamp (ms since epoch).
    pub created_at: u64,
    /// Whether this has been read/dismissed.
    pub dismissed: bool,
}

/// Notification manager.
#[derive(Debug, Clone, Default)]
pub struct NotificationManager {
    /// Active notifications.
    pub notifications: Vec<Notification>,
    /// Maximum visible notifications.
    pub max_visible: usize,
    /// Whether notifications are enabled.
    pub enabled: bool,
}

impl NotificationManager {
    /// Create new manager.
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            max_visible: 5,
            enabled: true,
        }
    }

    /// Add a notification.
    pub fn notify(&mut self, message: String, level: NotifyLevel, timeout: u64) {
        if !self.enabled {
            return;
        }
        self.notifications.push(Notification {
            message,
            level,
            timeout,
            created_at: 0, // Would use real time.
            dismissed: false,
        });
    }

    /// Dismiss the most recent notification.
    pub fn dismiss_latest(&mut self) {
        for n in self.notifications.iter_mut().rev() {
            if !n.dismissed {
                n.dismissed = true;
                return;
            }
        }
    }

    /// Dismiss all notifications.
    pub fn dismiss_all(&mut self) {
        for n in &mut self.notifications {
            n.dismissed = true;
        }
    }

    /// Get visible (non-dismissed) notifications.
    pub fn visible(&self) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| !n.dismissed)
            .rev()
            .take(self.max_visible)
            .collect()
    }

    /// Remove dismissed notifications.
    pub fn gc(&mut self) {
        self.notifications.retain(|n| !n.dismissed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notify_and_visible() {
        let mut mgr = NotificationManager::new();
        mgr.notify("hello".into(), NotifyLevel::Info, 3000);
        assert_eq!(mgr.visible().len(), 1);
    }

    #[test]
    fn dismiss_latest() {
        let mut mgr = NotificationManager::new();
        mgr.notify("a".into(), NotifyLevel::Info, 0);
        mgr.notify("b".into(), NotifyLevel::Warn, 0);
        mgr.dismiss_latest();
        assert_eq!(mgr.visible().len(), 1);
    }

    #[test]
    fn gc_removes_dismissed() {
        let mut mgr = NotificationManager::new();
        mgr.notify("a".into(), NotifyLevel::Info, 0);
        mgr.dismiss_all();
        mgr.gc();
        assert!(mgr.notifications.is_empty());
    }
}
