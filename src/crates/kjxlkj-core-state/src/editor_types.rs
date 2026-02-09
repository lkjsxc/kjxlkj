//! Editor-level type definitions.

use kjxlkj_core_edit::CursorPosition;
use kjxlkj_core_types::BufferId;

/// A stored mark: buffer id + cursor position.
#[derive(Debug, Clone, Copy)]
pub struct MarkEntry {
    pub buffer: BufferId,
    pub cursor: CursorPosition,
}

/// A quickfix list entry.
#[derive(Debug, Clone)]
pub struct QuickfixEntry {
    /// File path.
    pub file: String,
    /// Line number (1-indexed).
    pub line: usize,
    /// Column number (1-indexed).
    pub col: usize,
    /// Error type (E, W, I).
    pub kind: char,
    /// Error text.
    pub text: String,
}
