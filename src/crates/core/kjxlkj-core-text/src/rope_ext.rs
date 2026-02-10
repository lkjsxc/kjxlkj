//! Rope extension utilities.

use crate::grapheme::{grapheme_count, grapheme_to_byte_offset};
use ropey::Rope;

/// Get line content as a string (without trailing newline).
pub fn line_content(rope: &Rope, line_idx: usize) -> String {
    if line_idx >= rope.len_lines() {
        return String::new();
    }
    let line = rope.line(line_idx);
    let s = line.to_string();
    s.trim_end_matches(&['\n', '\r'][..]).to_string()
}

/// Get the number of graphemes in a line.
pub fn line_grapheme_count(rope: &Rope, line_idx: usize) -> usize {
    grapheme_count(&line_content(rope, line_idx))
}

/// Convert line and grapheme offset to byte offset in rope.
pub fn position_to_byte(rope: &Rope, line: usize, grapheme: usize) -> usize {
    if line >= rope.len_lines() {
        return rope.len_bytes();
    }
    let line_start = rope.line_to_byte(line);
    let line_str = line_content(rope, line);
    let grapheme_byte = grapheme_to_byte_offset(&line_str, grapheme);
    line_start + grapheme_byte
}

/// Insert text at a position.
pub fn insert_at(rope: &mut Rope, line: usize, grapheme: usize, text: &str) {
    let byte_offset = position_to_byte(rope, line, grapheme);
    rope.insert(byte_offset, text);
}

/// Delete a range of text.
pub fn delete_range(
    rope: &mut Rope,
    start_line: usize,
    start_grapheme: usize,
    end_line: usize,
    end_grapheme: usize,
) {
    let start = position_to_byte(rope, start_line, start_grapheme);
    let end = position_to_byte(rope, end_line, end_grapheme);
    if start < end && end <= rope.len_bytes() {
        rope.remove(start..end);
    }
}

/// Create a rope from text content.
pub fn rope_from_str(s: &str) -> Rope {
    Rope::from_str(s)
}

/// Create an empty rope.
pub fn empty_rope() -> Rope {
    Rope::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_content() {
        let rope = Rope::from_str("hello\nworld\n");
        assert_eq!(line_content(&rope, 0), "hello");
        assert_eq!(line_content(&rope, 1), "world");
    }

    #[test]
    fn test_insert_at() {
        let mut rope = Rope::from_str("hello");
        insert_at(&mut rope, 0, 5, " world");
        assert_eq!(rope.to_string(), "hello world");
    }

    #[test]
    fn test_delete_range() {
        let mut rope = Rope::from_str("hello world");
        delete_range(&mut rope, 0, 5, 0, 11);
        assert_eq!(rope.to_string(), "hello");
    }
}
