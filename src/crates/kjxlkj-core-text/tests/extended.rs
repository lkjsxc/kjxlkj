//! Extended tests for text buffer operations and edge cases.

use kjxlkj_core_text::*;
use kjxlkj_core_types::Position;

// ──────────── Buffer creation ────────────

#[test]
fn empty_buffer_has_one_line() {
    let buf = TextBuffer::new();
    assert_eq!(buf.line_count(), 1);
}

#[test]
fn from_text_empty() {
    let buf = TextBuffer::from_text("");
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.line_to_string(0), "");
}

#[test]
fn from_text_single_line() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.line_to_string(0), "hello");
}

#[test]
fn from_text_multiline() {
    let buf = TextBuffer::from_text("a\nb\nc");
    assert_eq!(buf.line_count(), 3);
    assert_eq!(buf.line_to_string(0), "a");
    assert_eq!(buf.line_to_string(1), "b");
    assert_eq!(buf.line_to_string(2), "c");
}

#[test]
fn from_text_trailing_newline() {
    let buf = TextBuffer::from_text("a\nb\n");
    assert_eq!(buf.line_count(), 3);
    assert_eq!(buf.line_to_string(2), "");
}

// ──────────── Line operations ────────────

#[test]
fn line_returns_some() {
    let buf = TextBuffer::from_text("hello");
    assert!(buf.line(0).is_some());
}

#[test]
fn line_returns_none_out_of_bounds() {
    let buf = TextBuffer::from_text("hello");
    assert!(buf.line(99).is_none());
}

#[test]
fn line_len_excludes_newline() {
    let buf = TextBuffer::from_text("abc\ndef");
    assert_eq!(buf.line_len(0), 3);
    assert_eq!(buf.line_len(1), 3);
}

#[test]
fn line_len_empty_line() {
    let buf = TextBuffer::from_text("a\n\nc");
    assert_eq!(buf.line_len(1), 0);
}

// ──────────── char_at ────────────

#[test]
fn char_at_first() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(buf.char_at(Position::new(0, 0)), Some('h'));
}

#[test]
fn char_at_last() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(buf.char_at(Position::new(0, 4)), Some('o'));
}

#[test]
fn char_at_second_line() {
    let buf = TextBuffer::from_text("abc\nxyz");
    assert_eq!(buf.char_at(Position::new(1, 0)), Some('x'));
    assert_eq!(buf.char_at(Position::new(1, 2)), Some('z'));
}

#[test]
fn char_at_out_of_bounds() {
    let buf = TextBuffer::from_text("abc");
    assert_eq!(buf.char_at(Position::new(0, 99)), None);
    assert_eq!(buf.char_at(Position::new(99, 0)), None);
}

// ──────────── Insert operations ────────────

#[test]
fn insert_char_at_start() {
    let mut buf = TextBuffer::from_text("ello");
    buf.insert_char(Position::new(0, 0), 'h');
    assert_eq!(buf.line_to_string(0), "hello");
}

#[test]
fn insert_char_at_end() {
    let mut buf = TextBuffer::from_text("hell");
    buf.insert_char(Position::new(0, 4), 'o');
    assert_eq!(buf.line_to_string(0), "hello");
}

#[test]
fn insert_char_in_middle() {
    let mut buf = TextBuffer::from_text("hllo");
    buf.insert_char(Position::new(0, 1), 'e');
    assert_eq!(buf.line_to_string(0), "hello");
}

#[test]
fn insert_newline() {
    let mut buf = TextBuffer::from_text("abcd");
    buf.insert_char(Position::new(0, 2), '\n');
    assert_eq!(buf.line_count(), 2);
    assert_eq!(buf.line_to_string(0), "ab");
    assert_eq!(buf.line_to_string(1), "cd");
}

#[test]
fn insert_text_string() {
    let mut buf = TextBuffer::from_text("hd");
    buf.insert_text(Position::new(0, 1), "ello worl");
    assert_eq!(buf.line_to_string(0), "hello world");
}

// ──────────── Delete operations ────────────

#[test]
fn delete_range_single_char() {
    let mut buf = TextBuffer::from_text("hello");
    use kjxlkj_core_types::Range;
    let deleted = buf.delete_range(Range::new(
        Position::new(0, 0),
        Position::new(0, 1),
    ));
    assert_eq!(deleted, "h");
    assert_eq!(buf.line_to_string(0), "ello");
}

#[test]
fn delete_between_positions() {
    let mut buf = TextBuffer::from_text("hello");
    let deleted = buf.delete_between(Position::new(0, 1), Position::new(0, 4));
    assert_eq!(deleted, "ell");
    assert_eq!(buf.line_to_string(0), "ho");
}

#[test]
fn delete_line_first() {
    let mut buf = TextBuffer::from_text("aaa\nbbb\nccc");
    buf.delete_line(0);
    assert_eq!(buf.line_to_string(0), "bbb");
}

#[test]
fn delete_line_middle() {
    let mut buf = TextBuffer::from_text("aaa\nbbb\nccc");
    buf.delete_line(1);
    assert_eq!(buf.line_to_string(0), "aaa");
    assert_eq!(buf.line_to_string(1), "ccc");
}

#[test]
fn delete_line_last() {
    let mut buf = TextBuffer::from_text("aaa\nbbb");
    buf.delete_line(1);
    // Deleting last line leaves trailing newline → 2 rope lines
    assert!(buf.line_count() <= 2);
}

// ──────────── text_in_range ────────────

#[test]
fn text_in_range_single_line() {
    let buf = TextBuffer::from_text("hello world");
    let t = buf.text_in_range(Position::new(0, 0), Position::new(0, 5));
    assert_eq!(t, "hello");
}

#[test]
fn text_in_range_multiline() {
    let buf = TextBuffer::from_text("abc\ndefgh\nijk");
    let t = buf.text_in_range(Position::new(0, 0), Position::new(1, 3));
    // From (0,0) to (1,3) — includes the newline
    assert!(t.contains("abc"));
}

// ──────────── Clamp position ────────────

#[test]
fn clamp_position_valid() {
    let buf = TextBuffer::from_text("hello");
    let p = buf.clamp_position(Position::new(0, 3));
    assert_eq!(p, Position::new(0, 3));
}

#[test]
fn clamp_position_past_end() {
    let buf = TextBuffer::from_text("abc");
    let p = buf.clamp_position(Position::new(0, 100));
    assert!(p.col <= 2);
}

#[test]
fn clamp_position_past_lines() {
    let buf = TextBuffer::from_text("abc\ndef");
    let p = buf.clamp_position(Position::new(100, 0));
    assert!(p.line <= 1);
}

#[test]
fn clamp_position_insert_past_end() {
    let buf = TextBuffer::from_text("abc");
    let p = buf.clamp_position_insert(Position::new(0, 100));
    assert!(p.col <= 3);
}

// ──────────── pos_to_char_idx / char_idx_to_pos ────────────

#[test]
fn roundtrip_pos_char_idx() {
    let buf = TextBuffer::from_text("abc\ndef");
    let idx = buf.pos_to_char_idx(Position::new(1, 1));
    let pos = buf.char_idx_to_pos(idx);
    assert_eq!(pos, Position::new(1, 1));
}

#[test]
fn pos_to_char_idx_line_zero() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(buf.pos_to_char_idx(Position::new(0, 0)), 0);
    assert_eq!(buf.pos_to_char_idx(Position::new(0, 3)), 3);
}

#[test]
fn pos_to_char_idx_clamps() {
    let buf = TextBuffer::from_text("abc");
    // col 100 clamps to line_len(0) = 3
    let idx = buf.pos_to_char_idx(Position::new(0, 100));
    assert_eq!(idx, 3);
}

// ──────────── Word functions ────────────

#[test]
fn is_word_char_basics() {
    assert!(is_word_char('a'));
    assert!(is_word_char('Z'));
    assert!(is_word_char('0'));
    assert!(is_word_char('_'));
    assert!(!is_word_char(' '));
    assert!(!is_word_char('.'));
    assert!(!is_word_char('!'));
}

#[test]
fn char_class_word() {
    assert_eq!(CharClass::of('a'), CharClass::Word);
    assert_eq!(CharClass::of('_'), CharClass::Word);
    assert_eq!(CharClass::of('9'), CharClass::Word);
}

#[test]
fn char_class_whitespace() {
    assert_eq!(CharClass::of(' '), CharClass::Whitespace);
    assert_eq!(CharClass::of('\t'), CharClass::Whitespace);
}

#[test]
fn char_class_punctuation() {
    assert_eq!(CharClass::of('.'), CharClass::Punctuation);
    assert_eq!(CharClass::of('!'), CharClass::Punctuation);
    assert_eq!(CharClass::of('+'), CharClass::Punctuation);
}

#[test]
fn word_forward_multiline() {
    let buf = TextBuffer::from_text("hello\nworld");
    let p = word_start_forward(&buf, Position::new(0, 0));
    assert_eq!(p, Position::new(1, 0));
}

#[test]
fn word_forward_punctuation() {
    let buf = TextBuffer::from_text("hello.world");
    let p = word_start_forward(&buf, Position::new(0, 0));
    assert_eq!(p, Position::new(0, 5)); // stops at '.'
}

#[test]
fn word_backward_to_start() {
    let buf = TextBuffer::from_text("hello world");
    // At start of "world" (col 6), b goes to start of previous word "hello" (col 0)
    let p = word_start_backward(&buf, Position::new(0, 6));
    assert_eq!(p, Position::new(0, 0));
}

#[test]
fn word_end_forward_single() {
    let buf = TextBuffer::from_text("hello");
    let p = word_end_forward(&buf, Position::new(0, 0));
    assert_eq!(p, Position::new(0, 4));
}

// ──────────── Grapheme functions ────────────

#[test]
fn grapheme_width_ascii() {
    assert_eq!(grapheme_width("a"), 1);
    assert_eq!(grapheme_width("!"), 1);
    assert_eq!(grapheme_width(" "), 1);
}

#[test]
fn grapheme_width_cjk() {
    assert_eq!(grapheme_width("中"), 2);
}

#[test]
fn grapheme_width_tab() {
    // Tab width depends on unicode-width version
    let w = grapheme_width("\t");
    assert!(w <= 1);
}

#[test]
fn next_grapheme_boundary_ascii() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(next_grapheme_boundary(&buf, Position::new(0, 0)), Position::new(0, 1));
    assert_eq!(next_grapheme_boundary(&buf, Position::new(0, 1)), Position::new(0, 2));
    assert_eq!(next_grapheme_boundary(&buf, Position::new(0, 4)), Position::new(0, 5));
}

#[test]
fn prev_grapheme_boundary_ascii() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(prev_grapheme_boundary(&buf, Position::new(0, 5)), Position::new(0, 4));
    assert_eq!(prev_grapheme_boundary(&buf, Position::new(0, 1)), Position::new(0, 0));
}

#[test]
fn display_width_to_col_basic() {
    let s = "hello";
    assert_eq!(display_width_to_col(s, 0), 0);
    assert_eq!(display_width_to_col(s, 3), 3);
}

#[test]
fn line_display_width_basic() {
    assert_eq!(line_display_width("hello"), 5);
    assert_eq!(line_display_width(""), 0);
}

// ──────────── Snapshot ────────────

#[test]
fn snapshot_empty_buffer() {
    let buf = TextBuffer::new();
    let snap = BufferSnapshot::from_buffer(
        &buf,
        0,
        24,
        Position::new(0, 0),
    );
    assert_eq!(snap.total_lines, 1);
    assert_eq!(snap.first_line, 0);
}

#[test]
fn snapshot_viewport() {
    let buf = TextBuffer::from_text(
        &(0..100).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n"),
    );
    let snap = BufferSnapshot::from_buffer(
        &buf,
        10,
        20,
        Position::new(15, 0),
    );
    assert_eq!(snap.first_line, 10);
    assert!(snap.lines.len() <= 20);
    assert_eq!(snap.total_lines, 100);
}

#[test]
fn snapshot_cursor() {
    let buf = TextBuffer::from_text("abc\ndef");
    let snap = BufferSnapshot::from_buffer(
        &buf,
        0,
        24,
        Position::new(1, 2),
    );
    assert_eq!(snap.cursor, Position::new(1, 2));
}

#[test]
fn snapshot_clone() {
    let buf = TextBuffer::from_text("hello");
    let snap = BufferSnapshot::from_buffer(
        &buf,
        0,
        24,
        Position::new(0, 0),
    );
    let snap2 = snap.clone();
    assert_eq!(snap2.total_lines, 1);
}
