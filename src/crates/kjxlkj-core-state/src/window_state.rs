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
    pub wrap: bool,
    pub scrolloff: usize,
    pub sidescrolloff: usize,
    pub visual_anchor: Option<Position>,
}

impl WindowState {
    pub fn new(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id, buffer_id, cursor_line: 0, cursor_col: 0, top_line: 0, left_col: 0,
            height: 24, width: 80, wrap: true, scrolloff: 3, sidescrolloff: 0, visual_anchor: None,
        }
    }

    pub fn cursor(&self) -> Position { Position::new(self.cursor_line, self.cursor_col) }

    pub fn set_cursor(&mut self, pos: Position) { self.cursor_line = pos.line; self.cursor_col = pos.col; }

    /// Ensure the cursor is visible within the viewport, scrolling if needed.
    pub fn ensure_cursor_visible(&mut self) {
        self.ensure_cursor_visible_vertical();
        self.ensure_cursor_visible_horizontal();
    }

    /// Vertical cursor-follow per viewport spec.
    pub fn ensure_cursor_visible_vertical(&mut self) {
        if self.height == 0 { return; }
        let v_margin = self.scrolloff.min((self.height.saturating_sub(1)) / 2);
        let c_row = self.cursor_line as isize - self.top_line as isize;
        if c_row < v_margin as isize {
            self.top_line = self.cursor_line.saturating_sub(v_margin);
        } else if c_row > (self.height.saturating_sub(1 + v_margin)) as isize {
            self.top_line = self.cursor_line.saturating_sub(self.height.saturating_sub(1) - v_margin);
        }
    }

    /// Horizontal cursor-follow (no-wrap only) per viewport spec.
    pub fn ensure_cursor_visible_horizontal(&mut self) {
        if self.wrap { self.left_col = 0; return; }
        if self.width == 0 { return; }
        let h_margin = self.sidescrolloff.min((self.width.saturating_sub(1)) / 2);
        let c_x = self.cursor_col as isize - self.left_col as isize;
        if c_x < h_margin as isize {
            self.left_col = self.cursor_col.saturating_sub(h_margin);
        } else if c_x > (self.width.saturating_sub(1 + h_margin)) as isize {
            self.left_col = self.cursor_col.saturating_sub(self.width.saturating_sub(1) - h_margin);
        }
    }

    /// Legacy API: ensure cursor visible with explicit scrolloff.
    pub fn ensure_cursor_visible_with_scrolloff(&mut self, scroll_off: usize) {
        let saved = self.scrolloff;
        self.scrolloff = scroll_off;
        self.ensure_cursor_visible();
        self.scrolloff = saved;
    }

    /// Center cursor in viewport (zz).
    pub fn center_cursor(&mut self) { self.top_line = self.cursor_line.saturating_sub(self.height / 2); }

    /// Cursor to top of viewport (zt).
    pub fn cursor_to_top(&mut self) { self.top_line = self.cursor_line; }

    /// Cursor to bottom of viewport (zb).
    pub fn cursor_to_bottom(&mut self) { self.top_line = self.cursor_line.saturating_sub(self.height.saturating_sub(1)); }
}
