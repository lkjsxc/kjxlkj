//! Linear undo/redo stack.

use kjxlkj_core_types::types::Position;
use serde::{Deserialize, Serialize};

/// Kind of text change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeKind {
    Insert,
    Delete,
}

/// A single text mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextChange {
    pub kind: ChangeKind,
    pub position: Position,
    pub text: String,
}

/// One logical edit operation (may contain multiple text changes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoEntry {
    pub changes: Vec<TextChange>,
    pub cursor_before: Position,
    pub cursor_after: Position,
    pub timestamp: u64,
}

/// Linear undo/redo history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoTree {
    entries: Vec<UndoEntry>,
    current: usize,
}

impl UndoTree {
    /// Create an empty undo tree.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
        }
    }

    /// Push a new entry, truncating any redo history beyond the current point.
    pub fn push(&mut self, entry: UndoEntry) {
        self.entries.truncate(self.current);
        self.entries.push(entry);
        self.current = self.entries.len();
    }

    /// Undo the last change, returning the entry that should be reversed.
    pub fn undo(&mut self) -> Option<&UndoEntry> {
        if self.current == 0 {
            return None;
        }
        self.current -= 1;
        Some(&self.entries[self.current])
    }

    /// Redo the next change, returning the entry that should be re-applied.
    pub fn redo(&mut self) -> Option<&UndoEntry> {
        if self.current >= self.entries.len() {
            return None;
        }
        let entry = &self.entries[self.current];
        self.current += 1;
        Some(entry)
    }

    pub fn can_undo(&self) -> bool {
        self.current > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current < self.entries.len()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Discard all history.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current = 0;
    }
}

impl Default for UndoTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(ts: u64) -> UndoEntry {
        UndoEntry {
            changes: vec![TextChange {
                kind: ChangeKind::Insert,
                position: Position::new(0, 0),
                text: format!("t{ts}"),
            }],
            cursor_before: Position::new(0, 0),
            cursor_after: Position::new(0, 2),
            timestamp: ts,
        }
    }

    #[test]
    fn push_and_undo() {
        let mut tree = UndoTree::new();
        tree.push(entry(1));
        tree.push(entry(2));
        assert_eq!(tree.entry_count(), 2);
        assert_eq!(tree.current_index(), 2);

        let e = tree.undo().unwrap();
        assert_eq!(e.timestamp, 2);
        assert_eq!(tree.current_index(), 1);
    }

    #[test]
    fn redo_after_undo() {
        let mut tree = UndoTree::new();
        tree.push(entry(1));
        tree.undo();
        assert!(tree.can_redo());
        let e = tree.redo().unwrap();
        assert_eq!(e.timestamp, 1);
        assert!(!tree.can_redo());
    }

    #[test]
    fn push_truncates_redo() {
        let mut tree = UndoTree::new();
        tree.push(entry(1));
        tree.push(entry(2));
        tree.undo();
        tree.push(entry(3));
        assert_eq!(tree.entry_count(), 2);
        assert!(!tree.can_redo());
    }

    #[test]
    fn undo_on_empty() {
        let mut tree = UndoTree::new();
        assert!(tree.undo().is_none());
        assert!(!tree.can_undo());
    }

    #[test]
    fn clear_resets() {
        let mut tree = UndoTree::new();
        tree.push(entry(1));
        tree.clear();
        assert_eq!(tree.entry_count(), 0);
        assert_eq!(tree.current_index(), 0);
    }
}
