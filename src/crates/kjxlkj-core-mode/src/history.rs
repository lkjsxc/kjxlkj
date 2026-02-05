//! Command-line history management.
//!
//! Provides separate histories for command, search, expression, and input types.

use std::collections::VecDeque;

/// Type of history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HistoryType {
    /// Ex command history (:).
    Command,
    /// Search history (/ ?).
    Search,
    /// Expression history (=).
    Expression,
    /// User input history (@).
    Input,
    /// Debug history (>).
    Debug,
}

/// A single history instance for one type.
#[derive(Debug, Clone)]
pub struct HistoryList {
    /// History entries (oldest first).
    entries: VecDeque<String>,
    /// Maximum size.
    max_size: usize,
    /// Current navigation position (None = at prompt).
    position: Option<usize>,
    /// Current prefix for filtering.
    prefix: String,
}

impl HistoryList {
    /// Create new history with default max size.
    pub fn new() -> Self {
        Self::with_max_size(1000)
    }

    /// Create history with custom max size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_size,
            position: None,
            prefix: String::new(),
        }
    }

    /// Add an entry to history.
    pub fn add(&mut self, entry: impl Into<String>) {
        let entry = entry.into();
        if entry.is_empty() {
            return;
        }

        // Remove duplicate if exists.
        if let Some(pos) = self.entries.iter().position(|e| e == &entry) {
            self.entries.remove(pos);
        }

        // Add to end.
        self.entries.push_back(entry);

        // Trim if over max size.
        while self.entries.len() > self.max_size {
            self.entries.pop_front();
        }

        self.reset_navigation();
    }

    /// Get number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if history is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get entry at index (0 = oldest).
    pub fn get(&self, index: usize) -> Option<&str> {
        self.entries.get(index).map(|s| s.as_str())
    }

    /// Reset navigation to prompt.
    pub fn reset_navigation(&mut self) {
        self.position = None;
        self.prefix.clear();
    }

    /// Set prefix for filtered navigation.
    pub fn set_prefix(&mut self, prefix: impl Into<String>) {
        self.prefix = prefix.into();
        self.position = None;
    }

    /// Navigate to previous entry (older).
    pub fn prev(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        let matching = self.matching_indices();
        if matching.is_empty() {
            return None;
        }

        let new_pos = match self.position {
            None => matching.len().saturating_sub(1),
            Some(pos) => {
                if let Some(current_idx) = matching.iter().position(|&i| i == pos) {
                    if current_idx > 0 {
                        current_idx - 1
                    } else {
                        return self.entries.get(pos).map(|s| s.as_str());
                    }
                } else {
                    matching.len().saturating_sub(1)
                }
            }
        };

        if let Some(&idx) = matching.get(new_pos) {
            self.position = Some(idx);
            self.entries.get(idx).map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Navigate to next entry (newer).
    pub fn next_entry(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        let matching = self.matching_indices();
        if matching.is_empty() {
            return None;
        }

        match self.position {
            None => None,
            Some(pos) => {
                if let Some(current_idx) = matching.iter().position(|&i| i == pos) {
                    if current_idx < matching.len() - 1 {
                        let new_pos = matching[current_idx + 1];
                        self.position = Some(new_pos);
                        self.entries.get(new_pos).map(|s| s.as_str())
                    } else {
                        // At end, return to prompt.
                        self.position = None;
                        None
                    }
                } else {
                    self.position = None;
                    None
                }
            }
        }
    }

    /// Get current entry if navigating.
    pub fn current(&self) -> Option<&str> {
        self.position.and_then(|pos| self.entries.get(pos).map(|s| s.as_str()))
    }

    /// Get all entries as iterator.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|s| s.as_str())
    }

    /// Get indices matching current prefix.
    fn matching_indices(&self) -> Vec<usize> {
        if self.prefix.is_empty() {
            (0..self.entries.len()).collect()
        } else {
            self.entries
                .iter()
                .enumerate()
                .filter(|(_, e)| e.starts_with(&self.prefix))
                .map(|(i, _)| i)
                .collect()
        }
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.reset_navigation();
    }
}

impl Default for HistoryList {
    fn default() -> Self {
        Self::new()
    }
}

/// Manager for all history types.
#[derive(Debug, Clone)]
pub struct HistoryManager {
    /// Command history.
    pub command: HistoryList,
    /// Search history.
    pub search: HistoryList,
    /// Expression history.
    pub expression: HistoryList,
    /// Input history.
    pub input: HistoryList,
    /// Debug history.
    pub debug: HistoryList,
}

impl HistoryManager {
    /// Create new history manager.
    pub fn new() -> Self {
        Self {
            command: HistoryList::new(),
            search: HistoryList::new(),
            expression: HistoryList::new(),
            input: HistoryList::new(),
            debug: HistoryList::new(),
        }
    }

    /// Get history by type.
    pub fn get(&self, history_type: HistoryType) -> &HistoryList {
        match history_type {
            HistoryType::Command => &self.command,
            HistoryType::Search => &self.search,
            HistoryType::Expression => &self.expression,
            HistoryType::Input => &self.input,
            HistoryType::Debug => &self.debug,
        }
    }

    /// Get mutable history by type.
    pub fn get_mut(&mut self, history_type: HistoryType) -> &mut HistoryList {
        match history_type {
            HistoryType::Command => &mut self.command,
            HistoryType::Search => &mut self.search,
            HistoryType::Expression => &mut self.expression,
            HistoryType::Input => &mut self.input,
            HistoryType::Debug => &mut self.debug,
        }
    }

    /// Add to history.
    pub fn add(&mut self, history_type: HistoryType, entry: impl Into<String>) {
        self.get_mut(history_type).add(entry);
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_add() {
        let mut history = HistoryList::new();
        history.add("first");
        history.add("second");
        assert_eq!(history.len(), 2);
        assert_eq!(history.get(0), Some("first"));
        assert_eq!(history.get(1), Some("second"));
    }

    #[test]
    fn test_history_dedup() {
        let mut history = HistoryList::new();
        history.add("first");
        history.add("second");
        history.add("first");
        // Duplicate removed, re-added at end.
        assert_eq!(history.len(), 2);
        assert_eq!(history.get(0), Some("second"));
        assert_eq!(history.get(1), Some("first"));
    }

    #[test]
    fn test_history_max_size() {
        let mut history = HistoryList::with_max_size(3);
        history.add("1");
        history.add("2");
        history.add("3");
        history.add("4");
        assert_eq!(history.len(), 3);
        assert_eq!(history.get(0), Some("2"));
        assert_eq!(history.get(2), Some("4"));
    }

    #[test]
    fn test_history_navigation() {
        let mut history = HistoryList::new();
        history.add("first");
        history.add("second");
        history.add("third");

        // Navigate backwards (newest first).
        assert_eq!(history.prev(), Some("third"));
        assert_eq!(history.prev(), Some("second"));
        assert_eq!(history.prev(), Some("first"));
        assert_eq!(history.prev(), Some("first")); // Stay at oldest.

        // Navigate forwards.
        assert_eq!(history.next_entry(), Some("second"));
        assert_eq!(history.next_entry(), Some("third"));
        assert_eq!(history.next_entry(), None); // Back to prompt.
    }

    #[test]
    fn test_history_prefix_filter() {
        let mut history = HistoryList::new();
        history.add("write");
        history.add("quit");
        history.add("wq");
        history.add("qa");

        history.set_prefix("w");
        assert_eq!(history.prev(), Some("wq"));
        assert_eq!(history.prev(), Some("write"));

        history.set_prefix("q");
        assert_eq!(history.prev(), Some("qa"));
        assert_eq!(history.prev(), Some("quit"));
    }

    #[test]
    fn test_history_manager() {
        let mut manager = HistoryManager::new();
        manager.add(HistoryType::Command, ":write");
        manager.add(HistoryType::Search, "pattern");

        assert_eq!(manager.get(HistoryType::Command).len(), 1);
        assert_eq!(manager.get(HistoryType::Search).len(), 1);
    }

    #[test]
    fn test_history_empty_entry() {
        let mut history = HistoryList::new();
        history.add("");
        assert!(history.is_empty());
    }

    #[test]
    fn test_history_clear() {
        let mut history = HistoryList::new();
        history.add("test");
        history.clear();
        assert!(history.is_empty());
    }
}
