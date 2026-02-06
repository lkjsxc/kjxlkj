//! Comprehensive tests for core-edit text objects.

use kjxlkj_core_edit::*;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::*;

// ──────────── Word text object ────────────

#[test]
fn inner_word_at_start() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 0), TextObjectKind::Word, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 5))));
}

#[test]
fn inner_word_at_middle() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 2), TextObjectKind::Word, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 5))));
}

#[test]
fn inner_word_at_end_of_word() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 4), TextObjectKind::Word, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 5))));
}

#[test]
fn around_word_includes_space() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 2), TextObjectKind::Word, false);
    // "hello " (with trailing space)
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 6))));
}

#[test]
fn inner_word_punctuation() {
    let buf = TextBuffer::from_text("foo.bar");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Word, true);
    // The '.' is punctuation, so it's its own "word"
    assert_eq!(r, Some(Range::new(Position::new(0, 3), Position::new(0, 4))));
}

// ──────────── WORD text object ────────────

#[test]
fn inner_big_word() {
    let buf = TextBuffer::from_text("foo.bar baz");
    let r = find_text_object(&buf, Position::new(0, 2), TextObjectKind::WORD, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 7))));
}

#[test]
fn around_big_word() {
    let buf = TextBuffer::from_text("foo.bar baz");
    let r = find_text_object(&buf, Position::new(0, 2), TextObjectKind::WORD, false);
    // "foo.bar " with trailing space
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 8))));
}

// ──────────── Quoted text objects ────────────

#[test]
fn inner_double_quote() {
    let buf = TextBuffer::from_text(r#"say "hello" now"#);
    let r = find_text_object(&buf, Position::new(0, 6), TextObjectKind::DoubleQuote, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 5), Position::new(0, 10))));
}

#[test]
fn around_double_quote() {
    let buf = TextBuffer::from_text(r#"say "hello" now"#);
    let r = find_text_object(&buf, Position::new(0, 6), TextObjectKind::DoubleQuote, false);
    assert_eq!(r, Some(Range::new(Position::new(0, 4), Position::new(0, 11))));
}

#[test]
fn inner_single_quote() {
    let buf = TextBuffer::from_text("say 'hello' now");
    let r = find_text_object(&buf, Position::new(0, 6), TextObjectKind::SingleQuote, true);
    assert!(r.is_some());
}

#[test]
fn inner_backtick() {
    let buf = TextBuffer::from_text("a `code` b");
    let r = find_text_object(&buf, Position::new(0, 4), TextObjectKind::BackTick, true);
    assert!(r.is_some());
}

// ──────────── Delimiter text objects ────────────

#[test]
fn inner_paren() {
    let buf = TextBuffer::from_text("fn(a, b)");
    let r = find_text_object(&buf, Position::new(0, 4), TextObjectKind::Paren, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 3), Position::new(0, 7))));
}

#[test]
fn around_paren() {
    let buf = TextBuffer::from_text("fn(a, b)");
    let r = find_text_object(&buf, Position::new(0, 4), TextObjectKind::Paren, false);
    assert_eq!(r, Some(Range::new(Position::new(0, 2), Position::new(0, 8))));
}

#[test]
fn inner_bracket() {
    let buf = TextBuffer::from_text("[1, 2, 3]");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Bracket, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 1), Position::new(0, 8))));
}

#[test]
fn around_bracket() {
    let buf = TextBuffer::from_text("[1, 2, 3]");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Bracket, false);
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 9))));
}

#[test]
fn inner_brace() {
    let buf = TextBuffer::from_text("{ hello }");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Brace, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 1), Position::new(0, 8))));
}

#[test]
fn around_brace() {
    let buf = TextBuffer::from_text("{ hello }");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Brace, false);
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 9))));
}

#[test]
fn inner_angle_bracket() {
    let buf = TextBuffer::from_text("<div>");
    let r = find_text_object(&buf, Position::new(0, 2), TextObjectKind::AngleBracket, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 1), Position::new(0, 4))));
}

#[test]
fn nested_paren() {
    let buf = TextBuffer::from_text("((inner))");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Paren, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 2), Position::new(0, 7))));
}

#[test]
fn no_matching_paren() {
    let buf = TextBuffer::from_text("hello world");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Paren, true);
    assert_eq!(r, None);
}

// ──────────── Paragraph text object ────────────

#[test]
fn inner_paragraph() {
    let buf = TextBuffer::from_text("abc\ndef\n\nghi");
    let r = find_text_object(&buf, Position::new(0, 0), TextObjectKind::Paragraph, true);
    assert!(r.is_some());
    let r = r.unwrap();
    assert_eq!(r.start.line, 0);
}

#[test]
fn around_paragraph() {
    let buf = TextBuffer::from_text("abc\ndef\n\nghi");
    let r = find_text_object(&buf, Position::new(0, 0), TextObjectKind::Paragraph, false);
    assert!(r.is_some());
}

// ──────────── Sentence text object ────────────

#[test]
fn inner_sentence() {
    let buf = TextBuffer::from_text("Hello world. Foo bar.");
    let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Sentence, true);
    assert!(r.is_some());
}

// ──────────── Tag text object ────────────

#[test]
fn tag_not_implemented() {
    let buf = TextBuffer::from_text("<div>hello</div>");
    let r = find_text_object(&buf, Position::new(0, 6), TextObjectKind::Tag, true);
    assert_eq!(r, None); // Not implemented yet
}

// ──────────── Edge cases ────────────

#[test]
fn text_object_empty_line() {
    let buf = TextBuffer::from_text("");
    let r = find_text_object(&buf, Position::new(0, 0), TextObjectKind::Word, true);
    assert_eq!(r, None);
}

#[test]
fn text_object_single_char() {
    let buf = TextBuffer::from_text("a");
    let r = find_text_object(&buf, Position::new(0, 0), TextObjectKind::Word, true);
    assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 1))));
}

#[test]
fn multiline_paren() {
    let buf = TextBuffer::from_text("(\n  hello\n)");
    let r = find_text_object(&buf, Position::new(1, 3), TextObjectKind::Paren, true);
    assert!(r.is_some());
    let r = r.unwrap();
    assert_eq!(r.start, Position::new(0, 1));
    assert_eq!(r.end, Position::new(2, 0));
}

#[test]
fn multiline_brace() {
    let buf = TextBuffer::from_text("{\n  x\n  y\n}");
    let r = find_text_object(&buf, Position::new(1, 2), TextObjectKind::Brace, false);
    assert!(r.is_some());
    let r = r.unwrap();
    assert_eq!(r.start, Position::new(0, 0));
}
