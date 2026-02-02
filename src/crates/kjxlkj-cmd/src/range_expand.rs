//! Ex command range expansion.
//!
//! Expands range specifiers to actual line numbers.

use crate::range_types::{ExpandedRange, RangeContext, RangeSpec};

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
