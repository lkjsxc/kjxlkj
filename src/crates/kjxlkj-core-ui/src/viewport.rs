//! Viewport tracking.

/// Viewport state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    /// Width in columns.
    pub width: u16,
    /// Height in rows (excluding status line).
    pub height: u16,
    /// First visible line.
    pub first_line: usize,
    /// First visible column.
    pub first_col: usize,
}

impl Viewport {
    /// Create a new viewport.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            first_line: 0,
            first_col: 0,
        }
    }

    /// Get the visible line count.
    pub fn visible_lines(&self) -> usize {
        self.height as usize
    }

    /// Get the last visible line index.
    pub fn last_line(&self) -> usize {
        self.first_line + self.visible_lines().saturating_sub(1)
    }

    /// Check if a line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.first_line && line <= self.last_line()
    }

    /// Scroll to ensure a line is visible.
    pub fn ensure_visible(&mut self, line: usize, margin: usize) {
        let margin = margin.min(self.visible_lines() / 4);
        if line < self.first_line + margin {
            self.first_line = line.saturating_sub(margin);
        } else if line > self.last_line().saturating_sub(margin) {
            let needed = line + margin + 1;
            if needed > self.visible_lines() {
                self.first_line = needed - self.visible_lines();
            }
        }
    }

    /// Resize the viewport.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Center on a line.
    pub fn center_on_line(&mut self, line: usize, total_lines: usize) {
        let half = self.visible_lines() / 2;
        if line >= half {
            self.first_line = line - half;
        } else {
            self.first_line = 0;
        }
        // Don't scroll past end
        let max_first = total_lines.saturating_sub(self.visible_lines());
        self.first_line = self.first_line.min(max_first);
    }

    /// Scroll to top.
    pub fn scroll_to_top(&mut self, line: usize) {
        self.first_line = line;
    }

    /// Scroll to bottom.
    pub fn scroll_to_bottom(&mut self, line: usize, total_lines: usize) {
        let visible = self.visible_lines();
        if line + 1 >= visible {
            self.first_line = line + 1 - visible;
        } else {
            self.first_line = 0;
        }
        let max_first = total_lines.saturating_sub(visible);
        self.first_line = self.first_line.min(max_first);
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
    fn viewport_new_dimensions() {
        let v = Viewport::new(100, 50);
        assert_eq!(v.width, 100);
        assert_eq!(v.height, 50);
    }

    #[test]
    fn viewport_first_line_zero() {
        let v = Viewport::new(80, 24);
        assert_eq!(v.first_line, 0);
    }

    #[test]
    fn viewport_is_line_visible() {
        let v = Viewport::new(80, 10);
        assert!(v.is_line_visible(5));
    }

    #[test]
    fn viewport_line_not_visible() {
        let v = Viewport::new(80, 10);
        assert!(!v.is_line_visible(15));
    }

    #[test]
    fn viewport_ensure_visible() {
        let mut v = Viewport::new(80, 10);
        v.ensure_visible(15, 20);
        assert!(v.is_line_visible(15));
    }

    #[test]
    fn viewport_default_values() {
        let v = Viewport::default();
        assert_eq!(v.width, 80);
        assert_eq!(v.height, 24);
    }

    #[test]
    fn viewport_scroll_top() {
        let mut v = Viewport::new(80, 10);
        v.scroll_to_top(5);
        assert_eq!(v.first_line, 5);
    }

    #[test]
    fn viewport_visible_lines() {
        let v = Viewport::new(80, 20);
        assert_eq!(v.visible_lines(), 20);
    }
}
