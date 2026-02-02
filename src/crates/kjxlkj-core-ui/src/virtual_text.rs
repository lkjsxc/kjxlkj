//! Virtual text support.
//!
//! Provides virtual text overlays for inline hints, diagnostics, etc.

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

/// Virtual text state for a buffer.
#[derive(Debug, Clone, Default)]
pub struct VirtualTextState {
    /// Virtual texts by ID.
    texts: std::collections::HashMap<usize, VirtualText>,
    /// Virtual texts by line.
    by_line: std::collections::HashMap<usize, Vec<usize>>,
    /// Next ID.
    next_id: usize,
}

impl VirtualTextState {
    /// Creates new virtual text state.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    /// Adds virtual text.
    pub fn add(&mut self, mut vt: VirtualText) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        vt.id = id;

        let line = vt.line;
        self.texts.insert(id, vt);
        self.by_line.entry(line).or_default().push(id);

        id
    }

    /// Removes virtual text by ID.
    pub fn remove(&mut self, id: usize) -> bool {
        if let Some(vt) = self.texts.remove(&id) {
            if let Some(ids) = self.by_line.get_mut(&vt.line) {
                ids.retain(|&i| i != id);
                if ids.is_empty() {
                    self.by_line.remove(&vt.line);
                }
            }
            true
        } else {
            false
        }
    }

    /// Gets virtual text by ID.
    pub fn get(&self, id: usize) -> Option<&VirtualText> {
        self.texts.get(&id)
    }

    /// Gets all virtual texts at a line.
    pub fn at_line(&self, line: usize) -> Vec<&VirtualText> {
        self.by_line
            .get(&line)
            .map(|ids| ids.iter().filter_map(|id| self.texts.get(id)).collect())
            .unwrap_or_default()
    }

    /// Gets all virtual texts at a line in deterministic render order.
    pub fn ordered_at_line(&self, line: usize) -> Vec<&VirtualText> {
        let mut vts = self.at_line(line);
        vts.sort_by_key(|vt| (pos_key(vt.pos), vt.id));
        vts
    }

    /// Clears all virtual texts.
    pub fn clear(&mut self) {
        self.texts.clear();
        self.by_line.clear();
    }

    /// Returns count of virtual texts.
    pub fn len(&self) -> usize {
        self.texts.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.texts.is_empty()
    }
}

fn pos_key(pos: VirtualTextPos) -> (u8, usize) {
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
    fn test_virtual_text_state_add() {
        let mut state = VirtualTextState::new();
        let id = state.add(VirtualText::eol(0, 10, "hint", "Hint"));

        assert!(state.get(id).is_some());
        assert_eq!(state.at_line(10).len(), 1);
    }

    #[test]
    fn test_virtual_text_state_remove() {
        let mut state = VirtualTextState::new();
        let id = state.add(VirtualText::eol(0, 10, "hint", "Hint"));

        assert!(state.remove(id));
        assert!(state.at_line(10).is_empty());
    }

    #[test]
    fn test_virtual_text_state_clear() {
        let mut state = VirtualTextState::new();
        state.add(VirtualText::eol(0, 10, "hint", "Hint"));
        state.add(VirtualText::eol(0, 20, "hint2", "Hint"));

        state.clear();
        assert!(state.is_empty());
    }

    #[test]
    fn test_virtual_text_add_chunk_increases_length() {
        let mut vt = VirtualText::eol(0, 1, "a", "A");
        assert_eq!(vt.text_len(), 1);
        vt.add_chunk("bc", "B");
        assert_eq!(vt.text_len(), 3);
    }

    #[test]
    fn test_virtual_text_state_remove_cleans_line_index() {
        let mut state = VirtualTextState::new();
        let id = state.add(VirtualText::eol(0, 10, "hint", "Hint"));
        assert_eq!(state.at_line(10).len(), 1);
        assert!(state.remove(id));
        assert!(state.at_line(10).is_empty());
        assert!(state.by_line.get(&10).is_none());
    }

    #[test]
    fn test_virtual_text_state_ordered_at_line() {
        let mut state = VirtualTextState::new();
        state.add(VirtualText::inline(0, 10, 20, "b", "B"));
        state.add(VirtualText::inline(0, 10, 5, "a", "A"));
        state.add(VirtualText::eol(0, 10, "e", "E"));

        let ordered = state.ordered_at_line(10);
        assert_eq!(ordered.len(), 3);
        assert_eq!(ordered[0].pos, VirtualTextPos::Inline(5));
        assert_eq!(ordered[1].pos, VirtualTextPos::Inline(20));
        assert_eq!(ordered[2].pos, VirtualTextPos::EndOfLine);
    }

    #[test]
    fn test_virtual_text_pos_key_stable() {
        assert!(pos_key(VirtualTextPos::Overlay(1)) < pos_key(VirtualTextPos::Inline(0)));
        assert!(pos_key(VirtualTextPos::Inline(0)) < pos_key(VirtualTextPos::EndOfLine));
        assert!(pos_key(VirtualTextPos::EndOfLine) < pos_key(VirtualTextPos::RightAlign));
    }
}
