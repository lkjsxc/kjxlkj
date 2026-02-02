//! Unit tests for text buffer.

use super::*;
use kjxlkj_core_types::{Position, Range};

#[cfg(test)]
mod text_buffer_tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let buf = TextBuffer::new();
        assert_eq!(buf.char_count(), 0);
        assert_eq!(buf.line_count(), 1); // Empty buffer has 1 line
    }

    #[test]
    fn test_from_str() {
        let buf = TextBuffer::from_str("hello\nworld");
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0), "hello");
        assert_eq!(buf.line(1), "world");
    }

    #[test]
    fn test_insert() {
        let mut buf = TextBuffer::new();
        buf.insert(Position::origin(), "hello");
        assert_eq!(buf.to_string(), "hello");
    }

    #[test]
    fn test_insert_multiline() {
        let mut buf = TextBuffer::new();
        buf.insert(Position::origin(), "line1\nline2\nline3");
        assert_eq!(buf.line_count(), 3);
        assert_eq!(buf.line(0), "line1");
        assert_eq!(buf.line(2), "line3");
    }

    #[test]
    fn test_delete() {
        let mut buf = TextBuffer::from_str("hello world");
        buf.delete(Range::from_coords(0, 5, 0, 11));
        assert_eq!(buf.to_string(), "hello");
    }

    #[test]
    fn test_slice() {
        let buf = TextBuffer::from_str("hello world");
        let slice = buf.slice(Range::from_coords(0, 0, 0, 5));
        assert_eq!(slice, "hello");
    }

    #[test]
    fn test_version_increments() {
        let mut buf = TextBuffer::new();
        let v1 = buf.version();
        buf.insert(Position::origin(), "a");
        let v2 = buf.version();
        assert!(v2.raw() > v1.raw());
    }

    #[test]
    fn test_position_to_char_idx() {
        let buf = TextBuffer::from_str("abc\ndef\nghi");
        assert_eq!(buf.position_to_char_idx(Position::new(0, 0)), 0);
        assert_eq!(buf.position_to_char_idx(Position::new(0, 2)), 2);
        assert_eq!(buf.position_to_char_idx(Position::new(1, 0)), 4);
        assert_eq!(buf.position_to_char_idx(Position::new(2, 2)), 10);
    }

    #[test]
    fn test_char_idx_to_position() {
        let buf = TextBuffer::from_str("abc\ndef");
        assert_eq!(buf.char_idx_to_position(0), Position::new(0, 0));
        assert_eq!(buf.char_idx_to_position(2), Position::new(0, 2));
        assert_eq!(buf.char_idx_to_position(4), Position::new(1, 0));
    }
}

#[cfg(test)]
mod grapheme_tests {
    use super::*;

    #[test]
    fn test_grapheme_count() {
        assert_eq!(grapheme_count("hello"), 5);
        assert_eq!(grapheme_count("héllo"), 5);
    }

    #[test]
    fn test_grapheme_width() {
        assert_eq!(grapheme_width("a"), 1);
        assert_eq!(grapheme_width("世"), 2);
    }

    #[test]
    fn test_grapheme_iter() {
        let graphemes: Vec<_> = GraphemeIter::new("abc").collect();
        assert_eq!(graphemes, vec!["a", "b", "c"]);
    }
}

#[cfg(test)]
mod rope_ext_tests {
    use super::*;
    use ropey::Rope;

    #[test]
    fn test_line_content() {
        let rope = Rope::from_str("hello\nworld\n");
        assert_eq!(rope.line_content(0), "hello");
        assert_eq!(rope.line_content(1), "world");
    }

    #[test]
    fn test_line_grapheme_count() {
        let rope = Rope::from_str("hello\nworld");
        assert_eq!(rope.line_grapheme_count(0), 5);
        assert_eq!(rope.line_grapheme_count(1), 5);
    }
}
