//! Grapheme and character utilities.

use unicode_width::UnicodeWidthStr;

/// Get the display width of a grapheme cluster.
pub fn grapheme_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Check if a character is a word character (alphanumeric or underscore).
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_width() {
        assert_eq!(grapheme_width("a"), 1);
        assert_eq!(grapheme_width("中"), 2);
    }

    #[test]
    fn test_is_word_char() {
        assert!(is_word_char('a'));
        assert!(is_word_char('Z'));
        assert!(is_word_char('5'));
        assert!(is_word_char('_'));
        assert!(!is_word_char(' '));
        assert!(!is_word_char('.'));
    }
}
