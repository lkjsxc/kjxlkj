//! Extended tests for editing operations.

use kjxlkj_core_edit::*;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::*;

// ──────────── Motion edge cases ────────────

#[test]
fn motion_right_clamps_to_line_end() {
    let buf = TextBuffer::from_text("abc");
    let p = apply_motion(&buf, Position::new(0, 2), MotionKind::Right, 10);
    assert_eq!(p.col, 2); // last char
}

#[test]
fn motion_left_at_start() {
    let buf = TextBuffer::from_text("abc");
    let p = apply_motion(&buf, Position::new(0, 0), MotionKind::Left, 5);
    assert_eq!(p.col, 0);
}

#[test]
fn motion_up_at_first_line() {
    let buf = TextBuffer::from_text("abc\ndef");
    let p = apply_motion(&buf, Position::new(0, 1), MotionKind::Up, 1);
    assert_eq!(p.line, 0);
}

#[test]
fn motion_down_at_last_line() {
    let buf = TextBuffer::from_text("abc\ndef");
    let p = apply_motion(&buf, Position::new(1, 1), MotionKind::Down, 10);
    assert_eq!(p.line, 1);
}

#[test]
fn motion_line_start_from_middle() {
    let buf = TextBuffer::from_text("hello world");
    let p = apply_motion(&buf, Position::new(0, 7), MotionKind::LineStart, 1);
    assert_eq!(p.col, 0);
}

#[test]
fn motion_line_end_already_at_end() {
    let buf = TextBuffer::from_text("abc");
    let p = apply_motion(&buf, Position::new(0, 2), MotionKind::LineEnd, 1);
    assert_eq!(p.col, 2);
}

#[test]
fn motion_first_non_blank_all_spaces() {
    let buf = TextBuffer::from_text("     ");
    let p = apply_motion(&buf, Position::new(0, 0), MotionKind::FirstNonBlank, 1);
    // All spaces, first_non_blank should go to col 0 or end
    assert!(p.col <= 5);
}

#[test]
fn motion_word_forward_at_end() {
    let buf = TextBuffer::from_text("abc");
    let p = apply_motion(&buf, Position::new(0, 2), MotionKind::WordForward, 1);
    assert_eq!(p.line, 0);
}

#[test]
fn motion_word_backward_at_start() {
    let buf = TextBuffer::from_text("abc def");
    let p = apply_motion(&buf, Position::new(0, 0), MotionKind::WordBackward, 1);
    assert_eq!(p, Position::new(0, 0));
}

#[test]
fn motion_file_start_from_anywhere() {
    let buf = TextBuffer::from_text("a\nb\nc\nd");
    let p = apply_motion(&buf, Position::new(3, 0), MotionKind::FileStart, 1);
    assert_eq!(p, Position::new(0, 0));
}

#[test]
fn motion_file_end_from_start() {
    let buf = TextBuffer::from_text("a\nb\nc");
    let p = apply_motion(&buf, Position::new(0, 0), MotionKind::FileEnd, 1);
    assert_eq!(p.line, 2);
}

#[test]
fn motion_word_end_forward() {
    let buf = TextBuffer::from_text("hello world");
    let p = apply_motion(&buf, Position::new(0, 0), MotionKind::WordForwardEnd, 1);
    assert_eq!(p.col, 4); // end of "hello"
}

#[test]
fn motion_down_count() {
    let buf = TextBuffer::from_text("a\nb\nc\nd\ne");
    let p = apply_motion(&buf, Position::new(0, 0), MotionKind::Down, 3);
    assert_eq!(p.line, 3);
}

#[test]
fn motion_up_count() {
    let buf = TextBuffer::from_text("a\nb\nc\nd\ne");
    let p = apply_motion(&buf, Position::new(4, 0), MotionKind::Up, 2);
    assert_eq!(p.line, 2);
}

// ──────────── Compute motion range ────────────

#[test]
fn compute_range_right() {
    let buf = TextBuffer::from_text("hello");
    let range = compute_motion_range(&buf, Position::new(0, 0), MotionKind::Right, 1);
    assert!(range.start.col < range.end.col);
}

#[test]
fn compute_range_left() {
    let buf = TextBuffer::from_text("hello");
    let range = compute_motion_range(&buf, Position::new(0, 3), MotionKind::Left, 1);
    assert!(range.start.col < range.end.col);
}

#[test]
fn compute_range_file_end() {
    let buf = TextBuffer::from_text("a\nb\nc");
    let range = compute_motion_range(&buf, Position::new(0, 0), MotionKind::FileEnd, 1);
    assert!(range.end.line >= 2);
}

// ──────────── Operator edge cases ────────────

#[test]
fn operator_delete_empty_range() {
    let mut buf = TextBuffer::from_text("hello");
    let range = Range::new(Position::new(0, 0), Position::new(0, 0));
    let result = apply_operator(&mut buf, OperatorKind::Delete, range, false);
    assert_eq!(buf.line_to_string(0), "hello"); // nothing deleted
    assert!(result.deleted_text.is_none() || result.deleted_text.unwrap().text.is_empty());
}

#[test]
fn operator_yank_preserves_buffer() {
    let mut buf = TextBuffer::from_text("hello");
    let range = Range::new(Position::new(0, 0), Position::new(0, 5));
    let result = apply_operator(&mut buf, OperatorKind::Yank, range, false);
    assert_eq!(buf.line_to_string(0), "hello"); // unchanged
    assert!(result.deleted_text.is_some());
}

#[test]
fn operator_change_enters_insert() {
    let mut buf = TextBuffer::from_text("hello world");
    let range = Range::new(Position::new(0, 0), Position::new(0, 5));
    let result = apply_operator(&mut buf, OperatorKind::Change, range, false);
    assert!(result.enter_insert);
}

#[test]
fn operator_delete_linewise() {
    let mut buf = TextBuffer::from_text("aaa\nbbb\nccc");
    let range = Range::new(Position::new(0, 0), Position::new(1, 3));
    let result = apply_operator(&mut buf, OperatorKind::Delete, range, true);
    assert!(result.deleted_text.is_some());
}

#[test]
fn operator_uppercase() {
    let mut buf = TextBuffer::from_text("hello");
    let range = Range::new(Position::new(0, 0), Position::new(0, 5));
    apply_operator(&mut buf, OperatorKind::Uppercase, range, false);
    assert_eq!(buf.line_to_string(0), "HELLO");
}

#[test]
fn operator_lowercase() {
    let mut buf = TextBuffer::from_text("HELLO");
    let range = Range::new(Position::new(0, 0), Position::new(0, 5));
    apply_operator(&mut buf, OperatorKind::Lowercase, range, false);
    assert_eq!(buf.line_to_string(0), "hello");
}

#[test]
fn operator_toggle_case() {
    let mut buf = TextBuffer::from_text("hElLo");
    let range = Range::new(Position::new(0, 0), Position::new(0, 5));
    apply_operator(&mut buf, OperatorKind::ToggleCase, range, false);
    assert_eq!(buf.line_to_string(0), "HeLlO");
}

// ──────────── Text object edge cases ────────────

#[test]
fn text_object_word_at_start() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 0), TextObjectKind::Word, false);
    assert!(r.is_some());
    let r = r.unwrap();
    assert_eq!(r.start, Position::new(0, 0));
}

#[test]
fn text_object_inner_word() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 0), TextObjectKind::Word, true);
    assert!(r.is_some());
}

#[test]
fn text_object_paren_nested() {
    let buf = TextBuffer::from_text("((inner))");
    let r = find_text_object(&buf, Position::new(0, 2), TextObjectKind::Paren, true);
    assert!(r.is_some());
}

#[test]
fn text_object_paren_no_match() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 0), TextObjectKind::Paren, true);
    assert!(r.is_none());
}

#[test]
fn text_object_bracket() {
    let buf = TextBuffer::from_text("[hello]");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Bracket, true);
    assert!(r.is_some());
}

#[test]
fn text_object_brace() {
    let buf = TextBuffer::from_text("{hello}");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Brace, true);
    assert!(r.is_some());
}

#[test]
fn text_object_double_quote() {
    let buf = TextBuffer::from_text("say \"hello\" please");
    let r = find_text_object(&buf, Position::new(0, 6), TextObjectKind::DoubleQuote, true);
    assert!(r.is_some());
}

#[test]
fn text_object_backtick() {
    let buf = TextBuffer::from_text("say `hello` please");
    let r = find_text_object(&buf, Position::new(0, 6), TextObjectKind::BackTick, true);
    assert!(r.is_some());
}

#[test]
fn text_object_paragraph() {
    let buf = TextBuffer::from_text("a\nb\n\nc\nd");
    let r = find_text_object(&buf, Position::new(0, 0), TextObjectKind::Paragraph, false);
    assert!(r.is_some());
}
