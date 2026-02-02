//! Word motion utilities.
//!
//! Helper functions for word-based cursor movement (w, W, b, B, e, E).

pub use crate::char_class::CharClass;

/// Finds the next word start position (w motion).
pub fn next_word_start(text: &str, pos: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    if pos >= chars.len() {
        return None;
    }

    let mut i = pos;

    // Skip current word/punct class.
    let current_class = CharClass::classify(chars[i]);
    if current_class != CharClass::Whitespace {
        while i < chars.len() && CharClass::classify(chars[i]) == current_class {
            i += 1;
        }
    }

    // Skip whitespace.
    while i < chars.len() && CharClass::classify(chars[i]) == CharClass::Whitespace {
        i += 1;
    }

    if i < chars.len() { Some(i) } else { None }
}

/// Finds the next WORD start position (W motion).
pub fn next_bigword_start(text: &str, pos: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    if pos >= chars.len() {
        return None;
    }

    let mut i = pos;

    // Skip non-whitespace.
    while i < chars.len() && !chars[i].is_whitespace() {
        i += 1;
    }

    // Skip whitespace.
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }

    if i < chars.len() { Some(i) } else { None }
}

/// Finds the previous word start position (b motion).
pub fn prev_word_start(text: &str, pos: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    if pos == 0 || chars.is_empty() {
        return None;
    }

    let mut i = pos.min(chars.len()) - 1;

    // Skip whitespace before cursor.
    while i > 0 && CharClass::classify(chars[i]) == CharClass::Whitespace {
        i -= 1;
    }

    // Determine class of word we're in.
    let target_class = CharClass::classify(chars[i]);

    // Move to start of this word.
    while i > 0 && CharClass::classify(chars[i - 1]) == target_class {
        i -= 1;
    }

    Some(i)
}

/// Finds the previous WORD start position (B motion).
pub fn prev_bigword_start(text: &str, pos: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    if pos == 0 || chars.is_empty() {
        return None;
    }

    let mut i = pos.min(chars.len()) - 1;

    // Skip whitespace.
    while i > 0 && chars[i].is_whitespace() {
        i -= 1;
    }

    // Move to start of WORD.
    while i > 0 && !chars[i - 1].is_whitespace() {
        i -= 1;
    }

    Some(i)
}

/// Finds the word end position (e motion).
pub fn word_end(text: &str, pos: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    if pos >= chars.len() {
        return None;
    }

    let mut i = pos + 1;
    if i >= chars.len() {
        return None;
    }

    // Skip whitespace.
    while i < chars.len() && CharClass::classify(chars[i]) == CharClass::Whitespace {
        i += 1;
    }

    if i >= chars.len() {
        return None;
    }

    // Determine class.
    let target_class = CharClass::classify(chars[i]);

    // Move to end of word.
    while i + 1 < chars.len() && CharClass::classify(chars[i + 1]) == target_class {
        i += 1;
    }

    Some(i)
}

/// Finds the WORD end position (E motion).
pub fn bigword_end(text: &str, pos: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    if pos >= chars.len() {
        return None;
    }

    let mut i = pos + 1;
    if i >= chars.len() {
        return None;
    }

    // Skip whitespace.
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }

    if i >= chars.len() {
        return None;
    }

    // Move to end of WORD.
    while i + 1 < chars.len() && !chars[i + 1].is_whitespace() {
        i += 1;
    }

    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_class_word() {
        assert_eq!(CharClass::classify('a'), CharClass::Word);
        assert_eq!(CharClass::classify('Z'), CharClass::Word);
        assert_eq!(CharClass::classify('_'), CharClass::Word);
    }

    #[test]
    fn test_char_class_punct() {
        assert_eq!(CharClass::classify('.'), CharClass::Punctuation);
        assert_eq!(CharClass::classify('!'), CharClass::Punctuation);
    }

    #[test]
    fn test_char_class_whitespace() {
        assert_eq!(CharClass::classify(' '), CharClass::Whitespace);
        assert_eq!(CharClass::classify('\t'), CharClass::Whitespace);
    }

    #[test]
    fn test_next_word_start() {
        let text = "hello world";
        assert_eq!(next_word_start(text, 0), Some(6));
    }

    #[test]
    fn test_next_word_start_punct() {
        let text = "foo.bar";
        assert_eq!(next_word_start(text, 0), Some(3));
        assert_eq!(next_word_start(text, 3), Some(4));
    }

    #[test]
    fn test_prev_word_start() {
        let text = "hello world";
        assert_eq!(prev_word_start(text, 8), Some(6));
        assert_eq!(prev_word_start(text, 6), Some(0));
    }

    #[test]
    fn test_next_bigword_start() {
        let text = "foo.bar baz";
        assert_eq!(next_bigword_start(text, 0), Some(8));
    }

    #[test]
    fn test_word_end() {
        let text = "hello world";
        assert_eq!(word_end(text, 0), Some(4));
    }

    #[test]
    fn test_bigword_end() {
        let text = "foo.bar baz";
        assert_eq!(bigword_end(text, 0), Some(6));
    }
}
