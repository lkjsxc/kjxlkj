//! Message and notification model.
//!
//! Manages command-line messages and optional floating notifications.
//! See /docs/spec/features/ui/notifications.md

/// Message severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MsgLevel {
    /// Hidden in UI, logged only.
    Debug,
    /// Normal informational message.
    Info,
    /// Non-critical warning (yellow).
    Warn,
    /// Failure (red).
    Error,
}

impl MsgLevel {
    /// Highlight group for this level.
    pub fn highlight(self) -> &'static str {
        match self {
            Self::Debug => "Normal",
            Self::Info => "Normal",
            Self::Warn => "WarningMsg",
            Self::Error => "ErrorMsg",
        }
    }
}

/// A single message entry.
#[derive(Debug, Clone)]
pub struct Message {
    /// Unique monotonic id.
    pub id: u64,
    /// Severity level.
    pub level: MsgLevel,
    /// Text content.
    pub text: String,
}

/// Notification position on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyPosition {
    TopRight,
    BottomRight,
}

/// Message history and current display state.
#[derive(Debug)]
pub struct MessageStore {
    /// All messages for the session.
    history: Vec<Message>,
    /// Next id counter.
    next_id: u64,
    /// Current command-line message (cleared on next keypress).
    pub current: Option<Message>,
    /// Maximum history entries.
    cap: usize,
}

impl MessageStore {
    /// Create with default capacity (200).
    pub fn new() -> Self {
        Self { history: Vec::new(), next_id: 1, current: None, cap: 200 }
    }

    /// Add a message at the given level.
    pub fn push(&mut self, level: MsgLevel, text: String) {
        let msg = Message { id: self.next_id, level, text };
        self.next_id += 1;
        if level >= MsgLevel::Info {
            self.current = Some(msg.clone());
        }
        self.history.push(msg);
        if self.history.len() > self.cap {
            self.history.remove(0);
        }
    }

    /// Push an info message.
    pub fn info(&mut self, text: String) { self.push(MsgLevel::Info, text); }

    /// Push a warning.
    pub fn warn(&mut self, text: String) { self.push(MsgLevel::Warn, text); }

    /// Push an error.
    pub fn error(&mut self, text: String) { self.push(MsgLevel::Error, text); }

    /// Clear the current displayed message (on next keypress).
    pub fn clear_current(&mut self) { self.current = None; }

    /// Clear all history.
    pub fn clear_history(&mut self) { self.history.clear(); }

    /// Full history as a slice.
    pub fn history(&self) -> &[Message] { &self.history }

    /// Number of messages in history.
    pub fn len(&self) -> usize { self.history.len() }

    /// Whether history is empty.
    pub fn is_empty(&self) -> bool { self.history.is_empty() }

    /// Messages filtered by minimum level.
    pub fn by_level(&self, min: MsgLevel) -> Vec<&Message> {
        self.history.iter().filter(|m| m.level >= min).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_sets_current() {
        let mut store = MessageStore::new();
        store.info("hello".into());
        assert!(store.current.is_some());
        assert_eq!(store.current.as_ref().unwrap().text, "hello");
    }

    #[test]
    fn debug_does_not_set_current() {
        let mut store = MessageStore::new();
        store.push(MsgLevel::Debug, "dbg".into());
        assert!(store.current.is_none());
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn clear_current() {
        let mut store = MessageStore::new();
        store.info("test".into());
        store.clear_current();
        assert!(store.current.is_none());
    }

    #[test]
    fn history_cap_enforced() {
        let mut store = MessageStore::new();
        for i in 0..250 {
            store.info(format!("msg{}", i));
        }
        assert_eq!(store.len(), 200);
        assert_eq!(store.history()[0].text, "msg50");
    }

    #[test]
    fn by_level_filters() {
        let mut store = MessageStore::new();
        store.info("i".into());
        store.warn("w".into());
        store.error("e".into());
        let warnings = store.by_level(MsgLevel::Warn);
        assert_eq!(warnings.len(), 2); // warn + error
    }

    #[test]
    fn clear_history() {
        let mut store = MessageStore::new();
        store.info("a".into());
        store.clear_history();
        assert!(store.is_empty());
    }

    #[test]
    fn ids_increment() {
        let mut store = MessageStore::new();
        store.info("a".into());
        store.info("b".into());
        assert_eq!(store.history()[0].id, 1);
        assert_eq!(store.history()[1].id, 2);
    }

    #[test]
    fn level_ordering() {
        assert!(MsgLevel::Debug < MsgLevel::Info);
        assert!(MsgLevel::Info < MsgLevel::Warn);
        assert!(MsgLevel::Warn < MsgLevel::Error);
    }
}
