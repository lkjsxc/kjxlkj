//! Tests for text objects.
use kjxlkj_core_edit::find_text_object;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Position, TextObjectScope, TextObjectType};

fn buf(text: &str) -> TextBuffer {
    TextBuffer::from_text(BufferId(1), "t".into(), text)
}

#[test]
fn word_inner_basic() {
    let b = buf("hello world");
    let r = find_text_object(
        &b,
        Position::new(0, 6),
        TextObjectType::Word,
        TextObjectScope::Inner,
    )
    .unwrap();
    assert_eq!(r.start, Position::new(0, 6));
    assert_eq!(r.end, Position::new(0, 11));
}
#[test]
fn word_outer_includes_trailing_space() {
    let b = buf("hello world foo");
    let r = find_text_object(
        &b,
        Position::new(0, 0),
        TextObjectType::Word,
        TextObjectScope::Outer,
    )
    .unwrap();
    assert!(r.end.col > 5);
}
#[test]
fn big_word_inner() {
    let b = buf("foo-bar baz");
    let r = find_text_object(
        &b,
        Position::new(0, 1),
        TextObjectType::BigWord,
        TextObjectScope::Inner,
    )
    .unwrap();
    assert_eq!(r.start.col, 0);
    assert_eq!(r.end.col, 7);
}
#[test]
fn double_quote_inner() {
    let b = buf("say \"hello\" now");
    let r = find_text_object(
        &b,
        Position::new(0, 6),
        TextObjectType::DoubleQuote,
        TextObjectScope::Inner,
    )
    .unwrap();
    assert_eq!(r.start.col, 5);
    assert_eq!(r.end.col, 10);
}
#[test]
fn double_quote_outer() {
    let b = buf("say \"hello\" now");
    let r = find_text_object(
        &b,
        Position::new(0, 6),
        TextObjectType::DoubleQuote,
        TextObjectScope::Outer,
    )
    .unwrap();
    assert_eq!(r.start.col, 4);
    assert_eq!(r.end.col, 11);
}
#[test]
fn single_quote_inner() {
    let b = buf("it's 'fine' ok");
    let r = find_text_object(
        &b,
        Position::new(0, 7),
        TextObjectType::SingleQuote,
        TextObjectScope::Inner,
    )
    .unwrap();
    assert_eq!(r.start.col, 6);
}
#[test]
fn paren_outer() {
    let b = buf("fn(a, b)");
    let r = find_text_object(
        &b,
        Position::new(0, 4),
        TextObjectType::Paren,
        TextObjectScope::Outer,
    )
    .unwrap();
    assert_eq!(r.start.col, 2);
    assert_eq!(r.end.col, 8);
}
#[test]
fn bracket_inner() {
    let b = buf("arr[1, 2]");
    let r = find_text_object(
        &b,
        Position::new(0, 5),
        TextObjectType::Bracket,
        TextObjectScope::Inner,
    )
    .unwrap();
    assert_eq!(r.start.col, 4);
    assert_eq!(r.end.col, 8);
}
#[test]
fn brace_inner() {
    let b = buf("{ x + y }");
    let r = find_text_object(
        &b,
        Position::new(0, 3),
        TextObjectType::Brace,
        TextObjectScope::Inner,
    )
    .unwrap();
    assert_eq!(r.start.col, 1);
    assert_eq!(r.end.col, 8);
}
#[test]
fn paragraph_text_object() {
    let b = buf("line1\nline2\n\nline4\n");
    let r = find_text_object(
        &b,
        Position::new(0, 0),
        TextObjectType::Paragraph,
        TextObjectScope::Inner,
    )
    .unwrap();
    assert_eq!(r.start.line, 0);
    assert_eq!(r.end.line, 1);
}
#[test]
fn sentence_text_object() {
    let b = buf("Hello world. Foo bar.");
    let r = find_text_object(
        &b,
        Position::new(0, 14),
        TextObjectType::Sentence,
        TextObjectScope::Inner,
    )
    .unwrap();
    assert!(r.start.col >= 13);
}
#[test]
fn tag_inner() {
    let b = buf("<div>content</div>");
    let r = find_text_object(
        &b,
        Position::new(0, 1),
        TextObjectType::Tag,
        TextObjectScope::Inner,
    )
    .unwrap();
    assert_eq!(r.start.col, 5);
    assert_eq!(r.end.col, 12);
}
#[test]
fn tag_outer() {
    let b = buf("<div>content</div>");
    let r = find_text_object(
        &b,
        Position::new(0, 1),
        TextObjectType::Tag,
        TextObjectScope::Outer,
    )
    .unwrap();
    assert_eq!(r.start.col, 0);
    assert_eq!(r.end.col, 18);
}
