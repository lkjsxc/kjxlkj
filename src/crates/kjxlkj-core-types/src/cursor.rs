//! Cursor types.

use serde::{Deserialize, Serialize};

use crate::position::LineCol;

/// Cursor state within a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    /// Current position (line, column).
    pub position: LineCol,
    /// Desired column for vertical movement (sticky column).
    pub desired_col: usize,
}

impl Cursor {
    /// Create a new cursor at position with desired column.
    pub fn new(position: LineCol, desired_col: usize) -> Self {
        Self {
            position,
            desired_col,
        }
    }

    /// Create a cursor at position, setting desired_col to current column.
    pub fn at(position: LineCol) -> Self {
        Self {
            position,
            desired_col: position.col,
        }
    }

    /// Create a cursor at the origin (0, 0).
    pub fn origin() -> Self {
        Self::at(LineCol::origin())
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::origin()
    }
}

/// Visual cursor shape for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CursorShape {
    /// Block cursor (Normal mode).
    Block,
    /// Vertical bar cursor (Insert mode).
    Bar,
    /// Underline cursor (Replace mode).
    Underline,
    /// Hollow block cursor (Visual mode, inactive window).
    Hollow,
}

impl Default for CursorShape {
    fn default() -> Self {
        Self::Block
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_at_sets_desired_col() {
        let pos = LineCol::new(5, 10);
        let cursor = Cursor::at(pos);
        assert_eq!(cursor.desired_col, 10);
    }

    #[test]
    fn cursor_origin_is_zero() {
        let cursor = Cursor::origin();
        assert_eq!(cursor.position.line, 0);
        assert_eq!(cursor.position.col, 0);
    }
}
