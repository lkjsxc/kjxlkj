//! History types.

/// History entry type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryType {
    /// Command-line history (:commands).
    Command,
    /// Search history (/).
    Search,
    /// Expression history (=).
    Expression,
    /// Input history (@).
    Input,
    /// Debug history.
    Debug,
}

/// A history entry.
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// The entry text.
    pub text: String,
    /// Timestamp (seconds since epoch).
    pub timestamp: u64,
}

impl HistoryEntry {
    /// Creates a new history entry.
    pub fn new(text: &str, timestamp: u64) -> Self {
        Self {
            text: text.to_string(),
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_entry() {
        let entry = HistoryEntry::new("test", 12345);
        assert_eq!(entry.text, "test");
        assert_eq!(entry.timestamp, 12345);
    }

    #[test]
    fn test_history_type_eq() {
        assert_eq!(HistoryType::Command, HistoryType::Command);
        assert_ne!(HistoryType::Command, HistoryType::Search);
    }

    #[test]
    fn test_history_entry_clone() {
        let entry = HistoryEntry::new("clone_test", 999);
        let cloned = entry.clone();
        assert_eq!(entry.text, cloned.text);
        assert_eq!(entry.timestamp, cloned.timestamp);
    }
}
