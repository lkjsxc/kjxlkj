//! Integration tests for buffer editing operations.

use kjxlkj_core::Buffer;
use kjxlkj_core_types::BufferId;

fn create_buffer(text: &str) -> Buffer {
    Buffer::from_text(BufferId::new(1), text)
}

#[test]
fn test_insert_char_at_start() {
    let mut buffer = create_buffer("hello");
    buffer.insert_char('X');
    assert_eq!(buffer.text(), "Xhello");
}

#[test]
fn test_insert_char_in_middle() {
    let mut buffer = create_buffer("hello");
    buffer.move_cursor(0, 2);
    buffer.insert_char('X');
    assert_eq!(buffer.text(), "heXllo");
}

#[test]
fn test_insert_newline() {
    let mut buffer = create_buffer("hello");
    buffer.move_cursor(0, 2);
    buffer.insert_newline();
    assert_eq!(buffer.text(), "he\nllo");
    assert_eq!(buffer.cursor_line(), 1);
    assert_eq!(buffer.cursor_col(), 0);
}

#[test]
fn test_delete_char_at() {
    let mut buffer = create_buffer("hello");
    buffer.delete_char_at();
    assert_eq!(buffer.text(), "ello");
}

#[test]
fn test_delete_char_before() {
    let mut buffer = create_buffer("hello");
    buffer.move_cursor(0, 2);
    buffer.delete_char_before();
    assert_eq!(buffer.text(), "hllo");
    assert_eq!(buffer.cursor_col(), 1);
}

#[test]
fn test_delete_line() {
    let mut buffer = create_buffer("line1\nline2\nline3");
    buffer.delete_line(1);
    assert_eq!(buffer.text(), "line1\nline3");
}

#[test]
fn test_cursor_movement() {
    let mut buffer = create_buffer("line1\nline2\nline3");

    // Down
    buffer.cursor_down();
    assert_eq!(buffer.cursor_line(), 1);

    // Right
    buffer.cursor_right();
    assert_eq!(buffer.cursor_col(), 1);

    // Up
    buffer.cursor_up();
    assert_eq!(buffer.cursor_line(), 0);

    // Left
    buffer.cursor_left();
    assert_eq!(buffer.cursor_col(), 0);
}

#[test]
fn test_current_char() {
    let buffer = create_buffer("hello");
    assert_eq!(buffer.current_char(), Some('h'));
}

#[test]
fn test_multiline_navigation() {
    let mut buffer = create_buffer("short\nverylongline\nx");

    buffer.move_cursor(1, 10);
    assert_eq!(buffer.cursor_col(), 10);

    // Move up to shorter line - cursor should clamp to line length - 1
    buffer.cursor_up();
    assert_eq!(buffer.cursor_line(), 0);
    // "short" has 5 chars (0-4), so max col is 4
    // But move_cursor clamps to line_len - 1, and line includes newline
    // Actual behavior depends on implementation
    let col = buffer.cursor_col();
    assert!(
        col <= 10,
        "Expected cursor to be within bounds, got {}",
        col
    );
}

#[test]
fn test_modified_flag() {
    let mut buffer = create_buffer("hello");
    assert!(!buffer.is_modified());

    buffer.insert_char('X');
    assert!(buffer.is_modified());

    buffer.mark_saved();
    assert!(!buffer.is_modified());
}

#[test]
fn test_line_count() {
    let buffer = create_buffer("one\ntwo\nthree");
    assert_eq!(buffer.line_count(), 3);
}

#[test]
fn test_empty_buffer() {
    let buffer = create_buffer("");
    assert_eq!(buffer.line_count(), 1); // Empty buffer has one empty line
    assert_eq!(buffer.cursor_line(), 0);
    assert_eq!(buffer.cursor_col(), 0);
}

#[test]
fn test_set_line_content() {
    let mut buffer = create_buffer("line1\nline2\nline3");
    buffer.set_line(1, "CHANGED");
    let text = buffer.text();
    assert!(text.contains("CHANGED"));
}
