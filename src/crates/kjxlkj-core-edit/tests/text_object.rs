use kjxlkj_core_edit::find_text_object;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range, TextObjectKind};

#[test]
fn inner_word() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 1), TextObjectKind::Word, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 5))));
}

#[test]
fn around_word() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 1), TextObjectKind::Word, false);
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 6))));
}

#[test]
fn inner_double_quote() {
    let buf = TextBuffer::from_text(r#"say "hello" there"#);
    let r = find_text_object(&buf, Position::new(0, 6), TextObjectKind::DoubleQuote, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 5), Position::new(0, 10))));
}

#[test]
fn around_paren() {
    let buf = TextBuffer::from_text("fn(a, b)");
    let r = find_text_object(&buf, Position::new(0, 4), TextObjectKind::Paren, false);
    assert_eq!(r, Some(Range::new(Position::new(0, 2), Position::new(0, 8))));
}

#[test]
fn inner_brace() {
    let buf = TextBuffer::from_text("{ hello }");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Brace, true);
    assert!(r.is_some());
}
