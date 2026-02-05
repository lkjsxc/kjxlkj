//! Editor snapshots for rendering.

use kjxlkj_core_types::{BufferId, BufferName, Cursor, Mode, Position};

/// Snapshot of a buffer's visible content.
#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    /// Buffer ID.
    pub id: BufferId,
    /// Buffer name.
    pub name: BufferName,
    /// Visible lines (already sliced to viewport).
    pub lines: Vec<String>,
    /// First visible line index.
    pub first_line: usize,
    /// Total line count.
    pub total_lines: usize,
    /// Whether buffer is modified.
    pub modified: bool,
}

/// Snapshot of a window.
#[derive(Debug, Clone)]
pub struct WindowSnapshot {
    /// Cursor position.
    pub cursor: Cursor,
    /// Visual selection anchor (if in visual mode).
    pub selection_anchor: Option<Position>,
    /// Buffer snapshot.
    pub buffer: BufferSnapshot,
    /// Viewport top line.
    pub top_line: usize,
    /// Viewport left column.
    pub left_col: usize,
}

/// Complete editor snapshot for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    /// Current mode.
    pub mode: Mode,
    /// Active window snapshot.
    pub window: WindowSnapshot,
    /// Command line content (if in command mode).
    pub command_line: String,
    /// Status message.
    pub status: String,
    /// Terminal dimensions.
    pub width: u16,
    pub height: u16,
}

impl EditorSnapshot {
    /// Create a new editor snapshot.
    pub fn new(
        mode: Mode,
        window: WindowSnapshot,
        command_line: String,
        status: String,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            mode,
            window,
            command_line,
            status,
            width,
            height,
        }
    }
}
