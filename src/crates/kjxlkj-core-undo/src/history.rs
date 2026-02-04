//! Undo history management.

use crate::Transaction;

/// Maximum number of undo steps to keep.
const MAX_UNDO_HISTORY: usize = 1000;

/// Manages undo/redo history for a buffer.
#[derive(Debug, Clone, Default)]
pub struct UndoHistory {
    /// Stack of undoable transactions.
    undo_stack: Vec<Transaction>,
    /// Stack of redoable transactions.
    redo_stack: Vec<Transaction>,
}

impl UndoHistory {
    /// Create a new empty history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a transaction onto the undo stack.
    pub fn push(&mut self, transaction: Transaction) {
        if transaction.is_empty() {
            return;
        }

        self.undo_stack.push(transaction);
        self.redo_stack.clear();

        // Limit history size
        while self.undo_stack.len() > MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the last transaction, returning the inverse to apply.
    pub fn undo(&mut self) -> Option<Transaction> {
        self.undo_stack.pop().map(|tx| {
            let inverse = tx.inverse();
            self.redo_stack.push(tx);
            inverse
        })
    }

    /// Redo the last undone transaction.
    pub fn redo(&mut self) -> Option<Transaction> {
        self.redo_stack.pop().map(|tx| {
            self.undo_stack.push(tx.clone());
            tx
        })
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of undo steps.
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redo steps.
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edit, Transaction};
    use kjxlkj_core_types::Position;

    #[test]
    fn test_undo_redo() {
        let mut history = UndoHistory::new();

        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "hello"));
        history.push(tx);

        assert!(history.can_undo());
        assert!(!history.can_redo());

        let inverse = history.undo().unwrap();
        assert!(!inverse.is_empty());

        assert!(!history.can_undo());
        assert!(history.can_redo());

        history.redo();
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_push_clears_redo() {
        let mut history = UndoHistory::new();

        let mut tx1 = Transaction::new();
        tx1.push(Edit::insert(Position::new(0, 0), "a"));
        history.push(tx1);

        history.undo();
        assert!(history.can_redo());

        let mut tx2 = Transaction::new();
        tx2.push(Edit::insert(Position::new(0, 0), "b"));
        history.push(tx2);

        assert!(!history.can_redo());
    }
}
