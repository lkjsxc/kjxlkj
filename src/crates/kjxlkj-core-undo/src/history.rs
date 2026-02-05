//! Undo history implementation.

use kjxlkj_core_types::{BufferVersion, Position, Range};

/// The kind of edit operation.
#[derive(Debug, Clone, PartialEq)]
pub enum EditKind {
    Insert {
        pos: Position,
        text: String,
    },
    Delete {
        range: Range,
        text: String,
    },
    Replace {
        range: Range,
        old: String,
        new: String,
    },
}

/// A single edit operation that can be undone/redone.
#[derive(Debug, Clone)]
pub struct Edit {
    pub kind: EditKind,
    pub version_before: BufferVersion,
    pub version_after: BufferVersion,
    pub cursor_before: Position,
    pub cursor_after: Position,
}

impl Edit {
    /// Create an insert edit.
    pub fn insert(
        pos: Position,
        text: String,
        version_before: BufferVersion,
        version_after: BufferVersion,
        cursor_after: Position,
    ) -> Self {
        Self {
            kind: EditKind::Insert { pos, text },
            version_before,
            version_after,
            cursor_before: pos,
            cursor_after,
        }
    }

    /// Create a delete edit.
    pub fn delete(
        range: Range,
        text: String,
        version_before: BufferVersion,
        version_after: BufferVersion,
        cursor_after: Position,
    ) -> Self {
        Self {
            kind: EditKind::Delete { range, text },
            version_before,
            version_after,
            cursor_before: range.start,
            cursor_after,
        }
    }

    /// Get the inverse edit for undo.
    pub fn inverse(&self) -> EditKind {
        match &self.kind {
            EditKind::Insert { pos, text } => EditKind::Delete {
                range: Range::from_coords(
                    pos.line,
                    pos.column,
                    pos.line,
                    pos.column + text.chars().count(),
                ),
                text: text.clone(),
            },
            EditKind::Delete { range, text } => EditKind::Insert {
                pos: range.start,
                text: text.clone(),
            },
            EditKind::Replace { range, old, new } => EditKind::Replace {
                range: Range::from_coords(
                    range.start.line,
                    range.start.column,
                    range.start.line,
                    range.start.column + new.chars().count(),
                ),
                old: new.clone(),
                new: old.clone(),
            },
        }
    }
}

/// Undo history for a buffer.
#[derive(Debug, Default)]
pub struct UndoHistory {
    undo_stack: Vec<Edit>,
    redo_stack: Vec<Edit>,
    transaction: Option<Vec<Edit>>,
}

impl UndoHistory {
    /// Create a new empty history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a transaction (groups multiple edits into one undo).
    pub fn begin_transaction(&mut self) {
        self.transaction = Some(Vec::new());
    }

    /// Commit the current transaction.
    pub fn commit_transaction(&mut self) {
        if let Some(edits) = self.transaction.take() {
            if !edits.is_empty() {
                // Merge edits into one compound edit
                let first = edits.first().unwrap();
                let last = edits.last().unwrap();
                let compound = Edit {
                    kind: edits[0].kind.clone(), // Simplified
                    version_before: first.version_before,
                    version_after: last.version_after,
                    cursor_before: first.cursor_before,
                    cursor_after: last.cursor_after,
                };
                self.undo_stack.push(compound);
                self.redo_stack.clear();
            }
        }
    }

    /// Abort the current transaction.
    pub fn abort_transaction(&mut self) {
        self.transaction = None;
    }

    /// Record an edit.
    pub fn record(&mut self, edit: Edit) {
        if let Some(ref mut txn) = self.transaction {
            txn.push(edit);
        } else {
            self.undo_stack.push(edit);
            self.redo_stack.clear();
        }
    }

    /// Pop the last edit for undo.
    pub fn undo(&mut self) -> Option<Edit> {
        let edit = self.undo_stack.pop()?;
        self.redo_stack.push(edit.clone());
        Some(edit)
    }

    /// Pop the last undone edit for redo.
    pub fn redo(&mut self) -> Option<Edit> {
        let edit = self.redo_stack.pop()?;
        self.undo_stack.push(edit.clone());
        Some(edit)
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of undo states.
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redo states.
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.transaction = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_and_undo() {
        let mut history = UndoHistory::new();
        let edit = Edit::insert(
            Position::new(0, 0),
            "hello".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 5),
        );
        history.record(edit);
        assert!(history.can_undo());
        assert!(!history.can_redo());

        let undone = history.undo().unwrap();
        assert_eq!(undone.cursor_after, Position::new(0, 5));
        assert!(history.can_redo());
        assert!(!history.can_undo());
    }

    #[test]
    fn test_redo() {
        let mut history = UndoHistory::new();
        let edit = Edit::insert(
            Position::new(0, 0),
            "a".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 1),
        );
        history.record(edit);
        history.undo();
        assert!(history.can_redo());

        let redone = history.redo().unwrap();
        assert_eq!(redone.cursor_after, Position::new(0, 1));
        assert!(history.can_undo());
    }

    #[test]
    fn test_new_edit_clears_redo() {
        let mut history = UndoHistory::new();
        history.record(Edit::insert(
            Position::new(0, 0),
            "a".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 1),
        ));
        history.undo();
        assert!(history.can_redo());

        history.record(Edit::insert(
            Position::new(0, 0),
            "b".to_string(),
            BufferVersion::new(1),
            BufferVersion::new(2),
            Position::new(0, 1),
        ));
        assert!(!history.can_redo());
    }
}
