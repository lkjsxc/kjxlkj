//! Viewport types.

use kjxlkj_core_types::Position;
use serde::{Deserialize, Serialize};

use crate::Dimensions;

/// Viewport into a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Viewport {
    /// Top-left position in the buffer.
    pub top_left: Position,
    /// Dimensions of the viewport.
    pub dimensions: Dimensions,
}

impl Viewport {
    /// Creates a new viewport.
    pub fn new(top_left: Position, dimensions: Dimensions) -> Self {
        Self {
            top_left,
            dimensions,
        }
    }

    /// Returns the first visible line.
    pub fn first_line(&self) -> usize {
        self.top_left.line
    }

    /// Returns the last visible line (exclusive).
    pub fn last_line(&self) -> usize {
        self.top_left.line + self.dimensions.height as usize
    }

    /// Returns true if a line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.first_line() && line < self.last_line()
    }

    /// Returns true if a position is visible.
    pub fn is_position_visible(&self, pos: Position) -> bool {
        self.is_line_visible(pos.line)
            && pos.col >= self.top_left.col
            && pos.col < self.top_left.col + self.dimensions.width as usize
    }

    /// Scrolls to make a line visible.
    pub fn scroll_to_line(&mut self, line: usize) {
        if line < self.first_line() {
            self.top_left.line = line;
        } else if line >= self.last_line() {
            self.top_left.line = line.saturating_sub(self.dimensions.height as usize - 1);
        }
    }

    /// Scrolls to make a position visible with context lines.
    pub fn scroll_to_position(&mut self, pos: Position, context: usize) {
        // Vertical scrolling
        if pos.line < self.first_line() + context {
            self.top_left.line = pos.line.saturating_sub(context);
        } else if pos.line + context >= self.last_line() {
            let new_top = pos.line + context + 1;
            self.top_left.line = new_top.saturating_sub(self.dimensions.height as usize);
        }
        
        // Horizontal scrolling
        let width = self.dimensions.width as usize;
        if pos.col < self.top_left.col {
            self.top_left.col = pos.col.saturating_sub(width / 4);
        } else if pos.col >= self.top_left.col + width {
            self.top_left.col = pos.col.saturating_sub(width * 3 / 4);
        }
    }

    /// Scrolls down by n lines.
    pub fn scroll_down(&mut self, lines: usize, max_line: usize) {
        let new_top = self.top_left.line + lines;
        self.top_left.line = new_top.min(max_line.saturating_sub(self.dimensions.height as usize));
    }

    /// Scrolls up by n lines.
    pub fn scroll_up(&mut self, lines: usize) {
        self.top_left.line = self.top_left.line.saturating_sub(lines);
    }

    /// Scrolls down by half a page.
    pub fn scroll_half_page_down(&mut self, max_line: usize) {
        self.scroll_down(self.dimensions.height as usize / 2, max_line);
    }

    /// Scrolls up by half a page.
    pub fn scroll_half_page_up(&mut self) {
        self.scroll_up(self.dimensions.height as usize / 2);
    }

    /// Scrolls down by a full page.
    pub fn scroll_page_down(&mut self, max_line: usize) {
        self.scroll_down(self.dimensions.height as usize, max_line);
    }

    /// Scrolls up by a full page.
    pub fn scroll_page_up(&mut self) {
        self.scroll_up(self.dimensions.height as usize);
    }

    /// Centers the viewport on a line.
    pub fn center_on_line(&mut self, line: usize) {
        let half_height = self.dimensions.height as usize / 2;
        self.top_left.line = line.saturating_sub(half_height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_new() {
        let vp = Viewport::new(Position::origin(), Dimensions::new(80, 24));
        assert_eq!(vp.first_line(), 0);
        assert_eq!(vp.last_line(), 24);
    }

    #[test]
    fn test_is_line_visible() {
        let vp = Viewport::new(Position::new(10, 0), Dimensions::new(80, 24));
        assert!(!vp.is_line_visible(9));
        assert!(vp.is_line_visible(10));
        assert!(vp.is_line_visible(33));
        assert!(!vp.is_line_visible(34));
    }

    #[test]
    fn test_scroll_to_line() {
        let mut vp = Viewport::new(Position::new(10, 0), Dimensions::new(80, 24));
        vp.scroll_to_line(5);
        assert_eq!(vp.first_line(), 5);
        
        vp.scroll_to_line(50);
        assert!(vp.is_line_visible(50));
    }

    #[test]
    fn test_scroll_down() {
        let mut vp = Viewport::new(Position::origin(), Dimensions::new(80, 24));
        vp.scroll_down(10, 100);
        assert_eq!(vp.first_line(), 10);
    }

    #[test]
    fn test_scroll_up() {
        let mut vp = Viewport::new(Position::new(20, 0), Dimensions::new(80, 24));
        vp.scroll_up(10);
        assert_eq!(vp.first_line(), 10);
    }

    #[test]
    fn test_center_on_line() {
        let mut vp = Viewport::new(Position::origin(), Dimensions::new(80, 24));
        vp.center_on_line(50);
        assert!(vp.is_line_visible(50));
        assert_eq!(vp.first_line(), 38);
    }

    #[test]
    fn test_scroll_page() {
        let mut vp = Viewport::new(Position::origin(), Dimensions::new(80, 24));
        vp.scroll_page_down(100);
        assert_eq!(vp.first_line(), 24);
        
        vp.scroll_page_up();
        assert_eq!(vp.first_line(), 0);
    }
}
