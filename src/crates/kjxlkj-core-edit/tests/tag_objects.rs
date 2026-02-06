//! Tests for tag text objects (it/at).

use kjxlkj_core_edit::find_text_object;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, TextObjectKind};

fn buf(text: &str) -> TextBuffer {
    TextBuffer::from_text(text)
}

#[test]
fn inner_tag_simple() {
    let b = buf("<div>hello</div>");
    let r = find_text_object(&b, Position::new(0, 6), TextObjectKind::Tag, true).unwrap();
    assert_eq!(r.start, Position::new(0, 5));
    assert_eq!(r.end, Position::new(0, 10));
}

#[test]
fn around_tag_simple() {
    let b = buf("<div>hello</div>");
    let r = find_text_object(&b, Position::new(0, 6), TextObjectKind::Tag, false).unwrap();
    assert_eq!(r.start, Position::new(0, 0));
    assert_eq!(r.end, Position::new(0, 16));
}

#[test]
fn inner_tag_with_attrs() {
    let b = buf("<span class=\"x\">content</span>");
    let r = find_text_object(&b, Position::new(0, 17), TextObjectKind::Tag, true).unwrap();
    assert_eq!(r.start, Position::new(0, 16));
    assert_eq!(r.end, Position::new(0, 23));
}

#[test]
fn tag_multiline() {
    let b = buf("<div>\n  hello\n</div>");
    let r = find_text_object(&b, Position::new(1, 2), TextObjectKind::Tag, true).unwrap();
    assert_eq!(r.start, Position::new(0, 5));
    assert_eq!(r.end, Position::new(2, 0));
}

#[test]
fn tag_multiline_around() {
    let b = buf("<div>\n  hello\n</div>");
    let r = find_text_object(&b, Position::new(1, 2), TextObjectKind::Tag, false).unwrap();
    assert_eq!(r.start, Position::new(0, 0));
    assert_eq!(r.end, Position::new(2, 6));
}

#[test]
fn no_tag_returns_none() {
    let b = buf("just plain text");
    let r = find_text_object(&b, Position::new(0, 3), TextObjectKind::Tag, true);
    assert!(r.is_none());
}

#[test]
fn nested_tags_inner() {
    let b = buf("<div><span>hi</span></div>");
    // Cursor inside "hi" — should find <span>
    let r = find_text_object(&b, Position::new(0, 12), TextObjectKind::Tag, true).unwrap();
    assert_eq!(r.start, Position::new(0, 11));
    assert_eq!(r.end, Position::new(0, 13));
}
