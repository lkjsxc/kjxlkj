//! Text statistics functions.
//!
//! Functions for counting words, characters, lines, and bytes.

/// Counts words in text.
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Counts characters in text.
pub fn char_count(text: &str) -> usize {
    text.chars().count()
}

/// Counts bytes in text.
pub fn byte_count(text: &str) -> usize {
    text.len()
}

/// Counts lines in text.
pub fn line_count(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.lines().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world foo"), 3);
    }

    #[test]
    fn test_word_count_empty() {
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn test_char_count() {
        assert_eq!(char_count("hello"), 5);
    }

    #[test]
    fn test_char_count_unicode() {
        assert_eq!(char_count("日本語"), 3);
    }

    #[test]
    fn test_byte_count() {
        assert_eq!(byte_count("hello"), 5);
    }

    #[test]
    fn test_line_count() {
        assert_eq!(line_count("a\nb\nc"), 3);
    }

    #[test]
    fn test_line_count_empty() {
        assert_eq!(line_count(""), 0);
    }
}
