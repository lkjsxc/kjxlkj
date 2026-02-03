//! Viewport state.

/// A viewport defines visible area of a buffer.
#[derive(Debug, Clone, Copy, Default)]
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
    /// Creates a new viewport.
    pub fn new(top_line: usize, height: usize, left_col: usize, width: usize) -> Self {
        Self {
            top_line,
            height,
            left_col,
            width,
        }
    }

    /// Returns the last visible line (exclusive).
    pub fn bottom_line(&self) -> usize {
        self.top_line + self.height
    }

    /// Returns true if the given line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.top_line && line < self.bottom_line()
    }

    /// Adjusts the viewport to ensure the cursor line is visible.
    pub fn follow_cursor(&mut self, cursor_line: usize, scroll_off: usize) {
        let effective_top = self.top_line + scroll_off;
        let effective_bottom = self.bottom_line().saturating_sub(scroll_off);
        if cursor_line < effective_top {
            self.top_line = cursor_line.saturating_sub(scroll_off);
        } else if cursor_line >= effective_bottom && self.height > 0 {
            self.top_line = cursor_line
                .saturating_sub(self.height.saturating_sub(1))
                .saturating_add(scroll_off);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follow_cursor_down() {
        let mut vp = Viewport::new(0, 10, 0, 80);
        vp.follow_cursor(15, 2);
        assert!(vp.is_line_visible(15));
    }

    #[test]
    fn follow_cursor_up() {
        let mut vp = Viewport::new(10, 10, 0, 80);
        vp.follow_cursor(5, 2);
        assert!(vp.is_line_visible(5));
    }
}
