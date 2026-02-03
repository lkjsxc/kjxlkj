//! Position types for text locations.

use serde::{Deserialize, Serialize};

/// A position in a buffer (0-indexed line and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Position {
    /// Line number (0-indexed).
    pub line: u32,
    /// Column number (0-indexed, in graphemes).
    pub col: u32,
}

impl Position {
    /// Create a new position.
    pub fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }

    /// Create position at origin (0, 0).
    pub fn origin() -> Self {
        Self::default()
    }

    /// Check if this position is before another.
    pub fn is_before(&self, other: &Self) -> bool {
        self.line < other.line || (self.line == other.line && self.col < other.col)
    }

    /// Check if this position is after another.
    pub fn is_after(&self, other: &Self) -> bool {
        self.line > other.line || (self.line == other.line && self.col > other.col)
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
