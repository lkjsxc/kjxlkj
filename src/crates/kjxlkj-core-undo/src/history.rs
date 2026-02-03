//! Undo history management.

use crate::UndoGroup;

/// Linear undo history with redo support.
#[derive(Debug, Default)]
pub struct UndoHistory {
    undo_stack: Vec<UndoGroup>,
    redo_stack: Vec<UndoGroup>,
}

impl UndoHistory {
    /// Creates a new empty history.
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Pushes a new undo group.
    pub fn push(&mut self, group: UndoGroup) {
        if !group.is_empty() {
            self.undo_stack.push(group);
            self.redo_stack.clear();
        }
    }

    /// Pops an undo group for undoing.
    pub fn undo(&mut self) -> Option<UndoGroup> {
        if let Some(group) = self.undo_stack.pop() {
            let redo_group = group.inverse();
            self.redo_stack.push(group);
            Some(redo_group)
        } else {
            None
        }
    }

    /// Pops a redo group for redoing.
    pub fn redo(&mut self) -> Option<UndoGroup> {
        if let Some(group) = self.redo_stack.pop() {
            self.undo_stack.push(group.clone());
            Some(group)
        } else {
            None
        }
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
}

#[cfg(test)]
mod tests {
    use crate::EditOperation;
    use kjxlkj_core_types::LineCol;

    use super::*;

    #[test]
    fn undo_redo_cycle() {
        let mut history = UndoHistory::new();
        let mut group = UndoGroup::new();
        group.push(EditOperation::Insert {
            pos: LineCol::new(0, 0),
            text: "x".to_string(),
        });
        history.push(group);
        assert!(history.can_undo());
        assert!(!history.can_redo());
        history.undo();
        assert!(!history.can_undo());
        assert!(history.can_redo());
        history.redo();
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }
}
