//! Position types.

use serde::{Deserialize, Serialize};

/// A byte offset into a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct ByteOffset(pub usize);

/// A character offset into a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CharOffset(pub usize);

/// A line-column position (both 0-indexed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineCol {
    /// Line number (0-indexed).
    pub line: u32,
    /// Column number (0-indexed, in grapheme clusters).
    pub col: u32,
}

impl PartialOrd for LineCol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LineCol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            ord => ord,
        }
    }
}

impl LineCol {
    /// Creates a new line-column position.
    pub fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }

    /// Returns the origin (0, 0).
    pub fn origin() -> Self {
        Self { line: 0, col: 0 }
    }
}

impl Default for LineCol {
    fn default() -> Self {
        Self::origin()
    }
}

/// A 2D point for UI coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_creation() {
        let lc = LineCol::new(10, 5);
        assert_eq!(lc.line, 10);
        assert_eq!(lc.col, 5);
    }
}
