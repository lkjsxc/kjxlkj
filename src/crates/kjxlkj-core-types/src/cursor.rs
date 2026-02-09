/// Cursor position in grapheme coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    /// Zero-based line number.
    pub line: usize,
    /// Zero-based grapheme offset within the line.
    pub grapheme: usize,
}

impl CursorPosition {
    pub fn new(line: usize, grapheme: usize) -> Self {
        Self { line, grapheme }
    }

    pub fn origin() -> Self {
        Self {
            line: 0,
            grapheme: 0,
        }
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self::origin()
    }
}
