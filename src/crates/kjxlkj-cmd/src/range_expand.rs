//! Ex command range expansion.
//!
//! Expands range specifiers to actual line numbers.

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

/// Expands a single range specifier to a line number.
pub fn expand_spec(spec: &RangeSpec, ctx: &RangeContext) -> Option<usize> {
    match spec {
        RangeSpec::CurrentLine => Some(ctx.current_line),
        RangeSpec::LastLine => Some(ctx.total_lines.saturating_sub(1)),
        RangeSpec::Line(n) => {
            // Convert 1-indexed to 0-indexed.
            if *n == 0 {
                Some(0)
            } else {
                Some(n.saturating_sub(1).min(ctx.total_lines.saturating_sub(1)))
            }
        }
        RangeSpec::Offset(n) => {
            let new_line = if *n >= 0 {
                ctx.current_line.saturating_add(*n as usize)
            } else {
                ctx.current_line.saturating_sub((-n) as usize)
            };
            Some(new_line.min(ctx.total_lines.saturating_sub(1)))
        }
        RangeSpec::Visual { is_start } => {
            if *is_start {
                ctx.visual_start.map(|p| p.line)
            } else {
                ctx.visual_end.map(|p| p.line)
            }
        }
        RangeSpec::All => None, // Handled separately
        RangeSpec::Mark(_) | RangeSpec::SearchForward(_) | RangeSpec::SearchBackward(_) => {
            // Would need mark store or search to resolve.
            None
        }
    }
}

/// Expands a range to start and end lines.
pub fn expand_range(
    start: &RangeSpec,
    end: Option<&RangeSpec>,
    ctx: &RangeContext,
) -> Option<ExpandedRange> {
    // Handle % specially.
    if matches!(start, RangeSpec::All) {
        return Some(ExpandedRange::new(0, ctx.total_lines.saturating_sub(1)));
    }

    let start_line = expand_spec(start, ctx)?;
    let end_line = end
        .and_then(|e| expand_spec(e, ctx))
        .unwrap_or(start_line);

    let (s, e) = if start_line <= end_line {
        (start_line, end_line)
    } else {
        (end_line, start_line)
    };

    Some(ExpandedRange::new(s, e))
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
    fn test_expand_current_line() {
        let ctx = RangeContext::new(10, 100);
        assert_eq!(expand_spec(&RangeSpec::CurrentLine, &ctx), Some(10));
    }

    #[test]
    fn test_expand_last_line() {
        let ctx = RangeContext::new(10, 100);
        assert_eq!(expand_spec(&RangeSpec::LastLine, &ctx), Some(99));
    }

    #[test]
    fn test_expand_line_number() {
        let ctx = RangeContext::new(10, 100);
        assert_eq!(expand_spec(&RangeSpec::Line(5), &ctx), Some(4)); // 1-indexed
    }

    #[test]
    fn test_expand_offset_positive() {
        let ctx = RangeContext::new(10, 100);
        assert_eq!(expand_spec(&RangeSpec::Offset(5), &ctx), Some(15));
    }

    #[test]
    fn test_expand_offset_negative() {
        let ctx = RangeContext::new(10, 100);
        assert_eq!(expand_spec(&RangeSpec::Offset(-3), &ctx), Some(7));
    }

    #[test]
    fn test_expand_range_all() {
        let ctx = RangeContext::new(10, 100);
        let r = expand_range(&RangeSpec::All, None, &ctx).unwrap();
        assert_eq!(r.start, 0);
        assert_eq!(r.end, 99);
    }
}
