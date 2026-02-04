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

    #[test]
    fn test_multibyte_line_end() {
        assert_eq!(line_end_col("中文字", false), 2);
        assert_eq!(line_end_col("中文字", true), 3);
    }

    #[test]
    fn test_mixed_content_len() {
        assert_eq!(line_len("abc中"), 4);
    }

    #[test]
    fn test_whitespace_only_line() {
        assert_eq!(line_len("   "), 3);
        assert_eq!(line_end_col("   ", false), 2);
    }

    #[test]
    fn test_tab_line_len() {
        assert_eq!(line_len("\t"), 1);
    }

    #[test]
    fn test_newline_stripped() {
        // line_len doesn't count trailing newline
        assert_eq!(line_len("hello\n"), 6);
    }

    #[test]
    fn test_emoji_sequence() {
        assert_eq!(line_len("👨‍👩‍👧"), 5); // Multi-code-point emoji
    }

    #[test]
    fn test_line_end_insert_mode() {
        // Insert mode: can go past last char
        assert_eq!(line_end_col("test", true), 4);
    }

    #[test]
    fn test_line_end_normal_mode() {
        // Normal mode: stop at last char
        assert_eq!(line_end_col("test", false), 3);
    }

    #[test]
    fn test_line_len_combining_chars() {
        // e + combining accent
        assert_eq!(line_len("e\u{0301}"), 2);
    }

    #[test]
    fn test_single_space() {
        assert_eq!(line_len(" "), 1);
    }
}
