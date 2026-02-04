//! Buffer tests.

use super::TextBuffer;
use kjxlkj_core_types::{BufferId, Position};

#[test]
fn buffer_insert_and_read() {
    let mut buf = TextBuffer::new(BufferId::new(1));
    buf.insert(Position::new(0, 0), "Hello, World!");
    assert_eq!(buf.line(0), Some("Hello, World!".to_string()));
    assert_eq!(buf.line_count(), 1);
}

#[test]
fn buffer_multiline() {
    let buf = TextBuffer::from_str(BufferId::new(1), "Line 1\nLine 2\nLine 3");
    assert_eq!(buf.line_count(), 3);
    assert_eq!(buf.line(0), Some("Line 1".to_string()));
}

#[test]
fn buffer_delete_range() {
    let mut buf = TextBuffer::from_str(BufferId::new(1), "Hello, World!");
    buf.delete_range(Position::new(0, 5), Position::new(0, 7));
    assert_eq!(buf.line(0), Some("HelloWorld!".to_string()));
}

#[test]
fn buffer_position_conversion() {
    let buf = TextBuffer::from_str(BufferId::new(1), "AB\nCD\nEF");
    assert_eq!(buf.pos_to_char_idx(Position::new(0, 0)), Some(0));
    assert_eq!(buf.pos_to_char_idx(Position::new(1, 0)), Some(3));
    assert_eq!(buf.char_idx_to_pos(3), Position::new(1, 0));
}

#[test]
fn buffer_empty() {
    let buf = TextBuffer::new(BufferId::new(1));
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.char_count(), 0);
}

#[test]
fn buffer_insert_newline() {
    let mut buf = TextBuffer::new(BufferId::new(1));
    buf.insert(Position::new(0, 0), "Hello\nWorld");
    assert_eq!(buf.line_count(), 2);
    assert_eq!(buf.line(0), Some("Hello".to_string()));
}

#[test]
fn buffer_modified_flag() {
    let mut buf = TextBuffer::new(BufferId::new(1));
    assert!(!buf.is_modified());
    buf.insert(Position::new(0, 0), "x");
    assert!(buf.is_modified());
    buf.mark_saved();
    assert!(!buf.is_modified());
}

#[test]
fn buffer_delete_cross_line() {
    let mut buf = TextBuffer::from_str(BufferId::new(1), "Line 1\nLine 2");
    buf.delete_range(Position::new(0, 4), Position::new(1, 4));
    assert_eq!(buf.line_count(), 1);
}

#[test]
fn buffer_text_range() {
    let buf = TextBuffer::from_str(BufferId::new(1), "ABCDEF");
    let text = buf.text_range(Position::new(0, 1), Position::new(0, 4));
    assert_eq!(text, Some("BCD".to_string()));
}

#[test]
fn buffer_version_increments() {
    let mut buf = TextBuffer::new(BufferId::new(1));
    let v1 = buf.version();
    buf.insert(Position::new(0, 0), "x");
    let v2 = buf.version();
    assert!(v2 > v1);
}
