//! Undo history stack.

use kjxlkj_core_types::Cursor;

/// A single change that can be undone.
#[derive(Debug, Clone)]
pub struct Change {
    /// Byte offset where the change occurred.
    pub offset: usize,
    /// Text that was deleted (empty for pure insertions).
    pub deleted: String,
    /// Text that was inserted (empty for pure deletions).
    pub inserted: String,
    /// Cursor position before the change.
    pub cursor_before: Cursor,
    /// Cursor position after the change.
    pub cursor_after: Cursor,
}

impl Change {
    /// Create a new change.
    pub fn new(
        offset: usize,
        deleted: String,
        inserted: String,
        cursor_before: Cursor,
        cursor_after: Cursor,
    ) -> Self {
        Self {
            offset,
            deleted,
            inserted,
            cursor_before,
            cursor_after,
        }
    }

    /// Create an insertion change.
    pub fn insert(
        offset: usize,
        text: String,
        cursor_before: Cursor,
        cursor_after: Cursor,
    ) -> Self {
        Self::new(offset, String::new(), text, cursor_before, cursor_after)
    }

    /// Create a deletion change.
    pub fn delete(
        offset: usize,
        text: String,
        cursor_before: Cursor,
        cursor_after: Cursor,
    ) -> Self {
        Self::new(offset, text, String::new(), cursor_before, cursor_after)
    }

    /// Invert this change (for undo).
    pub fn invert(&self) -> Self {
        Self {
            offset: self.offset,
            deleted: self.inserted.clone(),
            inserted: self.deleted.clone(),
            cursor_before: self.cursor_after,
            cursor_after: self.cursor_before,
        }
    }
}

/// Undo/redo history.
#[derive(Debug, Default)]
pub struct UndoHistory {
    /// Changes that can be undone.
    undo_stack: Vec<Change>,
    /// Changes that can be redone.
    redo_stack: Vec<Change>,
    /// Whether we're in a change group.
    in_group: bool,
    /// Accumulated changes in current group.
    group_changes: Vec<Change>,
}

impl UndoHistory {
    /// Create a new empty history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a change.
    pub fn record(&mut self, change: Change) {
        // Clear redo stack when new changes are made.
        self.redo_stack.clear();

        if self.in_group {
            self.group_changes.push(change);
        } else {
            self.undo_stack.push(change);
        }
    }

    /// Start a change group (multiple changes as one undo unit).
    pub fn begin_group(&mut self) {
        self.in_group = true;
        self.group_changes.clear();
    }

    /// End a change group.
    pub fn end_group(&mut self) {
        if self.in_group && !self.group_changes.is_empty() {
            // Merge group into a single logical change.
            // For simplicity, we store them sequentially.
            for change in self.group_changes.drain(..) {
                self.undo_stack.push(change);
            }
        }
        self.in_group = false;
    }

    /// Pop a change for undo. Returns None if no changes to undo.
    pub fn undo(&mut self) -> Option<Change> {
        let change = self.undo_stack.pop()?;
        let inverted = change.invert();
        self.redo_stack.push(change);
        Some(inverted)
    }

    /// Pop a change for redo. Returns None if no changes to redo.
    pub fn redo(&mut self) -> Option<Change> {
        let change = self.redo_stack.pop()?;
        self.undo_stack.push(change.clone());
        Some(change)
    }

    /// Check if there are changes to undo.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if there are changes to redo.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.group_changes.clear();
        self.in_group = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_and_undo() {
        let mut history = UndoHistory::new();
        let change = Change::insert(0, "hello".to_string(), Cursor::origin(), Cursor::new(0, 5));
        history.record(change);
        assert!(history.can_undo());
        let undo = history.undo().unwrap();
        assert_eq!(undo.deleted, "hello");
        assert!(undo.inserted.is_empty());
    }

    #[test]
    fn test_redo() {
        let mut history = UndoHistory::new();
        let change = Change::insert(0, "hi".to_string(), Cursor::origin(), Cursor::new(0, 2));
        history.record(change);
        history.undo();
        assert!(history.can_redo());
        let redo = history.redo().unwrap();
        assert_eq!(redo.inserted, "hi");
    }
}
