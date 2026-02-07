//! Long-line segmentation for rendering.

use unicode_width::UnicodeWidthChar;

/// Maximum number of display columns before segmentation.
pub const MAX_RENDER_COLS: usize = 10_000;

/// True if `c` is a wide (double-width) character.
pub fn is_wide_char(c: char) -> bool {
    c.width().unwrap_or(0) > 1
}

/// Split a long line into segments of at most `max_width` display columns.
pub fn segment_line(line: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![line.to_string()];
    }
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut col = 0usize;

    for c in line.chars() {
        let w = if c == '\t' { 8 } else { c.width().unwrap_or(0) };
        if col + w > max_width && !current.is_empty() {
            segments.push(std::mem::take(&mut current));
            col = 0;
        }
        current.push(c);
        col += w;
    }
    if !current.is_empty() {
        segments.push(current);
    }
    if segments.is_empty() {
        segments.push(String::new());
    }
    segments
}

/// Extract a slice of a line by display columns [start_col, end_col).
pub fn safe_slice(line: &str, start_col: usize, end_col: usize) -> String {
    if start_col >= end_col {
        return String::new();
    }
    let mut result = String::new();
    let mut col = 0usize;

    for c in line.chars() {
        let w = if c == '\t' { 8 } else { c.width().unwrap_or(0) };
        if col >= end_col {
            break;
        }
        if col + w > start_col {
            result.push(c);
        }
        col += w;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_short_line() {
        let segs = segment_line("hello", 80);
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0], "hello");
    }

    #[test]
    fn segment_split() {
        let line = "abcdef";
        let segs = segment_line(line, 3);
        assert_eq!(segs.len(), 2);
        assert_eq!(segs[0], "abc");
        assert_eq!(segs[1], "def");
    }

    #[test]
    fn safe_slice_basic() {
        assert_eq!(safe_slice("hello world", 6, 11), "world");
    }

    #[test]
    fn is_wide() {
        assert!(is_wide_char('你'));
        assert!(!is_wide_char('a'));
    }
}
