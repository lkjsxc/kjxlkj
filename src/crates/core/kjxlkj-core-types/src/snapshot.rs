//! Editor snapshot for rendering.
//!
//! See /docs/spec/architecture/render-pipeline.md for normative fields.

use crate::{Mode, WindowId};
use serde::{Deserialize, Serialize};

/// Immutable snapshot of editor state for the render task.
///
/// Contains all data needed to render without querying core or services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSnapshot {
    /// Monotonic sequence number for stale detection.
    pub sequence: u64,
    /// Current editing mode.
    pub mode: Mode,
    /// Terminal dimensions (cols, rows).
    pub terminal_size: (u16, u16),
    /// Command-line content and cursor state.
    pub cmdline: CmdlineState,
    /// Active notifications.
    pub notifications: Vec<String>,
    /// Layout summary for state dumps.
    pub layout_summary: String,
    /// Focused window ID.
    pub focused_window: WindowId,
    /// Rendered lines for each visible window.
    pub window_contents: Vec<WindowContent>,
}

impl Default for EditorSnapshot {
    fn default() -> Self {
        Self {
            sequence: 0,
            mode: Mode::Normal,
            terminal_size: (80, 24),
            cmdline: CmdlineState::default(),
            notifications: Vec::new(),
            layout_summary: String::new(),
            focused_window: WindowId(0),
            window_contents: Vec::new(),
        }
    }
}

/// Command-line state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CmdlineState {
    pub prefix: String,
    pub content: String,
    pub cursor_pos: usize,
    pub active: bool,
}

/// Rendered content for one visible window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowContent {
    pub window_id: WindowId,
    pub rect: Rect,
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub window_type: String,
    pub statusline: String,
}

/// Rectangle for window geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_basic_construction() {
        let r = Rect::new(0, 0, 80, 24);
        assert_eq!(r.width, 80);
        assert_eq!(r.height, 24);
    }
}
