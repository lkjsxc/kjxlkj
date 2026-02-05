//! Undo history.

use kjxlkj_core_types::{Position, Range};

/// An edit operation that can be undone.
#[derive(Debug, Clone)]
pub struct Edit {
    /// Range that was affected.
    pub range: Range,
    /// Text that was deleted (empty for pure insert).
    pub old_text: String,
    /// Text that was inserted (empty for pure delete).
    pub new_text: String,
    /// Cursor position before the edit.
    pub cursor_before: Position,
    /// Cursor position after the edit.
    pub cursor_after: Position,
}

/// Undo history.
#[derive(Debug, Clone, Default)]
pub struct UndoHistory {
    undo_stack: Vec<Edit>,
    redo_stack: Vec<Edit>,
}

impl UndoHistory {
    /// Create a new undo history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an edit.
    pub fn record(&mut self, edit: Edit) {
        self.undo_stack.push(edit);
        self.redo_stack.clear();
    }

    /// Get the next edit to undo.
    pub fn undo(&mut self) -> Option<Edit> {
        let edit = self.undo_stack.pop()?;
        self.redo_stack.push(edit.clone());
        Some(edit)
    }

    /// Get the next edit to redo.
    pub fn redo(&mut self) -> Option<Edit> {
        let edit = self.redo_stack.pop()?;
        self.undo_stack.push(edit.clone());
        Some(edit)
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_history() {
        let history = UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_record_and_undo() {
        let mut history = UndoHistory::new();
        let edit = Edit {
            range: Range::new(Position::origin(), Position::origin()),
            old_text: String::new(),
            new_text: "hello".to_string(),
            cursor_before: Position::origin(),
            cursor_after: Position::new(0, 5),
        };
        history.record(edit);
        assert!(history.can_undo());
        let undone = history.undo();
        assert!(undone.is_some());
        assert!(history.can_redo());
    }
}
