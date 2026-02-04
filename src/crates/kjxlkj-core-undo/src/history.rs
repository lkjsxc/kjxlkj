//! Undo history management.

use crate::Transaction;
use kjxlkj_core_types::Position;

/// Linear undo/redo history stack.
#[derive(Debug, Clone, Default)]
pub struct UndoHistory {
    /// Past transactions (undo stack).
    undo_stack: Vec<Transaction>,
    /// Future transactions (redo stack).
    redo_stack: Vec<Transaction>,
    /// Maximum history size.
    max_size: usize,
}

impl UndoHistory {
    /// Create a new undo history.
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 1000,
        }
    }

    /// Create with a specific max size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    /// Push a transaction onto the undo stack.
    pub fn push(&mut self, transaction: Transaction) {
        self.redo_stack.clear();
        self.undo_stack.push(transaction);
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Pop a transaction for undo.
    pub fn undo(&mut self) -> Option<Transaction> {
        let tx = self.undo_stack.pop()?;
        self.redo_stack.push(tx.clone());
        Some(tx)
    }

    /// Pop a transaction for redo.
    pub fn redo(&mut self) -> Option<Transaction> {
        let tx = self.redo_stack.pop()?;
        self.undo_stack.push(tx.clone());
        Some(tx)
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the cursor position after undoing.
    pub fn undo_cursor_position(&self) -> Option<Position> {
        self.undo_stack.last().map(|tx| tx.cursor_before)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edit, EditKind};

    fn make_tx(text: &str) -> Transaction {
        Transaction {
            edits: vec![Edit {
                kind: EditKind::Insert,
                position: Position::new(0, 0),
                text: text.to_string(),
            }],
            cursor_before: Position::new(0, 0),
            cursor_after: Position::new(0, text.len()),
        }
    }

    #[test]
    fn undo_redo_cycle() {
        let mut history = UndoHistory::new();
        let tx = make_tx("hello");
        history.push(tx);
        assert!(history.can_undo());
        assert!(!history.can_redo());
        let undone = history.undo();
        assert!(undone.is_some());
        assert!(!history.can_undo());
        assert!(history.can_redo());
        let redone = history.redo();
        assert!(redone.is_some());
        assert!(history.can_undo());
    }

    #[test]
    fn push_clears_redo() {
        let mut history = UndoHistory::new();
        history.push(make_tx("a"));
        history.push(make_tx("b"));
        history.undo();
        assert!(history.can_redo());
        history.push(make_tx("c"));
        assert!(!history.can_redo());
    }
}
