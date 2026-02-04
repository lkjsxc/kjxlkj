//! Position types for text navigation.

use serde::{Deserialize, Serialize};

/// Line and column position (0-indexed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineCol {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed, grapheme cluster index).
    pub col: usize,
}

impl LineCol {
    /// Create a new position.
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Origin position (0, 0).
    pub fn origin() -> Self {
        Self::new(0, 0)
    }
}

impl Default for LineCol {
    fn default() -> Self {
        Self::origin()
    }
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

/// Absolute byte offset in buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize, Default)]
pub struct ByteOffset(pub usize);

impl ByteOffset {
    /// Create a new byte offset.
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }

    /// Get the raw offset value.
    pub fn as_usize(self) -> usize {
        self.0
    }
}

/// Character (grapheme cluster) offset in buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize, Default)]
pub struct CharOffset(pub usize);

impl CharOffset {
    /// Create a new char offset.
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }

    /// Get the raw offset value.
    pub fn as_usize(self) -> usize {
        self.0
    }
}

/// Generic position in the buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    /// Line and column position.
    LineCol(LineCol),
    /// Byte offset.
    Byte(ByteOffset),
    /// Character offset.
    Char(CharOffset),
}

impl Default for Position {
    fn default() -> Self {
        Position::LineCol(LineCol::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linecol_ordering() {
        let a = LineCol::new(1, 5);
        let b = LineCol::new(2, 0);
        let c = LineCol::new(1, 10);
        assert!(a < b);
        assert!(a < c);
        assert!(c < b);
    }

    #[test]
    fn linecol_origin() {
        let origin = LineCol::origin();
        assert_eq!(origin.line, 0);
        assert_eq!(origin.col, 0);
    }
}
