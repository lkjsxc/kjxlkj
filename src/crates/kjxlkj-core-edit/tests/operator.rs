use kjxlkj_core_edit::apply_operator;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{OperatorKind, Position, Range};

#[test]
fn delete_charwise() {
    let mut buf = TextBuffer::from_text("hello world");
    let result = apply_operator(&mut buf, OperatorKind::Delete, Range::new(Position::new(0, 0), Position::new(0, 5)), false);
    assert_eq!(buf.text(), " world");
    assert!(result.deleted_text.is_some());
    assert_eq!(result.deleted_text.unwrap().text, "hello");
}

#[test]
fn change_enters_insert() {
    let mut buf = TextBuffer::from_text("hello world");
    let result = apply_operator(&mut buf, OperatorKind::Change, Range::new(Position::new(0, 0), Position::new(0, 5)), false);
    assert!(result.enter_insert);
}

#[test]
fn indent_outdent() {
    let mut buf = TextBuffer::from_text("hello\nworld");
    apply_operator(&mut buf, OperatorKind::Indent, Range::new(Position::new(0, 0), Position::new(1, 0)), false);
    assert!(buf.text().starts_with("    hello"));
}

#[test]
fn toggle_case() {
    let mut buf = TextBuffer::from_text("Hello");
    apply_operator(&mut buf, OperatorKind::ToggleCase, Range::new(Position::new(0, 0), Position::new(0, 5)), false);
    assert_eq!(buf.text(), "hELLO");
}
