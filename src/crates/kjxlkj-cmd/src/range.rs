//! Range specifications for Ex commands.

use std::ops::RangeInclusive;

/// A range in an Ex command.
#[derive(Debug, Clone, PartialEq)]
pub enum Range {
    /// Current line (.)
    Current,
    /// Last line ($)
    Last,
    /// All lines (%)
    All,
    /// Specific line number.
    Line(usize),
    /// Range from start to end (inclusive).
    FromTo(usize, usize),
    /// Relative offset from current (+n or -n).
    Relative(i32),
    /// Mark reference ('a)
    Mark(char),
}

impl Range {
    /// Creates a line range.
    pub fn line(n: usize) -> Self {
        Self::Line(n)
    }

    /// Creates a from-to range.
    pub fn from_to(start: usize, end: usize) -> Self {
        Self::FromTo(start, end)
    }

    /// Resolves the range to concrete line numbers.
    pub fn resolve(
        &self,
        current: usize,
        total: usize,
    ) -> Option<RangeInclusive<usize>> {
        match self {
            Range::Current => Some(current..=current),
            Range::Last => {
                if total > 0 {
                    Some((total - 1)..=(total - 1))
                } else {
                    None
                }
            }
            Range::All => {
                if total > 0 {
                    Some(0..=(total - 1))
                } else {
                    None
                }
            }
            Range::Line(n) => {
                if *n < total {
                    Some(*n..=*n)
                } else {
                    None
                }
            }
            Range::FromTo(start, end) => {
                if *start < total && *end < total && start <= end {
                    Some(*start..=*end)
                } else {
                    None
                }
            }
            Range::Relative(offset) => {
                let target = (current as i32 + offset) as usize;
                if target < total {
                    Some(target..=target)
                } else {
                    None
                }
            }
            Range::Mark(_) => None, // Requires mark lookup
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_current() {
        let range = Range::Current;
        assert_eq!(range.resolve(5, 100), Some(5..=5));
    }

    #[test]
    fn test_range_last() {
        let range = Range::Last;
        assert_eq!(range.resolve(5, 100), Some(99..=99));
    }

    #[test]
    fn test_range_all() {
        let range = Range::All;
        assert_eq!(range.resolve(5, 100), Some(0..=99));
    }

    #[test]
    fn test_range_line() {
        let range = Range::Line(42);
        assert_eq!(range.resolve(5, 100), Some(42..=42));
    }

    #[test]
    fn test_range_from_to() {
        let range = Range::FromTo(10, 20);
        assert_eq!(range.resolve(5, 100), Some(10..=20));
    }

    #[test]
    fn test_range_relative() {
        let range = Range::Relative(-3);
        assert_eq!(range.resolve(10, 100), Some(7..=7));
    }
}
