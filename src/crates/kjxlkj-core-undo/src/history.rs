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
