//! Virtual text types.

/// Virtual text position relative to line content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualTextPos {
    /// At end of line.
    EndOfLine,
    /// Overlay at specific column (replaces text visually).
    Overlay(usize),
    /// Right-aligned in window.
    RightAlign,
    /// Inline at specific column (shifts text).
    Inline(usize),
}

/// A virtual text chunk.
#[derive(Debug, Clone)]
pub struct VirtualTextChunk {
    /// Text content.
    pub text: String,
    /// Highlight group.
    pub highlight: String,
}

impl VirtualTextChunk {
    /// Creates a new virtual text chunk.
    pub fn new(text: &str, highlight: &str) -> Self {
        Self {
            text: text.to_string(),
            highlight: highlight.to_string(),
        }
    }
}

/// A virtual text entry.
#[derive(Debug, Clone)]
pub struct VirtualText {
    /// ID for this virtual text.
    pub id: usize,
    /// Line number (1-based).
    pub line: usize,
    /// Position.
    pub pos: VirtualTextPos,
    /// Text chunks.
    pub chunks: Vec<VirtualTextChunk>,
}

impl VirtualText {
    /// Creates new virtual text at end of line.
    pub fn eol(id: usize, line: usize, text: &str, highlight: &str) -> Self {
        Self {
            id,
            line,
            pos: VirtualTextPos::EndOfLine,
            chunks: vec![VirtualTextChunk::new(text, highlight)],
        }
    }

    /// Creates new inline virtual text.
    pub fn inline(id: usize, line: usize, col: usize, text: &str, highlight: &str) -> Self {
        Self {
            id,
            line,
            pos: VirtualTextPos::Inline(col),
            chunks: vec![VirtualTextChunk::new(text, highlight)],
        }
    }

    /// Adds a chunk.
    pub fn add_chunk(&mut self, text: &str, highlight: &str) {
        self.chunks.push(VirtualTextChunk::new(text, highlight));
    }

    /// Returns total text length.
    pub fn text_len(&self) -> usize {
        self.chunks.iter().map(|c| c.text.len()).sum()
    }
}

/// Returns sort key for virtual text position.
pub fn pos_key(pos: VirtualTextPos) -> (u8, usize) {
    match pos {
        VirtualTextPos::Overlay(col) => (0, col),
        VirtualTextPos::Inline(col) => (1, col),
        VirtualTextPos::EndOfLine => (2, usize::MAX),
        VirtualTextPos::RightAlign => (3, usize::MAX),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_text_chunk() {
        let chunk = VirtualTextChunk::new("hint", "Hint");
        assert_eq!(chunk.text, "hint");
        assert_eq!(chunk.highlight, "Hint");
    }

    #[test]
    fn test_virtual_text_eol() {
        let vt = VirtualText::eol(0, 10, "// hint", "Comment");
        assert_eq!(vt.pos, VirtualTextPos::EndOfLine);
        assert_eq!(vt.text_len(), 7);
    }

    #[test]
    fn test_virtual_text_inline() {
        let vt = VirtualText::inline(0, 5, 10, ": String", "Type");
        assert_eq!(vt.pos, VirtualTextPos::Inline(10));
    }

    #[test]
    fn test_virtual_text_add_chunk() {
        let mut vt = VirtualText::eol(0, 1, "a", "A");
        assert_eq!(vt.text_len(), 1);
        vt.add_chunk("bc", "B");
        assert_eq!(vt.text_len(), 3);
    }

    #[test]
    fn test_pos_key_stable() {
        assert!(pos_key(VirtualTextPos::Overlay(1)) < pos_key(VirtualTextPos::Inline(0)));
        assert!(pos_key(VirtualTextPos::Inline(0)) < pos_key(VirtualTextPos::EndOfLine));
        assert!(pos_key(VirtualTextPos::EndOfLine) < pos_key(VirtualTextPos::RightAlign));
    }
}
