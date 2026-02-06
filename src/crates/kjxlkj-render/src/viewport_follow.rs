/// Viewport follow rules — scrolloff, viewport clamping, cursor-following behavior.

/// Viewport follow policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FollowPolicy { Always, OnChange, Manual }

/// Viewport state for a window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportState {
    pub top_line: usize,
    pub left_col: usize,
    pub visible_lines: usize,
    pub visible_cols: usize,
    pub total_lines: usize,
    pub scrolloff: usize,
    pub sidescrolloff: usize,
}

impl ViewportState {
    pub fn new(visible_lines: usize, visible_cols: usize) -> Self {
        Self { top_line: 0, left_col: 0, visible_lines, visible_cols,
            total_lines: 0, scrolloff: 0, sidescrolloff: 0 }
    }

    /// Bottom visible line (inclusive).
    pub fn bottom_line(&self) -> usize {
        self.top_line + self.visible_lines.saturating_sub(1)
    }

    /// Clamp top_line so it doesn't exceed total lines.
    pub fn clamp(&mut self) {
        if self.total_lines == 0 { self.top_line = 0; return; }
        let max_top = self.total_lines.saturating_sub(1);
        self.top_line = self.top_line.min(max_top);
    }

    /// Follow cursor vertically: adjust top_line to keep cursor visible with scrolloff.
    pub fn follow_cursor_v(&mut self, cursor_line: usize) {
        let so = self.scrolloff.min(self.visible_lines / 2);
        if cursor_line < self.top_line + so {
            self.top_line = cursor_line.saturating_sub(so);
        } else if cursor_line + so >= self.top_line + self.visible_lines {
            self.top_line = cursor_line + so + 1 - self.visible_lines;
        }
        self.clamp();
    }

    /// Follow cursor horizontally: adjust left_col to keep cursor visible with sidescrolloff.
    pub fn follow_cursor_h(&mut self, cursor_col: usize) {
        let sso = self.sidescrolloff.min(self.visible_cols / 2);
        if cursor_col < self.left_col + sso {
            self.left_col = cursor_col.saturating_sub(sso);
        } else if cursor_col + sso >= self.left_col + self.visible_cols {
            self.left_col = cursor_col + sso + 1 - self.visible_cols;
        }
    }

    /// Resize the viewport, clamping as needed.
    pub fn resize(&mut self, visible_lines: usize, visible_cols: usize) {
        self.visible_lines = visible_lines;
        self.visible_cols = visible_cols;
        self.clamp();
    }

    /// Scroll by a number of lines (positive = down, negative = up).
    pub fn scroll(&mut self, delta: i64) {
        if delta >= 0 {
            self.top_line = self.top_line.saturating_add(delta as usize);
        } else {
            self.top_line = self.top_line.saturating_sub((-delta) as usize);
        }
        self.clamp();
    }

    /// Check if a line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.top_line && line <= self.bottom_line()
    }

    /// Percentage through the file.
    pub fn scroll_percent(&self) -> f64 {
        if self.total_lines == 0 { return 0.0; }
        (self.top_line as f64 / self.total_lines.saturating_sub(1).max(1) as f64) * 100.0
    }
}

/// Center viewport on a cursor line.
pub fn center_on_line(vp: &mut ViewportState, line: usize) {
    vp.top_line = line.saturating_sub(vp.visible_lines / 2);
    vp.clamp();
}

/// Move viewport so cursor line is at the top.
pub fn cursor_to_top(vp: &mut ViewportState, line: usize) {
    vp.top_line = line.saturating_sub(vp.scrolloff);
    vp.clamp();
}

/// Move viewport so cursor line is at the bottom.
pub fn cursor_to_bottom(vp: &mut ViewportState, line: usize) {
    let offset = vp.visible_lines.saturating_sub(1).saturating_sub(vp.scrolloff);
    vp.top_line = line.saturating_sub(offset);
    vp.clamp();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vp(lines: usize, total: usize, so: usize) -> ViewportState {
        let mut v = ViewportState::new(lines, 80);
        v.total_lines = total; v.scrolloff = so;
        v
    }

    #[test]
    fn follow_cursor_down() {
        let mut v = vp(20, 100, 3);
        v.follow_cursor_v(25);
        assert!(v.is_line_visible(25));
    }

    #[test]
    fn follow_cursor_up() {
        let mut v = vp(20, 100, 3);
        v.top_line = 50;
        v.follow_cursor_v(48);
        assert!(v.is_line_visible(48));
    }

    #[test]
    fn clamp_past_end() {
        let mut v = vp(20, 30, 0);
        v.top_line = 50;
        v.clamp();
        assert_eq!(v.top_line, 29);
    }

    #[test]
    fn scroll_down_up() {
        let mut v = vp(20, 100, 0);
        v.scroll(10); assert_eq!(v.top_line, 10);
        v.scroll(-5); assert_eq!(v.top_line, 5);
        v.scroll(-100); assert_eq!(v.top_line, 0);
    }

    #[test]
    fn center_on() {
        let mut v = vp(20, 100, 0);
        center_on_line(&mut v, 50);
        assert_eq!(v.top_line, 40);
        assert!(v.is_line_visible(50));
    }

    #[test]
    fn cursor_top_bottom() {
        let mut v = vp(20, 100, 3);
        cursor_to_top(&mut v, 50);
        assert_eq!(v.top_line, 47);
        cursor_to_bottom(&mut v, 50);
        assert!(v.is_line_visible(50));
    }

    #[test]
    fn horizontal_follow() {
        let mut v = ViewportState::new(20, 80);
        v.sidescrolloff = 5; v.total_lines = 100;
        v.follow_cursor_h(100);
        assert!(v.left_col > 0);
    }

    #[test]
    fn resize_clamps() {
        let mut v = vp(20, 50, 0);
        v.top_line = 45;
        v.resize(10, 80);
        assert!(v.top_line <= 49);
    }

    #[test]
    fn scroll_percent_top() {
        let v = vp(20, 100, 0);
        assert!((v.scroll_percent() - 0.0).abs() < 0.01);
    }
}
