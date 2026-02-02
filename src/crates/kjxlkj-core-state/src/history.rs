//! Command history persistence.

use crate::history_types::{HistoryEntry, HistoryType};
use std::collections::VecDeque;

/// History list for a specific type.
#[derive(Debug, Clone)]
pub struct HistoryList {
    /// Entries (most recent first).
    entries: VecDeque<HistoryEntry>,
    /// Maximum entries.
    max_entries: usize,
    /// Current navigation position.
    position: Option<usize>,
}

impl Default for HistoryList {
    fn default() -> Self {
        Self::new(100)
    }
}

impl HistoryList {
    /// Creates a new history list.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries,
            position: None,
        }
    }

    /// Adds an entry.
    pub fn add(&mut self, text: &str, timestamp: u64) {
        // Remove duplicates.
        self.entries.retain(|e| e.text != text);

        // Add at front.
        self.entries.push_front(HistoryEntry::new(text, timestamp));

        // Trim to max.
        while self.entries.len() > self.max_entries {
            self.entries.pop_back();
        }

        self.position = None;
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Starts navigation.
    pub fn start_nav(&mut self) {
        self.position = None;
    }

    /// Navigates to older entry.
    pub fn older(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        let new_pos = match self.position {
            None => 0,
            Some(p) if p + 1 < self.entries.len() => p + 1,
            Some(p) => p,
        };

        self.position = Some(new_pos);
        self.entries.get(new_pos).map(|e| e.text.as_str())
    }

    /// Navigates to newer entry.
    pub fn newer(&mut self) -> Option<&str> {
        match self.position {
            None | Some(0) => {
                self.position = None;
                None
            }
            Some(p) => {
                self.position = Some(p - 1);
                self.entries.get(p - 1).map(|e| e.text.as_str())
            }
        }
    }

    /// Searches for entries matching prefix.
    pub fn search(&self, prefix: &str) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.text.starts_with(prefix))
            .map(|e| e.text.as_str())
            .collect()
    }

    /// Returns all entries.
    pub fn all(&self) -> Vec<&str> {
        self.entries.iter().map(|e| e.text.as_str()).collect()
    }

    /// Clears the history.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.position = None;
    }
}

/// Complete history state.
#[derive(Debug, Clone, Default)]
pub struct History {
    /// Command history.
    pub command: HistoryList,
    /// Search history.
    pub search: HistoryList,
    /// Expression history.
    pub expression: HistoryList,
}

impl History {
    /// Creates new history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets history for a type.
    pub fn get(&self, typ: HistoryType) -> &HistoryList {
        match typ {
            HistoryType::Command => &self.command,
            HistoryType::Search => &self.search,
            HistoryType::Expression => &self.expression,
            _ => &self.command,
        }
    }

    /// Gets mutable history for a type.
    pub fn get_mut(&mut self, typ: HistoryType) -> &mut HistoryList {
        match typ {
            HistoryType::Command => &mut self.command,
            HistoryType::Search => &mut self.search,
            HistoryType::Expression => &mut self.expression,
            _ => &mut self.command,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_list_add() {
        let mut list = HistoryList::new(10);
        list.add("first", 1);
        list.add("second", 2);

        assert_eq!(list.len(), 2);
        assert_eq!(list.all()[0], "second");
    }

    #[test]
    fn test_history_list_dedup() {
        let mut list = HistoryList::new(10);
        list.add("cmd", 1);
        list.add("other", 2);
        list.add("cmd", 3);

        assert_eq!(list.len(), 2);
        assert_eq!(list.all()[0], "cmd");
    }

    #[test]
    fn test_history_list_navigation() {
        let mut list = HistoryList::new(10);
        list.add("first", 1);
        list.add("second", 2);
        list.add("third", 3);

        assert_eq!(list.older(), Some("third"));
        assert_eq!(list.older(), Some("second"));
        assert_eq!(list.newer(), Some("third"));
    }

    #[test]
    fn test_history_list_search() {
        let mut list = HistoryList::new(10);
        list.add("write", 1);
        list.add("wq", 2);
        list.add("edit", 3);

        let results = list.search("w");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_history_types() {
        let mut hist = History::new();
        hist.command.add("cmd", 1);
        hist.search.add("pattern", 2);

        assert_eq!(hist.get(HistoryType::Command).len(), 1);
        assert_eq!(hist.get(HistoryType::Search).len(), 1);
    }

    #[test]
    fn test_history_list_clear() {
        let mut list = HistoryList::new(10);
        list.add("test", 1);
        list.clear();
        assert!(list.is_empty());
    }
}
