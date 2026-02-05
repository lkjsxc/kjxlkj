//! Word boundary detection.

/// The kind of word for movement purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordKind {
    /// A word consisting of word characters (alphanumeric + underscore).
    Word,
    /// A WORD consisting of any non-whitespace characters.
    WORD,
}

/// Character classification for word boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharClass {
    /// Whitespace.
    Whitespace,
    /// Word character (alphanumeric or underscore).
    Word,
    /// Punctuation/other.
    Punct,
}

fn classify_char(c: char) -> CharClass {
    if c.is_whitespace() {
        CharClass::Whitespace
    } else if c.is_alphanumeric() || c == '_' {
        CharClass::Word
    } else {
        CharClass::Punct
    }
}

/// Find the next word boundary position in a string.
pub fn find_word_boundary(s: &str, start: usize, forward: bool, kind: WordKind) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    if len == 0 {
        return 0;
    }

    let start = start.min(len.saturating_sub(1));

    match kind {
        WordKind::WORD => find_word_boundary_impl(&chars, start, forward, true),
        WordKind::Word => find_word_boundary_impl(&chars, start, forward, false),
    }
}

fn find_word_boundary_impl(chars: &[char], start: usize, forward: bool, is_word: bool) -> usize {
    let len = chars.len();

    if forward {
        let mut pos = start;

        // Skip current word/non-whitespace
        if is_word {
            while pos < len && !chars[pos].is_whitespace() {
                pos += 1;
            }
        } else {
            let start_class = classify_char(chars[pos]);
            while pos < len && classify_char(chars[pos]) == start_class {
                pos += 1;
            }
        }

        // Skip whitespace
        while pos < len && chars[pos].is_whitespace() {
            pos += 1;
        }

        pos.min(len)
    } else {
        if start == 0 {
            return 0;
        }

        let mut pos = start.saturating_sub(1);

        // Skip whitespace
        while pos > 0 && chars[pos].is_whitespace() {
            pos -= 1;
        }

        if is_word {
            // Skip non-whitespace
            while pos > 0 && !chars[pos - 1].is_whitespace() {
                pos -= 1;
            }
        } else {
            // Skip same class
            let end_class = classify_char(chars[pos]);
            while pos > 0 && classify_char(chars[pos - 1]) == end_class {
                pos -= 1;
            }
        }

        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word_boundary_forward() {
        let s = "hello world";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 6);
        assert_eq!(find_word_boundary(s, 6, true, WordKind::Word), 11);
    }

    #[test]
    fn test_find_word_boundary_backward() {
        let s = "hello world";
        assert_eq!(find_word_boundary(s, 10, false, WordKind::Word), 6);
        assert_eq!(find_word_boundary(s, 6, false, WordKind::Word), 0);
    }

    #[test]
    fn test_find_word_boundary_with_punct() {
        let s = "hello.world";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 5);
        assert_eq!(find_word_boundary(s, 0, true, WordKind::WORD), 11);
    }

    #[test]
    fn test_word_kind_equality() {
        assert_eq!(WordKind::Word, WordKind::Word);
        assert_ne!(WordKind::Word, WordKind::WORD);
    }

    #[test]
    fn test_classify_char_whitespace() {
        assert_eq!(classify_char(' '), CharClass::Whitespace);
        assert_eq!(classify_char('\t'), CharClass::Whitespace);
        assert_eq!(classify_char('\n'), CharClass::Whitespace);
    }

    #[test]
    fn test_classify_char_word() {
        assert_eq!(classify_char('a'), CharClass::Word);
        assert_eq!(classify_char('Z'), CharClass::Word);
        assert_eq!(classify_char('_'), CharClass::Word);
        assert_eq!(classify_char('5'), CharClass::Word);
    }

    #[test]
    fn test_classify_char_punct() {
        assert_eq!(classify_char('.'), CharClass::Punct);
        assert_eq!(classify_char('-'), CharClass::Punct);
        assert_eq!(classify_char('!'), CharClass::Punct);
    }

    #[test]
    fn test_empty_string() {
        let s = "";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 0);
        assert_eq!(find_word_boundary(s, 0, false, WordKind::Word), 0);
    }

    #[test]
    fn test_single_word() {
        let s = "hello";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 5);
        assert_eq!(find_word_boundary(s, 4, false, WordKind::Word), 0);
    }

    #[test]
    fn test_multiple_spaces() {
        let s = "hello    world";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 9);
    }

    #[test]
    fn test_word_at_start() {
        let s = "hello";
        assert_eq!(find_word_boundary(s, 0, false, WordKind::Word), 0);
    }

    #[test]
    fn test_word_at_end() {
        let s = "hello";
        assert_eq!(find_word_boundary(s, 4, true, WordKind::Word), 5);
    }

    #[test]
    fn test_WORD_with_punct() {
        let s = "hello-world test";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::WORD), 12);
    }

    #[test]
    fn test_word_boundary_at_boundary() {
        let s = "hello world";
        assert_eq!(find_word_boundary(s, 5, true, WordKind::Word), 6);
    }

    #[test]
    fn test_backward_multiple_words() {
        let s = "one two three";
        assert_eq!(find_word_boundary(s, 12, false, WordKind::Word), 8);
        assert_eq!(find_word_boundary(s, 7, false, WordKind::Word), 4);
        assert_eq!(find_word_boundary(s, 3, false, WordKind::Word), 0);
    }

    #[test]
    fn test_word_kind_debug() {
        let kind = WordKind::Word;
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Word"));
    }

    #[test]
    fn test_word_kind_clone() {
        let kind = WordKind::WORD;
        let cloned = kind.clone();
        assert_eq!(kind, cloned);
    }

    #[test]
    fn test_char_class_debug() {
        let class = CharClass::Whitespace;
        let debug = format!("{:?}", class);
        assert!(debug.contains("Whitespace"));
    }

    #[test]
    fn test_underscore_is_word() {
        let s = "hello_world";
        // Underscore should be part of word
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 11);
    }

    #[test]
    fn test_hyphen_word_boundary() {
        let s = "hello-world";
        // Hyphen is punct, so should stop at it
        let boundary = find_word_boundary(s, 0, true, WordKind::Word);
        assert!(boundary <= 6);
    }

    #[test]
    fn test_classify_unicode_letter() {
        assert_eq!(classify_char('α'), CharClass::Word);
        assert_eq!(classify_char('日'), CharClass::Word);
    }

    #[test]
    fn test_only_whitespace() {
        let s = "   ";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 3);
    }

    #[test]
    fn test_word_boundary_tabs() {
        let s = "hello\tworld";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 6);
    }

    #[test]
    fn test_word_boundary_punct_only() {
        let s = "...";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 3);
    }

    #[test]
    fn test_WORD_backward() {
        let s = "hello-world test";
        assert_eq!(find_word_boundary(s, 15, false, WordKind::WORD), 12);
    }

    #[test]
    fn test_mixed_punct_letters() {
        let s = "a.b.c";
        let pos = find_word_boundary(s, 0, true, WordKind::Word);
        assert!(pos <= 2);
    }

    #[test]
    fn test_word_kind_copy() {
        let kind = WordKind::Word;
        let copied = kind;
        assert_eq!(kind, copied);
    }

    #[test]
    fn test_word_kind_eq() {
        assert_eq!(WordKind::Word, WordKind::Word);
        assert_eq!(WordKind::WORD, WordKind::WORD);
        assert_ne!(WordKind::Word, WordKind::WORD);
    }

    #[test]
    fn test_classify_digit() {
        assert_eq!(classify_char('5'), CharClass::Word);
        assert_eq!(classify_char('0'), CharClass::Word);
    }

    #[test]
    fn test_classify_newline() {
        assert_eq!(classify_char('\n'), CharClass::Whitespace);
    }

    #[test]
    fn test_classify_tab() {
        assert_eq!(classify_char('\t'), CharClass::Whitespace);
    }

    #[test]
    fn test_classify_space() {
        assert_eq!(classify_char(' '), CharClass::Whitespace);
    }

    #[test]
    fn test_classify_at_sign() {
        assert_eq!(classify_char('@'), CharClass::Punct);
    }

    #[test]
    fn test_classify_hash() {
        assert_eq!(classify_char('#'), CharClass::Punct);
    }

    #[test]
    fn test_word_boundary_newline() {
        let s = "hello\nworld";
        assert_eq!(find_word_boundary(s, 0, true, WordKind::Word), 6);
    }

    #[test]
    fn test_word_boundary_long_text() {
        let s = "the quick brown fox jumps over the lazy dog";
        let pos = find_word_boundary(s, 0, true, WordKind::Word);
        assert_eq!(pos, 4);
    }
}
