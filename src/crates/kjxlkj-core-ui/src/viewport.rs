//! Viewport representation.

use kjxlkj_core_types::LineCol;
use serde::{Deserialize, Serialize};

/// Viewport dimensions and scroll position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viewport {
    /// Width in columns.
    pub width: u16,
    /// Height in rows.
    pub height: u16,
    /// First visible line (0-indexed).
    pub scroll_top: usize,
    /// Horizontal scroll offset.
    pub scroll_left: usize,
}

impl Viewport {
    /// Create a new viewport.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            scroll_top: 0,
            scroll_left: 0,
        }
    }

    /// Get the number of visible lines.
    pub fn visible_lines(&self) -> usize {
        self.height as usize
    }

    /// Get the last visible line index.
    pub fn scroll_bottom(&self) -> usize {
        self.scroll_top + self.visible_lines().saturating_sub(1)
    }

    /// Check if a line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.scroll_top && line <= self.scroll_bottom()
    }

    /// Scroll to ensure a position is visible.
    pub fn ensure_visible(&mut self, pos: LineCol, margin: usize) {
        // Vertical scrolling
        if pos.line < self.scroll_top + margin {
            self.scroll_top = pos.line.saturating_sub(margin);
        } else if pos.line + margin >= self.scroll_top + self.visible_lines() {
            self.scroll_top = (pos.line + margin + 1).saturating_sub(self.visible_lines());
        }
    }

    /// Scroll down by a number of lines.
    pub fn scroll_down(&mut self, lines: usize, max_line: usize) {
        let max_scroll = max_line.saturating_sub(self.visible_lines().saturating_sub(1));
        self.scroll_top = (self.scroll_top + lines).min(max_scroll);
    }

    /// Scroll up by a number of lines.
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_top = self.scroll_top.saturating_sub(lines);
    }

    /// Scroll by half the viewport height.
    pub fn scroll_half_down(&mut self, max_line: usize) {
        self.scroll_down(self.visible_lines() / 2, max_line);
    }

    /// Scroll by half the viewport height upward.
    pub fn scroll_half_up(&mut self) {
        self.scroll_up(self.visible_lines() / 2);
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
    fn viewport_basics() {
        let vp = Viewport::new(80, 24);
        assert_eq!(vp.visible_lines(), 24);
        assert_eq!(vp.scroll_bottom(), 23);
    }

    #[test]
    fn ensure_visible_scrolls_down() {
        let mut vp = Viewport::new(80, 10);
        vp.ensure_visible(LineCol::new(15, 0), 2);
        assert!(vp.is_line_visible(15));
    }

    #[test]
    fn ensure_visible_scrolls_up() {
        let mut vp = Viewport::new(80, 10);
        vp.scroll_top = 20;
        vp.ensure_visible(LineCol::new(5, 0), 2);
        assert!(vp.is_line_visible(5));
    }
}
