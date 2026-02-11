//! Undo/redo tree for buffer edits.
//!
//! Each undo group captures a sequence of edits (insert mode session).
//! Undo tree branches when edits occur after undo.

/// A single edit operation for undo tracking.
#[derive(Debug, Clone)]
pub enum EditOp {
    Insert {
        byte_offset: usize,
        text: String,
    },
    Delete {
        byte_offset: usize,
        text: String,
    },
}

/// One undo group (e.g., one insert mode session).
#[derive(Debug, Clone)]
pub struct UndoGroup {
    pub ops: Vec<EditOp>,
    pub cursor_before: (usize, usize),
    pub cursor_after: (usize, usize),
}

/// Linear undo history (tree variant deferred to later wave).
#[derive(Debug, Default)]
pub struct UndoHistory {
    groups: Vec<UndoGroup>,
    position: usize,
    current_group: Option<UndoGroup>,
}

impl UndoHistory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Begin a new undo group.
    pub fn begin_group(&mut self, cursor: (usize, usize)) {
        self.current_group = Some(UndoGroup {
            ops: Vec::new(),
            cursor_before: cursor,
            cursor_after: cursor,
        });
    }

    /// Record an edit in the current group.
    pub fn record(&mut self, op: EditOp) {
        if let Some(ref mut group) = self.current_group {
            group.ops.push(op);
        }
    }

    /// End the current undo group.
    pub fn end_group(&mut self, cursor: (usize, usize)) {
        if let Some(mut group) = self.current_group.take() {
            if !group.ops.is_empty() {
                group.cursor_after = cursor;
                self.groups.truncate(self.position);
                self.groups.push(group);
                self.position = self.groups.len();
            }
        }
    }

    /// Undo the last group, returning it for replay.
    pub fn undo(&mut self) -> Option<&UndoGroup> {
        if self.position == 0 {
            return None;
        }
        self.position -= 1;
        Some(&self.groups[self.position])
    }

    /// Redo the next group, returning it for replay.
    pub fn redo(&mut self) -> Option<&UndoGroup> {
        if self.position >= self.groups.len() {
            return None;
        }
        let group = &self.groups[self.position];
        self.position += 1;
        Some(group)
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        self.position > 0
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        self.position < self.groups.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_redo_cycle() {
        let mut h = UndoHistory::new();
        h.begin_group((0, 0));
        h.record(EditOp::Insert {
            byte_offset: 0,
            text: "hello".to_string(),
        });
        h.end_group((0, 5));

        assert!(h.can_undo());
        let g = h.undo().unwrap();
        assert_eq!(g.ops.len(), 1);
        assert!(!h.can_undo());
        assert!(h.can_redo());

        let g = h.redo().unwrap();
        assert_eq!(g.ops.len(), 1);
    }

    #[test]
    fn undo_truncates_redo_on_new_edit() {
        let mut h = UndoHistory::new();
        h.begin_group((0, 0));
        h.record(EditOp::Insert {
            byte_offset: 0,
            text: "a".to_string(),
        });
        h.end_group((0, 1));

        h.begin_group((0, 1));
        h.record(EditOp::Insert {
            byte_offset: 1,
            text: "b".to_string(),
        });
        h.end_group((0, 2));

        h.undo(); // undo "b"
        // New edit after undo should truncate redo
        h.begin_group((0, 1));
        h.record(EditOp::Insert {
            byte_offset: 1,
            text: "c".to_string(),
        });
        h.end_group((0, 2));

        assert!(!h.can_redo());
    }
}
