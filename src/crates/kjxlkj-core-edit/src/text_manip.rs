//! Text manipulation utilities.
//!
//! Implements sorting, alignment, and formatting as specified in
//! `/docs/spec/editing/text-manipulation/`.

use std::cmp::Ordering;

/// Sort order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortOrder {
    /// Ascending (A-Z, 0-9).
    #[default]
    Ascending,
    /// Descending (Z-A, 9-0).
    Descending,
}

/// Sort options.
#[derive(Debug, Clone, Default)]
pub struct SortOptions {
    /// Sort order.
    pub order: SortOrder,
    /// Case-insensitive sorting.
    pub ignore_case: bool,
    /// Remove duplicate lines.
    pub unique: bool,
    /// Numeric sort (treat as numbers).
    pub numeric: bool,
    /// Start column for sorting (0-indexed).
    pub start_col: Option<usize>,
    /// End column for sorting (exclusive).
    pub end_col: Option<usize>,
}

/// Sort lines.
pub fn sort_lines(lines: &[String], options: &SortOptions) -> Vec<String> {
    let mut result: Vec<String> = lines.to_vec();

    result.sort_by(|a, b| {
        let key_a = extract_sort_key(a, options);
        let key_b = extract_sort_key(b, options);

        let cmp = if options.numeric {
            compare_numeric(&key_a, &key_b)
        } else if options.ignore_case {
            key_a.to_lowercase().cmp(&key_b.to_lowercase())
        } else {
            key_a.cmp(&key_b)
        };

        match options.order {
            SortOrder::Ascending => cmp,
            SortOrder::Descending => cmp.reverse(),
        }
    });

    if options.unique {
        result.dedup();
    }

    result
}

/// Extract the portion of a line used for sorting.
fn extract_sort_key(line: &str, options: &SortOptions) -> String {
    let start = options.start_col.unwrap_or(0);
    let end = options.end_col.unwrap_or(line.len());

    let chars: Vec<char> = line.chars().collect();
    let start = start.min(chars.len());
    let end = end.min(chars.len());

    chars[start..end].iter().collect()
}

/// Compare strings as numbers.
fn compare_numeric(a: &str, b: &str) -> Ordering {
    let num_a: f64 = a.trim().parse().unwrap_or(0.0);
    let num_b: f64 = b.trim().parse().unwrap_or(0.0);
    num_a.partial_cmp(&num_b).unwrap_or(Ordering::Equal)
}

/// Alignment direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// Left-aligned.
    #[default]
    Left,
    /// Right-aligned.
    Right,
    /// Center-aligned.
    Center,
}

/// Align text to a specific width.
pub fn align_text(text: &str, width: usize, alignment: Alignment) -> String {
    let text_len = text.chars().count();
    if text_len >= width {
        return text.to_string();
    }

    let padding = width - text_len;
    match alignment {
        Alignment::Left => format!("{}{}", text, " ".repeat(padding)),
        Alignment::Right => format!("{}{}", " ".repeat(padding), text),
        Alignment::Center => {
            let left = padding / 2;
            let right = padding - left;
            format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
        }
    }
}

/// Align lines to the same width.
pub fn align_lines(lines: &[String], alignment: Alignment) -> Vec<String> {
    let max_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    lines
        .iter()
        .map(|l| align_text(l, max_width, alignment))
        .collect()
}

/// Increment number at position in text.
pub fn increment_number(text: &str, pos: usize, delta: i64) -> Option<(String, usize, usize)> {
    // Find number at or near position
    let chars: Vec<char> = text.chars().collect();
    let mut start = pos;
    let mut end = pos;

    // Find start of number
    while start > 0 && is_numeric_char(chars.get(start.saturating_sub(1)).copied()) {
        start -= 1;
    }

    // Find end of number
    while end < chars.len() && is_numeric_char(chars.get(end).copied()) {
        end += 1;
    }

    if start == end {
        return None;
    }

    let num_str: String = chars[start..end].iter().collect();
    let num: i64 = num_str.parse().ok()?;
    let new_num = num + delta;
    let new_str = new_num.to_string();

    let prefix: String = chars[..start].iter().collect();
    let suffix: String = chars[end..].iter().collect();
    let result = format!("{}{}{}", prefix, new_str, suffix);

    Some((result, start, start + new_str.len()))
}

/// Check if a character is part of a number.
fn is_numeric_char(c: Option<char>) -> bool {
    matches!(c, Some('0'..='9') | Some('-'))
}

/// Join lines with a separator.
pub fn join_lines(lines: &[String], separator: &str, preserve_indent: bool) -> String {
    if lines.is_empty() {
        return String::new();
    }

    if preserve_indent {
        lines.join(separator)
    } else {
        lines
            .iter()
            .map(|l| l.trim())
            .collect::<Vec<_>>()
            .join(separator)
    }
}

/// Reverse lines.
pub fn reverse_lines(lines: &[String]) -> Vec<String> {
    lines.iter().rev().cloned().collect()
}

/// Shuffle lines (for deterministic tests, use a seeded RNG).
pub fn shuffle_lines_seeded(lines: &[String], seed: u64) -> Vec<String> {
    let mut result = lines.to_vec();
    // Simple deterministic shuffle using seed
    let len = result.len();
    if len < 2 {
        return result;
    }

    let mut rng = seed;
    for i in (1..len).rev() {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (rng as usize) % (i + 1);
        result.swap(i, j);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_ascending() {
        let lines = vec!["c".to_string(), "a".to_string(), "b".to_string()];
        let result = sort_lines(&lines, &SortOptions::default());
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_sort_descending() {
        let lines = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let opts = SortOptions {
            order: SortOrder::Descending,
            ..Default::default()
        };
        let result = sort_lines(&lines, &opts);
        assert_eq!(result, vec!["c", "b", "a"]);
    }

    #[test]
    fn test_sort_ignore_case() {
        let lines = vec!["B".to_string(), "a".to_string(), "C".to_string()];
        let opts = SortOptions {
            ignore_case: true,
            ..Default::default()
        };
        let result = sort_lines(&lines, &opts);
        assert_eq!(result, vec!["a", "B", "C"]);
    }

    #[test]
    fn test_sort_numeric() {
        let lines = vec!["10".to_string(), "2".to_string(), "1".to_string()];
        let opts = SortOptions {
            numeric: true,
            ..Default::default()
        };
        let result = sort_lines(&lines, &opts);
        assert_eq!(result, vec!["1", "2", "10"]);
    }

    #[test]
    fn test_sort_unique() {
        let lines = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let opts = SortOptions {
            unique: true,
            ..Default::default()
        };
        let result = sort_lines(&lines, &opts);
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn test_align_left() {
        let result = align_text("hi", 5, Alignment::Left);
        assert_eq!(result, "hi   ");
    }

    #[test]
    fn test_align_right() {
        let result = align_text("hi", 5, Alignment::Right);
        assert_eq!(result, "   hi");
    }

    #[test]
    fn test_align_center() {
        let result = align_text("hi", 6, Alignment::Center);
        assert_eq!(result, "  hi  ");
    }

    #[test]
    fn test_increment_number() {
        let result = increment_number("foo 42 bar", 4, 1);
        assert_eq!(result, Some(("foo 43 bar".to_string(), 4, 6)));
    }

    #[test]
    fn test_decrement_number() {
        let result = increment_number("value: 100", 7, -1);
        assert_eq!(result, Some(("value: 99".to_string(), 7, 9)));
    }

    #[test]
    fn test_join_lines() {
        let lines = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = join_lines(&lines, " ", false);
        assert_eq!(result, "a b c");
    }

    #[test]
    fn test_reverse_lines() {
        let lines = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let result = reverse_lines(&lines);
        assert_eq!(result, vec!["3", "2", "1"]);
    }

    #[test]
    fn test_shuffle_deterministic() {
        let lines = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let r1 = shuffle_lines_seeded(&lines, 42);
        let r2 = shuffle_lines_seeded(&lines, 42);
        assert_eq!(r1, r2);
    }
}
