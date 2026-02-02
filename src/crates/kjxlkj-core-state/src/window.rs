//! Window state.

use kjxlkj_core_types::{BufferId, Cursor, WindowId};
use kjxlkj_core_ui::Viewport;
use serde::{Deserialize, Serialize};

/// Window state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    /// Window ID.
    pub id: WindowId,
    /// Buffer being displayed.
    pub buffer_id: BufferId,
    /// Cursor state.
    pub cursor: Cursor,
    /// Viewport.
    pub viewport: Viewport,
    /// Line numbers enabled.
    pub line_numbers: bool,
    /// Wrap enabled.
    pub wrap: bool,
}

impl WindowState {
    /// Creates a new window.
    pub fn new(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            buffer_id,
            cursor: Cursor::origin(),
            viewport: Viewport::default(),
            line_numbers: true,
            wrap: false,
        }
    }

    /// Ensures cursor is visible in viewport.
    pub fn ensure_cursor_visible(&mut self) {
        self.viewport.scroll_to_line(self.cursor.line());
    }
}
