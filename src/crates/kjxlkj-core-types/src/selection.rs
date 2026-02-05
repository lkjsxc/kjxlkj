//! Selection types for visual mode.

use super::position::LineCol;
use serde::{Deserialize, Serialize};

/// Selection anchor and cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
    /// Anchor position (fixed end).
    pub anchor: LineCol,
    /// Cursor position (moving end).
    pub cursor: LineCol,
}

impl Selection {
    /// Create a new selection.
    pub fn new(anchor: LineCol, cursor: LineCol) -> Self {
        Self { anchor, cursor }
    }

    /// Create a selection at a single point.
    pub fn point(pos: LineCol) -> Self {
        Self {
            anchor: pos,
            cursor: pos,
        }
    }

    /// Get the start of the selection (earlier position).
    pub fn start(&self) -> LineCol {
        if self.anchor.line < self.cursor.line
            || (self.anchor.line == self.cursor.line && self.anchor.col <= self.cursor.col)
        {
            self.anchor
        } else {
            self.cursor
        }
    }

    /// Get the end of the selection (later position).
    pub fn end(&self) -> LineCol {
        if self.anchor.line > self.cursor.line
            || (self.anchor.line == self.cursor.line && self.anchor.col >= self.cursor.col)
        {
            self.anchor
        } else {
            self.cursor
        }
    }

    /// Swap anchor and cursor.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.anchor, &mut self.cursor);
    }

    /// Check if selection is empty.
    pub fn is_empty(&self) -> bool {
        self.anchor == self.cursor
    }
}

/// Selection mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMode {
    /// Character-wise selection.
    Char,
    /// Line-wise selection.
    Line,
    /// Block selection.
    Block,
}
