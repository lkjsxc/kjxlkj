//! Transaction and edit types.

use kjxlkj_core_types::Position;

/// A single edit operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    /// The kind of edit.
    pub kind: EditKind,
    /// Position where the edit occurred.
    pub position: Position,
    /// Text involved (inserted or deleted).
    pub text: String,
}

/// The kind of edit operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditKind {
    /// Text was inserted.
    Insert,
    /// Text was deleted.
    Delete,
}

impl Edit {
    /// Create an insert edit.
    pub fn insert(position: Position, text: String) -> Self {
        Self {
            kind: EditKind::Insert,
            position,
            text,
        }
    }

    /// Create a delete edit.
    pub fn delete(position: Position, text: String) -> Self {
        Self {
            kind: EditKind::Delete,
            position,
            text,
        }
    }

    /// Invert this edit (for undo).
    pub fn invert(&self) -> Self {
        Self {
            kind: match self.kind {
                EditKind::Insert => EditKind::Delete,
                EditKind::Delete => EditKind::Insert,
            },
            position: self.position,
            text: self.text.clone(),
        }
    }
}

/// A transaction groups multiple edits as one undo step.
#[derive(Debug, Clone, Default)]
pub struct Transaction {
    /// The edits in this transaction.
    pub edits: Vec<Edit>,
    /// Cursor position before the transaction.
    pub cursor_before: Position,
    /// Cursor position after the transaction.
    pub cursor_after: Position,
}

impl Transaction {
    /// Create a new empty transaction.
    pub fn new(cursor_before: Position) -> Self {
        Self {
            edits: Vec::new(),
            cursor_before,
            cursor_after: cursor_before,
        }
    }

    /// Add an edit to the transaction.
    pub fn push(&mut self, edit: Edit) {
        self.edits.push(edit);
    }

    /// Set the final cursor position.
    pub fn set_cursor_after(&mut self, pos: Position) {
        self.cursor_after = pos;
    }

    /// Create an inverted transaction (for undo).
    pub fn invert(&self) -> Self {
        Self {
            edits: self.edits.iter().rev().map(|e| e.invert()).collect(),
            cursor_before: self.cursor_after,
            cursor_after: self.cursor_before,
        }
    }

    /// Check if the transaction is empty.
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }
}
