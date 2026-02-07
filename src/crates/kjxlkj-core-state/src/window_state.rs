//! Window state: tracks cursor, viewport, and buffer association.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Position, WindowId};

/// State for a single editor window/split.
#[derive(Debug, Clone)]
pub struct WindowState {
    pub id: WindowId,
    pub buffer_id: BufferId,
    pub cursor: Position,
    pub viewport_top: usize,
    pub viewport_left: usize,
    pub width: u16,
    pub height: u16,
}

impl WindowState {
    pub fn new(id: WindowId, buffer_id: BufferId, width: u16, height: u16) -> Self {
        Self {
            id,
            buffer_id,
            cursor: Position::ZERO,
            viewport_top: 0,
            viewport_left: 0,
            width,
            height,
        }
    }

    /// Get the content of the line the cursor is on.
    pub fn cursor_line_content(&self, buffer: &TextBuffer) -> Option<String> {
        buffer.line(self.cursor.line)
    }

    /// Clamp cursor to valid buffer bounds.
    pub fn clamp_cursor(&mut self, buffer: &TextBuffer) {
        let max_line = buffer.line_count().saturating_sub(1);
        self.cursor.line = self.cursor.line.min(max_line);
        let line_len = buffer.line_len(self.cursor.line);
        let max_col = if line_len > 0 { line_len.saturating_sub(1) } else { 0 };
        self.cursor.col = self.cursor.col.min(max_col);
    }

    /// Scroll viewport down.
    pub fn scroll_down(&mut self, lines: usize, max_line: usize) {
        self.viewport_top = (self.viewport_top + lines).min(max_line);
    }

    /// Scroll viewport up.
    pub fn scroll_up(&mut self, lines: usize) {
        self.viewport_top = self.viewport_top.saturating_sub(lines);
    }

    /// Ensure cursor is within visible viewport.
    pub fn ensure_cursor_visible(&mut self) {
        if self.cursor.line < self.viewport_top {
            self.viewport_top = self.cursor.line;
        }
        let bottom = self.viewport_top + (self.height as usize).saturating_sub(1);
        if self.cursor.line > bottom {
            self.viewport_top =
                self.cursor.line.saturating_sub((self.height as usize).saturating_sub(1));
        }
    }

    /// Get visible line range (start, end exclusive).
    pub fn visible_range(&self) -> (usize, usize) {
        let start = self.viewport_top;
        let end = self.viewport_top + self.height as usize;
        (start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_window() {
        let w = WindowState::new(WindowId(1), BufferId(1), 80, 24);
        assert_eq!(w.cursor, Position::ZERO);
        assert_eq!(w.width, 80);
        assert_eq!(w.height, 24);
    }

    #[test]
    fn scroll_operations() {
        let mut w = WindowState::new(WindowId(1), BufferId(1), 80, 24);
        w.scroll_down(5, 100);
        assert_eq!(w.viewport_top, 5);
        w.scroll_up(3);
        assert_eq!(w.viewport_top, 2);
        w.scroll_up(10);
        assert_eq!(w.viewport_top, 0);
    }

    #[test]
    fn visible_range() {
        let w = WindowState::new(WindowId(1), BufferId(1), 80, 10);
        assert_eq!(w.visible_range(), (0, 10));
    }

    #[test]
    fn ensure_cursor_visible_scrolls() {
        let mut w = WindowState::new(WindowId(1), BufferId(1), 80, 10);
        w.cursor.line = 15;
        w.ensure_cursor_visible();
        assert!(w.viewport_top > 0);
    }
}
