/// Notification helpers for EditorState.
use kjxlkj_core_ui::{Notification, NotificationLevel};

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn notify_info(&mut self, msg: &str) {
        self.notifications.push(Notification {
            message: msg.to_string(),
            level: NotificationLevel::Info,
        });
    }

    pub(crate) fn notify_error(&mut self, msg: &str) {
        self.notifications.push(Notification {
            message: msg.to_string(),
            level: NotificationLevel::Error,
        });
    }
}
