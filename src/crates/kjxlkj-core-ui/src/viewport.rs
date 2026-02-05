//! Viewport handling.

use kjxlkj_core_types::Cursor;
use serde::{Deserialize, Serialize};

/// Viewport configuration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Viewport {
    /// First visible line (0-indexed).
    pub top_line: usize,
    /// First visible column (for horizontal scroll).
    pub left_col: usize,
    /// Number of visible lines.
    pub height: usize,
    /// Number of visible columns.
    pub width: usize,
    /// Scroll offset from top of viewport.
    pub scrolloff: usize,
}

impl Viewport {
    /// Create a new viewport.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            top_line: 0,
            left_col: 0,
            height,
            width,
            scrolloff: 0,
        }
    }

    /// Set scrolloff (lines to keep above/below cursor).
    pub fn with_scrolloff(mut self, scrolloff: usize) -> Self {
        self.scrolloff = scrolloff;
        self
    }

    /// Get the last visible line.
    pub fn bottom_line(&self) -> usize {
        self.top_line + self.height.saturating_sub(1)
    }

    /// Check if a line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.top_line && line <= self.bottom_line()
    }

    /// Adjust viewport to keep cursor visible.
    pub fn follow_cursor(&mut self, cursor: &Cursor, total_lines: usize) {
        let min_top = cursor.line.saturating_sub(self.height.saturating_sub(1 + self.scrolloff));
        let max_top = cursor.line.saturating_sub(self.scrolloff);

        if self.top_line < min_top {
            self.top_line = min_top;
        }
        if self.top_line > max_top {
            self.top_line = max_top;
        }

        // Clamp to valid range
        let max_valid_top = total_lines.saturating_sub(self.height);
        self.top_line = self.top_line.min(max_valid_top);
    }

    /// Scroll by delta lines.
    pub fn scroll(&mut self, delta: isize, total_lines: usize) {
        if delta > 0 {
            self.top_line = self.top_line.saturating_add(delta as usize);
        } else {
            self.top_line = self.top_line.saturating_sub((-delta) as usize);
        }

        let max_top = total_lines.saturating_sub(self.height);
        self.top_line = self.top_line.min(max_top);
    }

    /// Center the viewport on a line.
    pub fn center_on(&mut self, line: usize, total_lines: usize) {
        let half = self.height / 2;
        self.top_line = line.saturating_sub(half);

        let max_top = total_lines.saturating_sub(self.height);
        self.top_line = self.top_line.min(max_top);
    }

    /// Resize the viewport.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_visible() {
        let vp = Viewport::new(80, 10);
        assert!(vp.is_line_visible(0));
        assert!(vp.is_line_visible(9));
        assert!(!vp.is_line_visible(10));
    }

    #[test]
    fn test_viewport_follow_cursor() {
        let mut vp = Viewport::new(80, 10);
        let cursor = Cursor::new(15, 0);
        vp.follow_cursor(&cursor, 100);
        assert!(vp.is_line_visible(15));
    }

    #[test]
    fn test_viewport_scroll() {
        let mut vp = Viewport::new(80, 10);
        vp.scroll(5, 100);
        assert_eq!(vp.top_line, 5);
    }

    #[test]
    fn test_viewport_center() {
        let mut vp = Viewport::new(80, 10);
        vp.center_on(50, 100);
        assert_eq!(vp.top_line, 45);
    }
}
