//! Quickfix list implementation.
//!
//! Provides a list for navigating errors, search results, etc.

use std::path::PathBuf;

/// A single quickfix entry.
#[derive(Debug, Clone)]
pub struct QuickfixEntry {
    /// File path.
    pub path: PathBuf,
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub col: usize,
    /// Entry text/message.
    pub text: String,
    /// Entry type.
    pub kind: QuickfixKind,
}

/// Kind of quickfix entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickfixKind {
    /// Error.
    Error,
    /// Warning.
    Warning,
    /// Info.
    Info,
    /// Note.
    Note,
    /// Search result.
    Search,
}

impl QuickfixEntry {
    /// Creates a new quickfix entry.
    pub fn new(path: PathBuf, line: usize, col: usize, text: &str) -> Self {
        Self {
            path,
            line,
            col,
            text: text.to_string(),
            kind: QuickfixKind::Error,
        }
    }

    /// Sets the kind.
    pub fn with_kind(mut self, kind: QuickfixKind) -> Self {
        self.kind = kind;
        self
    }

    /// Returns formatted location string.
    pub fn location(&self) -> String {
        format!("{}:{}:{}", self.path.display(), self.line, self.col)
    }
}

/// The quickfix list.
#[derive(Debug, Clone, Default)]
pub struct QuickfixList {
    /// Entries in the list.
    entries: Vec<QuickfixEntry>,
    /// Current position in the list.
    current: usize,
    /// Title/description of the list.
    title: String,
}

impl QuickfixList {
    /// Creates a new empty quickfix list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a quickfix list with a title.
    pub fn with_title(title: &str) -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
            title: title.to_string(),
        }
    }

    /// Adds an entry to the list.
    pub fn add(&mut self, entry: QuickfixEntry) {
        self.entries.push(entry);
    }

    /// Sets the entries, replacing existing ones.
    pub fn set(&mut self, entries: Vec<QuickfixEntry>) {
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
    pub fn current_entry(&self) -> Option<&QuickfixEntry> {
        self.entries.get(self.current)
    }

    /// Returns the current index (0-based).
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Moves to the next entry.
    pub fn next(&mut self) -> Option<&QuickfixEntry> {
        if self.current + 1 < self.entries.len() {
            self.current += 1;
        }
        self.current_entry()
    }

    /// Moves to the previous entry.
    pub fn prev(&mut self) -> Option<&QuickfixEntry> {
        if self.current > 0 {
            self.current -= 1;
        }
        self.current_entry()
    }

    /// Moves to the first entry.
    pub fn first(&mut self) -> Option<&QuickfixEntry> {
        self.current = 0;
        self.current_entry()
    }

    /// Moves to the last entry.
    pub fn last(&mut self) -> Option<&QuickfixEntry> {
        if !self.entries.is_empty() {
            self.current = self.entries.len() - 1;
        }
        self.current_entry()
    }

    /// Moves to a specific index (0-based).
    pub fn goto(&mut self, index: usize) -> Option<&QuickfixEntry> {
        if index < self.entries.len() {
            self.current = index;
        }
        self.current_entry()
    }

    /// Returns all entries.
    pub fn entries(&self) -> &[QuickfixEntry] {
        &self.entries
    }

    /// Returns the title.
    pub fn title(&self) -> &str {
        &self.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<QuickfixEntry> {
        vec![
            QuickfixEntry::new(PathBuf::from("a.rs"), 1, 1, "error 1"),
            QuickfixEntry::new(PathBuf::from("b.rs"), 2, 5, "error 2"),
            QuickfixEntry::new(PathBuf::from("c.rs"), 10, 3, "error 3"),
        ]
    }

    #[test]
    fn test_quickfix_entry_location() {
        let entry = QuickfixEntry::new(PathBuf::from("src/main.rs"), 42, 10, "test");
        assert_eq!(entry.location(), "src/main.rs:42:10");
    }

    #[test]
    fn test_quickfix_list_navigation() {
        let mut list = QuickfixList::new();
        list.set(sample_entries());

        assert_eq!(list.current_index(), 0);
        list.next();
        assert_eq!(list.current_index(), 1);
        list.prev();
        assert_eq!(list.current_index(), 0);
    }

    #[test]
    fn test_quickfix_list_first_last() {
        let mut list = QuickfixList::new();
        list.set(sample_entries());

        list.last();
        assert_eq!(list.current_index(), 2);
        list.first();
        assert_eq!(list.current_index(), 0);
    }

    #[test]
    fn test_quickfix_list_goto() {
        let mut list = QuickfixList::new();
        list.set(sample_entries());

        list.goto(2);
        assert_eq!(list.current_index(), 2);
        list.goto(99); // Out of bounds, stays at 2
        assert_eq!(list.current_index(), 2);
    }

    #[test]
    fn test_quickfix_list_clear() {
        let mut list = QuickfixList::new();
        list.set(sample_entries());
        list.goto(2);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.current_index(), 0);
    }

    #[test]
    fn test_quickfix_kind() {
        let entry = QuickfixEntry::new(PathBuf::from("a.rs"), 1, 1, "msg")
            .with_kind(QuickfixKind::Warning);
        assert_eq!(entry.kind, QuickfixKind::Warning);
    }
}
