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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn transaction_new_is_empty() {
        let tx = Transaction::new(Position::new(0, 0));
        assert!(tx.is_empty());
    }

    #[test]
    fn transaction_cursor_before() {
        let tx = Transaction::new(Position::new(5, 10));
        assert_eq!(tx.cursor_before, Position::new(5, 10));
    }

    #[test]
    fn transaction_push_not_empty() {
        let mut tx = Transaction::new(Position::new(0, 0));
        tx.push(Edit::insert(Position::new(0, 0), "a".to_string()));
        assert!(!tx.is_empty());
    }

    #[test]
    fn edit_insert_kind() {
        let edit = Edit::insert(Position::new(0, 0), "a".to_string());
        assert!(matches!(edit.kind, EditKind::Insert));
    }

    #[test]
    fn edit_delete_kind() {
        let edit = Edit::delete(Position::new(0, 0), "a".to_string());
        assert!(matches!(edit.kind, EditKind::Delete));
    }

    #[test]
    fn transaction_cursor_after() {
        let tx = Transaction::new(Position::new(0, 0));
        assert_eq!(tx.cursor_after, Position::new(0, 0));
    }

    #[test]
    fn edit_text_content() {
        let edit = Edit::insert(Position::new(0, 0), "hello".to_string());
        assert_eq!(edit.text, "hello");
    }

    #[test]
    fn edit_position_field() {
        let pos = Position::new(5, 10);
        let edit = Edit::insert(pos, "x".to_string());
        assert_eq!(edit.position, pos);
    }

    #[test]
    fn transaction_set_cursor_after() {
        let mut tx = Transaction::new(Position::new(0, 0));
        tx.cursor_after = Position::new(1, 2);
        assert_eq!(tx.cursor_after.line, 1);
    }

    #[test]
    fn transaction_multiple_edits() {
        let mut tx = Transaction::new(Position::new(0, 0));
        tx.push(Edit::insert(Position::new(0, 0), "a".to_string()));
        tx.push(Edit::insert(Position::new(0, 1), "b".to_string()));
        assert_eq!(tx.edits.len(), 2);
    }
}
