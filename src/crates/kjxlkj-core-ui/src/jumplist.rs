//! Jumplist and changelist models for navigation history.
//!
//! Implements jump navigation as specified in
//! `/docs/spec/features/navigation/jumplist.md`.

use std::path::PathBuf;

/// A jump location in history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JumpEntry {
    /// File path.
    pub path: PathBuf,
    /// Line number (0-based).
    pub line: usize,
    /// Column number (0-based).
    pub col: usize,
}

impl JumpEntry {
    /// Create a new jump entry.
    pub fn new(path: PathBuf, line: usize, col: usize) -> Self {
        Self { path, line, col }
    }

    /// Create from position in current buffer.
    pub fn from_position(path: PathBuf, line: usize, col: usize) -> Self {
        Self { path, line, col }
    }
}

/// Jumplist for Ctrl-o / Ctrl-i navigation.
#[derive(Debug, Default)]
pub struct Jumplist {
    /// Jump entries.
    entries: Vec<JumpEntry>,
    /// Current position in the list.
    cursor: usize,
    /// Maximum entries to keep.
    max_entries: usize,
}

impl Jumplist {
    /// Create a new jumplist.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cursor: 0,
            max_entries: 100,
        }
    }

    /// Create with specific max entries.
    pub fn with_max(max: usize) -> Self {
        Self {
            entries: Vec::new(),
            cursor: 0,
            max_entries: max,
        }
    }

    /// Push a new jump entry. Clears forward history.
    pub fn push(&mut self, entry: JumpEntry) {
        // If we're not at the end, truncate forward history
        if self.cursor < self.entries.len() {
            self.entries.truncate(self.cursor);
        }

        // Don't add duplicate consecutive entries
        if let Some(last) = self.entries.last() {
            if last == &entry {
                return;
            }
        }

        self.entries.push(entry);
        self.cursor = self.entries.len();

        // Trim old entries if over max
        if self.entries.len() > self.max_entries {
            let remove_count = self.entries.len() - self.max_entries;
            self.entries.drain(0..remove_count);
            self.cursor = self.cursor.saturating_sub(remove_count);
        }
    }

    /// Jump back (Ctrl-o). Returns the entry to jump to.
    pub fn back(&mut self) -> Option<&JumpEntry> {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.entries.get(self.cursor)
        } else {
            None
        }
    }

    /// Jump forward (Ctrl-i). Returns the entry to jump to.
    pub fn forward(&mut self) -> Option<&JumpEntry> {
        if self.cursor < self.entries.len() {
            self.cursor += 1;
            self.entries.get(self.cursor.saturating_sub(1))
        } else {
            None
        }
    }

    /// Get all entries.
    pub fn entries(&self) -> &[JumpEntry] {
        &self.entries
    }

    /// Get current position.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Get entry count.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.cursor = 0;
    }
}

/// A change location for g; / g, navigation.
#[derive(Debug, Clone)]
pub struct ChangeEntry {
    /// File path.
    pub path: PathBuf,
    /// Line number (0-based).
    pub line: usize,
    /// Column number (0-based).
    pub col: usize,
}

impl ChangeEntry {
    /// Create a new change entry.
    pub fn new(path: PathBuf, line: usize, col: usize) -> Self {
        Self { path, line, col }
    }
}

/// Changelist for g; / g, navigation.
#[derive(Debug, Default)]
pub struct Changelist {
    /// Change entries.
    entries: Vec<ChangeEntry>,
    /// Current position.
    cursor: usize,
    /// Maximum entries.
    max_entries: usize,
}

impl Changelist {
    /// Create a new changelist.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cursor: 0,
            max_entries: 100,
        }
    }

    /// Record a change.
    pub fn record(&mut self, entry: ChangeEntry) {
        self.entries.push(entry);
        self.cursor = self.entries.len();

        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
            self.cursor = self.cursor.saturating_sub(1);
        }
    }

    /// Go to previous change (g;).
    pub fn prev(&mut self) -> Option<&ChangeEntry> {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.entries.get(self.cursor)
        } else {
            None
        }
    }

    /// Go to next change (g,).
    pub fn next_change(&mut self) -> Option<&ChangeEntry> {
        if self.cursor < self.entries.len() {
            let entry = self.entries.get(self.cursor);
            self.cursor += 1;
            entry
        } else {
            None
        }
    }

    /// Get all entries.
    pub fn entries(&self) -> &[ChangeEntry] {
        &self.entries
    }

    /// Get entry count.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.cursor = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_entry_new() {
        let entry = JumpEntry::new(PathBuf::from("/test.rs"), 10, 5);
        assert_eq!(entry.line, 10);
        assert_eq!(entry.col, 5);
    }

    #[test]
    fn test_jumplist_new() {
        let list = Jumplist::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_jumplist_push() {
        let mut list = Jumplist::new();
        list.push(JumpEntry::new(PathBuf::from("/a.rs"), 1, 0));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_jumplist_back_forward() {
        let mut list = Jumplist::new();
        list.push(JumpEntry::new(PathBuf::from("/a.rs"), 1, 0));
        list.push(JumpEntry::new(PathBuf::from("/b.rs"), 2, 0));
        list.push(JumpEntry::new(PathBuf::from("/c.rs"), 3, 0));

        // Cursor is at end (after last entry)
        assert_eq!(list.cursor(), 3);

        // Go back
        let entry = list.back().unwrap();
        assert_eq!(entry.line, 3);

        let entry = list.back().unwrap();
        assert_eq!(entry.line, 2);

        // Go forward
        let entry = list.forward().unwrap();
        assert_eq!(entry.line, 2);
    }

    #[test]
    fn test_jumplist_no_duplicates() {
        let mut list = Jumplist::new();
        let entry = JumpEntry::new(PathBuf::from("/a.rs"), 1, 0);
        list.push(entry.clone());
        list.push(entry);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_jumplist_truncates_forward() {
        let mut list = Jumplist::new();
        list.push(JumpEntry::new(PathBuf::from("/a.rs"), 1, 0));
        list.push(JumpEntry::new(PathBuf::from("/b.rs"), 2, 0));
        list.push(JumpEntry::new(PathBuf::from("/c.rs"), 3, 0));

        // Go back twice
        list.back();
        list.back();

        // Push new entry - should truncate forward history
        list.push(JumpEntry::new(PathBuf::from("/d.rs"), 4, 0));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_jumplist_max_entries() {
        let mut list = Jumplist::with_max(3);
        for i in 0..5 {
            list.push(JumpEntry::new(PathBuf::from(format!("/{}.rs", i)), i, 0));
        }
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_jumplist_clear() {
        let mut list = Jumplist::new();
        list.push(JumpEntry::new(PathBuf::from("/a.rs"), 1, 0));
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_change_entry_new() {
        let entry = ChangeEntry::new(PathBuf::from("/test.rs"), 5, 10);
        assert_eq!(entry.line, 5);
    }

    #[test]
    fn test_changelist_new() {
        let list = Changelist::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_changelist_record() {
        let mut list = Changelist::new();
        list.record(ChangeEntry::new(PathBuf::from("/a.rs"), 1, 0));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_changelist_navigation() {
        let mut list = Changelist::new();
        list.record(ChangeEntry::new(PathBuf::from("/a.rs"), 1, 0));
        list.record(ChangeEntry::new(PathBuf::from("/b.rs"), 2, 0));
        list.record(ChangeEntry::new(PathBuf::from("/c.rs"), 3, 0));

        // Navigate back
        let entry = list.prev().unwrap();
        assert_eq!(entry.line, 3);

        let entry = list.prev().unwrap();
        assert_eq!(entry.line, 2);

        // Navigate forward
        let entry = list.next_change().unwrap();
        assert_eq!(entry.line, 2);
    }

    #[test]
    fn test_changelist_clear() {
        let mut list = Changelist::new();
        list.record(ChangeEntry::new(PathBuf::from("/a.rs"), 1, 0));
        list.clear();
        assert!(list.is_empty());
    }
}
