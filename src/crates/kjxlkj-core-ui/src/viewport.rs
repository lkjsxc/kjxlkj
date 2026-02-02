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
}
