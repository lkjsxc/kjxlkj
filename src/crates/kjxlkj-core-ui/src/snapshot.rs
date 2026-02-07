//! Snapshot types for rendering the editor state.

use kjxlkj_core_types::{BufferId, BufferVersion, Mode, Position, WindowId};
use serde::{Deserialize, Serialize};

/// Complete snapshot of editor state for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSnapshot {
    pub mode: Mode,
    pub buffers: Vec<BufferSnapshot>,
    pub active_buffer: BufferId,
    pub windows: Vec<WindowSnap>,
    pub status_line: String,
    pub command_line: String,
    pub message: Option<String>,
    pub cursor: CursorState,
    pub tab_line: Option<String>,
    pub terminal_width: u16,
    pub terminal_height: u16,
}

/// Snapshot of a single buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferSnapshot {
    pub id: BufferId,
    pub name: String,
    pub lines: Vec<String>,
    pub line_count: usize,
    pub modified: bool,
    pub filetype: String,
    pub version: BufferVersion,
}

/// Snapshot of a window viewport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSnap {
    pub id: WindowId,
    pub buffer_id: BufferId,
    pub cursor: Position,
    pub top_line: usize,
    pub left_col: usize,
    pub width: u16,
    pub height: u16,
}

/// Current cursor state for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorState {
    pub position: Position,
    pub shape: CursorShape,
    pub visible: bool,
    pub blink: bool,
}

/// Visual cursor shape.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CursorShape {
    Block,
    Line,
    Underline,
}

/// Hint for cursor rendering position.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorHint {
    pub show: bool,
    pub row: u16,
    pub col: u16,
}

impl CursorHint {
    /// Create a visible cursor hint.
    pub fn visible(row: u16, col: u16) -> Self {
        Self {
            show: true,
            row,
            col,
        }
    }

    /// Create a hidden cursor hint.
    pub fn hidden() -> Self {
        Self {
            show: false,
            row: 0,
            col: 0,
        }
    }
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            position: Position::ZERO,
            shape: CursorShape::Block,
            visible: true,
            blink: false,
        }
    }
}

impl Default for CursorShape {
    fn default() -> Self {
        Self::Block
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_hint_visible() {
        let h = CursorHint::visible(5, 10);
        assert!(h.show);
        assert_eq!(h.row, 5);
        assert_eq!(h.col, 10);
    }

    #[test]
    fn cursor_hint_hidden() {
        let h = CursorHint::hidden();
        assert!(!h.show);
    }

    #[test]
    fn default_cursor_state() {
        let cs = CursorState::default();
        assert!(cs.visible);
        assert_eq!(cs.shape, CursorShape::Block);
    }
}
