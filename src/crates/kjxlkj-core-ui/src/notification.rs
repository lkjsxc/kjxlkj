//! Notification types for display.

use serde::{Deserialize, Serialize};

/// Notification severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
}

/// A notification message to display in the message area.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Message text.
    pub message: String,
    /// Severity level.
    pub level: NotificationLevel,
    /// Unix timestamp when this notification was created.
    pub timestamp: u64,
}

impl Notification {
    /// Create an info notification.
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: NotificationLevel::Info,
            timestamp: 0,
        }
    }

    /// Create a warning notification.
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: NotificationLevel::Warning,
            timestamp: 0,
        }
    }

    /// Create an error notification.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: NotificationLevel::Error,
            timestamp: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_levels() {
        let n = Notification::error("test");
        assert_eq!(n.level, NotificationLevel::Error);
        assert_eq!(n.message, "test");
    }
}
