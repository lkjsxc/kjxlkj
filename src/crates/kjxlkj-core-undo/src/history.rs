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
        self.redo_stack.pop().inspect(|tx| {
            self.undo_stack.push(tx.clone());
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

    #[test]
    fn test_history_new() {
        let history = UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_history_clear() {
        let mut history = UndoHistory::new();
        
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "a"));
        history.push(tx);
        
        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_undo_count() {
        let mut history = UndoHistory::new();
        assert_eq!(history.undo_count(), 0);
        
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "a"));
        history.push(tx);
        
        assert_eq!(history.undo_count(), 1);
    }

    #[test]
    fn test_redo_count() {
        let mut history = UndoHistory::new();
        
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "a"));
        history.push(tx);
        
        history.undo();
        assert_eq!(history.redo_count(), 1);
    }

    #[test]
    fn test_undo_on_empty() {
        let mut history = UndoHistory::new();
        assert!(history.undo().is_none());
    }

    #[test]
    fn test_redo_on_empty() {
        let mut history = UndoHistory::new();
        assert!(history.redo().is_none());
    }

    #[test]
    fn test_multiple_undo() {
        let mut history = UndoHistory::new();
        
        for i in 0..3 {
            let mut tx = Transaction::new();
            tx.push(Edit::insert(Position::new(0, i), "x"));
            history.push(tx);
        }
        
        assert_eq!(history.undo_count(), 3);
        
        history.undo();
        history.undo();
        
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 2);
    }

    #[test]
    fn test_empty_transaction_ignored() {
        let mut history = UndoHistory::new();
        
        let tx = Transaction::new();
        history.push(tx);
        
        assert_eq!(history.undo_count(), 0);
    }

    #[test]
    fn test_redo_after_new_change() {
        let mut history = UndoHistory::new();
        
        // Push first change
        let mut tx1 = Transaction::new();
        tx1.push(Edit::insert(Position::new(0, 0), "a"));
        history.push(tx1);
        
        // Undo it
        history.undo();
        assert!(history.can_redo());
        
        // Push new change - should clear redo stack
        let mut tx2 = Transaction::new();
        tx2.push(Edit::insert(Position::new(0, 0), "b"));
        history.push(tx2);
        
        assert!(!history.can_redo());
    }

    #[test]
    fn test_undo_redo_cycle() {
        let mut history = UndoHistory::new();
        
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "test"));
        history.push(tx);
        
        // Multiple undo/redo cycles
        for _ in 0..5 {
            assert!(history.can_undo());
            history.undo();
            assert!(history.can_redo());
            history.redo();
        }
        
        // Should still be undoable
        assert!(history.can_undo());
    }

    #[test]
    fn test_max_history_limit() {
        let mut history = UndoHistory::new();
        
        // Push more than MAX_UNDO_HISTORY
        for i in 0..1010 {
            let mut tx = Transaction::new();
            tx.push(Edit::insert(Position::new(0, 0), i.to_string()));
            history.push(tx);
        }
        
        // Should be limited
        assert!(history.undo_count() <= 1000);
    }

    #[test]
    fn test_history_clone() {
        let mut history = UndoHistory::new();
        
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "clone"));
        history.push(tx);
        
        let cloned = history.clone();
        assert_eq!(cloned.undo_count(), 1);
    }

    #[test]
    fn test_history_default() {
        let history: UndoHistory = Default::default();
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_history_debug() {
        let history = UndoHistory::new();
        let debug = format!("{:?}", history);
        assert!(debug.contains("UndoHistory"));
    }

    #[test]
    fn test_undo_returns_inverse() {
        let mut history = UndoHistory::new();
        
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "inserted"));
        history.push(tx);
        
        let inverse = history.undo().unwrap();
        // The inverse should contain a delete
        assert!(!inverse.is_empty());
    }

    #[test]
    fn test_redo_returns_original() {
        let mut history = UndoHistory::new();
        
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "text"));
        history.push(tx);
        
        history.undo();
        let redo_tx = history.redo().unwrap();
        assert!(!redo_tx.is_empty());
    }

    #[test]
    fn test_clear_removes_all() {
        let mut history = UndoHistory::new();
        
        for _ in 0..5 {
            let mut tx = Transaction::new();
            tx.push(Edit::insert(Position::new(0, 0), "x"));
            history.push(tx);
        }
        
        history.undo();
        history.undo();
        
        assert!(history.undo_count() > 0);
        assert!(history.redo_count() > 0);
        
        history.clear();
        
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_single_undo_then_redo() {
        let mut history = UndoHistory::new();
        
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "single"));
        history.push(tx);
        
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
        
        history.undo();
        
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 1);
        
        history.redo();
        
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_can_undo_after_push() {
        let mut history = UndoHistory::new();
        assert!(!history.can_undo());
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "x"));
        history.push(tx);
        assert!(history.can_undo());
    }

    #[test]
    fn test_can_redo_after_undo() {
        let mut history = UndoHistory::new();
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "y"));
        history.push(tx);
        assert!(!history.can_redo());
        history.undo();
        assert!(history.can_redo());
    }

    #[test]
    fn test_undo_count_zero_initially() {
        let history = UndoHistory::new();
        assert_eq!(history.undo_count(), 0);
    }

    #[test]
    fn test_redo_count_zero_initially() {
        let history = UndoHistory::new();
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_push_multiple_transactions() {
        let mut history = UndoHistory::new();
        for i in 1..=10 {
            let mut tx = Transaction::new();
            tx.push(Edit::insert(Position::new(0, 0), "x"));
            history.push(tx);
            assert_eq!(history.undo_count(), i);
        }
    }
}

