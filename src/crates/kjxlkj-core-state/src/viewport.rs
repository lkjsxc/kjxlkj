//! Viewport state and scrolling logic.

/// Viewport state for a window.
#[derive(Debug, Clone)]
pub struct ViewportState {
    /// First visible line (zero-based).
    pub top_line: usize,
    /// Leftmost visible column (for horizontal scroll).
    pub left_col: usize,
    /// Text area height in rows (excluding statusline).
    pub height: u16,
    /// Text area width in columns (excluding gutter).
    pub width: u16,
}

impl ViewportState {
    pub fn new() -> Self {
        Self {
            top_line: 0,
            left_col: 0,
            height: 24,
            width: 80,
        }
    }

    /// Ensure cursor line is visible, adjusting top_line.
    pub fn follow_cursor(
        &mut self,
        cursor_line: usize,
        scroll_off: u16,
        line_count: usize,
    ) {
        let off = scroll_off as usize;
        let h = self.height as usize;

        // Cursor above visible area.
        if cursor_line < self.top_line + off {
            self.top_line = cursor_line.saturating_sub(off);
        }

        // Cursor below visible area.
        if h > 0 && cursor_line >= self.top_line + h - off.min(h - 1) {
            self.top_line = cursor_line
                .saturating_sub(h - 1 - off.min(h - 1));
        }

        // Clamp top_line.
        let max_top = line_count.saturating_sub(1);
        if self.top_line > max_top {
            self.top_line = max_top;
        }
    }

    /// Ensure cursor column is visible (nowrap mode).
    pub fn follow_cursor_col(
        &mut self,
        cursor_col: usize,
        side_scroll_off: u16,
    ) {
        let off = side_scroll_off as usize;
        let w = self.width as usize;

        if w == 0 {
            return;
        }

        if cursor_col < self.left_col + off {
            self.left_col = cursor_col.saturating_sub(off);
        }

        if cursor_col >= self.left_col + w - off.min(w - 1) {
            self.left_col =
                cursor_col.saturating_sub(w - 1 - off.min(w - 1));
        }
    }

    /// Set dimensions.
    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Bottom visible line (exclusive).
    pub fn bottom_line(&self) -> usize {
        self.top_line + self.height as usize
    }

    /// Whether a line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.top_line && line < self.bottom_line()
    }

    /// Scroll up by `n` lines.
    pub fn scroll_up(&mut self, n: usize) {
        self.top_line = self.top_line.saturating_sub(n);
    }

    /// Scroll down by `n` lines, clamped by line_count.
    pub fn scroll_down(&mut self, n: usize, line_count: usize) {
        let max_top = line_count.saturating_sub(1);
        self.top_line = (self.top_line + n).min(max_top);
    }

    /// Center the view on a specific line.
    pub fn center_on(&mut self, line: usize) {
        let half = (self.height as usize) / 2;
        self.top_line = line.saturating_sub(half);
    }
}

impl Default for ViewportState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follow_cursor_down() {
        let mut vp = ViewportState::new();
        vp.height = 10;
        vp.follow_cursor(15, 2, 100);
        assert!(vp.top_line > 0);
        assert!(vp.is_line_visible(15));
    }

    #[test]
    fn follow_cursor_up() {
        let mut vp = ViewportState::new();
        vp.height = 10;
        vp.top_line = 20;
        vp.follow_cursor(5, 2, 100);
        assert!(vp.is_line_visible(5));
    }

    #[test]
    fn scroll_up_clamp() {
        let mut vp = ViewportState::new();
        vp.top_line = 3;
        vp.scroll_up(10);
        assert_eq!(vp.top_line, 0);
    }

    #[test]
    fn center_on_line() {
        let mut vp = ViewportState::new();
        vp.height = 20;
        vp.center_on(50);
        assert_eq!(vp.top_line, 40);
    }
}
