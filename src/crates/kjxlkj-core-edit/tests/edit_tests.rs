//! Tests for text objects, visual selection, auto-pairs, and extended text objects.
use kjxlkj_core_edit::find_text_object;
use kjxlkj_core_edit::visual::{VisualKind, VisualSelection};
use kjxlkj_core_edit::{default_pairs, should_auto_close, should_skip_over, AutoPairConfig};
use kjxlkj_core_edit::{find_argument, find_entire_buffer, find_indent_level, find_number, TextRange};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Position, Range, TextObjectScope, TextObjectType};

fn buf(text: &str) -> TextBuffer {
    TextBuffer::from_text(BufferId(1), "t".into(), text)
}

#[test]
fn word_inner_basic() {
    let b = buf("hello world");
    let r = find_text_object(&b, Position::new(0, 6), TextObjectType::Word, TextObjectScope::Inner).unwrap();
    assert_eq!(r.start, Position::new(0, 6));
    assert_eq!(r.end, Position::new(0, 11));
}
#[test]
fn word_outer_includes_trailing_space() {
    let b = buf("hello world foo");
    let r = find_text_object(&b, Position::new(0, 0), TextObjectType::Word, TextObjectScope::Outer).unwrap();
    assert!(r.end.col > 5);
}
#[test]
fn big_word_inner() {
    let b = buf("foo-bar baz");
    let r = find_text_object(&b, Position::new(0, 1), TextObjectType::BigWord, TextObjectScope::Inner).unwrap();
    assert_eq!(r.start.col, 0);
    assert_eq!(r.end.col, 7);
}
#[test]
fn double_quote_inner() {
    let b = buf("say \"hello\" now");
    let r = find_text_object(&b, Position::new(0, 6), TextObjectType::DoubleQuote, TextObjectScope::Inner).unwrap();
    assert_eq!(r.start.col, 5);
    assert_eq!(r.end.col, 10);
}
#[test]
fn double_quote_outer() {
    let b = buf("say \"hello\" now");
    let r = find_text_object(&b, Position::new(0, 6), TextObjectType::DoubleQuote, TextObjectScope::Outer).unwrap();
    assert_eq!(r.start.col, 4);
    assert_eq!(r.end.col, 11);
}
#[test]
fn single_quote_inner() {
    let b = buf("it's 'fine' ok");
    let r = find_text_object(&b, Position::new(0, 7), TextObjectType::SingleQuote, TextObjectScope::Inner).unwrap();
    assert_eq!(r.start.col, 6);
}
#[test]
fn paren_outer() {
    let b = buf("fn(a, b)");
    let r = find_text_object(&b, Position::new(0, 4), TextObjectType::Paren, TextObjectScope::Outer).unwrap();
    assert_eq!(r.start.col, 2);
    assert_eq!(r.end.col, 8);
}
#[test]
fn bracket_inner() {
    let b = buf("arr[1, 2]");
    let r = find_text_object(&b, Position::new(0, 5), TextObjectType::Bracket, TextObjectScope::Inner).unwrap();
    assert_eq!(r.start.col, 4);
    assert_eq!(r.end.col, 8);
}
#[test]
fn brace_inner() {
    let b = buf("{ x + y }");
    let r = find_text_object(&b, Position::new(0, 3), TextObjectType::Brace, TextObjectScope::Inner).unwrap();
    assert_eq!(r.start.col, 1);
    assert_eq!(r.end.col, 8);
}
#[test]
fn paragraph_text_object() {
    let b = buf("line1\nline2\n\nline4\n");
    let r = find_text_object(&b, Position::new(0, 0), TextObjectType::Paragraph, TextObjectScope::Inner).unwrap();
    assert_eq!(r.start.line, 0);
    assert_eq!(r.end.line, 1);
}
#[test]
fn sentence_text_object() {
    let b = buf("Hello world. Foo bar.");
    let r = find_text_object(&b, Position::new(0, 14), TextObjectType::Sentence, TextObjectScope::Inner).unwrap();
    assert!(r.start.col >= 13);
}
#[test]
fn tag_inner() {
    let b = buf("<div>content</div>");
    let r = find_text_object(&b, Position::new(0, 1), TextObjectType::Tag, TextObjectScope::Inner).unwrap();
    assert_eq!(r.start.col, 5);
    assert_eq!(r.end.col, 12);
}
#[test]
fn tag_outer() {
    let b = buf("<div>content</div>");
    let r = find_text_object(&b, Position::new(0, 1), TextObjectType::Tag, TextObjectScope::Outer).unwrap();
    assert_eq!(r.start.col, 0);
    assert_eq!(r.end.col, 18);
}
#[test]
fn visual_new_and_contains() {
    let sel = VisualSelection::new(VisualKind::Char, Position::new(0, 2), Position::new(0, 8));
    assert!(sel.contains(Position::new(0, 5)));
    assert!(!sel.contains(Position::new(0, 9)));
}
#[test]
fn visual_line_contains_whole_line() {
    let sel = VisualSelection::new(VisualKind::Line, Position::new(1, 0), Position::new(3, 0));
    assert!(sel.contains(Position::new(2, 99)));
    assert!(!sel.contains(Position::new(0, 0)));
}
#[test]
fn visual_swap_ends() {
    let mut sel = VisualSelection::new(VisualKind::Char, Position::new(0, 0), Position::new(0, 10));
    sel.swap_ends();
    assert_eq!(sel.anchor, Position::new(0, 10));
    assert_eq!(sel.cursor, Position::new(0, 0));
}
#[test]
fn visual_block_cols() {
    let sel = VisualSelection::new(VisualKind::Block, Position::new(0, 10), Position::new(3, 3));
    let (l, r) = sel.block_cols();
    assert_eq!(l, 3);
    assert_eq!(r, 11);
}
#[test]
fn visual_extract_line() {
    let b = buf("aaa\nbbb\nccc\n");
    let sel = VisualSelection::new(VisualKind::Line, Position::new(0, 0), Position::new(1, 0));
    let lines = sel.extract_selection(&b);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "aaa");
}
#[test]
fn auto_close_bracket() {
    let cfg = AutoPairConfig::default();
    assert_eq!(should_auto_close('[', &cfg), Some(']'));
    assert_eq!(should_auto_close('{', &cfg), Some('}'));
}
#[test]
fn skip_over_closing_bracket() {
    let cfg = AutoPairConfig::default();
    assert!(should_skip_over(']', Some(']'), &cfg));
    assert!(!should_skip_over(']', Some('a'), &cfg));
}
#[test]
fn default_pairs_count() {
    assert_eq!(default_pairs().len(), 3);
}
#[test]
fn find_argument_inner() {
    let r = find_argument(&["fn(aaa, bbb, ccc)"], Position::new(0, 9), true).unwrap();
    assert_eq!(r.start.col, 8);
    assert_eq!(r.end.col, 11);
}
#[test]
fn find_indent_level_basic() {
    let lines = ["if true:", "    foo", "    bar", "end"];
    let r = find_indent_level(&lines.map(|s| s), Position::new(1, 0), true).unwrap();
    assert_eq!(r.start.line, 1);
    assert_eq!(r.end.line, 2);
}
#[test]
fn find_entire_buffer_range() {
    let r = find_entire_buffer(5, false);
    assert_eq!(r.start, Position::ZERO);
    assert_eq!(r.end.line, 4);
}
#[test]
fn find_number_decimal() {
    let r = find_number("val = 42;", 6).unwrap();
    assert_eq!(r.start.col, 6);
    assert_eq!(r.end.col, 8);
}
#[test]
fn text_range_empty() {
    let tr = TextRange::new(Position::new(0, 5), Position::new(0, 5));
    assert!(tr.is_empty());
}
