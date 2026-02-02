//! Syntax highlighting caches.

use crate::highlight_types::{HighlightGroup, HighlightSpan};

/// Highlights for a single line.
#[derive(Debug, Clone, Default)]
pub struct LineHighlights {
    /// Spans in the line.
    spans: Vec<HighlightSpan>,
}

impl LineHighlights {
    /// Creates empty highlights.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a span.
    pub fn add(&mut self, span: HighlightSpan) {
        self.spans.push(span);
    }

    /// Returns the spans.
    pub fn spans(&self) -> &[HighlightSpan] {
        &self.spans
    }

    /// Returns the group at a byte offset.
    pub fn group_at(&self, offset: usize) -> Option<HighlightGroup> {
        self.spans
            .iter()
            .find(|s| offset >= s.start && offset < s.end)
            .map(|s| s.group)
    }

    /// Clears all spans.
    pub fn clear(&mut self) {
        self.spans.clear();
    }
}

/// Buffer highlights cache.
#[derive(Debug, Clone, Default)]
pub struct BufferHighlights {
    /// Per-line highlights.
    lines: Vec<LineHighlights>,
}

impl BufferHighlights {
    /// Creates empty buffer highlights.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ensures capacity for n lines.
    pub fn ensure_lines(&mut self, n: usize) {
        if self.lines.len() < n {
            self.lines.resize_with(n, LineHighlights::new);
        }
    }

    /// Gets highlights for a line.
    pub fn line(&self, idx: usize) -> Option<&LineHighlights> {
        self.lines.get(idx)
    }

    /// Gets mutable highlights for a line.
    pub fn line_mut(&mut self, idx: usize) -> Option<&mut LineHighlights> {
        self.lines.get_mut(idx)
    }

    /// Sets highlights for a line.
    pub fn set_line(&mut self, idx: usize, hl: LineHighlights) {
        self.ensure_lines(idx + 1);
        self.lines[idx] = hl;
    }

    /// Invalidates a range of lines.
    pub fn invalidate(&mut self, start: usize, end: usize) {
        for i in start..end.min(self.lines.len()) {
            self.lines[i].clear();
        }
    }

    /// Clears all highlights.
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Returns line count.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_highlights() {
        let mut hl = LineHighlights::new();
        hl.add(HighlightSpan::new(0, 5, HighlightGroup::Keyword));
        hl.add(HighlightSpan::new(6, 10, HighlightGroup::String));

        assert_eq!(hl.group_at(2), Some(HighlightGroup::Keyword));
        assert_eq!(hl.group_at(7), Some(HighlightGroup::String));
        assert_eq!(hl.group_at(5), None);
    }

    #[test]
    fn test_buffer_highlights() {
        let mut buf = BufferHighlights::new();
        buf.ensure_lines(5);
        assert_eq!(buf.line_count(), 5);

        let mut hl = LineHighlights::new();
        hl.add(HighlightSpan::new(0, 3, HighlightGroup::Comment));
        buf.set_line(2, hl);

        assert_eq!(
            buf.line(2).unwrap().group_at(1),
            Some(HighlightGroup::Comment)
        );
    }

    #[test]
    fn test_invalidate() {
        let mut buf = BufferHighlights::new();
        let mut hl = LineHighlights::new();
        hl.add(HighlightSpan::new(0, 5, HighlightGroup::String));
        buf.set_line(3, hl);

        assert!(!buf.line(3).unwrap().spans().is_empty());
        buf.invalidate(2, 5);
        assert!(buf.line(3).unwrap().spans().is_empty());
    }

    #[test]
    fn test_line_highlights_clear() {
        let mut hl = LineHighlights::new();
        hl.add(HighlightSpan::new(0, 5, HighlightGroup::Normal));
        hl.clear();
        assert!(hl.spans().is_empty());
    }

    #[test]
    fn test_buffer_highlights_clear() {
        let mut buf = BufferHighlights::new();
        buf.ensure_lines(10);
        buf.clear();
        assert_eq!(buf.line_count(), 0);
    }
}
