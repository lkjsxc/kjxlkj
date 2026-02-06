//! Comprehensive tests for core-edit operators.

use kjxlkj_core_edit::*;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::*;

// ──────────── Delete operator ────────────

#[test]
fn delete_charwise_start() {
    let mut buf = TextBuffer::from_text("hello world");
    let r = apply_operator(
        &mut buf, OperatorKind::Delete,
        Range::new(Position::new(0, 0), Position::new(0, 5)),
        false,
    );
    assert_eq!(buf.text(), " world");
    assert_eq!(r.deleted_text.unwrap().text, "hello");
    assert!(!r.enter_insert);
}

#[test]
fn delete_charwise_middle() {
    let mut buf = TextBuffer::from_text("abcdef");
    apply_operator(
        &mut buf, OperatorKind::Delete,
        Range::new(Position::new(0, 2), Position::new(0, 4)),
        false,
    );
    assert_eq!(buf.text(), "abef");
}

#[test]
fn delete_linewise_single() {
    let mut buf = TextBuffer::from_text("aaa\nbbb\nccc");
    let r = apply_operator(
        &mut buf, OperatorKind::Delete,
        Range::new(Position::new(1, 0), Position::new(1, 2)),
        true,
    );
    assert!(!buf.text().contains("bbb"));
    let dt = r.deleted_text.unwrap();
    assert_eq!(dt.reg_type, RegisterType::Linewise);
}

#[test]
fn delete_linewise_multiple() {
    let mut buf = TextBuffer::from_text("a\nb\nc\nd\ne");
    apply_operator(
        &mut buf, OperatorKind::Delete,
        Range::new(Position::new(1, 0), Position::new(3, 0)),
        true,
    );
    // Lines b, c, d should be deleted
    let text = buf.text();
    assert!(!text.contains("b\n"));
    assert!(!text.contains("c\n"));
}

#[test]
fn delete_empty_range() {
    let mut buf = TextBuffer::from_text("hello");
    let r = apply_operator(
        &mut buf, OperatorKind::Delete,
        Range::new(Position::new(0, 2), Position::new(0, 2)),
        false,
    );
    assert_eq!(buf.text(), "hello");
    assert_eq!(r.deleted_text.unwrap().text, "");
}

// ──────────── Yank operator ────────────

#[test]
fn yank_charwise() {
    let buf = TextBuffer::from_text("hello world");
    let mut mbuf = buf;
    let r = apply_operator(
        &mut mbuf, OperatorKind::Yank,
        Range::new(Position::new(0, 0), Position::new(0, 5)),
        false,
    );
    // Yank doesn't modify buffer
    assert_eq!(mbuf.text(), "hello world");
    let dt = r.deleted_text.unwrap();
    assert_eq!(dt.text, "hello");
    assert_eq!(dt.reg_type, RegisterType::Charwise);
}

#[test]
fn yank_linewise() {
    let buf = TextBuffer::from_text("aaa\nbbb\nccc");
    let mut mbuf = buf;
    let r = apply_operator(
        &mut mbuf, OperatorKind::Yank,
        Range::new(Position::new(0, 0), Position::new(1, 0)),
        true,
    );
    assert_eq!(mbuf.text(), "aaa\nbbb\nccc");
    let dt = r.deleted_text.unwrap();
    assert!(dt.text.contains("aaa"));
    assert_eq!(dt.reg_type, RegisterType::Linewise);
}

#[test]
fn yank_cursor_stays() {
    let buf = TextBuffer::from_text("hello world");
    let mut mbuf = buf;
    let r = apply_operator(
        &mut mbuf, OperatorKind::Yank,
        Range::new(Position::new(0, 3), Position::new(0, 8)),
        false,
    );
    assert_eq!(r.new_cursor, Position::new(0, 3));
}

// ──────────── Change operator ────────────

#[test]
fn change_enters_insert_mode() {
    let mut buf = TextBuffer::from_text("hello world");
    let r = apply_operator(
        &mut buf, OperatorKind::Change,
        Range::new(Position::new(0, 0), Position::new(0, 5)),
        false,
    );
    assert!(r.enter_insert);
    assert_eq!(buf.text(), " world");
}

#[test]
fn change_linewise() {
    let mut buf = TextBuffer::from_text("aaa\nbbb\nccc");
    let r = apply_operator(
        &mut buf, OperatorKind::Change,
        Range::new(Position::new(1, 0), Position::new(1, 2)),
        true,
    );
    assert!(r.enter_insert);
    assert!(!buf.text().contains("bbb"));
}

// ──────────── Indent / Outdent operators ────────────

#[test]
fn indent_single_line() {
    let mut buf = TextBuffer::from_text("hello");
    apply_operator(
        &mut buf, OperatorKind::Indent,
        Range::new(Position::new(0, 0), Position::new(0, 4)),
        false,
    );
    assert_eq!(buf.text(), "    hello");
}

#[test]
fn indent_multiple_lines() {
    let mut buf = TextBuffer::from_text("a\nb\nc");
    apply_operator(
        &mut buf, OperatorKind::Indent,
        Range::new(Position::new(0, 0), Position::new(2, 0)),
        false,
    );
    assert!(buf.line_to_string(0).starts_with("    "));
    assert!(buf.line_to_string(1).starts_with("    "));
    assert!(buf.line_to_string(2).starts_with("    "));
}

#[test]
fn outdent_removes_spaces() {
    let mut buf = TextBuffer::from_text("    hello");
    apply_operator(
        &mut buf, OperatorKind::Outdent,
        Range::new(Position::new(0, 0), Position::new(0, 4)),
        false,
    );
    assert_eq!(buf.text(), "hello");
}

#[test]
fn outdent_partial_spaces() {
    let mut buf = TextBuffer::from_text("  hello");
    apply_operator(
        &mut buf, OperatorKind::Outdent,
        Range::new(Position::new(0, 0), Position::new(0, 4)),
        false,
    );
    assert_eq!(buf.text(), "hello");
}

// ──────────── Case operators ────────────

#[test]
fn uppercase_operator() {
    let mut buf = TextBuffer::from_text("hello");
    apply_operator(
        &mut buf, OperatorKind::Uppercase,
        Range::new(Position::new(0, 0), Position::new(0, 5)),
        false,
    );
    assert_eq!(buf.text(), "HELLO");
}

#[test]
fn lowercase_operator() {
    let mut buf = TextBuffer::from_text("HELLO");
    apply_operator(
        &mut buf, OperatorKind::Lowercase,
        Range::new(Position::new(0, 0), Position::new(0, 5)),
        false,
    );
    assert_eq!(buf.text(), "hello");
}

#[test]
fn toggle_case_operator() {
    let mut buf = TextBuffer::from_text("Hello World");
    apply_operator(
        &mut buf, OperatorKind::ToggleCase,
        Range::new(Position::new(0, 0), Position::new(0, 11)),
        false,
    );
    assert_eq!(buf.text(), "hELLO wORLD");
}

#[test]
fn toggle_case_empty() {
    let mut buf = TextBuffer::from_text("hello");
    apply_operator(
        &mut buf, OperatorKind::ToggleCase,
        Range::new(Position::new(0, 2), Position::new(0, 2)),
        false,
    );
    assert_eq!(buf.text(), "hello");
}
