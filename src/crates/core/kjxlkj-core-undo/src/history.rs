//! Undo history storage.

use crate::UndoGroup;

/// Maximum number of undo groups to keep.
const MAX_UNDO_DEPTH: usize = 1000;

/// Undo history for a buffer.
#[derive(Debug, Default)]
pub struct UndoHistory {
    /// Past undo groups.
    past: Vec<UndoGroup>,
    /// Future undo groups (for redo).
    future: Vec<UndoGroup>,
    /// Current in-progress group.
    current: Option<UndoGroup>,
}

impl UndoHistory {
    /// Create a new empty history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new undo group.
    pub fn begin_group(&mut self, cursor: kjxlkj_core_types::CursorPosition) {
        if let Some(group) = self.current.take() {
            if !group.is_empty() {
                self.push_group(group);
            }
        }
        self.current = Some(UndoGroup::new(cursor));
    }

    /// End the current undo group.
    pub fn end_group(&mut self, cursor: kjxlkj_core_types::CursorPosition) {
        if let Some(mut group) = self.current.take() {
            group.set_cursor_after(cursor);
            if !group.is_empty() {
                self.push_group(group);
            }
        }
    }

    /// Add an operation to the current group.
    pub fn push_op(&mut self, op: crate::EditOp) {
        if let Some(group) = &mut self.current {
            group.push(op);
        }
    }

    /// Push a completed group to history.
    fn push_group(&mut self, group: UndoGroup) {
        self.past.push(group);
        self.future.clear();

        // Limit depth.
        if self.past.len() > MAX_UNDO_DEPTH {
            self.past.remove(0);
        }
    }

    /// Pop the last group for undo.
    pub fn undo(&mut self) -> Option<UndoGroup> {
        self.past.pop().inspect(|group| {
            self.future.push(group.clone());
        })
    }

    /// Pop the last undone group for redo.
    pub fn redo(&mut self) -> Option<UndoGroup> {
        self.future.pop().inspect(|group| {
            self.past.push(group.clone());
        })
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.past.clear();
        self.future.clear();
        self.current = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditOp;
    use kjxlkj_core_types::CursorPosition;

    #[test]
    fn test_undo_redo() {
        let mut history = UndoHistory::new();
        let pos = CursorPosition::origin();

        history.begin_group(pos);
        history.push_op(EditOp::insert(pos, "hello".to_string()));
        history.end_group(pos);

        assert!(history.can_undo());
        assert!(!history.can_redo());

        let group = history.undo().unwrap();
        assert_eq!(group.ops.len(), 1);

        assert!(!history.can_undo());
        assert!(history.can_redo());

        let group = history.redo().unwrap();
        assert_eq!(group.ops.len(), 1);
    }
}
