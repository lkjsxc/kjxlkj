//! Position types for kjxlkj editor.

use serde::{Deserialize, Serialize};

/// Zero-based line index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(Serialize, Deserialize)]
pub struct LineIdx(pub usize);

impl LineIdx {
    /// Creates a new line index.
    pub fn new(idx: usize) -> Self {
        Self(idx)
    }

    /// Returns the raw index value.
    pub fn as_usize(self) -> usize {
        self.0
    }

    /// Returns the 1-based line number for display.
    pub fn to_line_number(self) -> usize {
        self.0 + 1
    }

    /// Creates from a 1-based line number.
    pub fn from_line_number(num: usize) -> Self {
        Self(num.saturating_sub(1))
    }
}

/// Byte offset within a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(Serialize, Deserialize)]
pub struct ByteOffset(pub usize);

impl ByteOffset {
    /// Creates a new byte offset.
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }

    /// Returns the raw offset value.
    pub fn as_usize(self) -> usize {
        self.0
    }
}

/// Character (grapheme cluster) offset within a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(Serialize, Deserialize)]
pub struct CharOffset(pub usize);

impl CharOffset {
    /// Creates a new character offset.
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }

    /// Returns the raw offset value.
    pub fn as_usize(self) -> usize {
        self.0
    }
}

/// A position in a buffer (line and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Position {
    /// Zero-based line index.
    pub line: LineIdx,
    /// Zero-based column (character offset within line).
    pub col: CharOffset,
}

impl Position {
    /// Creates a new position.
    pub fn new(line: usize, col: usize) -> Self {
        Self {
            line: LineIdx(line),
            col: CharOffset(col),
        }
    }

    /// Creates position at start of document.
    pub fn origin() -> Self {
        Self::new(0, 0)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.line.cmp(&other.line).then(self.col.cmp(&other.col))
    }
}
