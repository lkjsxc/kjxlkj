//! Cursor types for kjxlkj editor.

use serde::{Deserialize, Serialize};
use crate::position::Position;

/// Cursor position in a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cursor {
    /// The actual position of the cursor.
    position: Position,
    /// Preferred column when moving vertically.
    /// Vim calls this "curswant".
    preferred_col: Option<usize>,
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new(Position::origin())
    }
}

impl Cursor {
    /// Creates a new cursor at the given position.
    pub fn new(position: Position) -> Self {
        Self {
            position,
            preferred_col: None,
        }
    }

    /// Returns the current position.
    pub fn position(&self) -> Position {
        self.position
    }

    /// Returns the line index.
    pub fn line(&self) -> usize {
        self.position.line.as_usize()
    }

    /// Returns the column offset.
    pub fn col(&self) -> usize {
        self.position.col.as_usize()
    }

    /// Sets the position, clearing preferred column.
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
        self.preferred_col = None;
    }

    /// Moves to a new position, preserving preferred column.
    pub fn move_to(&mut self, position: Position) {
        self.position = position;
    }

    /// Returns the preferred column for vertical movement.
    pub fn preferred_col(&self) -> usize {
        self.preferred_col.unwrap_or(self.position.col.as_usize())
    }

    /// Sets the preferred column for vertical movement.
    pub fn set_preferred_col(&mut self, col: usize) {
        self.preferred_col = Some(col);
    }

    /// Clears the preferred column.
    pub fn clear_preferred_col(&mut self) {
        self.preferred_col = None;
    }
}

/// Visual selection anchor and cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Selection {
    /// The anchor (start) of the selection.
    pub anchor: Position,
    /// The cursor (end) of the selection.
    pub cursor: Position,
}

impl Selection {
    /// Creates a new selection.
    pub fn new(anchor: Position, cursor: Position) -> Self {
        Self { anchor, cursor }
    }

    /// Creates a collapsed selection at a position.
    pub fn collapsed(position: Position) -> Self {
        Self {
            anchor: position,
            cursor: position,
        }
    }

    /// Returns the start position (minimum of anchor and cursor).
    pub fn start(&self) -> Position {
        std::cmp::min(self.anchor, self.cursor)
    }

    /// Returns the end position (maximum of anchor and cursor).
    pub fn end(&self) -> Position {
        std::cmp::max(self.anchor, self.cursor)
    }

    /// Returns true if the selection is empty.
    pub fn is_empty(&self) -> bool {
        self.anchor == self.cursor
    }

    /// Returns true if the selection is reversed.
    pub fn is_reversed(&self) -> bool {
        self.cursor < self.anchor
    }
}
