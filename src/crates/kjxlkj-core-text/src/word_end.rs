//! Word end motion utilities.

use crate::char_class::CharClass;

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
    fn test_word_end() {
        let text = "hello world";
        assert_eq!(word_end(text, 0), Some(4));
    }

    #[test]
    fn test_word_end_punct() {
        let text = "foo.bar";
        assert_eq!(word_end(text, 0), Some(2));
    }

    #[test]
    fn test_bigword_end() {
        let text = "foo.bar baz";
        assert_eq!(bigword_end(text, 0), Some(6));
    }

    #[test]
    fn test_word_end_at_end() {
        let text = "hello";
        assert_eq!(word_end(text, 4), None);
    }
}
