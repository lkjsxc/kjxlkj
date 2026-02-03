//! Tests for text module.

use super::*;

mod grapheme_tests {
    use super::*;

    #[test]
    fn test_grapheme_count_ascii() {
        assert_eq!(grapheme_count("hello"), 5);
        assert_eq!(grapheme_count(""), 0);
    }

    #[test]
    fn test_grapheme_count_unicode() {
        assert_eq!(grapheme_count("héllo"), 5);
        assert_eq!(grapheme_count("日本語"), 3);
        // Emoji with ZWJ
        assert_eq!(grapheme_count("👨‍👩‍👧"), 1);
    }

    #[test]
    fn test_grapheme_to_byte_offset() {
        assert_eq!(grapheme_to_byte_offset("hello", 0), 0);
        assert_eq!(grapheme_to_byte_offset("hello", 2), 2);
        assert_eq!(grapheme_to_byte_offset("hello", 5), 5);

        // Unicode
        assert_eq!(grapheme_to_byte_offset("héllo", 1), 1);
        assert_eq!(grapheme_to_byte_offset("héllo", 2), 3); // é is 2 bytes
    }

    #[test]
    fn test_word_boundaries() {
        let s = "hello world";
        assert_eq!(word_start(s, 3), 0);
        assert_eq!(word_end(s, 0), 5);
        assert_eq!(next_word_start(s, 0), 6);
    }

    #[test]
    fn test_grapheme_slice() {
        assert_eq!(grapheme_slice("hello", 1, 4), "ell");
        assert_eq!(grapheme_slice("日本語", 1, 2), "本");
    }
}

mod buffer_tests {
    use super::*;

    #[test]
    fn test_buffer_new() {
        let buf = TextBuffer::new();
        assert_eq!(buf.line_count(), 1);
        assert!(!buf.is_modified());
    }

    #[test]
    fn test_buffer_from_text() {
        let buf = TextBuffer::from_text("hello\nworld");
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0), Some("hello".to_string()));
        assert_eq!(buf.line(1), Some("world".to_string()));
    }

    #[test]
    fn test_buffer_insert() {
        let mut buf = TextBuffer::from_text("hello");
        buf.insert(kjxlkj_core_types::Position::new(0, 5), " world")
            .unwrap();
        assert_eq!(buf.line(0), Some("hello world".to_string()));
        assert!(buf.is_modified());
    }

    #[test]
    fn test_buffer_delete_line() {
        let mut buf = TextBuffer::from_text("line1\nline2\nline3");
        let deleted = buf.delete_line(1).unwrap();
        assert!(deleted.starts_with("line2"));
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(1), Some("line3".to_string()));
    }

    #[test]
    fn test_buffer_line_len() {
        let buf = TextBuffer::from_text("hello\n日本語");
        assert_eq!(buf.line_len(0), 5);
        assert_eq!(buf.line_len(1), 3);
    }

    #[test]
    fn test_buffer_version_increments() {
        let mut buf = TextBuffer::new();
        let v0 = buf.version();
        buf.insert(kjxlkj_core_types::Position::new(0, 0), "x")
            .unwrap();
        let v1 = buf.version();
        assert!(v1 > v0);
    }
}
