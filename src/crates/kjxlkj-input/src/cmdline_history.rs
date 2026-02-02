//! Command-line history management.

/// History management for command line.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CmdHistory {
    /// History entries.
    entries: Vec<String>,
    /// Current index.
    index: Option<usize>,
}

impl CmdHistory {
    /// Creates a new history.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns if history is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Adds an entry.
    pub fn add(&mut self, entry: String) {
        if !entry.is_empty() {
            self.entries.push(entry);
        }
    }

    /// Resets the navigation index.
    pub fn reset_index(&mut self) {
        self.index = None;
    }

    /// Goes to previous entry.
    pub fn prev(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }
        let idx = match self.index {
            Some(i) if i > 0 => i - 1,
            None if !self.entries.is_empty() => self.entries.len() - 1,
            _ => return None,
        };
        self.index = Some(idx);
        Some(&self.entries[idx])
    }

    /// Goes to next entry.
    pub fn next(&mut self) -> Option<&str> {
        let idx = match self.index {
            Some(i) => i + 1,
            None => return None,
        };
        if idx >= self.entries.len() {
            self.index = None;
            None
        } else {
            self.index = Some(idx);
            Some(&self.entries[idx])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_new() {
        let hist = CmdHistory::new();
        assert!(hist.is_empty());
    }

    #[test]
    fn test_history_add() {
        let mut hist = CmdHistory::new();
        hist.add("test".into());
        assert!(!hist.is_empty());
    }

    #[test]
    fn test_history_prev() {
        let mut hist = CmdHistory::new();
        hist.add("first".into());
        hist.add("second".into());
        assert_eq!(hist.prev(), Some("second"));
        assert_eq!(hist.prev(), Some("first"));
    }

    #[test]
    fn test_history_next() {
        let mut hist = CmdHistory::new();
        hist.add("first".into());
        hist.add("second".into());
        hist.prev();
        hist.prev();
        assert_eq!(hist.next(), Some("second"));
    }
}
