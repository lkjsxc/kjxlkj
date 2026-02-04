//! Viewport calculation.

/// Viewport dimensions and scroll position.
#[derive(Debug, Clone, Copy, Default)]
pub struct Viewport {
    /// First visible line (0-indexed).
    pub top_line: usize,
    /// Number of visible lines.
    pub height: usize,
    /// Number of visible columns.
    pub width: usize,
}

impl Viewport {
    /// Create a new viewport.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            top_line: 0,
            height,
            width,
        }
    }

    /// Get the range of visible lines.
    pub fn visible_lines(&self) -> std::ops::Range<usize> {
        self.top_line..(self.top_line + self.height)
    }

    /// Check if a line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.top_line && line < self.top_line + self.height
    }

    /// Scroll to ensure a line is visible.
    pub fn scroll_to_line(&mut self, line: usize) {
        if line < self.top_line {
            self.top_line = line;
        } else if line >= self.top_line + self.height {
            self.top_line = line.saturating_sub(self.height - 1);
        }
    }

    /// Scroll down by lines.
    pub fn scroll_down(&mut self, lines: usize, max_lines: usize) {
        let max_top = max_lines.saturating_sub(self.height);
        self.top_line = (self.top_line + lines).min(max_top);
    }

    /// Scroll up by lines.
    pub fn scroll_up(&mut self, lines: usize) {
        self.top_line = self.top_line.saturating_sub(lines);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_scroll() {
        let mut vp = Viewport::new(80, 24);
        assert_eq!(vp.top_line, 0);
        assert!(vp.is_line_visible(0));
        assert!(vp.is_line_visible(23));
        assert!(!vp.is_line_visible(24));

        vp.scroll_to_line(30);
        assert_eq!(vp.top_line, 7);
    }

    #[test]
    fn test_scroll_up_down() {
        let mut vp = Viewport::new(80, 10);
        vp.scroll_down(5, 100);
        assert_eq!(vp.top_line, 5);
        vp.scroll_up(3);
        assert_eq!(vp.top_line, 2);
    }
}
