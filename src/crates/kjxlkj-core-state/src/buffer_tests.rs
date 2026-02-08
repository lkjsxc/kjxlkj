//! Tests for BufferState.

use crate::buffer::BufferState;
use kjxlkj_core_types::BufferId;

#[test]
fn new_buffer() {
    let buf = BufferState::new(BufferId(1));
    assert_eq!(buf.name, "[No Name]");
    assert!(!buf.modified);
    assert_eq!(buf.line_count(), 1);
}

#[test]
fn insert_marks_modified() {
    let mut buf = BufferState::new(BufferId(1));
    buf.insert_at(0, 0, "hello");
    assert!(buf.modified);
    assert_eq!(buf.version, 1);
}
