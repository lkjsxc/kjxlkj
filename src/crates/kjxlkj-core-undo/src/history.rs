//! Undo history implementation.

use kjxlkj_core_types::Position;

/// A single edit operation that can be undone/redone.
#[derive(Debug, Clone)]
pub struct Edit {
    /// Position where the edit occurred.
    pub pos: Position,
    /// Text that was inserted (empty for deletions).
    pub inserted: String,
    /// Text that was deleted (empty for insertions).
    pub deleted: String,
    /// Cursor position before the edit.
    pub cursor_before: Position,
    /// Cursor position after the edit.
    pub cursor_after: Position,
}

impl Edit {
    /// Create an insertion edit.
    pub fn insert(pos: Position, text: String, cursor_before: Position) -> Self {
        let cursor_after = pos;
        Self {
            pos,
            inserted: text,
            deleted: String::new(),
            cursor_before,
            cursor_after,
        }
    }

    /// Create a deletion edit.
    pub fn delete(pos: Position, text: String, cursor_before: Position) -> Self {
        Self {
            pos,
            inserted: String::new(),
            deleted: text,
            cursor_before,
            cursor_after: pos,
        }
    }

    /// Create a replacement edit.
    pub fn replace(
        pos: Position,
        deleted: String,
        inserted: String,
        cursor_before: Position,
        cursor_after: Position,
    ) -> Self {
        Self {
            pos,
            inserted,
            deleted,
            cursor_before,
            cursor_after,
        }
    }

    /// Check if this is an insertion.
    pub fn is_insert(&self) -> bool {
        !self.inserted.is_empty() && self.deleted.is_empty()
    }

    /// Check if this is a deletion.
    pub fn is_delete(&self) -> bool {
        self.inserted.is_empty() && !self.deleted.is_empty()
    }
}

/// Linear undo/redo history.
#[derive(Debug, Clone, Default)]
pub struct UndoHistory {
    /// Past edits (for undo).
    past: Vec<Edit>,
    /// Future edits (for redo).
    future: Vec<Edit>,
}

impl UndoHistory {
    /// Create empty history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an edit.
    pub fn push(&mut self, edit: Edit) {
        self.past.push(edit);
        self.future.clear();
    }

    /// Pop an edit for undo.
    pub fn undo(&mut self) -> Option<Edit> {
        let edit = self.past.pop()?;
        self.future.push(edit.clone());
        Some(edit)
    }

    /// Pop an edit for redo.
    pub fn redo(&mut self) -> Option<Edit> {
        let edit = self.future.pop()?;
        self.past.push(edit.clone());
        Some(edit)
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.past.clear();
        self.future.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undo_redo() {
        let mut history = UndoHistory::new();
        let edit = Edit::insert(
            Position::new(0, 0),
            "hello".to_string(),
            Position::new(0, 0),
        );
        history.push(edit);
        assert!(history.can_undo());
        assert!(!history.can_redo());

        let undone = history.undo().unwrap();
        assert_eq!(undone.inserted, "hello");
        assert!(!history.can_undo());
        assert!(history.can_redo());

        let redone = history.redo().unwrap();
        assert_eq!(redone.inserted, "hello");
    }
}
