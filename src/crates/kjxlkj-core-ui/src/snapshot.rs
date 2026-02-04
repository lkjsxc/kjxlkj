//! Snapshot for rendering.

use kjxlkj_core_types::{Mode, Position, Range};

/// Immutable snapshot of editor state for rendering.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// Visible lines of text.
    pub lines: Vec<String>,
    /// First visible line index in the buffer.
    pub top_line: usize,
    /// Cursor position (relative to buffer).
    pub cursor: Position,
    /// Current mode.
    pub mode: Mode,
    /// Selection range (if any).
    pub selection: Option<Range>,
    /// Status message.
    pub status: String,
    /// Command line content (when in Command mode).
    pub cmdline: String,
    /// Total line count in buffer.
    pub total_lines: usize,
    /// File path (if any).
    pub file_path: Option<String>,
    /// Modified flag.
    pub modified: bool,
    /// Viewport dimensions.
    pub viewport_width: usize,
    pub viewport_height: usize,
}

impl Default for Snapshot {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
            top_line: 0,
            cursor: Position::zero(),
            mode: Mode::Normal,
            selection: None,
            status: String::new(),
            cmdline: String::new(),
            total_lines: 1,
            file_path: None,
            modified: false,
            viewport_width: 80,
            viewport_height: 24,
        }
    }
}

impl Snapshot {
    /// Get the cursor position relative to the viewport.
    pub fn cursor_viewport_pos(&self) -> (usize, usize) {
        let row = self.cursor.line.saturating_sub(self.top_line);
        let col = self.cursor.col;
        (row, col)
    }

    /// Check if cursor is on a visible line.
    pub fn is_cursor_visible(&self) -> bool {
        self.cursor.line >= self.top_line && self.cursor.line < self.top_line + self.viewport_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_default() {
        let snap = Snapshot::default();
        assert_eq!(snap.mode, Mode::Normal);
        assert_eq!(snap.cursor, Position::zero());
    }

    #[test]
    fn test_cursor_viewport_pos() {
        let snap = Snapshot {
            cursor: Position::new(10, 5),
            top_line: 5,
            ..Default::default()
        };
        assert_eq!(snap.cursor_viewport_pos(), (5, 5));
    }
}
