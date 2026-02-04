//! Viewport calculations.

/// A viewport into a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Viewport {
    /// First visible line (0-indexed).
    pub top_line: usize,
    /// Number of visible lines.
    pub height: usize,
    /// First visible column.
    pub left_col: usize,
    /// Number of visible columns.
    pub width: usize,
}

impl Viewport {
    /// Create a new viewport.
    pub fn new(top_line: usize, height: usize, left_col: usize, width: usize) -> Self {
        Self {
            top_line,
            height,
            left_col,
            width,
        }
    }

    /// Get the last visible line (exclusive).
    pub fn bottom_line(&self) -> usize {
        self.top_line + self.height
    }

    /// Check if a line is visible in the viewport.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.top_line && line < self.bottom_line()
    }

    /// Scroll to ensure a line is visible.
    pub fn scroll_to_line(&mut self, line: usize, buffer_line_count: usize) {
        if line < self.top_line {
            self.top_line = line;
        } else if line >= self.bottom_line() {
            self.top_line = line.saturating_sub(self.height - 1);
        }

        // Clamp to buffer bounds
        let max_top = buffer_line_count.saturating_sub(self.height);
        self.top_line = self.top_line.min(max_top);
    }

    /// Scroll down by a number of lines.
    pub fn scroll_down(&mut self, lines: usize, buffer_line_count: usize) {
        let max_top = buffer_line_count.saturating_sub(self.height);
        self.top_line = (self.top_line + lines).min(max_top);
    }

    /// Scroll up by a number of lines.
    pub fn scroll_up(&mut self, lines: usize) {
        self.top_line = self.top_line.saturating_sub(lines);
    }

    /// Center the viewport on a line.
    pub fn center_on_line(&mut self, line: usize, buffer_line_count: usize) {
        let half = self.height / 2;
        self.top_line = line.saturating_sub(half);
        let max_top = buffer_line_count.saturating_sub(self.height);
        self.top_line = self.top_line.min(max_top);
    }

    /// Scroll to put cursor at top.
    pub fn cursor_to_top(&mut self, cursor_line: usize) {
        self.top_line = cursor_line;
    }

    /// Scroll to put cursor at bottom.
    pub fn cursor_to_bottom(&mut self, cursor_line: usize) {
        self.top_line = cursor_line.saturating_sub(self.height - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_visible() {
        let vp = Viewport::new(10, 20, 0, 80);
        assert!(vp.is_line_visible(10));
        assert!(vp.is_line_visible(29));
        assert!(!vp.is_line_visible(9));
        assert!(!vp.is_line_visible(30));
    }

    #[test]
    fn test_viewport_scroll() {
        let mut vp = Viewport::new(0, 20, 0, 80);
        vp.scroll_to_line(25, 100);
        assert!(vp.is_line_visible(25));
    }

    #[test]
    fn test_viewport_center() {
        let mut vp = Viewport::new(0, 20, 0, 80);
        vp.center_on_line(50, 100);
        assert!(vp.is_line_visible(50));
    }
}
