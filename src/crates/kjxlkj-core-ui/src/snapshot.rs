//! Snapshot structures for rendering.

use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, CursorShape, Mode};
use serde::{Deserialize, Serialize};

use crate::status::StatusLine;
use crate::viewport::Viewport;

/// Snapshot of a single buffer for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferSnapshot {
    /// Buffer identifier.
    pub id: BufferId,
    /// Buffer name.
    pub name: BufferName,
    /// Buffer version.
    pub version: BufferVersion,
    /// Visible lines (within viewport).
    pub lines: Vec<String>,
    /// First line index of visible region.
    pub first_line: usize,
    /// Total line count.
    pub total_lines: usize,
    /// Whether the buffer is modified.
    pub modified: bool,
}

impl BufferSnapshot {
    /// Create a new buffer snapshot.
    pub fn new(id: BufferId, name: BufferName, version: BufferVersion) -> Self {
        Self {
            id,
            name,
            version,
            lines: Vec::new(),
            first_line: 0,
            total_lines: 0,
            modified: false,
        }
    }

    /// Get a line relative to the viewport.
    pub fn get_line(&self, viewport_line: usize) -> Option<&str> {
        self.lines.get(viewport_line).map(|s| s.as_str())
    }
}

/// Complete editor snapshot for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSnapshot {
    /// Current mode.
    pub mode: Mode,
    /// Active buffer snapshot.
    pub buffer: BufferSnapshot,
    /// Cursor position and shape.
    pub cursor: Cursor,
    /// Cursor shape for current mode.
    pub cursor_shape: CursorShape,
    /// Viewport state.
    pub viewport: Viewport,
    /// Status line content.
    pub status: StatusLine,
    /// Selection range (for visual modes).
    pub selection: Option<(usize, usize)>,
}

impl EditorSnapshot {
    /// Create a new editor snapshot.
    pub fn new(buffer: BufferSnapshot, mode: Mode, cursor: Cursor, viewport: Viewport) -> Self {
        let cursor_shape = match mode {
            Mode::Normal => CursorShape::Block,
            Mode::Insert => CursorShape::Bar,
            Mode::Replace => CursorShape::Underline,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => CursorShape::Hollow,
            Mode::Command => CursorShape::Block,
        };

        let status = StatusLine::from_state(
            mode,
            buffer.name.as_str(),
            buffer.modified,
            cursor.position.line,
            cursor.position.col,
            buffer.total_lines,
        );

        Self {
            mode,
            buffer,
            cursor,
            cursor_shape,
            viewport,
            status,
            selection: None,
        }
    }

    /// Get the cursor position relative to viewport.
    pub fn cursor_viewport_position(&self) -> Option<(u16, u16)> {
        let line = self.cursor.position.line;
        if line < self.viewport.scroll_top
            || line > self.viewport.scroll_top + self.viewport.visible_lines()
        {
            return None;
        }
        let row = (line - self.viewport.scroll_top) as u16;
        let col = self.cursor.position.col.saturating_sub(self.viewport.scroll_left) as u16;
        Some((col, row))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::LineCol;

    #[test]
    fn buffer_snapshot_creation() {
        let snap = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test.txt"),
            BufferVersion::default(),
        );
        assert_eq!(snap.name.as_str(), "test.txt");
    }

    #[test]
    fn cursor_shape_by_mode() {
        let buffer = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test"),
            BufferVersion::default(),
        );
        let viewport = Viewport::default();

        let normal = EditorSnapshot::new(buffer.clone(), Mode::Normal, Cursor::origin(), viewport);
        assert_eq!(normal.cursor_shape, CursorShape::Block);

        let insert = EditorSnapshot::new(buffer.clone(), Mode::Insert, Cursor::origin(), viewport);
        assert_eq!(insert.cursor_shape, CursorShape::Bar);
    }

    #[test]
    fn cursor_viewport_position_visible() {
        let buffer = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::new("test"),
            BufferVersion::default(),
        );
        let viewport = Viewport::new(80, 24);
        let cursor = Cursor::at(LineCol::new(5, 10));

        let snap = EditorSnapshot::new(buffer, Mode::Normal, cursor, viewport);
        let pos = snap.cursor_viewport_position();
        assert_eq!(pos, Some((10, 5)));
    }
}
