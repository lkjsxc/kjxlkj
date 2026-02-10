//! Edit operation types.

use kjxlkj_core_types::CursorPosition;

/// A single edit operation.
#[derive(Debug, Clone)]
pub enum EditOp {
    /// Insert text at position.
    Insert {
        /// Position where text was inserted.
        pos: CursorPosition,
        /// Text that was inserted.
        text: String,
    },
    /// Delete text range.
    Delete {
        /// Start position.
        start: CursorPosition,
        /// End position.
        end: CursorPosition,
        /// Text that was deleted (for undo).
        text: String,
    },
}

impl EditOp {
    /// Create an insert operation.
    pub fn insert(pos: CursorPosition, text: String) -> Self {
        Self::Insert { pos, text }
    }

    /// Create a delete operation.
    pub fn delete(start: CursorPosition, end: CursorPosition, text: String) -> Self {
        Self::Delete { start, end, text }
    }

    /// Get the inverse operation for undo.
    pub fn inverse(&self) -> Self {
        match self {
            EditOp::Insert { pos, text } => {
                let end = compute_end_position(*pos, text);
                EditOp::Delete {
                    start: *pos,
                    end,
                    text: text.clone(),
                }
            }
            EditOp::Delete { start, text, .. } => EditOp::Insert {
                pos: *start,
                text: text.clone(),
            },
        }
    }
}

/// Compute end position after inserting text.
fn compute_end_position(start: CursorPosition, text: &str) -> CursorPosition {
    let mut line = start.line;
    let mut grapheme = start.grapheme;

    for c in text.chars() {
        if c == '\n' {
            line += 1;
            grapheme = 0;
        } else {
            grapheme += 1;
        }
    }

    CursorPosition::new(line, grapheme)
}

/// A group of operations that form a single undo unit.
#[derive(Debug, Clone, Default)]
pub struct UndoGroup {
    /// Operations in this group.
    pub ops: Vec<EditOp>,
    /// Cursor position before the group.
    pub cursor_before: CursorPosition,
    /// Cursor position after the group.
    pub cursor_after: CursorPosition,
}

impl UndoGroup {
    /// Create a new empty undo group.
    pub fn new(cursor: CursorPosition) -> Self {
        Self {
            ops: Vec::new(),
            cursor_before: cursor,
            cursor_after: cursor,
        }
    }

    /// Add an operation to the group.
    pub fn push(&mut self, op: EditOp) {
        self.ops.push(op);
    }

    /// Check if the group is empty.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Set the cursor position after the group.
    pub fn set_cursor_after(&mut self, cursor: CursorPosition) {
        self.cursor_after = cursor;
    }
}
