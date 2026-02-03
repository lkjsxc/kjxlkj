//! Edit transactions for undo/redo.

use kjxlkj_core_types::{BufferVersion, Position, Range};

/// An atomic edit operation for undo/redo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditOp {
    /// Insert text at position.
    Insert {
        pos: Position,
        text: String,
    },
    /// Delete text in range.
    Delete {
        range: Range,
        deleted_text: String,
    },
}

impl EditOp {
    /// Create an inverse operation (for undo).
    pub fn inverse(&self) -> Self {
        match self {
            EditOp::Insert { pos, text } => {
                let end = compute_end_position(*pos, text);
                EditOp::Delete {
                    range: Range::new(*pos, end),
                    deleted_text: text.clone(),
                }
            }
            EditOp::Delete { range, deleted_text } => EditOp::Insert {
                pos: range.start,
                text: deleted_text.clone(),
            },
        }
    }
}

/// Compute end position after inserting text.
fn compute_end_position(start: Position, text: &str) -> Position {
    let lines: Vec<&str> = text.split('\n').collect();
    if lines.len() == 1 {
        Position::new(
            start.line,
            start.col + kjxlkj_core_text::grapheme_count(text) as u32,
        )
    } else {
        let last_line_len = kjxlkj_core_text::grapheme_count(lines.last().unwrap_or(&""));
        Position::new(
            start.line + (lines.len() - 1) as u32,
            last_line_len as u32,
        )
    }
}

/// A transaction groups multiple edit operations.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Operations in this transaction.
    pub ops: Vec<EditOp>,
    /// Buffer version before this transaction.
    pub before_version: BufferVersion,
    /// Cursor position before this transaction.
    pub cursor_before: Position,
    /// Cursor position after this transaction.
    pub cursor_after: Position,
}

impl Transaction {
    /// Create a new empty transaction.
    pub fn new(before_version: BufferVersion, cursor_before: Position) -> Self {
        Self {
            ops: Vec::new(),
            before_version,
            cursor_before,
            cursor_after: cursor_before,
        }
    }

    /// Add an operation to the transaction.
    pub fn push(&mut self, op: EditOp) {
        self.ops.push(op);
    }

    /// Set the cursor position after the transaction.
    pub fn set_cursor_after(&mut self, pos: Position) {
        self.cursor_after = pos;
    }

    /// Create the inverse transaction (for undo).
    pub fn inverse(&self) -> Self {
        Self {
            ops: self.ops.iter().rev().map(|op| op.inverse()).collect(),
            before_version: self.before_version,
            cursor_before: self.cursor_after,
            cursor_after: self.cursor_before,
        }
    }

    /// Check if the transaction is empty.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}
