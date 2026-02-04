//! Cursor position in a buffer.

use serde::{Deserialize, Serialize};

/// A cursor position within a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Cursor {
    /// Zero-based line index.
    pub line: usize,
    /// Zero-based column index (grapheme cluster offset).
    pub col: usize,
}

impl Cursor {
    /// Create a new cursor at the given line and column.
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Create a cursor at the origin (0, 0).
    pub fn origin() -> Self {
        Self { line: 0, col: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_origin() {
        let c = Cursor::origin();
        assert_eq!(c.line, 0);
        assert_eq!(c.col, 0);
    }

    #[test]
    fn cursor_new() {
        let c = Cursor::new(5, 10);
        assert_eq!(c.line, 5);
        assert_eq!(c.col, 10);
    }
}
