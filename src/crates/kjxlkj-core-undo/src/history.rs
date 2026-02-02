//! Linear undo history.

use kjxlkj_core_edit::Transaction;
use serde::{Deserialize, Serialize};

/// Linear undo history.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UndoHistory {
    /// Undo stack.
    undo_stack: Vec<Transaction>,
    /// Redo stack.
    redo_stack: Vec<Transaction>,
    /// Maximum history size.
    max_size: usize,
}

impl UndoHistory {
    /// Creates a new undo history.
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 1000,
        }
    }

    /// Creates with a maximum size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    /// Pushes a transaction.
    pub fn push(&mut self, transaction: Transaction) {
        if transaction.is_empty() {
            return;
        }
        self.redo_stack.clear();
        self.undo_stack.push(transaction);
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Undoes the last transaction.
    pub fn undo(&mut self) -> Option<Transaction> {
        let tx = self.undo_stack.pop()?;
        self.redo_stack.push(tx.clone());
        Some(tx)
    }

    /// Redoes the last undone transaction.
    pub fn redo(&mut self) -> Option<Transaction> {
        let tx = self.redo_stack.pop()?;
        self.undo_stack.push(tx.clone());
        Some(tx)
    }

    /// Returns true if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns true if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clears all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Returns the number of undo entries.
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Returns the number of redo entries.
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_edit::Edit;
    use kjxlkj_core_types::{BufferId, BufferVersion, Position};

    fn make_tx() -> Transaction {
        let mut tx = Transaction::new(BufferVersion::initial());
        tx.push(Edit::insert(BufferId::new(1), Position::origin(), "a"));
        tx
    }

    #[test]
    fn test_new_history() {
        let history = UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_push_and_undo() {
        let mut history = UndoHistory::new();
        history.push(make_tx());
        assert!(history.can_undo());
        
        let tx = history.undo();
        assert!(tx.is_some());
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_undo_and_redo() {
        let mut history = UndoHistory::new();
        history.push(make_tx());
        history.undo();
        
        let tx = history.redo();
        assert!(tx.is_some());
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_push_clears_redo() {
        let mut history = UndoHistory::new();
        history.push(make_tx());
        history.undo();
        assert!(history.can_redo());
        
        history.push(make_tx());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_max_size() {
        let mut history = UndoHistory::with_max_size(3);
        for _ in 0..5 {
            history.push(make_tx());
        }
        assert_eq!(history.undo_count(), 3);
    }

    #[test]
    fn test_clear() {
        let mut history = UndoHistory::new();
        history.push(make_tx());
        history.clear();
        assert!(!history.can_undo());
    }

    #[test]
    fn test_empty_transaction_not_pushed() {
        let mut history = UndoHistory::new();
        let tx = Transaction::new(BufferVersion::initial());
        history.push(tx);
        assert!(!history.can_undo());
    }
}
