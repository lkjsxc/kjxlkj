//! Undo/redo history management.

use kjxlkj_core_edit::Transaction;

/// Maximum history size.
const MAX_HISTORY_SIZE: usize = 1000;

/// Undo/redo history.
#[derive(Debug, Default)]
pub struct UndoHistory {
    /// Undo stack.
    undo_stack: Vec<Transaction>,
    /// Redo stack.
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
        while self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.remove(0);
        }
    }

    /// Pop the last transaction for undo.
    pub fn undo(&mut self) -> Option<Transaction> {
        let tx = self.undo_stack.pop()?;
        let inverse = tx.inverse();
        self.redo_stack.push(tx);
        Some(inverse)
    }

    /// Pop the last transaction for redo.
    /// Returns the transaction to re-apply.
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

    /// Get the number of undo steps available.
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redo steps available.
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}
