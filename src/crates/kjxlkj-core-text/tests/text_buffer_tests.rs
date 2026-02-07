//! Integration tests for TextBuffer and text manipulation.

use kjxlkj_core_text::*;
use kjxlkj_core_types::{BufferId, Position};

fn buf(text: &str) -> TextBuffer {
    TextBuffer::from_text(BufferId(1), "test".into(), text)
}

#[test]
fn create_empty_buffer() {
    let b = TextBuffer::new(BufferId(1), "empty".into());
    assert_eq!(b.line_count(), 1);
    assert_eq!(b.char_count(), 0);
}

#[test]
fn create_from_text() {
    let b = buf("hello\nworld\n");
    assert_eq!(b.line_count(), 3);
    assert_eq!(b.line(0), Some("hello".into()));
}

#[test]
fn insert_char() {
    let mut b = buf("abc\n");
    b.insert_char(Position::new(0, 1), 'X');
    assert_eq!(b.line(0), Some("aXbc".into()));
}

#[test]
fn insert_text() {
    let mut b = buf("hello\n");
    b.insert_text(Position::new(0, 5), " world");
    assert_eq!(b.line(0), Some("hello world".into()));
}

#[test]
fn delete_range() {
    let mut b = buf("abcdef\n");
    let deleted = b.delete_range(Position::new(0, 1), Position::new(0, 4));
    assert_eq!(deleted, "bcd");
    assert_eq!(b.line(0), Some("aef".into()));
}

#[test]
fn line_count() {
    let b = buf("a\nb\nc\n");
    assert_eq!(b.line_count(), 4);
}

#[test]
fn line_content() {
    let b = buf("first\nsecond\n");
    assert_eq!(b.line(0), Some("first".into()));
    assert_eq!(b.line(1), Some("second".into()));
    assert_eq!(b.line(99), None);
}

#[test]
fn replace_range() {
    let mut b = buf("hello world\n");
    b.delete_range(Position::new(0, 5), Position::new(0, 11));
    b.insert_text(Position::new(0, 5), " rust");
    assert_eq!(b.line(0), Some("hello rust".into()));
}

#[test]
fn line_length() {
    let b = buf("abcdef\n");
    assert_eq!(b.line_len(0), 6);
}

#[test]
fn word_forward_motion() {
    let col = word_start_forward("hello world", 0);
    assert_eq!(col, 6);
}

#[test]
fn word_backward_motion() {
    let col = word_start_backward("hello world", 8);
    assert_eq!(col, 6);
}

#[test]
fn grapheme_width() {
    assert!(display_width("a") == 1);
    assert!(display_width("全") == 2);
}

#[test]
fn display_width_ascii() {
    assert_eq!(display_width("abcde"), 5);
}

#[test]
fn display_width_cjk() {
    // Each CJK character is 2 columns wide
    assert_eq!(display_width("你好"), 4);
}

#[test]
fn join_lines_test() {
    let result = join_lines(&["a", "b", "c"], " ");
    assert_eq!(result, "a b c");
}

#[test]
fn case_upper() {
    assert_eq!(convert_case("hello", CaseKind::Upper), "HELLO");
}

#[test]
fn case_lower() {
    assert_eq!(convert_case("HELLO", CaseKind::Lower), "hello");
}

#[test]
fn case_toggle() {
    assert_eq!(convert_case("Hello", CaseKind::Toggle), "hELLO");
}

#[test]
fn sort_lines_test() {
    let mut v = vec!["c".into(), "a".into(), "b".into()];
    sort_lines(&mut v, false, false);
    assert_eq!(v, vec!["a", "b", "c"]);
}

#[test]
fn trim_trailing_whitespace() {
    assert_eq!(trim_trailing("hello   "), "hello");
}

#[test]
fn large_buffer_strategy() {
    assert_eq!(choose_strategy(500), LoadStrategy::Full);
    assert_eq!(choose_strategy(2 * 1024 * 1024), LoadStrategy::Chunked);
    assert_eq!(choose_strategy(200 * 1024 * 1024), LoadStrategy::Streamed);
}

#[test]
fn line_segmentation() {
    let segs = segment_line("abcdef", 3);
    assert_eq!(segs.len(), 2);
    assert_eq!(segs[0], "abc");
    assert_eq!(segs[1], "def");
}
