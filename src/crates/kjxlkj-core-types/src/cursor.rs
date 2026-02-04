//! Cursor type for buffer navigation.

use crate::Position;
use serde::{Deserialize, Serialize};

/// A cursor position within a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Cursor {
    /// The current position.
    pub position: Position,
    /// Preferred column for vertical movement (sticky column).
    /// Used when moving up/down to maintain horizontal position.
    pub preferred_col: Option<usize>,
}

impl Cursor {
    /// Create a new cursor at the given position.
    pub fn new(line: usize, col: usize) -> Self {
        Self {
            position: Position::new(line, col),
            preferred_col: None,
        }
    }

    /// Create a cursor from a position.
    pub fn from_position(position: Position) -> Self {
        Self {
            position,
            preferred_col: None,
        }
    }

    /// Get the line number (0-indexed).
    #[inline]
    pub fn line(&self) -> usize {
        self.position.line
    }

    /// Get the column number (0-indexed).
    #[inline]
    pub fn col(&self) -> usize {
        self.position.col
    }

    /// Move the cursor to a new position.
    pub fn move_to(&mut self, line: usize, col: usize) {
        self.position = Position::new(line, col);
        self.preferred_col = None;
    }

    /// Move the cursor vertically, preserving preferred column.
    pub fn move_vertical(&mut self, line: usize, line_len: usize) {
        let target_col = self.preferred_col.unwrap_or(self.position.col);
        self.position.line = line;
        self.position.col = target_col.min(line_len.saturating_sub(1).max(0));
        if self.preferred_col.is_none() {
            self.preferred_col = Some(target_col);
        }
    }

    /// Clear the preferred column (e.g., after horizontal movement).
    pub fn clear_preferred_col(&mut self) {
        self.preferred_col = None;
    }
}

impl From<Position> for Cursor {
    fn from(position: Position) -> Self {
        Self::from_position(position)
    }
}

impl From<(usize, usize)> for Cursor {
    fn from((line, col): (usize, usize)) -> Self {
        Self::new(line, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_new() {
        let cursor = Cursor::new(5, 10);
        assert_eq!(cursor.line(), 5);
        assert_eq!(cursor.col(), 10);
    }

    #[test]
    fn test_cursor_move_vertical() {
        let mut cursor = Cursor::new(0, 10);
        cursor.move_vertical(1, 5);
        assert_eq!(cursor.line(), 1);
        assert_eq!(cursor.col(), 4);
        assert_eq!(cursor.preferred_col, Some(10));
    }

    #[test]
    fn test_cursor_from_tuple() {
        let cursor: Cursor = (3, 7).into();
        assert_eq!(cursor.line(), 3);
        assert_eq!(cursor.col(), 7);
    }

    #[test]
    fn test_cursor_from_position() {
        let pos = Position::new(2, 5);
        let cursor = Cursor::from_position(pos);
        assert_eq!(cursor.position, pos);
    }

    #[test]
    fn test_cursor_move_to() {
        let mut cursor = Cursor::new(0, 0);
        cursor.preferred_col = Some(10);
        cursor.move_to(5, 3);
        assert_eq!(cursor.line(), 5);
        assert_eq!(cursor.col(), 3);
        assert!(cursor.preferred_col.is_none());
    }

    #[test]
    fn test_cursor_clear_preferred_col() {
        let mut cursor = Cursor::new(0, 0);
        cursor.preferred_col = Some(10);
        cursor.clear_preferred_col();
        assert!(cursor.preferred_col.is_none());
    }

    #[test]
    fn test_cursor_default() {
        let cursor = Cursor::default();
        assert_eq!(cursor.position, Position::ORIGIN);
    }
}

