//! Integration tests for core-state.

use crate::*;
use kjxlkj_core_types::BufferName;

#[test]
fn editor_workflow() {
    let mut editor = EditorState::new();

    // Open a file
    editor.open_buffer(BufferName::new("main.rs"), "fn main() {\n}\n");
    assert_eq!(editor.active_buffer().line_count(), 3);

    // Edit the buffer
    editor.active_buffer_mut().insert_at_cursor("// comment\n");
    assert!(editor.active_buffer().modified);

    // Generate snapshot
    let snap = editor.snapshot();
    assert!(snap.buffer.modified);
}

#[test]
fn cursor_movement() {
    let mut editor = EditorState::new();
    editor.open_buffer(
        BufferName::new("test"),
        "abcdef\nghijkl\nmnopqr",
    );

    editor.active_buffer_mut().move_cursor_lines(1);
    assert_eq!(editor.active_buffer().cursor.position.line, 1);

    editor.active_buffer_mut().move_cursor_cols(3);
    assert_eq!(editor.active_buffer().cursor.position.col, 3);
}

#[test]
fn viewport_follows_cursor() {
    let mut editor = EditorState::new();
    // Create a buffer with many lines
    let content: String = (0..100).map(|i| format!("line {}\n", i)).collect();
    editor.open_buffer(BufferName::new("test"), &content);
    editor.resize(80, 24);

    // Move cursor to line 50
    editor.active_buffer_mut().cursor.position.line = 50;
    editor.ensure_cursor_visible();

    assert!(editor.viewport.is_line_visible(50));
}
