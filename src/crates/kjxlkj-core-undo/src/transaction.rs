//! Transaction and edit types.

use kjxlkj_core_types::Position;

/// A single edit operation.
#[derive(Debug, Clone, PartialEq)]
pub enum Edit {
    /// Insert text at a position.
    Insert {
        /// Position where text was inserted.
        position: Position,
        /// The inserted text.
        text: String,
    },
    /// Delete text at a position.
    Delete {
        /// Position where text was deleted.
        position: Position,
        /// The deleted text.
        text: String,
    },
}

impl Edit {
    /// Create an insert edit.
    pub fn insert(position: Position, text: impl Into<String>) -> Self {
        Edit::Insert {
            position,
            text: text.into(),
        }
    }

    /// Create a delete edit.
    pub fn delete(position: Position, text: impl Into<String>) -> Self {
        Edit::Delete {
            position,
            text: text.into(),
        }
    }

    /// Get the inverse of this edit (for undo).
    pub fn inverse(&self) -> Self {
        match self {
            Edit::Insert { position, text } => Edit::Delete {
                position: *position,
                text: text.clone(),
            },
            Edit::Delete { position, text } => Edit::Insert {
                position: *position,
                text: text.clone(),
            },
        }
    }
}

/// A transaction is a group of edits that form a single undo step.
#[derive(Debug, Clone, Default)]
pub struct Transaction {
    /// The edits in this transaction.
    edits: Vec<Edit>,
    /// Cursor position before the transaction.
    cursor_before: Option<Position>,
    /// Cursor position after the transaction.
    cursor_after: Option<Position>,
}

impl Transaction {
    /// Create a new empty transaction.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an edit to the transaction.
    pub fn push(&mut self, edit: Edit) {
        self.edits.push(edit);
    }

    /// Set the cursor position before the transaction.
    pub fn set_cursor_before(&mut self, position: Position) {
        self.cursor_before = Some(position);
    }

    /// Set the cursor position after the transaction.
    pub fn set_cursor_after(&mut self, position: Position) {
        self.cursor_after = Some(position);
    }

    /// Get the edits in this transaction.
    pub fn edits(&self) -> &[Edit] {
        &self.edits
    }

    /// Get cursor position before transaction.
    pub fn cursor_before(&self) -> Option<Position> {
        self.cursor_before
    }

    /// Get cursor position after transaction.
    pub fn cursor_after(&self) -> Option<Position> {
        self.cursor_after
    }

    /// Check if the transaction is empty.
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    /// Get the inverse of this transaction (for undo).
    pub fn inverse(&self) -> Self {
        let mut inv = Transaction::new();
        // Reverse order for undo
        for edit in self.edits.iter().rev() {
            inv.push(edit.inverse());
        }
        // Swap cursor positions
        inv.cursor_before = self.cursor_after;
        inv.cursor_after = self.cursor_before;
        inv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_inverse() {
        let insert = Edit::insert(Position::new(0, 0), "hello");
        let delete = insert.inverse();
        assert!(matches!(delete, Edit::Delete { .. }));

        let restored = delete.inverse();
        assert_eq!(restored, insert);
    }

    #[test]
    fn test_transaction() {
        let mut tx = Transaction::new();
        tx.set_cursor_before(Position::new(0, 0));
        tx.push(Edit::insert(Position::new(0, 0), "hello"));
        tx.set_cursor_after(Position::new(0, 5));

        assert!(!tx.is_empty());
        assert_eq!(tx.edits().len(), 1);
    }

    #[test]
    fn test_transaction_inverse() {
        let mut tx = Transaction::new();
        tx.set_cursor_before(Position::new(0, 0));
        tx.push(Edit::insert(Position::new(0, 0), "hello"));
        tx.set_cursor_after(Position::new(0, 5));

        let inv = tx.inverse();
        assert_eq!(inv.cursor_before(), Some(Position::new(0, 5)));
        assert_eq!(inv.cursor_after(), Some(Position::new(0, 0)));
    }
}
