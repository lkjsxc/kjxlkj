//! Integration tests for core-ui.

use crate::*;
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, LineCol, Mode};

#[test]
fn full_snapshot_workflow() {
    let mut buffer = BufferSnapshot::new(
        BufferId::new(1),
        BufferName::new("main.rs"),
        BufferVersion::new(1),
    );
    buffer.lines = vec![
        "fn main() {".to_string(),
        "    println!(\"Hello\");".to_string(),
        "}".to_string(),
    ];
    buffer.total_lines = 3;

    let viewport = Viewport::new(80, 24);
    let cursor = Cursor::at(LineCol::new(1, 4));

    let snapshot = EditorSnapshot::new(buffer, Mode::Normal, cursor, viewport);

    assert_eq!(snapshot.buffer.total_lines, 3);
    assert_eq!(snapshot.cursor_viewport_position(), Some((4, 1)));
}

#[test]
fn viewport_scrolling_updates() {
    let mut viewport = Viewport::new(80, 10);
    assert_eq!(viewport.scroll_top, 0);

    viewport.scroll_down(5, 100);
    assert_eq!(viewport.scroll_top, 5);

    viewport.scroll_up(3);
    assert_eq!(viewport.scroll_top, 2);
}
