//! Edit operations.

use kjxlkj_core_types::{BufferId, Position, Range};
use serde::{Deserialize, Serialize};

/// Kind of edit operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditKind {
    /// Insert text at a position.
    Insert { text: String },
    /// Delete text in a range.
    Delete,
    /// Replace text in a range.
    Replace { text: String },
}

/// A single edit operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edit {
    /// Target buffer.
    pub buffer_id: BufferId,
    /// Range affected by the edit.
    pub range: Range,
    /// Kind of edit.
    pub kind: EditKind,
}

impl Edit {
    /// Creates an insert edit.
    pub fn insert(buffer_id: BufferId, pos: Position, text: impl Into<String>) -> Self {
        Self {
            buffer_id,
            range: Range::point(pos),
            kind: EditKind::Insert { text: text.into() },
        }
    }

    /// Creates a delete edit.
    pub fn delete(buffer_id: BufferId, range: Range) -> Self {
        Self {
            buffer_id,
            range,
            kind: EditKind::Delete,
        }
    }

    /// Creates a replace edit.
    pub fn replace(buffer_id: BufferId, range: Range, text: impl Into<String>) -> Self {
        Self {
            buffer_id,
            range,
            kind: EditKind::Replace { text: text.into() },
        }
    }

    /// Returns the deleted text content (if applicable).
    pub fn deleted_text(&self) -> Option<&str> {
        None
    }
}
