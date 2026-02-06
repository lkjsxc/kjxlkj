//! Comprehensive tests for core-text buffer, grapheme, word, snapshot.

use kjxlkj_core_text::*;
use kjxlkj_core_types::*;

// ──────────── TextBuffer creation ────────────

#[test]
fn empty_buffer_properties() {
    let buf = TextBuffer::new();
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.text(), "");
    assert!(!buf.is_modified());
    assert_eq!(buf.name(), "[No Name]");
    assert!(buf.file_path().is_none());
}

#[test]
fn from_text_single_line() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.line_len(0), 5);
    assert_eq!(buf.text(), "hello");
}

#[test]
fn from_text_multiple_lines() {
    let buf = TextBuffer::from_text("a\nb\nc\n");
    assert_eq!(buf.line_count(), 4); // trailing newline => 4 lines
    assert_eq!(buf.line_len(0), 1);
    assert_eq!(buf.line_len(1), 1);
    assert_eq!(buf.line_len(2), 1);
    assert_eq!(buf.line_len(3), 0);
}

#[test]
fn from_text_trailing_newline() {
    let buf = TextBuffer::from_text("abc\n");
    assert_eq!(buf.line_count(), 2);
    assert_eq!(buf.line_len(0), 3);
    assert_eq!(buf.line_len(1), 0);
}

#[test]
fn from_text_empty_string() {
    let buf = TextBuffer::from_text("");
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.line_len(0), 0);
}

// ──────────── name / file path ────────────

#[test]
fn set_name() {
    let mut buf = TextBuffer::new();
    buf.set_name("test.txt");
    assert_eq!(buf.name(), "test.txt");
}

#[test]
fn set_file_path_updates_name() {
    let mut buf = TextBuffer::new();
    buf.set_file_path("/tmp/foo/bar.txt");
    assert_eq!(buf.name(), "bar.txt");
    assert_eq!(
        buf.file_path().unwrap().to_str().unwrap(),
        "/tmp/foo/bar.txt"
    );
}

// ──────────── char_at ────────────

#[test]
fn char_at_valid() {
    let buf = TextBuffer::from_text("abc\ndef");
    assert_eq!(buf.char_at(Position::new(0, 0)), Some('a'));
    assert_eq!(buf.char_at(Position::new(0, 2)), Some('c'));
    assert_eq!(buf.char_at(Position::new(1, 0)), Some('d'));
    assert_eq!(buf.char_at(Position::new(1, 2)), Some('f'));
}

#[test]
fn char_at_newline() {
    let buf = TextBuffer::from_text("ab\ncd");
    // Position (0, 2) is clamped to the newline
    assert_eq!(buf.char_at(Position::new(0, 2)), Some('\n'));
}

#[test]
fn char_at_out_of_bounds() {
    let buf = TextBuffer::from_text("ab");
    assert_eq!(buf.char_at(Position::new(5, 0)), None);
}

// ──────────── insert operations ────────────

#[test]
fn insert_char_beginning() {
    let mut buf = TextBuffer::from_text("ello");
    buf.insert_char(Position::new(0, 0), 'h');
    assert_eq!(buf.text(), "hello");
    assert!(buf.is_modified());
}

#[test]
fn insert_char_end() {
    let mut buf = TextBuffer::from_text("hell");
    buf.insert_char(Position::new(0, 4), 'o');
    assert_eq!(buf.text(), "hello");
}

#[test]
fn insert_char_middle() {
    let mut buf = TextBuffer::from_text("hllo");
    buf.insert_char(Position::new(0, 1), 'e');
    assert_eq!(buf.text(), "hello");
}

#[test]
fn insert_text_empty() {
    let mut buf = TextBuffer::from_text("ab");
    buf.insert_text(Position::new(0, 1), "");
    assert_eq!(buf.text(), "ab");
}

#[test]
fn insert_text_multiline() {
    let mut buf = TextBuffer::from_text("ac");
    buf.insert_text(Position::new(0, 1), "b\n");
    assert_eq!(buf.text(), "ab\nc");
    assert_eq!(buf.line_count(), 2);
}

#[test]
fn insert_text_at_line_boundary() {
    let mut buf = TextBuffer::from_text("hello\nworld");
    buf.insert_text(Position::new(1, 0), "new ");
    assert_eq!(buf.line_to_string(1), "new world");
}

#[test]
fn insert_version_increments() {
    let mut buf = TextBuffer::from_text("a");
    let v0 = buf.version();
    buf.insert_char(Position::new(0, 0), 'x');
    let v1 = buf.version();
    buf.insert_char(Position::new(0, 0), 'y');
    let v2 = buf.version();
    assert!(v1 > v0);
    assert!(v2 > v1);
}

// ──────────── delete operations ────────────

#[test]
fn delete_range_single_char() {
    let mut buf = TextBuffer::from_text("abc");
    let del = buf.delete_range(Range::new(
        Position::new(0, 1),
        Position::new(0, 2),
    ));
    assert_eq!(del, "b");
    assert_eq!(buf.text(), "ac");
}

#[test]
fn delete_range_whole_line() {
    let mut buf = TextBuffer::from_text("abc\ndef");
    // To include the newline, we need to go to start of next line
    let del = buf.delete_range(Range::new(
        Position::new(0, 0),
        Position::new(1, 0),
    ));
    assert_eq!(del, "abc\n");
    assert_eq!(buf.text(), "def");
}

#[test]
fn delete_range_empty() {
    let mut buf = TextBuffer::from_text("abc");
    let del = buf.delete_range(Range::new(
        Position::new(0, 1),
        Position::new(0, 1),
    ));
    assert_eq!(del, "");
    assert_eq!(buf.text(), "abc");
}

#[test]
fn delete_range_reversed() {
    // If start > end, delete_range handles it via ordering
    let mut buf = TextBuffer::from_text("abcdef");
    let del = buf.delete_range(Range::new(
        Position::new(0, 4),
        Position::new(0, 1),
    ));
    assert_eq!(del, "bcd");
    assert_eq!(buf.text(), "aef");
}

#[test]
fn delete_range_cross_line() {
    let mut buf = TextBuffer::from_text("abc\ndef\nghi");
    let del = buf.delete_range(Range::new(
        Position::new(0, 1),
        Position::new(2, 1),
    ));
    assert_eq!(del, "bc\ndef\ng");
    assert_eq!(buf.text(), "ahi");
}

#[test]
fn delete_between_positions() {
    let mut buf = TextBuffer::from_text("hello");
    let del = buf.delete_between(Position::new(0, 1), Position::new(0, 3));
    assert_eq!(del, "el");
    assert_eq!(buf.text(), "hlo");
}

#[test]
fn delete_line_first() {
    let mut buf = TextBuffer::from_text("aaa\nbbb\nccc");
    let del = buf.delete_line(0);
    assert_eq!(del, "aaa\n");
    assert_eq!(buf.text(), "bbb\nccc");
}

#[test]
fn delete_line_last() {
    let mut buf = TextBuffer::from_text("aaa\nbbb");
    let del = buf.delete_line(1);
    assert_eq!(del, "bbb");
    assert_eq!(buf.text(), "aaa\n");
}

#[test]
fn delete_line_middle() {
    let mut buf = TextBuffer::from_text("a\nb\nc");
    let del = buf.delete_line(1);
    assert_eq!(del, "b\n");
    assert_eq!(buf.text(), "a\nc");
}

#[test]
fn delete_line_out_of_bounds() {
    let mut buf = TextBuffer::from_text("abc");
    let del = buf.delete_line(5);
    assert_eq!(del, "");
    assert_eq!(buf.text(), "abc");
}

// ──────────── text_in_range ────────────

#[test]
fn text_in_range_same_line() {
    let buf = TextBuffer::from_text("hello world");
    assert_eq!(
        buf.text_in_range(Position::new(0, 0), Position::new(0, 5)),
        "hello"
    );
}

#[test]
fn text_in_range_multi_line() {
    let buf = TextBuffer::from_text("abc\ndef\nghi");
    let t = buf.text_in_range(Position::new(0, 2), Position::new(2, 1));
    assert_eq!(t, "c\ndef\ng");
}

#[test]
fn text_in_range_empty() {
    let buf = TextBuffer::from_text("abc");
    assert_eq!(
        buf.text_in_range(Position::new(0, 1), Position::new(0, 1)),
        ""
    );
}

// ──────────── pos_to_char_idx / char_idx_to_pos ────────────

#[test]
fn pos_char_idx_roundtrip() {
    let buf = TextBuffer::from_text("abc\ndef\nghi");
    for (line, col) in [(0, 0), (0, 2), (1, 0), (1, 2), (2, 0), (2, 2)] {
        let pos = Position::new(line, col);
        let idx = buf.pos_to_char_idx(pos);
        let back = buf.char_idx_to_pos(idx);
        assert_eq!(back, pos, "roundtrip failed for {:?}", pos);
    }
}

#[test]
fn pos_to_char_idx_out_of_bounds_line() {
    let buf = TextBuffer::from_text("abc");
    let idx = buf.pos_to_char_idx(Position::new(10, 0));
    assert_eq!(idx, buf.rope().len_chars());
}

#[test]
fn pos_to_char_idx_out_of_bounds_col() {
    let buf = TextBuffer::from_text("abc\ndef");
    let idx = buf.pos_to_char_idx(Position::new(0, 100));
    // col clamped to line_len(0) = 3
    assert_eq!(idx, 3);
}

// ──────────── clamp_position ────────────

#[test]
fn clamp_position_valid() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(buf.clamp_position(Position::new(0, 2)), Position::new(0, 2));
}

#[test]
fn clamp_position_past_end_of_line() {
    let buf = TextBuffer::from_text("hi");
    // "hi" has 2 chars, normal mode last valid col = 1
    assert_eq!(buf.clamp_position(Position::new(0, 10)), Position::new(0, 1));
}

#[test]
fn clamp_position_past_last_line() {
    let buf = TextBuffer::from_text("a\nb");
    assert_eq!(buf.clamp_position(Position::new(5, 0)), Position::new(1, 0));
}

#[test]
fn clamp_position_insert_mode() {
    let buf = TextBuffer::from_text("hi");
    // Insert mode allows cursor after last char
    assert_eq!(
        buf.clamp_position_insert(Position::new(0, 2)),
        Position::new(0, 2)
    );
    assert_eq!(
        buf.clamp_position_insert(Position::new(0, 10)),
        Position::new(0, 2)
    );
}

#[test]
fn clamp_position_empty_buf() {
    let buf = TextBuffer::new();
    assert_eq!(buf.clamp_position(Position::new(0, 0)), Position::new(0, 0));
    assert_eq!(buf.clamp_position(Position::new(5, 5)), Position::new(0, 0));
}

// ──────────── line_to_string ────────────

#[test]
fn line_to_string_strips_newline() {
    let buf = TextBuffer::from_text("abc\ndef");
    assert_eq!(buf.line_to_string(0), "abc");
    assert_eq!(buf.line_to_string(1), "def");
}

#[test]
fn line_to_string_out_of_bounds() {
    let buf = TextBuffer::from_text("abc");
    assert_eq!(buf.line_to_string(5), "");
}

// ──────────── line_len edge cases ────────────

#[test]
fn line_len_out_of_bounds() {
    let buf = TextBuffer::from_text("abc");
    assert_eq!(buf.line_len(10), 0);
}

#[test]
fn line_len_empty_line() {
    let buf = TextBuffer::from_text("abc\n\ndef");
    assert_eq!(buf.line_len(0), 3);
    assert_eq!(buf.line_len(1), 0);
    assert_eq!(buf.line_len(2), 3);
}

// ──────────── modified flag ────────────

#[test]
fn modified_flag_after_insert() {
    let mut buf = TextBuffer::new();
    assert!(!buf.is_modified());
    buf.insert_char(Position::new(0, 0), 'x');
    assert!(buf.is_modified());
}

#[test]
fn modified_flag_resettable() {
    let mut buf = TextBuffer::new();
    buf.insert_char(Position::new(0, 0), 'x');
    assert!(buf.is_modified());
    buf.set_modified(false);
    assert!(!buf.is_modified());
}

// ──────────── BufferId uniqueness across buffers ────────────

#[test]
fn buffer_ids_unique() {
    let a = TextBuffer::new();
    let b = TextBuffer::new();
    assert_ne!(a.id(), b.id());
}

// ──────────── large buffer operations ────────────

#[test]
fn large_buffer_creation() {
    let text: String = (0..1000).map(|i| format!("line {}\n", i)).collect();
    let buf = TextBuffer::from_text(&text);
    assert_eq!(buf.line_count(), 1001);
    assert_eq!(buf.line_to_string(0), "line 0");
    assert_eq!(buf.line_to_string(999), "line 999");
}

#[test]
fn large_buffer_delete_middle() {
    let text: String = (0..100).map(|i| format!("line {}\n", i)).collect();
    let mut buf = TextBuffer::from_text(&text);
    assert_eq!(buf.line_count(), 101);
    buf.delete_line(50);
    assert_eq!(buf.line_count(), 100);
}

// ──────────── grapheme module ────────────

#[test]
fn grapheme_width_ascii() {
    assert_eq!(grapheme_width("a"), 1);
    assert_eq!(grapheme_width(" "), 1);
    assert_eq!(grapheme_width("Z"), 1);
}

#[test]
fn grapheme_width_cjk() {
    // CJK chars are typically double-width
    assert_eq!(grapheme_width("中"), 2);
    assert_eq!(grapheme_width("日"), 2);
}

#[test]
fn grapheme_width_empty() {
    assert_eq!(grapheme_width(""), 0);
}

#[test]
fn line_display_width_ascii() {
    assert_eq!(line_display_width("hello"), 5);
}

#[test]
fn display_width_to_col_basic() {
    assert_eq!(display_width_to_col("hello", 3), 3);
}

#[test]
fn display_width_to_col_beyond() {
    assert_eq!(display_width_to_col("hi", 10), 2);
}

// ──────────── word module ────────────

#[test]
fn word_char_alphabetic() {
    assert!(is_word_char('a'));
    assert!(is_word_char('Z'));
    assert!(is_word_char('_'));
    assert!(is_word_char('0'));
}

#[test]
fn non_word_char_punctuation() {
    assert!(!is_word_char('.'));
    assert!(!is_word_char(','));
    assert!(!is_word_char('!'));
}

#[test]
fn non_word_char_whitespace() {
    assert!(!is_word_char(' '));
    assert!(!is_word_char('\t'));
}

#[test]
fn is_word_char_check() {
    assert!(is_word_char('a'));
    assert!(is_word_char('_'));
    assert!(is_word_char('9'));
    assert!(!is_word_char(' '));
    assert!(!is_word_char('.'));
}

#[test]
fn word_start_forward_basic() {
    let buf = TextBuffer::from_text("hello world");
    let pos = word_start_forward(&buf, Position::new(0, 0));
    assert_eq!(pos, Position::new(0, 6));
}

#[test]
fn word_start_forward_punct() {
    let buf = TextBuffer::from_text("a.b");
    let pos = word_start_forward(&buf, Position::new(0, 0));
    // 'a' is word, '.' is punct — next word class boundary
    assert_eq!(pos, Position::new(0, 1));
}

#[test]
fn word_start_backward_basic() {
    let buf = TextBuffer::from_text("hello world");
    let pos = word_start_backward(&buf, Position::new(0, 8));
    assert_eq!(pos, Position::new(0, 6));
}

#[test]
fn word_start_backward_at_start() {
    let buf = TextBuffer::from_text("hello");
    let pos = word_start_backward(&buf, Position::new(0, 0));
    assert_eq!(pos, Position::new(0, 0));
}

#[test]
fn word_end_forward_basic() {
    let buf = TextBuffer::from_text("hello world");
    let pos = word_end_forward(&buf, Position::new(0, 0));
    assert_eq!(pos, Position::new(0, 4));
}

#[test]
fn word_end_forward_single_char() {
    let buf = TextBuffer::from_text("a b");
    let pos = word_end_forward(&buf, Position::new(0, 0));
    assert_eq!(pos, Position::new(0, 2));
}

// ──────────── snapshot module ────────────

#[test]
fn snapshot_creation() {
    let buf = TextBuffer::from_text("line1\nline2\nline3\nline4\nline5");
    let snap = BufferSnapshot::from_buffer(&buf, 1, 3, Position::new(1, 0));
    assert_eq!(snap.buffer_id, buf.id());
    assert_eq!(snap.first_line, 1);
    assert_eq!(snap.lines.len(), 3);
    assert_eq!(snap.lines[0], "line2");
    assert_eq!(snap.lines[1], "line3");
    assert_eq!(snap.lines[2], "line4");
}

#[test]
fn snapshot_beyond_buffer() {
    let buf = TextBuffer::from_text("a\nb");
    let snap = BufferSnapshot::from_buffer(&buf, 0, 10, Position::new(0, 0));
    assert_eq!(snap.lines.len(), 2);
}

#[test]
fn snapshot_empty_buffer() {
    let buf = TextBuffer::new();
    let snap = BufferSnapshot::from_buffer(&buf, 0, 5, Position::new(0, 0));
    assert_eq!(snap.lines.len(), 1);
    assert_eq!(snap.lines[0], "");
}

#[test]
fn snapshot_from_middle() {
    let text: String = (0..20).map(|i| format!("L{}\n", i)).collect();
    let buf = TextBuffer::from_text(&text);
    let snap = BufferSnapshot::from_buffer(&buf, 10, 5, Position::new(10, 0));
    assert_eq!(snap.first_line, 10);
    assert_eq!(snap.lines.len(), 5);
    assert_eq!(snap.lines[0], "L10");
}
