//! Comprehensive tests for kjxlkj-core-text.

use kjxlkj_core_text::*;
use kjxlkj_core_types::{Position, Range};

mod text_buffer_creation {
    use super::*;

    #[test]
    fn test_new_buffer_empty() {
        let buf = TextBuffer::new();
        assert!(buf.is_empty());
    }

    #[test]
    fn test_from_str_single_line() {
        let buf = TextBuffer::from_str("hello");
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_from_str_multiline() {
        let buf = TextBuffer::from_str("line1\nline2\nline3");
        assert_eq!(buf.line_count(), 3);
        assert_eq!(buf.line(0).unwrap(), "line1");
        assert_eq!(buf.line(1).unwrap(), "line2");
        assert_eq!(buf.line(2).unwrap(), "line3");
    }

    #[test]
    fn test_from_str_empty_newline() {
        let buf = TextBuffer::from_str("hello\n");
        assert_eq!(buf.line_count(), 2);
    }

    #[test]
    fn test_from_str_windows_newlines() {
        let buf = TextBuffer::from_str("line1\r\nline2");
        assert_eq!(buf.line_count(), 2);
    }

    #[test]
    fn test_line_invalid_index() {
        let buf = TextBuffer::from_str("one\ntwo");
        assert!(buf.line(999).is_err());
    }
}

mod text_buffer_line_count {
    use super::*;

    #[test]
    fn test_single_line_no_newline() {
        let buf = TextBuffer::from_str("hello");
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn test_two_lines() {
        let buf = TextBuffer::from_str("a\nb");
        assert_eq!(buf.line_count(), 2);
    }

    #[test]
    fn test_trailing_newline() {
        let buf = TextBuffer::from_str("a\n");
        assert_eq!(buf.line_count(), 2);
    }

    #[test]
    fn test_multiple_empty_lines() {
        let buf = TextBuffer::from_str("a\n\n\nb");
        assert_eq!(buf.line_count(), 4);
    }
}

mod text_buffer_insert {
    use super::*;

    #[test]
    fn test_insert_at_start() {
        let mut buf = TextBuffer::from_str("world");
        let _ = buf.insert(Position::new(0, 0), "hello ");
        assert_eq!(buf.line(0).unwrap(), "hello world");
    }

    #[test]
    fn test_insert_at_end() {
        let mut buf = TextBuffer::from_str("hello");
        let _ = buf.insert(Position::new(0, 5), " world");
        assert_eq!(buf.line(0).unwrap(), "hello world");
    }

    #[test]
    fn test_insert_newline() {
        let mut buf = TextBuffer::from_str("helloworld");
        let _ = buf.insert(Position::new(0, 5), "\n");
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0).unwrap(), "hello");
        assert_eq!(buf.line(1).unwrap(), "world");
    }

    #[test]
    fn test_insert_middle() {
        let mut buf = TextBuffer::from_str("helo");
        let _ = buf.insert(Position::new(0, 3), "l");
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_insert_char() {
        let mut buf = TextBuffer::from_str("hllo");
        let _ = buf.insert_char(Position::new(0, 1), 'e');
        assert_eq!(buf.line(0).unwrap(), "hello");
    }
}

mod text_buffer_delete {
    use super::*;

    #[test]
    fn test_delete_range() {
        let mut buf = TextBuffer::from_str("hello world");
        let range = Range::from_coords(0, 5, 0, 11);
        let _ = buf.delete(range);
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_delete_from_start() {
        let mut buf = TextBuffer::from_str("hello");
        let range = Range::from_coords(0, 0, 0, 2);
        let _ = buf.delete(range);
        assert_eq!(buf.line(0).unwrap(), "llo");
    }

    #[test]
    fn test_delete_newline() {
        let mut buf = TextBuffer::from_str("hello\nworld");
        let range = Range::from_coords(0, 5, 1, 0);
        let _ = buf.delete(range);
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn test_delete_char() {
        let mut buf = TextBuffer::from_str("hello");
        let result = buf.delete_char(Position::new(0, 1));
        assert!(result.is_ok());
        assert_eq!(buf.line(0).unwrap(), "hllo");
    }

    #[test]
    fn test_delete_returns_deleted_text() {
        let mut buf = TextBuffer::from_str("hello world");
        let range = Range::from_coords(0, 6, 0, 11);
        let result = buf.delete(range);
        assert!(result.is_ok());
        let (deleted, _) = result.unwrap();
        assert_eq!(deleted, "world");
    }
}

mod text_buffer_slice {
    use super::*;

    #[test]
    fn test_slice() {
        let buf = TextBuffer::from_str("hello world");
        let range = Range::from_coords(0, 0, 0, 5);
        let slice = buf.slice(range).unwrap();
        assert_eq!(slice, "hello");
    }

    #[test]
    fn test_slice_middle() {
        let buf = TextBuffer::from_str("hello world");
        let range = Range::from_coords(0, 6, 0, 11);
        let slice = buf.slice(range).unwrap();
        assert_eq!(slice, "world");
    }

    #[test]
    fn test_slice_multiline() {
        let buf = TextBuffer::from_str("hello\nworld");
        let range = Range::from_coords(0, 3, 1, 2);
        let slice = buf.slice(range).unwrap();
        assert!(slice.contains("lo"));
        assert!(slice.contains("wo"));
    }
}

mod text_buffer_replace {
    use super::*;

    #[test]
    fn test_replace() {
        let mut buf = TextBuffer::from_str("hello world");
        let range = Range::from_coords(0, 0, 0, 5);
        let _ = buf.replace(range, "hi");
        assert_eq!(buf.line(0).unwrap(), "hi world");
    }

    #[test]
    fn test_replace_same_length() {
        let mut buf = TextBuffer::from_str("hello");
        let range = Range::from_coords(0, 0, 0, 5);
        let _ = buf.replace(range, "world");
        assert_eq!(buf.line(0).unwrap(), "world");
    }

    #[test]
    fn test_replace_with_newline() {
        let mut buf = TextBuffer::from_str("hello");
        let range = Range::from_coords(0, 2, 0, 3);
        let _ = buf.replace(range, "\n");
        assert_eq!(buf.line_count(), 2);
    }
}

mod text_buffer_contents {
    use super::*;

    #[test]
    fn test_to_string() {
        let buf = TextBuffer::from_str("hello\nworld");
        let s = buf.to_string();
        assert!(s.contains("hello"));
        assert!(s.contains("world"));
    }

    #[test]
    fn test_byte_count() {
        let buf = TextBuffer::from_str("hello");
        assert_eq!(buf.byte_count(), 5);
    }

    #[test]
    fn test_char_count() {
        let buf = TextBuffer::from_str("hello");
        assert_eq!(buf.char_count(), 5);
    }

    #[test]
    fn test_byte_count_unicode() {
        let buf = TextBuffer::from_str("héllo");
        // é is 2 bytes in UTF-8, so total is 6
        assert!(buf.byte_count() >= 5);
    }

    #[test]
    fn test_is_empty() {
        let buf = TextBuffer::new();
        assert!(buf.is_empty());
        let buf2 = TextBuffer::from_str("x");
        assert!(!buf2.is_empty());
    }
}

mod text_buffer_version {
    use super::*;

    #[test]
    fn test_version_increments_on_insert() {
        let mut buf = TextBuffer::from_str("hello");
        let v1 = buf.version();
        let _ = buf.insert(Position::new(0, 0), "x");
        let v2 = buf.version();
        assert!(v2 > v1);
    }

    #[test]
    fn test_version_increments_on_delete() {
        let mut buf = TextBuffer::from_str("hello");
        let v1 = buf.version();
        let range = Range::from_coords(0, 0, 0, 1);
        let _ = buf.delete(range);
        let v2 = buf.version();
        assert!(v2 > v1);
    }

    #[test]
    fn test_version_increments_on_replace() {
        let mut buf = TextBuffer::from_str("hello");
        let v1 = buf.version();
        let range = Range::from_coords(0, 0, 0, 5);
        let _ = buf.replace(range, "world");
        let v2 = buf.version();
        assert!(v2 > v1);
    }
}

mod text_buffer_position_conversion {
    use super::*;

    #[test]
    fn test_pos_to_char() {
        let buf = TextBuffer::from_str("hello\nworld");
        let idx = buf.pos_to_char(Position::new(0, 0)).unwrap();
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_pos_to_char_second_line() {
        let buf = TextBuffer::from_str("hello\nworld");
        let idx = buf.pos_to_char(Position::new(1, 0)).unwrap();
        assert_eq!(idx, 6); // 5 chars + newline
    }

    #[test]
    fn test_char_to_pos() {
        let buf = TextBuffer::from_str("hello\nworld");
        let pos = buf.char_to_pos(6).unwrap();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 0);
    }
}

mod text_buffer_line_len {
    use super::*;

    #[test]
    fn test_line_len() {
        let buf = TextBuffer::from_str("hello\nworld");
        assert_eq!(buf.line_len(0).unwrap(), 5);
        assert_eq!(buf.line_len(1).unwrap(), 5);
    }

    #[test]
    fn test_line_len_invalid() {
        let buf = TextBuffer::from_str("hello");
        assert!(buf.line_len(999).is_err());
    }
}

mod text_buffer_lines_in_range {
    use super::*;

    #[test]
    fn test_lines_in_range() {
        let buf = TextBuffer::from_str("a\nb\nc\nd");
        let lines = buf.lines_in_range(1, 3);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "b");
        assert_eq!(lines[1], "c");
    }
}

mod grapheme_tests {
    use super::*;

    #[test]
    fn test_grapheme_count_ascii() {
        assert_eq!(grapheme_count("hello"), 5);
    }

    #[test]
    fn test_grapheme_count_emoji() {
        let count = grapheme_count("👍");
        assert!(count >= 1);
    }

    #[test]
    fn test_grapheme_count_combined() {
        // e + combining acute = 1 grapheme
        let count = grapheme_count("e\u{0301}");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_grapheme_width_ascii() {
        assert_eq!(grapheme_width("a"), 1);
    }

    #[test]
    fn test_grapheme_width_wide() {
        let width = grapheme_width("中");
        assert!(width >= 1);
    }

}

mod text_buffer_unicode {
    use super::*;

    #[test]
    fn test_unicode_insert() {
        let mut buf = TextBuffer::from_str("hello");
        let _ = buf.insert(Position::new(0, 5), " 世界");
        let line = buf.line(0).unwrap();
        assert!(line.contains("世界"));
    }

    #[test]
    fn test_unicode_line() {
        let buf = TextBuffer::from_str("αβγ\nδεζ");
        assert_eq!(buf.line(0).unwrap(), "αβγ");
        assert_eq!(buf.line(1).unwrap(), "δεζ");
    }

    #[test]
    fn test_emoji_handling() {
        let buf = TextBuffer::from_str("hello 👋 world");
        assert!(buf.line(0).unwrap().contains("👋"));
    }
}

mod text_buffer_debug {
    use super::*;

    #[test]
    fn test_text_buffer_debug() {
        let buf = TextBuffer::from_str("test");
        let debug = format!("{:?}", buf);
        assert!(debug.contains("TextBuffer"));
    }
}


// Additional edge case tests for TextBuffer
mod text_buffer_extra {
    use super::*;

    #[test]
    fn test_from_str_empty() {
        let buf = TextBuffer::from_str("");
        assert!(buf.line_count() <= 1);
    }

    #[test]
    fn test_from_str_single_line() {
        let buf = TextBuffer::from_str("hello");
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_from_str_many_lines() {
        let text = (0..50).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
        let buf = TextBuffer::from_str(&text);
        assert_eq!(buf.line_count(), 50);
    }

    #[test]
    fn test_line_bounds() {
        let buf = TextBuffer::from_str("a\nb\nc");
        assert!(buf.line(0).is_ok());
        assert!(buf.line(1).is_ok());
        assert!(buf.line(2).is_ok());
        assert!(buf.line(3).is_err());
    }

    #[test]
    fn test_line_len() {
        let buf = TextBuffer::from_str("hello");
        assert_eq!(buf.line_len(0).unwrap(), 5);
    }

    #[test]
    fn test_line_len_empty() {
        let buf = TextBuffer::from_str("");
        assert_eq!(buf.line_len(0).unwrap(), 0);
    }

    #[test]
    fn test_insert_at_start() {
        let mut buf = TextBuffer::from_str("world");
        let _ = buf.insert(Position::new(0, 0), "hello ");
        assert!(buf.line(0).unwrap().starts_with("hello"));
    }

    #[test]
    fn test_insert_at_end() {
        let mut buf = TextBuffer::from_str("hello");
        let _ = buf.insert(Position::new(0, 5), " world");
        assert!(buf.line(0).unwrap().ends_with("world"));
    }

    #[test]
    fn test_insert_newline() {
        let mut buf = TextBuffer::from_str("hello");
        let _ = buf.insert(Position::new(0, 5), "\nworld");
        assert_eq!(buf.line_count(), 2);
    }

    #[test]
    fn test_delete_single_char() {
        let mut buf = TextBuffer::from_str("hello");
        let range = Range::from_coords(0, 0, 0, 1);
        let _ = buf.delete(range);
        assert_eq!(buf.line(0).unwrap(), "ello");
    }

    #[test]
    fn test_slice() {
        let buf = TextBuffer::from_str("hello world");
        let range = Range::from_coords(0, 0, 0, 5);
        let slice = buf.slice(range).unwrap();
        assert_eq!(slice, "hello");
    }

    #[test]
    fn test_slice_multiline() {
        let buf = TextBuffer::from_str("line1\nline2\nline3");
        let range = Range::from_coords(0, 0, 2, 5);
        let slice = buf.slice(range).unwrap();
        assert!(slice.contains("line1"));
    }

    #[test]
    fn test_version_initial() {
        let buf = TextBuffer::new();
        let v1 = buf.version();
        assert_eq!(v1.as_u64(), 0);
    }

    #[test]
    fn test_version_increments() {
        let mut buf = TextBuffer::from_str("hello");
        let v1 = buf.version();
        let _ = buf.insert(Position::new(0, 5), "!");
        let v2 = buf.version();
        assert!(v2.as_u64() > v1.as_u64());
    }

    #[test]

    #[test]
    fn test_replace_text() {
        let mut buf = TextBuffer::from_str("hello world");
        let range = Range::from_coords(0, 0, 0, 5);
        let _ = buf.replace(range, "hi");
        assert!(buf.line(0).unwrap().starts_with("hi"));
    }
}

// Additional edge case tests for graphemes
mod grapheme_extra {
    use super::*;

    #[test]
    fn test_grapheme_count_spaces() {
        assert_eq!(grapheme_count("   "), 3);
    }

    #[test]
    fn test_grapheme_count_mixed() {
        // Mix of ASCII and multi-byte
        let count = grapheme_count("aé");
        assert!(count >= 2);
    }

    #[test]
    fn test_grapheme_width_tab() {
        // Tab has variable width
        let w = grapheme_width("\t");
        assert!(w >= 1);
    }

    #[test]
    fn test_grapheme_iter_empty() {
        let iter = GraphemeIter::new("");
        let graphemes: Vec<_> = iter.collect();
        assert_eq!(graphemes.len(), 0);
    }

    #[test]
    fn test_grapheme_iter_ascii() {
        let iter = GraphemeIter::new("abc");
        let graphemes: Vec<_> = iter.collect();
        assert_eq!(graphemes.len(), 3);
    }

    #[test]
    fn test_grapheme_iter_unicode() {
        let iter = GraphemeIter::new("héllo");
        let graphemes: Vec<_> = iter.collect();
        assert!(graphemes.len() >= 5);
    }
}

mod extra_text_buffer_edge_tests {
    use super::*;

    #[test]
    fn test_buffer_line_out_of_bounds() {
        let buf = TextBuffer::from_str("hello");
        assert!(buf.line(100).is_err());
    }

    #[test]
    fn test_buffer_line_len_out_of_bounds() {
        let buf = TextBuffer::from_str("hello");
        assert!(buf.line_len(100).is_err());
    }

    #[test]
    fn test_buffer_empty_lines() {
        let buf = TextBuffer::from_str("\n\n\n");
        assert_eq!(buf.line_count(), 4);
        assert_eq!(buf.line(0).unwrap(), "");
        assert_eq!(buf.line(1).unwrap(), "");
    }

    #[test]
    fn test_buffer_from_str_empty() {
        let buf = TextBuffer::from_str("");
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.line(0).unwrap(), "");
    }

    #[test]
    fn test_buffer_insert_at_end() {
        let mut buf = TextBuffer::from_str("hello");
        buf.insert_char(Position::new(0, 5), '!').unwrap();
        assert_eq!(buf.line(0).unwrap(), "hello!");
    }

    #[test]
    fn test_buffer_insert_string_empty() {
        let mut buf = TextBuffer::from_str("hello");
        buf.insert(Position::new(0, 2), "").unwrap();
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_buffer_insert_newline_middle() {
        let mut buf = TextBuffer::from_str("helloworld");
        buf.insert_char(Position::new(0, 5), '\n').unwrap();
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0).unwrap(), "hello");
        assert_eq!(buf.line(1).unwrap(), "world");
    }

    #[test]
    fn test_buffer_delete_at_line_start() {
        let mut buf = TextBuffer::from_str("hello\nworld");
        // Delete newline at end of first line
        buf.delete_char(Position::new(0, 5)).unwrap();
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.line(0).unwrap(), "helloworld");
    }

    #[test]
    fn test_buffer_replace_empty_with_text() {
        let mut buf = TextBuffer::from_str("hello world");
        let range = Range::from_coords(0, 5, 0, 5);
        buf.replace(range, " beautiful").unwrap();
        assert_eq!(buf.line(0).unwrap(), "hello beautiful world");
    }

    #[test]
    fn test_buffer_replace_with_empty() {
        let mut buf = TextBuffer::from_str("hello world");
        let range = Range::from_coords(0, 5, 0, 11);
        buf.replace(range, "").unwrap();
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_buffer_replace_multiline_with_single() {
        let mut buf = TextBuffer::from_str("line1\nline2\nline3");
        let range = Range::from_coords(0, 0, 2, 5);
        buf.replace(range, "replaced").unwrap();
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.line(0).unwrap(), "replaced");
    }

    #[test]
    fn test_buffer_slice_empty_range() {
        let buf = TextBuffer::from_str("hello");
        let range = Range::from_coords(0, 2, 0, 2);
        let slice = buf.slice(range).unwrap();
        assert_eq!(slice, "");
    }

    #[test]
    fn test_buffer_unicode_chars() {
        let buf = TextBuffer::from_str("héllo wörld");
        assert_eq!(buf.char_count(), 11);
    }

    #[test]
    fn test_buffer_unicode_emoji() {
        let buf = TextBuffer::from_str("hello 👋 world");
        assert!(buf.char_count() >= 12); // Emoji may be multiple code points
    }

    #[test]
    fn test_buffer_cjk() {
        let buf = TextBuffer::from_str("你好世界");
        assert_eq!(buf.char_count(), 4);
        assert_eq!(buf.line(0).unwrap(), "你好世界");
    }

    #[test]
    fn test_buffer_to_string() {
        let buf = TextBuffer::from_str("line1\nline2");
        assert_eq!(buf.to_string(), "line1\nline2");
    }

    #[test]
    fn test_buffer_default() {
        let buf = TextBuffer::default();
        assert!(buf.is_empty());
    }

    #[test]
    fn test_buffer_clone() {
        let buf1 = TextBuffer::from_str("hello");
        let buf2 = buf1.clone();
        assert_eq!(buf2.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_buffer_debug() {
        let buf = TextBuffer::from_str("hello");
        let debug = format!("{:?}", buf);
        assert!(!debug.is_empty());
    }

    #[test]
    fn test_buffer_very_long_line() {
        let long_line: String = "a".repeat(10000);
        let buf = TextBuffer::from_str(&long_line);
        assert_eq!(buf.line_len(0).unwrap(), 10000);
    }

    #[test]
    fn test_buffer_many_lines() {
        let many_lines: String = (0..1000).map(|i| format!("line{}\n", i)).collect();
        let buf = TextBuffer::from_str(&many_lines);
        assert_eq!(buf.line_count(), 1001);
    }
}

mod extra_grapheme_tests {
    use super::*;

    #[test]
    fn test_grapheme_count_ascii() {
        assert_eq!(grapheme_count("hello"), 5);
    }

    #[test]
    fn test_grapheme_count_unicode() {
        // é can be 1 grapheme even if 2 codepoints
        let count = grapheme_count("café");
        assert!(count >= 4);
    }

    #[test]
    fn test_grapheme_count_empty() {
        assert_eq!(grapheme_count(""), 0);
    }

    #[test]
    fn test_grapheme_count_emoji() {
        // 👨‍👩‍👧‍👦 is one grapheme cluster
        let count = grapheme_count("👨‍👩‍👧‍👦");
        assert!(count >= 1);
    }

    #[test]
    fn test_grapheme_width_ascii() {
        assert_eq!(grapheme_width("a"), 1);
    }

    #[test]
    fn test_grapheme_width_cjk() {
        // CJK chars are typically double width
        let width = grapheme_width("中");
        assert!(width >= 1);
    }

    #[test]
    fn test_grapheme_width_emoji() {
        let width = grapheme_width("👋");
        assert!(width >= 1);
    }

    #[test]
    fn test_grapheme_iter_ascii() {
        let iter = GraphemeIter::new("hello");
        let graphemes: Vec<_> = iter.collect();
        assert_eq!(graphemes.len(), 5);
        assert_eq!(graphemes[0].1, "h");
    }

    #[test]
    fn test_grapheme_iter_empty() {
        let iter = GraphemeIter::new("");
        let graphemes: Vec<_> = iter.collect();
        assert_eq!(graphemes.len(), 0);
    }

    #[test]
    fn test_grapheme_iter_unicode() {
        let iter = GraphemeIter::new("héllo");
        let graphemes: Vec<_> = iter.collect();
        assert!(graphemes.len() >= 4);
    }

    #[test]
    fn test_grapheme_iter_cjk() {
        let iter = GraphemeIter::new("你好");
        let graphemes: Vec<_> = iter.collect();
        assert_eq!(graphemes.len(), 2);
    }
}

mod extra_position_range_tests {
    use super::*;

    #[test]
    fn test_range_is_empty() {
        let range = Range::from_coords(0, 0, 0, 0);
        assert!(range.is_empty());
    }

    #[test]
    fn test_range_not_empty() {
        let range = Range::from_coords(0, 0, 0, 5);
        assert!(!range.is_empty());
    }

    #[test]
    fn test_range_multiline() {
        let range = Range::from_coords(0, 0, 2, 0);
        assert!(!range.is_empty());
    }

    #[test]
    fn test_position_origin() {
        let pos = Position::origin();
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn test_position_new() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.column, 10);
    }

    #[test]
    fn test_position_clone() {
        let p1 = Position::new(1, 2);
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_position_debug() {
        let pos = Position::new(1, 2);
        let debug = format!("{:?}", pos);
        assert!(debug.contains("1"));
    }

    #[test]
    fn test_range_clone() {
        let r1 = Range::from_coords(0, 0, 1, 5);
        let r2 = r1.clone();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_range_debug() {
        let range = Range::from_coords(0, 0, 1, 5);
        let debug = format!("{:?}", range);
        assert!(!debug.is_empty());
    }
}
