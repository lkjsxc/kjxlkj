//! Cursor representation.

use crate::Position;
use serde::{Deserialize, Serialize};

/// A cursor in a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Cursor {
    /// Current position.
    pub position: Position,
    /// Preferred column for vertical movement.
    pub preferred_col: Option<usize>,
}

impl Cursor {
    /// Create a new cursor at a position.
    pub fn new(position: Position) -> Self {
        Self {
            position,
            preferred_col: None,
        }
    }

    /// Create a cursor at (0, 0).
    pub fn origin() -> Self {
        Self::new(Position::default())
    }

    /// Set the preferred column.
    pub fn with_preferred_col(mut self, col: usize) -> Self {
        self.preferred_col = Some(col);
        self
    }

    /// Clear the preferred column.
    pub fn clear_preferred_col(&mut self) {
        self.preferred_col = None;
    }

    /// Get the line number.
    pub fn line(&self) -> usize {
        self.position.line
    }

    /// Get the column number.
    pub fn col(&self) -> usize {
        self.position.col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_creation() {
        let c = Cursor::new(Position::new(1, 2));
        assert_eq!(c.line(), 1);
        assert_eq!(c.col(), 2);
        assert!(c.preferred_col.is_none());
    }

    #[test]
    fn cursor_preferred_col() {
        let c = Cursor::origin().with_preferred_col(10);
        assert_eq!(c.preferred_col, Some(10));
    }

    #[test]
    fn cursor_origin_at_zero() {
        let c = Cursor::origin();
        assert_eq!(c.line(), 0);
        assert_eq!(c.col(), 0);
    }

    #[test]
    fn cursor_clear_preferred_col() {
        let mut c = Cursor::origin().with_preferred_col(10);
        c.clear_preferred_col();
        assert!(c.preferred_col.is_none());
    }

    #[test]
    fn cursor_equality() {
        let c1 = Cursor::new(Position::new(5, 10));
        let c2 = Cursor::new(Position::new(5, 10));
        assert_eq!(c1, c2);
    }

    #[test]
    fn cursor_default() {
        let c = Cursor::default();
        assert_eq!(c.line(), 0);
        assert_eq!(c.col(), 0);
    }

    #[test]
    fn cursor_clone() {
        let c = Cursor::new(Position::new(3, 4));
        let cloned = c.clone();
        assert_eq!(c, cloned);
    }

    #[test]
    fn cursor_position_access() {
        let c = Cursor::new(Position::new(10, 20));
        assert_eq!(c.position, Position::new(10, 20));
    }

    #[test]
    fn cursor_preferred_col_preserved() {
        let c = Cursor::origin().with_preferred_col(15);
        assert_eq!(c.preferred_col, Some(15));
        assert_eq!(c.col(), 0);
    }

    #[test]
    fn cursor_chained_with_preferred() {
        let c = Cursor::new(Position::new(1, 2)).with_preferred_col(5);
        assert_eq!(c.line(), 1);
        assert_eq!(c.col(), 2);
        assert_eq!(c.preferred_col, Some(5));
    }

    #[test]
    fn cursor_inequality() {
        let c1 = Cursor::new(Position::new(1, 2));
        let c2 = Cursor::new(Position::new(3, 4));
        assert_ne!(c1, c2);
    }

    #[test]
    fn cursor_new_at_line() {
        let c = Cursor::new(Position::new(5, 0));
        assert_eq!(c.line(), 5);
    }

    #[test]
    fn cursor_new_at_col() {
        let c = Cursor::new(Position::new(0, 10));
        assert_eq!(c.col(), 10);
    }

    #[test]
    fn cursor_both_coords() {
        let c1 = Cursor::new(Position::new(1, 2));
        let c2 = Cursor::new(Position::new(1, 2));
        assert_eq!(c1, c2);
    }

    #[test]
    fn cursor_preferred_col_none_default() {
        let c = Cursor::origin();
        assert_eq!(c.preferred_col, None);
    }
}
