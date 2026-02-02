//! History list management.

use crate::history_types::HistoryEntry;

/// List of history entries.
#[derive(Debug, Clone)]
pub struct HistoryList {
    entries: Vec<HistoryEntry>,
    max_size: usize,
    position: Option<usize>,
}

impl Default for HistoryList {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl HistoryList {
    /// Creates a new history list.
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
            position: None,
        }
    }

    /// Adds an entry to the history.
    pub fn add(&mut self, text: &str, timestamp: u64) {
        // Remove duplicates.
        self.entries.retain(|e| e.text != text);

        // Add at front.
        self.entries.insert(0, HistoryEntry::new(text, timestamp));

        // Trim to max size.
        if self.entries.len() > self.max_size {
            self.entries.truncate(self.max_size);
        }

        // Reset navigation.
        self.position = None;
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Starts navigation from the end.
    pub fn start_nav(&mut self) {
        self.position = None;
    }

    /// Goes to older entry.
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

    /// Goes to newer entry.
    pub fn newer(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        let new_pos = match self.position {
            None => return None,
            Some(0) => return None,
            Some(p) => p - 1,
        };

        self.position = Some(new_pos);
        self.entries.get(new_pos).map(|e| e.text.as_str())
    }

    /// Searches for entries starting with prefix.
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
    fn test_history_list_clear() {
        let mut list = HistoryList::new(10);
        list.add("test", 1);
        list.clear();
        assert!(list.is_empty());
    }
}
