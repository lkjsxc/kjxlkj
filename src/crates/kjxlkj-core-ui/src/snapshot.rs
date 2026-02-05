//! Snapshot types for rendering.

use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, Mode, WindowId};
use serde::{Deserialize, Serialize};

use crate::Viewport;

/// Snapshot of a buffer for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferSnapshot {
    pub id: BufferId,
    pub name: BufferName,
    pub version: BufferVersion,
    pub lines: Vec<String>,
    pub line_offset: usize,
    pub total_lines: usize,
    pub modified: bool,
}

impl BufferSnapshot {
    /// Create a new buffer snapshot.
    pub fn new(
        id: BufferId,
        name: BufferName,
        version: BufferVersion,
        lines: Vec<String>,
        line_offset: usize,
        total_lines: usize,
        modified: bool,
    ) -> Self {
        Self {
            id,
            name,
            version,
            lines,
            line_offset,
            total_lines,
            modified,
        }
    }
}

/// Snapshot of a window for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSnapshot {
    pub id: WindowId,
    pub buffer: BufferSnapshot,
    pub cursor: Cursor,
    pub viewport: Viewport,
    pub active: bool,
}

impl WindowSnapshot {
    /// Create a new window snapshot.
    pub fn new(
        id: WindowId,
        buffer: BufferSnapshot,
        cursor: Cursor,
        viewport: Viewport,
        active: bool,
    ) -> Self {
        Self {
            id,
            buffer,
            cursor,
            viewport,
            active,
        }
    }

    /// Get the cursor position relative to the viewport.
    pub fn cursor_screen_pos(&self) -> (usize, usize) {
        let row = self.cursor.line.saturating_sub(self.viewport.top_line);
        let col = self.cursor.column.saturating_sub(self.viewport.left_col);
        (row, col)
    }
}

/// Complete editor snapshot for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSnapshot {
    pub sequence: u64,
    pub windows: Vec<WindowSnapshot>,
    pub active_window: WindowId,
    pub mode: Mode,
    pub command_line: String,
    pub message: Option<String>,
    pub terminal_width: u16,
    pub terminal_height: u16,
}

impl EditorSnapshot {
    /// Create a new editor snapshot.
    pub fn new(
        sequence: u64,
        windows: Vec<WindowSnapshot>,
        active_window: WindowId,
        mode: Mode,
        command_line: String,
        message: Option<String>,
        terminal_width: u16,
        terminal_height: u16,
    ) -> Self {
        Self {
            sequence,
            windows,
            active_window,
            mode,
            command_line,
            message,
            terminal_width,
            terminal_height,
        }
    }

    /// Get the active window snapshot.
    pub fn active_window(&self) -> Option<&WindowSnapshot> {
        self.windows.iter().find(|w| w.id == self.active_window)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_screen_pos() {
        let buf = BufferSnapshot::new(
            BufferId::new(1),
            BufferName::unnamed(),
            BufferVersion::initial(),
            vec!["hello".to_string()],
            0,
            1,
            false,
        );
        let mut vp = Viewport::new(80, 24);
        vp.top_line = 5;
        let win = WindowSnapshot::new(
            WindowId::new(1),
            buf,
            Cursor::new(10, 3),
            vp,
            true,
        );
        let (row, col) = win.cursor_screen_pos();
        assert_eq!(row, 5);
        assert_eq!(col, 3);
    }
}
