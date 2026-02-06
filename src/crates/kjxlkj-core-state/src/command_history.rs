//! Command-line history search and navigation.
//!
//! Implements command history with prefix search, similar to Vim's
//! Ctrl-P / Ctrl-N history navigation in command mode.

/// Command history with dedup and prefix search.
#[derive(Debug, Clone)]
pub struct CommandHistory {
    entries: Vec<String>,
    max_entries: usize,
}

impl CommandHistory {
    pub fn new(max_entries: usize) -> Self {
        Self { entries: Vec::new(), max_entries }
    }

    /// Add an entry to history (dedup: move to front if exists).
    pub fn push(&mut self, entry: &str) {
        if entry.is_empty() { return; }
        self.entries.retain(|e| e != entry);
        self.entries.push(entry.to_string());
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    /// Get the total number of entries.
    pub fn len(&self) -> usize { self.entries.len() }

    /// Check if history is empty.
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }

    /// Get entry by index (0 = oldest, len-1 = newest).
    pub fn get(&self, index: usize) -> Option<&str> {
        self.entries.get(index).map(|s| s.as_str())
    }

    /// Navigate backward (older) from current index.
    /// If current is None, start from the newest entry.
    pub fn prev(&self, current: Option<usize>) -> Option<usize> {
        if self.entries.is_empty() { return None; }
        match current {
            None => Some(self.entries.len() - 1),
            Some(0) => Some(0),
            Some(idx) => Some(idx - 1),
        }
    }

    /// Navigate forward (newer) from current index.
    /// Returns None when going past the newest entry (back to input).
    pub fn next(&self, current: Option<usize>) -> Option<usize> {
        match current {
            None => None,
            Some(idx) if idx + 1 >= self.entries.len() => None,
            Some(idx) => Some(idx + 1),
        }
    }

    /// Search backward for entries matching a prefix.
    pub fn search_prefix(&self, prefix: &str, from: Option<usize>) -> Option<usize> {
        let start = from.unwrap_or(self.entries.len());
        (0..start).rev().find(|&i| self.entries[i].starts_with(prefix))
    }

    /// Search forward for entries matching a prefix.
    pub fn search_prefix_forward(&self, prefix: &str, from: Option<usize>) -> Option<usize> {
        let start = from.map(|i| i + 1).unwrap_or(0);
        (start..self.entries.len()).find(|&i| self.entries[i].starts_with(prefix))
    }

    /// Substring search backward.
    pub fn search_substring(&self, query: &str, from: Option<usize>) -> Option<usize> {
        let start = from.unwrap_or(self.entries.len());
        (0..start).rev().find(|&i| self.entries[i].contains(query))
    }

    /// Get all entries (oldest first).
    pub fn entries(&self) -> &[String] { &self.entries }

    /// Clear all history.
    pub fn clear(&mut self) { self.entries.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_get() {
        let mut h = CommandHistory::new(100);
        h.push("w"); h.push("q"); h.push("wq");
        assert_eq!(h.len(), 3);
        assert_eq!(h.get(0), Some("w"));
        assert_eq!(h.get(2), Some("wq"));
    }

    #[test]
    fn dedup_on_push() {
        let mut h = CommandHistory::new(100);
        h.push("w"); h.push("q"); h.push("w");
        assert_eq!(h.len(), 2);
        assert_eq!(h.get(1), Some("w")); // moved to end
    }

    #[test]
    fn max_entries_enforced() {
        let mut h = CommandHistory::new(3);
        h.push("a"); h.push("b"); h.push("c"); h.push("d");
        assert_eq!(h.len(), 3);
        assert_eq!(h.get(0), Some("b")); // oldest dropped
    }

    #[test]
    fn prev_next_navigation() {
        let mut h = CommandHistory::new(100);
        h.push("first"); h.push("second"); h.push("third");
        let idx = h.prev(None); assert_eq!(idx, Some(2));
        let idx = h.prev(idx); assert_eq!(idx, Some(1));
        let idx = h.prev(idx); assert_eq!(idx, Some(0));
        let idx = h.prev(idx); assert_eq!(idx, Some(0));
        let idx = h.next(idx); assert_eq!(idx, Some(1));
        let idx = h.next(Some(2)); assert_eq!(idx, None);
    }

    #[test]
    fn prefix_search() {
        let mut h = CommandHistory::new(100);
        h.push("write"); h.push("quit"); h.push("wq"); h.push("qa");
        let idx = h.search_prefix("w", None);
        assert_eq!(idx, Some(2)); // "wq" is the latest "w" prefix match
        let idx = h.search_prefix("w", idx);
        assert_eq!(idx, Some(0)); // "write"
    }

    #[test]
    fn substring_search() {
        let mut h = CommandHistory::new(100);
        h.push("set number"); h.push("quit"); h.push("set wrap");
        let idx = h.search_substring("set", None);
        assert_eq!(idx, Some(2)); // "set wrap"
        let idx = h.search_substring("set", idx);
        assert_eq!(idx, Some(0)); // "set number"
    }

    #[test]
    fn empty_push_ignored() {
        let mut h = CommandHistory::new(100);
        h.push(""); h.push("x"); h.push("");
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn clear_history() {
        let mut h = CommandHistory::new(100);
        h.push("a"); h.push("b");
        h.clear();
        assert!(h.is_empty());
    }
}
