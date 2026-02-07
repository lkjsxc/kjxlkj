use super::*;
use kjxlkj_core_types::{BufferId, Position};

#[test]
fn empty_buffer() {
    let b = TextBuffer::new(BufferId(1), "test".into());
    assert_eq!(b.char_count(), 0);
    assert_eq!(b.line_count(), 1); // ropey: empty rope has 1 line
}

#[test]
fn insert_and_read() {
    let mut b = TextBuffer::from_text(BufferId(1), "t".into(), "hello\nworld\n");
    assert_eq!(b.line_count(), 3);
    assert_eq!(b.line(0), Some("hello".into()));
    assert_eq!(b.line(1), Some("world".into()));
    b.insert_char(Position::new(0, 5), '!');
    assert_eq!(b.line(0), Some("hello!".into()));
    assert!(b.is_modified());
}

#[test]
fn delete_range_returns_text() {
    let mut b = TextBuffer::from_text(BufferId(1), "t".into(), "abcdef");
    let d = b.delete_range(Position::new(0, 1), Position::new(0, 4));
    assert_eq!(d, "bcd");
    assert_eq!(b.text(), "aef");
}

#[test]
fn round_trip_position() {
    let b = TextBuffer::from_text(BufferId(1), "t".into(), "abc\ndef\n");
    let pos = Position::new(1, 2);
    let idx = b.pos_to_char_idx(pos);
    assert_eq!(b.char_idx_to_pos(idx), pos);
}
