//! Geometric types: positions, ranges, rectangles, sizes.

use serde::{Deserialize, Serialize};

/// A zero-based line-column position in a buffer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

/// A range between two positions (start inclusive, end exclusive).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Return an ordered range (start <= end).
    pub fn ordered(self) -> Self {
        if self.start <= self.end {
            self
        } else {
            Self {
                start: self.end,
                end: self.start,
            }
        }
    }

    pub fn contains(&self, pos: Position) -> bool {
        let r = self.ordered();
        pos >= r.start && pos < r.end
    }
}

/// Terminal dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

/// A rectangular region on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_ordering() {
        assert!(Position::new(0, 0) < Position::new(0, 1));
        assert!(Position::new(0, 5) < Position::new(1, 0));
    }

    #[test]
    fn range_ordered() {
        let r = Range::new(Position::new(2, 0), Position::new(0, 0));
        let o = r.ordered();
        assert_eq!(o.start, Position::new(0, 0));
        assert_eq!(o.end, Position::new(2, 0));
    }

    #[test]
    fn range_contains() {
        let r = Range::new(Position::new(1, 0), Position::new(3, 0));
        assert!(r.contains(Position::new(1, 0)));
        assert!(r.contains(Position::new(2, 5)));
        assert!(!r.contains(Position::new(3, 0)));
        assert!(!r.contains(Position::new(0, 5)));
    }

    #[test]
    fn range_is_empty() {
        let r = Range::new(Position::new(1, 1), Position::new(1, 1));
        assert!(r.is_empty());
    }

    #[test]
    fn rect_dimensions() {
        let r = Rect::new(5, 10, 20, 30);
        assert_eq!(r.right(), 25);
        assert_eq!(r.bottom(), 40);
        assert_eq!(r.area(), 600);
    }
}
