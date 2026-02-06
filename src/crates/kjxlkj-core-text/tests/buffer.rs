use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range};

#[test]
fn new_buffer_empty() {
    let buf = TextBuffer::new();
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.text(), "");
    assert!(!buf.is_modified());
}

#[test]
fn from_text() {
    let buf = TextBuffer::from_text("hello\nworld");
    assert_eq!(buf.line_count(), 2);
    assert_eq!(buf.line_len(0), 5);
    assert_eq!(buf.line_len(1), 5);
}

#[test]
fn insert_char_updates_version() {
    let mut buf = TextBuffer::from_text("ab");
    let v0 = buf.version();
    buf.insert_char(Position::new(0, 1), 'X');
    assert!(buf.version() > v0);
    assert_eq!(buf.text(), "aXb");
}

#[test]
fn delete_range() {
    let mut buf = TextBuffer::from_text("abcdef");
    let del = buf.delete_range(Range::new(Position::new(0, 1), Position::new(0, 4)));
    assert_eq!(del, "bcd");
    assert_eq!(buf.text(), "aef");
}

#[test]
fn clamp_position() {
    let buf = TextBuffer::from_text("hello\nworld");
    let clamped = buf.clamp_position(Position::new(5, 0));
    assert_eq!(clamped.line, 1);
    assert_eq!(buf.clamp_position(Position::new(0, 100)), Position::new(0, 4));
}

#[test]
fn char_at() {
    let buf = TextBuffer::from_text("abc");
    assert_eq!(buf.char_at(Position::new(0, 0)), Some('a'));
    assert_eq!(buf.char_at(Position::new(0, 2)), Some('c'));
}

#[test]
fn delete_line() {
    let mut buf = TextBuffer::from_text("line1\nline2\nline3");
    let del = buf.delete_line(1);
    assert_eq!(del, "line2\n");
    assert_eq!(buf.text(), "line1\nline3");
}
