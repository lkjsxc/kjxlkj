//! Position in a text buffer.

use serde::{Deserialize, Serialize};

/// A position in a text buffer (0-based line and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(Serialize, Deserialize)]
pub struct Position {
    /// 0-based line index.
    pub line: usize,
    /// 0-based column index (grapheme clusters).
    pub column: usize,
}

impl Position {
    /// Create a new position.
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Origin position (0, 0).
    pub const fn origin() -> Self {
        Self { line: 0, column: 0 }
    }
}
