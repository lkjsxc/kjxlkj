//! Per-window state: cursor, viewport, buffer association.

use kjxlkj_core_types::{BufferId, Position, WindowId};

/// Per-window state tracking cursor, viewport, and buffer.
pub struct WindowState {
    pub id: WindowId,
    pub buffer_id: BufferId,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub top_line: usize,
    pub left_col: usize,
    pub height: usize,
    pub width: usize,
    /// Soft-wrap long lines (default true).
    pub wrap: bool,
    /// Vertical scroll margin (rows around cursor).
    pub scrolloff: usize,
    /// Horizontal scroll margin (cols around cursor).
    pub sidescrolloff: usize,
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
            left_col: 0,
            height: 24,
            width: 80,
            wrap: true,
            scrolloff: 3,
            sidescrolloff: 0,
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
        self.ensure_cursor_visible_vertical();
        self.ensure_cursor_visible_horizontal();
    }

    /// Vertical cursor-follow per viewport spec.
    pub fn ensure_cursor_visible_vertical(&mut self) {
        if self.height == 0 {
            return;
        }
        let v_margin = self.scrolloff
            .min((self.height.saturating_sub(1)) / 2);
        let c_row = self.cursor_line as isize
            - self.top_line as isize;
        let min_row = v_margin as isize;
        let max_row =
            (self.height.saturating_sub(1 + v_margin)) as isize;
        if c_row < min_row {
            self.top_line =
                self.cursor_line.saturating_sub(v_margin);
        } else if c_row > max_row {
            self.top_line = self.cursor_line
                .saturating_sub(self.height.saturating_sub(1) - v_margin);
        }
    }

    /// Horizontal cursor-follow (no-wrap only) per viewport spec.
    pub fn ensure_cursor_visible_horizontal(&mut self) {
        if self.wrap {
            self.left_col = 0;
            return;
        }
        if self.width == 0 {
            return;
        }
        let h_margin = self.sidescrolloff
            .min((self.width.saturating_sub(1)) / 2);
        let c_x = self.cursor_col as isize
            - self.left_col as isize;
        let min_x = h_margin as isize;
        let max_x =
            (self.width.saturating_sub(1 + h_margin)) as isize;
        if c_x < min_x {
            self.left_col =
                self.cursor_col.saturating_sub(h_margin);
        } else if c_x > max_x {
            self.left_col = self.cursor_col
                .saturating_sub(self.width.saturating_sub(1) - h_margin);
        }
    }

    /// Legacy API: ensure cursor visible with explicit scrolloff.
    pub fn ensure_cursor_visible_with_scrolloff(
        &mut self,
        scroll_off: usize,
    ) {
        let saved = self.scrolloff;
        self.scrolloff = scroll_off;
        self.ensure_cursor_visible();
        self.scrolloff = saved;
    }

    /// Center cursor in viewport (zz).
    pub fn center_cursor(&mut self) {
        self.top_line = self.cursor_line
            .saturating_sub(self.height / 2);
    }

    /// Cursor to top of viewport (zt).
    pub fn cursor_to_top(&mut self) {
        self.top_line = self.cursor_line;
    }

    /// Cursor to bottom of viewport (zb).
    pub fn cursor_to_bottom(&mut self) {
        self.top_line = self.cursor_line
            .saturating_sub(self.height.saturating_sub(1));
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
        assert_eq!(ws.left_col, 0);
        assert!(ws.wrap);
    }

    #[test]
    fn set_cursor() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.set_cursor(Position::new(5, 10));
        assert_eq!(ws.cursor_line, 5);
        assert_eq!(ws.cursor_col, 10);
    }

    #[test]
    fn vertical_follow_scrolls_down() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.height = 10;
        ws.scrolloff = 3;
        ws.cursor_line = 20;
        ws.ensure_cursor_visible();
        assert!(ws.cursor_line >= ws.top_line);
        assert!(ws.cursor_line < ws.top_line + ws.height);
    }

    #[test]
    fn vertical_follow_scrolls_up() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.height = 10;
        ws.scrolloff = 3;
        ws.top_line = 20;
        ws.cursor_line = 15;
        ws.ensure_cursor_visible();
        assert!(ws.top_line <= ws.cursor_line);
    }

    #[test]
    fn scrolloff_clamped_to_half() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.height = 5;
        ws.scrolloff = 10; // larger than half
        ws.cursor_line = 0;
        ws.top_line = 5;
        ws.ensure_cursor_visible();
        // Should still work — scrolloff clamped to 2
        assert!(ws.cursor_line >= ws.top_line);
    }

    #[test]
    fn horizontal_follow_no_wrap() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.wrap = false;
        ws.width = 20;
        ws.sidescrolloff = 5;
        ws.cursor_col = 50;
        ws.ensure_cursor_visible();
        assert!(ws.left_col > 0);
        assert!(ws.cursor_col >= ws.left_col);
        assert!(ws.cursor_col < ws.left_col + ws.width);
    }

    #[test]
    fn horizontal_forced_zero_when_wrap() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.wrap = true;
        ws.left_col = 10;
        ws.ensure_cursor_visible();
        assert_eq!(ws.left_col, 0);
    }

    #[test]
    fn center_cursor_zz() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.height = 20;
        ws.cursor_line = 50;
        ws.center_cursor();
        assert_eq!(ws.top_line, 40);
    }

    #[test]
    fn cursor_to_top_zt() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.cursor_line = 30;
        ws.cursor_to_top();
        assert_eq!(ws.top_line, 30);
    }

    #[test]
    fn cursor_to_bottom_zb() {
        let mut ws = WindowState::new(WindowId(1), BufferId(1));
        ws.height = 20;
        ws.cursor_line = 50;
        ws.cursor_to_bottom();
        assert_eq!(ws.top_line, 31);
    }
}
