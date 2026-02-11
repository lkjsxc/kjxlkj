//! Search utility functions extracted from search.rs.

use regex::Regex;

/// Byte offset for the nth character in a string.
pub fn byte_offset(s: &str, n: usize) -> usize {
    s.char_indices().nth(n).map(|(i, _)| i).unwrap_or(s.len())
}

/// Character offset for a byte position.
pub fn char_offset(s: &str, byte_pos: usize) -> usize {
    s[..byte_pos].chars().count()
}

/// Find the start byte of the last match in a string.
pub fn last_match(re: &Regex, s: &str) -> Option<usize> {
    let mut last = None;
    for m in re.find_iter(s) { last = Some(m.start()); }
    last
}

/// Find the last match whose char-start is strictly < col.
pub fn last_match_before(re: &Regex, s: &str, col: usize) -> Option<usize> {
    let col_byte = byte_offset(s, col);
    let mut last = None;
    for m in re.find_iter(s) {
        if m.start() < col_byte { last = Some(m.start()); }
    }
    last
}

/// Extract the word under the cursor from a line.
/// Returns the word text, or None if cursor is not on a word.
pub fn word_at(line: &str, col: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();
    if col >= chars.len() { return None; }
    if !is_word_char(chars[col]) { return None; }
    let mut start = col;
    while start > 0 && is_word_char(chars[start - 1]) { start -= 1; }
    let mut end = col;
    while end + 1 < chars.len() && is_word_char(chars[end + 1]) { end += 1; }
    Some(chars[start..=end].iter().collect())
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_at_middle() {
        assert_eq!(word_at("hello world foo", 6), Some("world".to_string()));
    }

    #[test]
    fn word_at_start() {
        assert_eq!(word_at("hello world", 0), Some("hello".to_string()));
    }

    #[test]
    fn word_at_non_word_char() {
        assert_eq!(word_at("hello world", 5), None);
    }

    #[test]
    fn word_at_end_of_line() {
        assert_eq!(word_at("hello", 4), Some("hello".to_string()));
    }

    #[test]
    fn word_at_empty() {
        assert_eq!(word_at("", 0), None);
    }

    #[test]
    fn word_at_underscore() {
        assert_eq!(word_at("foo_bar baz", 3), Some("foo_bar".to_string()));
    }
}
