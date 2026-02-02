//! Tests for TextRope.

use super::TextRope;
use kjxlkj_core_types::position::Position;

#[test]
fn test_new_rope_is_empty() {
    let rope = TextRope::new();
    assert!(rope.is_empty());
    assert_eq!(rope.len_chars(), 0);
    assert_eq!(rope.len_lines(), 1); // Empty rope has 1 line
}

#[test]
fn test_from_text() {
    let rope = TextRope::from_text("hello world");
    assert_eq!(rope.len_chars(), 11);
    assert_eq!(rope.len_lines(), 1);
}

#[test]
fn test_multiline() {
    let rope = TextRope::from_text("line1\nline2\nline3");
    assert_eq!(rope.len_lines(), 3);
    assert_eq!(rope.line(0), Some("line1\n".to_string()));
    assert_eq!(rope.line(1), Some("line2\n".to_string()));
    assert_eq!(rope.line(2), Some("line3".to_string()));
}

#[test]
fn test_insert() {
    let mut rope = TextRope::from_text("helloworld");
    rope.insert(5, " ");
    assert_eq!(rope.contents(), "hello world");
}

#[test]
fn test_remove() {
    let mut rope = TextRope::from_text("hello world");
    rope.remove(5, 6);
    assert_eq!(rope.contents(), "helloworld");
}

#[test]
fn test_replace() {
    let mut rope = TextRope::from_text("hello world");
    rope.replace(6, 11, "rust");
    assert_eq!(rope.contents(), "hello rust");
}

#[test]
fn test_char_at() {
    let rope = TextRope::from_text("hello");
    assert_eq!(rope.char_at(0), Some('h'));
    assert_eq!(rope.char_at(4), Some('o'));
    assert_eq!(rope.char_at(5), None);
}

#[test]
fn test_pos_to_char_idx() {
    let rope = TextRope::from_text("line1\nline2");
    assert_eq!(rope.pos_to_char_idx(Position::new(0, 0)), Some(0));
    assert_eq!(rope.pos_to_char_idx(Position::new(0, 4)), Some(4));
    assert_eq!(rope.pos_to_char_idx(Position::new(1, 0)), Some(6));
    assert_eq!(rope.pos_to_char_idx(Position::new(1, 2)), Some(8));
}

#[test]
fn test_char_idx_to_pos() {
    let rope = TextRope::from_text("line1\nline2");
    assert_eq!(rope.char_idx_to_pos(0), Some(Position::new(0, 0)));
    assert_eq!(rope.char_idx_to_pos(6), Some(Position::new(1, 0)));
    assert_eq!(rope.char_idx_to_pos(8), Some(Position::new(1, 2)));
}
