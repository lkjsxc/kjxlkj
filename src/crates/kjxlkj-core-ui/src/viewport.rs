//! Viewport for visible region.

/// The visible viewport region.
#[derive(Debug, Clone, Copy, Default)]
pub struct Viewport {
    /// First visible line (0-based).
    pub top_line: usize,
    /// Number of visible lines.
    pub height: usize,
    /// Number of visible columns.
    pub width: usize,
}

impl Viewport {
    /// Create a new viewport.
    pub fn new(top_line: usize, height: usize, width: usize) -> Self {
        Self {
            top_line,
            height,
            width,
        }
    }

    /// Get the last visible line (inclusive).
    pub fn bottom_line(&self) -> usize {
        self.top_line + self.height.saturating_sub(1)
    }

    /// Check if a line is visible.
    pub fn is_visible(&self, line: usize) -> bool {
        line >= self.top_line && line <= self.bottom_line()
    }

    /// Scroll to ensure a line is visible.
    pub fn scroll_to_line(&mut self, line: usize) {
        if line < self.top_line {
            self.top_line = line;
        } else if line > self.bottom_line() {
            self.top_line = line.saturating_sub(self.height.saturating_sub(1));
        }
    }

    /// Scroll down by n lines.
    pub fn scroll_down(&mut self, n: usize, max_lines: usize) {
        let max_top = max_lines.saturating_sub(self.height);
        self.top_line = (self.top_line + n).min(max_top);
    }

    /// Scroll up by n lines.
    pub fn scroll_up(&mut self, n: usize) {
        self.top_line = self.top_line.saturating_sub(n);
    }

    /// Center on a line.
    pub fn center_on(&mut self, line: usize, max_lines: usize) {
        let half = self.height / 2;
        if line >= half {
            self.top_line = line - half;
        } else {
            self.top_line = 0;
        }
        let max_top = max_lines.saturating_sub(self.height);
        self.top_line = self.top_line.min(max_top);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_visible() {
        let vp = Viewport::new(10, 20, 80);
        assert!(vp.is_visible(10));
        assert!(vp.is_visible(29));
        assert!(!vp.is_visible(9));
        assert!(!vp.is_visible(30));
    }

    #[test]
    fn test_scroll_to_line() {
        let mut vp = Viewport::new(10, 20, 80);
        vp.scroll_to_line(5);
        assert_eq!(vp.top_line, 5);
        vp.scroll_to_line(50);
        assert_eq!(vp.top_line, 31);
    }
}
