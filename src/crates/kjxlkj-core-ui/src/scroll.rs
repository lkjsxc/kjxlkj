//! Window scrolling utilities.
//!
//! Functions for viewport scrolling (Ctrl-D, Ctrl-U, Ctrl-F, Ctrl-B, zz, zt, zb).

/// Scroll direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Scroll up (content moves down).
    Up,
    /// Scroll down (content moves up).
    Down,
}

/// Scroll amount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAmount {
    /// Half page (Ctrl-D, Ctrl-U).
    HalfPage,
    /// Full page (Ctrl-F, Ctrl-B).
    FullPage,
    /// Single line (Ctrl-E, Ctrl-Y).
    Line,
    /// Multiple lines.
    Lines(usize),
}

/// Cursor position relative to viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorPosition {
    /// Center cursor in viewport (zz).
    Center,
    /// Move cursor to top of viewport (zt).
    Top,
    /// Move cursor to bottom of viewport (zb).
    Bottom,
}

/// Scroll state for a window.
#[derive(Debug, Clone)]
pub struct ScrollState {
    /// First visible line.
    pub top_line: usize,
    /// Number of visible lines.
    pub height: usize,
    /// Horizontal offset.
    pub left_col: usize,
    /// Width of visible area.
    pub width: usize,
}

impl ScrollState {
    /// Creates a new scroll state.
    pub fn new(top_line: usize, height: usize) -> Self {
        Self {
            top_line,
            height,
            left_col: 0,
            width: 80,
        }
    }

    /// Returns the last visible line.
    pub fn bottom_line(&self) -> usize {
        self.top_line + self.height.saturating_sub(1)
    }

    /// Returns whether a line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.top_line && line <= self.bottom_line()
    }

    /// Scrolls by the given amount.
    pub fn scroll(&mut self, direction: ScrollDirection, amount: ScrollAmount, total_lines: usize) {
        let delta = match amount {
            ScrollAmount::HalfPage => self.height / 2,
            ScrollAmount::FullPage => self.height.saturating_sub(2),
            ScrollAmount::Line => 1,
            ScrollAmount::Lines(n) => n,
        };

        match direction {
            ScrollDirection::Up => {
                self.top_line = self.top_line.saturating_sub(delta);
            }
            ScrollDirection::Down => {
                let max_top = total_lines.saturating_sub(1);
                self.top_line = (self.top_line + delta).min(max_top);
            }
        }
    }

    /// Ensures a line is visible by scrolling if needed.
    pub fn ensure_visible(&mut self, line: usize, total_lines: usize) {
        if line < self.top_line {
            self.top_line = line;
        } else if line > self.bottom_line() {
            self.top_line = line.saturating_sub(self.height.saturating_sub(1));
        }
        // Ensure we don't scroll past end.
        let max_top = total_lines.saturating_sub(1);
        self.top_line = self.top_line.min(max_top);
    }

    /// Positions cursor in viewport.
    pub fn position_cursor(&mut self, cursor_line: usize, position: CursorPosition, total_lines: usize) {
        match position {
            CursorPosition::Center => {
                let half = self.height / 2;
                self.top_line = cursor_line.saturating_sub(half);
            }
            CursorPosition::Top => {
                self.top_line = cursor_line;
            }
            CursorPosition::Bottom => {
                self.top_line = cursor_line.saturating_sub(self.height.saturating_sub(1));
            }
        }
        // Clamp to valid range.
        let max_top = total_lines.saturating_sub(1);
        self.top_line = self.top_line.min(max_top);
    }

    /// Scrolls horizontally.
    pub fn scroll_horizontal(&mut self, cols: isize) {
        if cols < 0 {
            self.left_col = self.left_col.saturating_sub((-cols) as usize);
        } else {
            self.left_col += cols as usize;
        }
    }

    /// Ensures a column is visible.
    pub fn ensure_col_visible(&mut self, col: usize) {
        if col < self.left_col {
            self.left_col = col;
        } else if col >= self.left_col + self.width {
            self.left_col = col.saturating_sub(self.width.saturating_sub(1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_state_new() {
        let state = ScrollState::new(0, 24);
        assert_eq!(state.top_line, 0);
        assert_eq!(state.height, 24);
        assert_eq!(state.bottom_line(), 23);
    }

    #[test]
    fn test_is_line_visible() {
        let state = ScrollState::new(10, 20);
        assert!(state.is_line_visible(10));
        assert!(state.is_line_visible(25));
        assert!(!state.is_line_visible(9));
        assert!(!state.is_line_visible(30));
    }

    #[test]
    fn test_scroll_down() {
        let mut state = ScrollState::new(0, 24);
        state.scroll(ScrollDirection::Down, ScrollAmount::HalfPage, 100);
        assert_eq!(state.top_line, 12);
    }

    #[test]
    fn test_scroll_up() {
        let mut state = ScrollState::new(20, 24);
        state.scroll(ScrollDirection::Up, ScrollAmount::HalfPage, 100);
        assert_eq!(state.top_line, 8);
    }

    #[test]
    fn test_ensure_visible_below() {
        let mut state = ScrollState::new(0, 24);
        state.ensure_visible(30, 100);
        assert!(state.is_line_visible(30));
    }

    #[test]
    fn test_ensure_visible_above() {
        let mut state = ScrollState::new(20, 24);
        state.ensure_visible(10, 100);
        assert!(state.is_line_visible(10));
    }

    #[test]
    fn test_position_cursor_center() {
        let mut state = ScrollState::new(0, 24);
        state.position_cursor(50, CursorPosition::Center, 100);
        assert!(state.is_line_visible(50));
    }

    #[test]
    fn test_position_cursor_top() {
        let mut state = ScrollState::new(0, 24);
        state.position_cursor(50, CursorPosition::Top, 100);
        assert_eq!(state.top_line, 50);
    }
}
