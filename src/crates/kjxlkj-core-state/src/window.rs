//! Window state.

use kjxlkj_core_types::{BufferId, Cursor, WindowId};
use kjxlkj_core_ui::Viewport;

/// State of a single window.
pub struct WindowState {
    pub id: WindowId,
    pub buffer_id: BufferId,
    pub cursor: Cursor,
    pub viewport: Viewport,
}

impl WindowState {
    /// Create a new window.
    pub fn new(id: WindowId, buffer_id: BufferId, width: usize, height: usize) -> Self {
        Self {
            id,
            buffer_id,
            cursor: Cursor::origin(),
            viewport: Viewport::new(width, height),
        }
    }

    /// Resize the window.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.viewport.resize(width, height);
    }

    /// Set buffer for this window.
    pub fn set_buffer(&mut self, buffer_id: BufferId) {
        self.buffer_id = buffer_id;
        self.cursor = Cursor::origin();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_state() {
        let win = WindowState::new(WindowId::new(1), BufferId::new(1), 80, 24);
        assert_eq!(win.cursor.line, 0);
        assert_eq!(win.viewport.width, 80);
        assert_eq!(win.viewport.height, 24);
    }
}
