//! Cursor types and position representation.

/// Cursor position as (line, grapheme_offset).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CursorPosition {
    /// Zero-based line number.
    pub line: usize,
    /// Zero-based grapheme offset within line.
    pub grapheme: usize,
}

impl CursorPosition {
    /// Create a new cursor position.
    pub fn new(line: usize, grapheme: usize) -> Self {
        Self { line, grapheme }
    }

    /// Create position at start of document.
    pub fn origin() -> Self {
        Self { line: 0, grapheme: 0 }
    }
}

/// Selection anchor for visual mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// Anchor position (start of selection).
    pub anchor: CursorPosition,
    /// Head position (cursor end of selection).
    pub head: CursorPosition,
}

impl Selection {
    /// Create a new selection from anchor to head.
    pub fn new(anchor: CursorPosition, head: CursorPosition) -> Self {
        Self { anchor, head }
    }

    /// Get the start position (earlier in document).
    pub fn start(&self) -> CursorPosition {
        if self.anchor.line < self.head.line
            || (self.anchor.line == self.head.line && self.anchor.grapheme <= self.head.grapheme)
        {
            self.anchor
        } else {
            self.head
        }
    }

    /// Get the end position (later in document).
    pub fn end(&self) -> CursorPosition {
        if self.anchor.line > self.head.line
            || (self.anchor.line == self.head.line && self.anchor.grapheme >= self.head.grapheme)
        {
            self.anchor
        } else {
            self.head
        }
    }
}

/// Cursor display shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorShape {
    /// Block cursor (default for normal mode).
    #[default]
    Block,
    /// Underline cursor.
    Underline,
    /// Bar cursor (for insert mode).
    Bar,
}
