//! Editor snapshot for rendering.

use kjxlkj_core_types::{BufferId, BufferVersion, Cursor, Mode, Selection};
use crate::Viewport;

/// A snapshot of a buffer for rendering.
#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    /// Buffer ID.
    pub id: BufferId,
    /// Buffer version.
    pub version: BufferVersion,
    /// Visible lines (content only).
    pub lines: Vec<String>,
    /// First visible line index.
    pub first_line: usize,
    /// Total line count.
    pub total_lines: usize,
    /// File path display name.
    pub name: String,
    /// Whether buffer is modified.
    pub modified: bool,
}

impl BufferSnapshot {
    /// Create an empty snapshot.
    pub fn empty() -> Self {
        Self {
            id: BufferId::default(),
            version: BufferVersion::default(),
            lines: vec![String::new()],
            first_line: 0,
            total_lines: 1,
            name: String::from("[No Name]"),
            modified: false,
        }
    }
}

/// Status line content.
#[derive(Debug, Clone, Default)]
pub struct StatusLine {
    /// Mode indicator.
    pub mode: String,
    /// File name.
    pub filename: String,
    /// Modified indicator.
    pub modified: bool,
    /// Line and column.
    pub position: String,
    /// File percentage.
    pub percentage: String,
    /// Status message.
    pub message: Option<String>,
    /// Command line content (if in command mode).
    pub command_line: Option<String>,
}

/// A complete editor snapshot for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    /// Buffer snapshot.
    pub buffer: BufferSnapshot,
    /// Cursor position.
    pub cursor: Cursor,
    /// Visual selection if any.
    pub selection: Option<Selection>,
    /// Current mode.
    pub mode: Mode,
    /// Viewport.
    pub viewport: Viewport,
    /// Status line.
    pub status: StatusLine,
}

impl EditorSnapshot {
    /// Create an empty snapshot.
    pub fn empty(viewport: Viewport) -> Self {
        Self {
            buffer: BufferSnapshot::empty(),
            cursor: Cursor::origin(),
            selection: None,
            mode: Mode::Normal,
            viewport,
            status: StatusLine::default(),
        }
    }

    /// Get the cursor position relative to viewport.
    pub fn cursor_screen_position(&self) -> (u16, u16) {
        let row = self.cursor.line().saturating_sub(self.viewport.first_line);
        let col = self.cursor.col();
        (col as u16, row as u16)
    }
}

impl Default for EditorSnapshot {
    fn default() -> Self {
        Self::empty(Viewport::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Position;

    #[test]
    fn empty_snapshot() {
        let snap = EditorSnapshot::empty(Viewport::new(80, 24));
        assert_eq!(snap.mode, Mode::Normal);
        assert_eq!(snap.buffer.lines.len(), 1);
    }

    #[test]
    fn cursor_screen_position() {
        let mut snap = EditorSnapshot::empty(Viewport::new(80, 24));
        snap.cursor = Cursor::new(Position::new(5, 10));
        snap.viewport.first_line = 3;
        let (col, row) = snap.cursor_screen_position();
        assert_eq!(row, 2); // 5 - 3
        assert_eq!(col, 10);
    }

    #[test]
    fn buffer_snapshot_empty() {
        let snap = BufferSnapshot::empty();
        assert_eq!(snap.name, "[No Name]");
        assert!(!snap.modified);
        assert_eq!(snap.total_lines, 1);
    }

    #[test]
    fn status_line_default() {
        let status = StatusLine::default();
        assert!(status.message.is_none());
        assert!(status.command_line.is_none());
    }

    #[test]
    fn snapshot_default() {
        let snap = EditorSnapshot::default();
        assert_eq!(snap.mode, Mode::Normal);
    }

    #[test]
    fn cursor_at_viewport_start() {
        let mut snap = EditorSnapshot::empty(Viewport::new(80, 24));
        snap.cursor = Cursor::new(Position::new(0, 0));
        snap.viewport.first_line = 0;
        let (col, row) = snap.cursor_screen_position();
        assert_eq!(row, 0);
        assert_eq!(col, 0);
    }

    #[test]
    fn snapshot_with_selection() {
        let mut snap = EditorSnapshot::empty(Viewport::new(80, 24));
        snap.selection = Some(Selection::new(
            Position::new(0, 0),
            Position::new(0, 5),
            kjxlkj_core_types::SelectionKind::Char,
        ));
        assert!(snap.selection.is_some());
    }

    #[test]
    fn snapshot_mode_change() {
        let mut snap = EditorSnapshot::default();
        snap.mode = Mode::Insert;
        assert_eq!(snap.mode, Mode::Insert);
    }

    #[test]
    fn buffer_snapshot_modified() {
        let mut snap = BufferSnapshot::empty();
        snap.modified = true;
        assert!(snap.modified);
    }

    #[test]
    fn status_line_with_message() {
        let mut status = StatusLine::default();
        status.message = Some("Test message".to_string());
        assert!(status.message.is_some());
        assert_eq!(status.message.as_ref().unwrap(), "Test message");
    }

    #[test]
    fn status_line_with_command() {
        let mut status = StatusLine::default();
        status.command_line = Some("w".to_string());
        assert!(status.command_line.is_some());
    }
}
