//! Undo history management.

use kjxlkj_core_types::{BufferVersion, Cursor};

use crate::transaction::Transaction;

/// Undo history for a buffer.
#[derive(Debug)]
pub struct UndoHistory {
    /// Stack of past transactions (for undo).
    undo_stack: Vec<Transaction>,
    /// Stack of undone transactions (for redo).
    redo_stack: Vec<Transaction>,
    /// Current transaction being built.
    current: Option<Transaction>,
    /// Maximum history size.
    max_size: usize,
}

impl UndoHistory {
    /// Create a new undo history.
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current: None,
            max_size: 1000,
        }
    }

    /// Create with a custom max size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current: None,
            max_size,
        }
    }

    /// Start a new transaction.
    pub fn begin(&mut self, cursor: Cursor, version: BufferVersion) {
        if self.current.is_some() {
            self.commit();
        }
        self.current = Some(Transaction::new(cursor, version));
    }

    /// Get the current transaction mutably.
    pub fn current_mut(&mut self) -> Option<&mut Transaction> {
        self.current.as_mut()
    }

    /// Commit the current transaction.
    pub fn commit(&mut self) {
        if let Some(tx) = self.current.take() {
            if tx.has_edits() {
                self.undo_stack.push(tx);
                self.redo_stack.clear();
                self.trim();
            }
        }
    }

    /// Abort the current transaction.
    pub fn abort(&mut self) {
        self.current = None;
    }

    /// Undo the last transaction.
    pub fn undo(&mut self) -> Option<&Transaction> {
        self.commit();
        if let Some(tx) = self.undo_stack.pop() {
            self.redo_stack.push(tx);
            self.redo_stack.last()
        } else {
            None
        }
    }

    /// Redo the last undone transaction.
    pub fn redo(&mut self) -> Option<&Transaction> {
        if let Some(tx) = self.redo_stack.pop() {
            self.undo_stack.push(tx);
            self.undo_stack.last()
        } else {
            None
        }
    }

    /// Check if undo is possible.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty() || self.current.as_ref().is_some_and(|t| t.has_edits())
    }

    /// Check if redo is possible.
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
        self.current = None;
    }

    /// Trim history to max size.
    fn trim(&mut self) {
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }
}

impl Default for UndoHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_edit::EditOp;
    use kjxlkj_core_types::CharOffset;

    #[test]
    fn empty_history() {
        let history = UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn undo_redo_cycle() {
        let mut history = UndoHistory::new();

        history.begin(Cursor::origin(), BufferVersion::default());
        history.current_mut().unwrap().push(EditOp::insert(CharOffset::new(0), "test"));
        history.commit();

        assert!(history.can_undo());
        assert!(!history.can_redo());

        history.undo();
        assert!(!history.can_undo());
        assert!(history.can_redo());

        history.redo();
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn redo_cleared_on_new_edit() {
        let mut history = UndoHistory::new();

        history.begin(Cursor::origin(), BufferVersion::default());
        history.current_mut().unwrap().push(EditOp::insert(CharOffset::new(0), "a"));
        history.commit();

        history.undo();
        assert!(history.can_redo());

        // New edit should clear redo
        history.begin(Cursor::origin(), BufferVersion::default());
        history.current_mut().unwrap().push(EditOp::insert(CharOffset::new(0), "b"));
        history.commit();

        assert!(!history.can_redo());
    }
}
