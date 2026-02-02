//! Location list implementation.
//!
//! Per-window list for navigating errors, search results, etc.

use std::path::PathBuf;

/// A single location list entry.
#[derive(Debug, Clone)]
pub struct LocationEntry {
    /// File path.
    pub path: PathBuf,
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub col: usize,
    /// Entry text/message.
    pub text: String,
}

impl LocationEntry {
    /// Creates a new location entry.
    pub fn new(path: PathBuf, line: usize, col: usize, text: &str) -> Self {
        Self {
            path,
            line,
            col,
            text: text.to_string(),
        }
    }

    /// Returns formatted location string.
    pub fn location(&self) -> String {
        format!("{}:{}:{}", self.path.display(), self.line, self.col)
    }
}

/// The location list (per-window).
#[derive(Debug, Clone, Default)]
pub struct LocationList {
    /// Entries in the list.
    entries: Vec<LocationEntry>,
    /// Current position.
    current: usize,
    /// Associated window ID.
    window_id: usize,
}

impl LocationList {
    /// Creates a new empty location list.
    pub fn new(window_id: usize) -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
            window_id,
        }
    }

    /// Returns the associated window ID.
    pub fn window_id(&self) -> usize {
        self.window_id
    }

    /// Sets the entries, replacing existing ones.
    pub fn set(&mut self, entries: Vec<LocationEntry>) {
        self.entries = entries;
        self.current = 0;
    }

    /// Clears the list.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current = 0;
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the current entry.
    pub fn current_entry(&self) -> Option<&LocationEntry> {
        self.entries.get(self.current)
    }

    /// Returns the current index (0-based).
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Moves to the next entry.
    pub fn advance_next(&mut self) -> Option<&LocationEntry> {
        if self.current + 1 < self.entries.len() {
            self.current += 1;
        }
        self.current_entry()
    }

    /// Moves to the previous entry.
    pub fn prev(&mut self) -> Option<&LocationEntry> {
        if self.current > 0 {
            self.current -= 1;
        }
        self.current_entry()
    }

    /// Moves to the first entry.
    pub fn first(&mut self) -> Option<&LocationEntry> {
        self.current = 0;
        self.current_entry()
    }

    /// Moves to the last entry.
    pub fn last(&mut self) -> Option<&LocationEntry> {
        if !self.entries.is_empty() {
            self.current = self.entries.len() - 1;
        }
        self.current_entry()
    }

    /// Moves to a specific index (0-based).
    pub fn goto(&mut self, index: usize) -> Option<&LocationEntry> {
        if index < self.entries.len() {
            self.current = index;
        }
        self.current_entry()
    }

    /// Returns all entries.
    pub fn entries(&self) -> &[LocationEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<LocationEntry> {
        vec![
            LocationEntry::new(PathBuf::from("a.rs"), 1, 1, "item 1"),
            LocationEntry::new(PathBuf::from("b.rs"), 2, 5, "item 2"),
            LocationEntry::new(PathBuf::from("c.rs"), 10, 3, "item 3"),
        ]
    }

    #[test]
    fn test_location_entry_location() {
        let entry = LocationEntry::new(PathBuf::from("src/lib.rs"), 42, 10, "test");
        assert_eq!(entry.location(), "src/lib.rs:42:10");
    }

    #[test]
    fn test_location_list_window_id() {
        let list = LocationList::new(5);
        assert_eq!(list.window_id(), 5);
    }

    #[test]
    fn test_location_list_navigation() {
        let mut list = LocationList::new(1);
        list.set(sample_entries());

        assert_eq!(list.current_index(), 0);
        list.advance_next();
        assert_eq!(list.current_index(), 1);
        list.prev();
        assert_eq!(list.current_index(), 0);
    }

    #[test]
    fn test_location_list_first_last() {
        let mut list = LocationList::new(1);
        list.set(sample_entries());

        list.last();
        assert_eq!(list.current_index(), 2);
        list.first();
        assert_eq!(list.current_index(), 0);
    }

    #[test]
    fn test_location_list_goto() {
        let mut list = LocationList::new(1);
        list.set(sample_entries());

        list.goto(2);
        assert_eq!(list.current_index(), 2);
    }

    #[test]
    fn test_location_list_clear() {
        let mut list = LocationList::new(1);
        list.set(sample_entries());
        list.goto(2);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.current_index(), 0);
    }
}
