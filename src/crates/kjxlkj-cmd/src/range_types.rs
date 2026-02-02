//! Range expansion types.

use kjxlkj_core_types::Position;

/// Range specifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeSpec {
    /// Current line (.).
    CurrentLine,
    /// Last line ($).
    LastLine,
    /// Specific line number.
    Line(usize),
    /// Offset from current (+n, -n).
    Offset(isize),
    /// Mark ('a - 'z).
    Mark(char),
    /// Visual selection ('<, '>).
    Visual { is_start: bool },
    /// Search forward (/pattern/).
    SearchForward(String),
    /// Search backward (?pattern?).
    SearchBackward(String),
    /// Entire file (%).
    All,
}

/// A range with start and end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandedRange {
    /// Start line (0-indexed).
    pub start: usize,
    /// End line (0-indexed, inclusive).
    pub end: usize,
}

impl ExpandedRange {
    /// Creates a new range.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Creates a single-line range.
    pub fn single(line: usize) -> Self {
        Self::new(line, line)
    }

    /// Returns the number of lines.
    pub fn line_count(&self) -> usize {
        self.end - self.start + 1
    }

    /// Returns iterator over lines.
    pub fn lines(&self) -> std::ops::RangeInclusive<usize> {
        self.start..=self.end
    }

    /// Returns whether the range contains a line.
    pub fn contains(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }
}

/// Context for range expansion.
pub struct RangeContext {
    /// Current line (0-indexed).
    pub current_line: usize,
    /// Total lines in buffer.
    pub total_lines: usize,
    /// Visual selection start (if any).
    pub visual_start: Option<Position>,
    /// Visual selection end (if any).
    pub visual_end: Option<Position>,
}

impl RangeContext {
    /// Creates a new context.
    pub fn new(current_line: usize, total_lines: usize) -> Self {
        Self {
            current_line,
            total_lines,
            visual_start: None,
            visual_end: None,
        }
    }

    /// Sets visual selection.
    pub fn with_visual(mut self, start: Position, end: Position) -> Self {
        self.visual_start = Some(start);
        self.visual_end = Some(end);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expanded_range_single() {
        let r = ExpandedRange::single(5);
        assert_eq!(r.line_count(), 1);
        assert!(r.contains(5));
    }

    #[test]
    fn test_expanded_range_multi() {
        let r = ExpandedRange::new(5, 10);
        assert_eq!(r.line_count(), 6);
        assert!(r.contains(7));
        assert!(!r.contains(11));
    }

    #[test]
    fn test_range_context_new() {
        let ctx = RangeContext::new(10, 100);
        assert_eq!(ctx.current_line, 10);
        assert_eq!(ctx.total_lines, 100);
    }
}
