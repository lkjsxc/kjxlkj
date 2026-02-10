//! Window state types.

use kjxlkj_core_types::{
    BufferId, CursorPosition, ExplorerStateId, TerminalId, Viewport, WindowContent, WindowId,
    WindowOptions,
};

/// Window state.
#[derive(Debug, Clone)]
pub struct Window {
    /// Window ID.
    pub id: WindowId,
    /// Content type.
    pub content: WindowContent,
    /// Cursor position (buffer windows).
    pub cursor: CursorPosition,
    /// Viewport state.
    pub viewport: Viewport,
    /// Window options.
    pub options: WindowOptions,
    /// Last focus sequence for tie-breaking.
    pub last_focus_seq: u64,
}

impl Window {
    /// Create a buffer window.
    pub fn buffer(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            content: WindowContent::Buffer(buffer_id),
            cursor: CursorPosition::origin(),
            viewport: Viewport::default(),
            options: WindowOptions::default(),
            last_focus_seq: 0,
        }
    }

    /// Create an explorer window.
    pub fn explorer(id: WindowId, state_id: ExplorerStateId) -> Self {
        Self {
            id,
            content: WindowContent::Explorer(state_id),
            cursor: CursorPosition::origin(),
            viewport: Viewport::default(),
            options: WindowOptions::default(),
            last_focus_seq: 0,
        }
    }

    /// Create a terminal window.
    pub fn terminal(id: WindowId, terminal_id: TerminalId) -> Self {
        Self {
            id,
            content: WindowContent::Terminal(terminal_id),
            cursor: CursorPosition::origin(),
            viewport: Viewport::default(),
            options: WindowOptions::default(),
            last_focus_seq: 0,
        }
    }
}
