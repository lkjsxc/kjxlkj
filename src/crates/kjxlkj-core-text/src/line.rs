//! Line-related utilities.

/// Get the end column of a line (last valid cursor position).
pub fn line_end_col(line: &str, for_insert: bool) -> usize {
    let len = line.chars().count();
    if for_insert || len == 0 {
        len
    } else {
        len.saturating_sub(1)
    }
}

/// Get the length of a line in characters.
pub fn line_len(line: &str) -> usize {
    line.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_end_col() {
        assert_eq!(line_end_col("hello", false), 4);
        assert_eq!(line_end_col("hello", true), 5);
        assert_eq!(line_end_col("", false), 0);
        assert_eq!(line_end_col("", true), 0);
    }

    #[test]
    fn test_line_len() {
        assert_eq!(line_len("hello"), 5);
        assert_eq!(line_len(""), 0);
        assert_eq!(line_len("中文"), 2);
    }

    #[test]
    fn test_single_char_line() {
        assert_eq!(line_end_col("a", false), 0);
        assert_eq!(line_end_col("a", true), 1);
    }

    #[test]
    fn test_unicode_line_len() {
        assert_eq!(line_len("🎉"), 1);
        assert_eq!(line_len("hello🎉"), 6);
    }
}
