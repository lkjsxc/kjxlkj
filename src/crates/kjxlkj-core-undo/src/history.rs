//! Simple linear undo history.

use kjxlkj_core_edit::EditOperation;

/// A simple linear undo history.
#[derive(Debug, Clone, Default)]
pub struct UndoHistory {
    /// Undo stack.
    undo_stack: Vec<UndoEntry>,
    /// Redo stack.
    redo_stack: Vec<UndoEntry>,
    /// Maximum history size.
    max_size: usize,
}

/// An entry in the undo history.
#[derive(Debug, Clone)]
pub struct UndoEntry {
    /// The operation to undo.
    pub operation: EditOperation,
    /// Cursor position before the operation.
    pub cursor_before: kjxlkj_core_types::position::Position,
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

    /// Creates a history with a custom max size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    /// Pushes an undo entry.
    pub fn push(&mut self, entry: UndoEntry) {
        // Clear redo stack on new edit
        self.redo_stack.clear();
        
        // Add to undo stack
        self.undo_stack.push(entry);
        
        // Trim if exceeds max size
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Pops an entry for undo.
    pub fn pop_undo(&mut self) -> Option<UndoEntry> {
        self.undo_stack.pop()
    }

    /// Pops an entry for redo.
    pub fn pop_redo(&mut self) -> Option<UndoEntry> {
        self.redo_stack.pop()
    }

    /// Pushes to the redo stack.
    pub fn push_redo(&mut self, entry: UndoEntry) {
        self.redo_stack.push(entry);
    }

    /// Returns true if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns true if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Returns the number of undo entries.
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Returns the number of redo entries.
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clears all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}
