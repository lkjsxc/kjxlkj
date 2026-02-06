//! Per-window state: cursor, viewport, buffer association.

use kjxlkj_core_types::{BufferId, Position, WindowId};

/// Per-window state tracking cursor, viewport, and buffer.
pub struct WindowState {
    pub id: WindowId,
    pub buffer_id: BufferId,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub top_line: usize,
    pub height: usize,
    /// Visual mode anchor (start of selection).
    pub visual_anchor: Option<Position>,
}

impl WindowState {
    pub fn new(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            buffer_id,
            cursor_line: 0,
            cursor_col: 0,
            top_line: 0,
            height: 24,
            visual_anchor: None,
        }
    }

    /// Get cursor as a Position.
    pub fn cursor(&self) -> Position {
        Position::new(self.cursor_line, self.cursor_col)
    }

    /// Set cursor from a Position.
    pub fn set_cursor(&mut self, pos: Position) {
        self.cursor_line = pos.line;
        self.cursor_col = pos.col;
    }

    /// Ensure the cursor is visible within the viewport, scrolling if needed.
    pub fn ensure_cursor_visible(&mut self) {
        self.ensure_cursor_visible_with_scrolloff(3);
    }

    /// Ensure cursor visible with a specific scrolloff value.
    pub fn ensure_cursor_visible_with_scrolloff(
        &mut self,
        scroll_off: usize,
    ) {
        if self.height == 0 {
            return;
        }
        // Scroll up if cursor above viewport + scrolloff
        if self.cursor_line
            < self.top_line.saturating_add(scroll_off)
        {
            self.top_line =
                self.cursor_line.saturating_sub(scroll_off);
        }
        // Scroll down if cursor below viewport - scrolloff
        let bottom = self.top_line + self.height;
        if self.cursor_line + scroll_off >= bottom {
            self.top_line = (self.cursor_line + scroll_off + 1)
                .saturating_sub(self.height);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_window_state() {
        let ws = WindowState::new(WindowId(1), BufferId(1));
        assert_eq!(ws.cursor(), Position::new(0, 0));
        assert_eq!(ws.top_line, 0);
    }

    #[test]
    fn set_cursor() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.set_cursor(Position::new(5, 10));
        assert_eq!(ws.cursor_line, 5);
        assert_eq!(ws.cursor_col, 10);
    }

    #[test]
    fn ensure_cursor_visible_scrolls_down() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.height = 10;
        ws.cursor_line = 20;
        ws.ensure_cursor_visible();
        assert!(ws.top_line > 0);
        // Cursor should be within viewport
        assert!(ws.cursor_line >= ws.top_line);
        assert!(ws.cursor_line < ws.top_line + ws.height);
    }

    #[test]
    fn ensure_cursor_visible_scrolls_up() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.height = 10;
        ws.top_line = 20;
        ws.cursor_line = 15;
        ws.ensure_cursor_visible();
        assert!(ws.top_line <= ws.cursor_line);
    }
}
