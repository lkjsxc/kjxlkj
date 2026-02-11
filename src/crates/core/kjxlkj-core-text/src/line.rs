//! Line-level utilities for rope content.

use ropey::Rope;

/// Return the number of lines in a rope.
pub fn line_count(rope: &Rope) -> usize {
    rope.len_lines()
}

/// Return the content of a specific line (0-based).
///
/// Returns `None` if out of bounds.
pub fn line_content(rope: &Rope, line: usize) -> Option<String> {
    if line >= rope.len_lines() {
        return None;
    }
    let slice = rope.line(line);
    Some(slice.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_count_basic() {
        let r = Rope::from_str("a\nb\nc");
        assert_eq!(line_count(&r), 3);
    }

    #[test]
    fn line_content_basic() {
        let r = Rope::from_str("hello\nworld");
        assert_eq!(line_content(&r, 0), Some("hello\n".to_string()));
        assert_eq!(line_content(&r, 1), Some("world".to_string()));
        assert_eq!(line_content(&r, 2), None);
    }
}
