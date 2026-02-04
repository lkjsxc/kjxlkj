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

    #[test]
    fn snapshot_empty_mode() {
        let snap = EditorSnapshot::empty(Viewport::default());
        assert_eq!(snap.mode, Mode::Normal);
    }

    #[test]
    fn snapshot_empty_selection() {
        let snap = EditorSnapshot::empty(Viewport::default());
        assert!(snap.selection.is_none());
    }

    #[test]
    fn snapshot_empty_cursor() {
        let snap = EditorSnapshot::empty(Viewport::default());
        assert_eq!(snap.cursor.line(), 0);
        assert_eq!(snap.cursor.col(), 0);
    }

    #[test]
    fn snapshot_default() {
        let snap = EditorSnapshot::default();
        assert_eq!(snap.mode, Mode::Normal);
    }

    #[test]
    fn snapshot_cursor_screen_pos() {
        let snap = EditorSnapshot::empty(Viewport::default());
        let (col, row) = snap.cursor_screen_position();
        assert_eq!(col, 0);
        assert_eq!(row, 0);
    }

    #[test]
    fn buffer_snapshot_empty() {
        let buf = BufferSnapshot::empty();
        assert_eq!(buf.total_lines, 1);
    }

    #[test]
    fn status_line_default() {
        let status = StatusLine::default();
        assert!(status.mode.is_empty());
    }

    #[test]
    fn buffer_snapshot_empty_lines() {
        let buf = BufferSnapshot::empty();
        assert!(buf.lines.len() <= 1);
    }

    #[test]
    fn editor_snapshot_viewport_default() {
        let snap = EditorSnapshot::default();
        assert_eq!(snap.viewport.first_line, 0);
    }

    #[test]
    fn editor_snapshot_no_selection() {
        let snap = EditorSnapshot::default();
        assert!(snap.selection.is_none());
    }

    #[test]
    fn status_line_mode_empty() {
        let status = StatusLine::default();
        assert_eq!(status.mode, "");
    }

    #[test]
    fn status_line_filename_default() {
        let status = StatusLine::default();
        assert!(status.filename.is_empty());
    }

    #[test]
    fn buffer_snapshot_id() {
        let buf = BufferSnapshot::empty();
        let _ = buf.id;
    }

    #[test]
    fn editor_snapshot_cursor() {
        let snap = EditorSnapshot::default();
        assert_eq!(snap.cursor.line(), 0);
    }

    #[test]
    fn editor_snapshot_mode() {
        let snap = EditorSnapshot::default();
        let _ = format!("{:?}", snap.mode);
    }
}
