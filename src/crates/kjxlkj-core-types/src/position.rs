//! Position types for text coordinates.

use serde::{Deserialize, Serialize};

/// Byte offset in buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct ByteOffset(pub usize);

impl ByteOffset {
    /// Create a new byte offset.
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }
}

/// Character index in line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct CharIndex(pub usize);

impl CharIndex {
    /// Create a new character index.
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

/// Line and column position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LineCol {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed).
    pub col: usize,
}

impl LineCol {
    /// Create a new line/column position.
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Create at origin.
    pub fn origin() -> Self {
        Self { line: 0, col: 0 }
    }
}

/// A range in the buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TextRange {
    /// Start position.
    pub start: LineCol,
    /// End position (exclusive).
    pub end: LineCol,
}

impl TextRange {
    /// Create a new text range.
    pub fn new(start: LineCol, end: LineCol) -> Self {
        Self { start, end }
    }

    /// Check if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Check if a position is within this range.
    pub fn contains(&self, pos: LineCol) -> bool {
        (pos.line > self.start.line || (pos.line == self.start.line && pos.col >= self.start.col))
            && (pos.line < self.end.line || (pos.line == self.end.line && pos.col < self.end.col))
    }
}
