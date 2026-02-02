//! Position types for text locations.

use serde::{Deserialize, Serialize};

/// A position in a text buffer (0-indexed line and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed, in grapheme clusters).
    pub col: usize,
}

impl Position {
    /// Creates a new position.
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Returns the origin position (0, 0).
    pub fn origin() -> Self {
        Self::default()
    }

    /// Creates a position at the start of a given line.
    pub fn line_start(line: usize) -> Self {
        Self { line, col: 0 }
    }

    /// Returns true if this position is at origin.
    pub fn is_origin(&self) -> bool {
        self.line == 0 && self.col == 0
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            ord => ord,
        }
    }
}
