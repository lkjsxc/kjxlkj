//! Viewport state.

/// Viewport state for a window.
#[derive(Debug, Clone, Default)]
pub struct Viewport {
    /// First visible line.
    pub top_line: usize,
    /// Left scroll offset.
    pub left_col: usize,
    /// Visible height in lines.
    pub height: usize,
    /// Visible width in columns.
    pub width: usize,
}

impl Viewport {
    /// Create a new viewport.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            top_line: 0,
            left_col: 0,
            height,
            width,
        }
    }

    /// Ensure a line is visible, adjusting top_line if needed.
    pub fn ensure_visible(&mut self, line: usize, scroll_off: usize) {
        let effective_scroll = scroll_off.min(self.height / 2);
        if line < self.top_line + effective_scroll {
            self.top_line = line.saturating_sub(effective_scroll);
        } else if line >= self.top_line + self.height - effective_scroll {
            self.top_line = line + 1 + effective_scroll - self.height;
        }
    }

    /// Get the range of visible lines.
    pub fn visible_lines(&self) -> std::ops::Range<usize> {
        self.top_line..self.top_line + self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_new() {
        let vp = Viewport::new(80, 24);
        assert_eq!(vp.width, 80);
        assert_eq!(vp.height, 24);
        assert_eq!(vp.top_line, 0);
    }

    #[test]
    fn test_ensure_visible_scrolls_down() {
        let mut vp = Viewport::new(80, 24);
        vp.ensure_visible(30, 3);
        assert!(vp.top_line > 0);
    }
}
