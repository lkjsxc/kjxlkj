//! Editor snapshot for rendering.

use kjxlkj_core_types::{Cursor, Mode, Selection};

use crate::status::StatusLine;
use crate::viewport::Viewport;

/// A line of text for rendering.
#[derive(Debug, Clone)]
pub struct SnapshotLine {
    /// Line index (0-based).
    pub line_idx: usize,
    /// Line content (without trailing newline).
    pub content: String,
}

/// Immutable snapshot of editor state for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    /// Visible lines.
    pub lines: Vec<SnapshotLine>,
    /// Current cursor position.
    pub cursor: Cursor,
    /// Current mode.
    pub mode: Mode,
    /// Current selection (if any).
    pub selection: Option<Selection>,
    /// Viewport info.
    pub viewport: Viewport,
    /// Status line.
    pub status: StatusLine,
    /// Command line content (if in command mode).
    pub command_line: Option<String>,
    /// Search pattern (for highlighting).
    pub search_pattern: Option<String>,
}

impl EditorSnapshot {
    /// Create a minimal empty snapshot.
    pub fn empty() -> Self {
        Self {
            lines: Vec::new(),
            cursor: Cursor::origin(),
            mode: Mode::Normal,
            selection: None,
            viewport: Viewport::default(),
            status: StatusLine::new(),
            command_line: None,
            search_pattern: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_snapshot() {
        let snap = EditorSnapshot::empty();
        assert!(snap.lines.is_empty());
        assert_eq!(snap.mode, Mode::Normal);
    }
}
