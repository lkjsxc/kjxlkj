//! Undo tree types: entries and groups.

use serde::{Deserialize, Serialize};

/// A single reversible edit operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoEntry {
    /// Starting char index of the affected range.
    pub start: usize,
    /// The removed text (for redo: text to remove again).
    pub old_text: String,
    /// The inserted text (for undo: text to remove).
    pub new_text: String,
    /// Cursor position before the edit.
    pub cursor_before: (usize, usize),
    /// Cursor position after the edit.
    pub cursor_after: (usize, usize),
}

/// A group of edits that form a single undo step.
///
/// All edits in an insert session (from `i` to `Esc`) are grouped.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoGroup {
    /// The individual edits in this group.
    pub entries: Vec<UndoEntry>,
    /// Timestamp when this group was created (ms since epoch).
    pub timestamp: u64,
}
