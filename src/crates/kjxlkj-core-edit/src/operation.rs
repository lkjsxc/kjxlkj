//! Edit operation definitions.

use kjxlkj_core_types::position::Position;

/// An atomic edit operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditOperation {
    /// Insert text at a position.
    Insert {
        /// Position to insert at.
        position: Position,
        /// Text to insert.
        text: String,
    },
    /// Delete text in a range.
    Delete {
        /// Start position.
        start: Position,
        /// End position.
        end: Position,
        /// The deleted text (for undo).
        deleted: String,
    },
    /// Replace text in a range.
    Replace {
        /// Start position.
        start: Position,
        /// End position.
        end: Position,
        /// Original text (for undo).
        old_text: String,
        /// New text.
        new_text: String,
    },
    /// A batch of operations executed atomically.
    Batch(Vec<EditOperation>),
}

impl EditOperation {
    /// Creates an insert operation.
    pub fn insert(position: Position, text: impl Into<String>) -> Self {
        Self::Insert {
            position,
            text: text.into(),
        }
    }

    /// Creates a delete operation.
    pub fn delete(start: Position, end: Position, deleted: impl Into<String>) -> Self {
        Self::Delete {
            start,
            end,
            deleted: deleted.into(),
        }
    }

    /// Returns the inverse of this operation (for undo).
    pub fn inverse(&self) -> Self {
        match self {
            Self::Insert { position, text } => Self::Delete {
                start: *position,
                end: *position, // Will be calculated properly
                deleted: text.clone(),
            },
            Self::Delete {
                start,
                end: _,
                deleted,
            } => Self::Insert {
                position: *start,
                text: deleted.clone(),
            },
            Self::Replace {
                start,
                end,
                old_text,
                new_text,
            } => Self::Replace {
                start: *start,
                end: *end,
                old_text: new_text.clone(),
                new_text: old_text.clone(),
            },
            Self::Batch(ops) => Self::Batch(ops.iter().rev().map(|op| op.inverse()).collect()),
        }
    }
}

/// Result of executing an edit operation.
#[derive(Debug, Clone)]
pub struct OperationResult {
    /// The cursor position after the operation.
    pub cursor: Position,
    /// The undo operation.
    pub undo: EditOperation,
}
