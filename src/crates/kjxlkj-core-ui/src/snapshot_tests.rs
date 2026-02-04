//! Tests for snapshot types.

use super::*;
use kjxlkj_core_types::{Cursor, Mode, Position, Selection};

#[test]
fn snapshot_creation() {
    let snap = EditorSnapshot::empty(Viewport::new(80, 24));
    assert_eq!(snap.mode, Mode::Normal);
    assert_eq!(snap.buffer.lines.len(), 1);
}

#[test]
fn cursor_screen_position() {
    let mut snap = EditorSnapshot::empty(Viewport::new(80, 24));
    snap.cursor = Cursor::new(Position::new(5, 10));
    snap.viewport.first_line = 3;
    let (col, row) = snap.cursor_screen_position();
    assert_eq!(row, 2); // 5 - 3
    assert_eq!(col, 10);
}

#[test]
fn buffer_snapshot_empty() {
    let snap = BufferSnapshot::empty();
    assert_eq!(snap.name, "[No Name]");
    assert!(!snap.modified);
    assert_eq!(snap.total_lines, 1);
}

#[test]
fn status_line_default() {
    let status = StatusLine::default();
    assert!(status.message.is_none());
    assert!(status.command_line.is_none());
}

#[test]
fn snapshot_default() {
    let snap = EditorSnapshot::default();
    assert_eq!(snap.mode, Mode::Normal);
}

#[test]
fn cursor_at_viewport_start() {
    let mut snap = EditorSnapshot::empty(Viewport::new(80, 24));
    snap.cursor = Cursor::new(Position::new(0, 0));
    snap.viewport.first_line = 0;
    let (col, row) = snap.cursor_screen_position();
    assert_eq!(row, 0);
    assert_eq!(col, 0);
}

#[test]
fn snapshot_with_selection() {
    let mut snap = EditorSnapshot::empty(Viewport::new(80, 24));
    snap.selection = Some(Selection::new(
        Position::new(0, 0),
        Position::new(0, 5),
        kjxlkj_core_types::SelectionKind::Char,
    ));
    assert!(snap.selection.is_some());
}

#[test]
fn snapshot_mode_change() {
    let mut snap = EditorSnapshot::default();
    snap.mode = Mode::Insert;
    assert_eq!(snap.mode, Mode::Insert);
}

#[test]
fn buffer_snapshot_modified() {
    let mut snap = BufferSnapshot::empty();
    snap.modified = true;
    assert!(snap.modified);
}

#[test]
fn status_line_with_message() {
    let mut status = StatusLine::default();
    status.message = Some("Test message".to_string());
    assert!(status.message.is_some());
    assert_eq!(status.message.as_ref().unwrap(), "Test message");
}

#[test]
fn status_line_with_command() {
    let mut status = StatusLine::default();
    status.command_line = Some("w".to_string());
    assert!(status.command_line.is_some());
}

#[test]
fn buffer_snapshot_name() {
    let snap = BufferSnapshot::empty();
    assert_eq!(snap.name, "[No Name]");
}

#[test]
fn buffer_snapshot_lines() {
    let snap = BufferSnapshot::empty();
    assert_eq!(snap.lines.len(), 1);
    assert!(snap.lines[0].is_empty());
}

#[test]
fn snapshot_viewport_access() {
    let snap = EditorSnapshot::empty(Viewport::new(120, 40));
    assert_eq!(snap.viewport.width, 120);
    assert_eq!(snap.viewport.height, 40);
}

#[test]
fn buffer_snapshot_version_default() {
    let snap = BufferSnapshot::empty();
    // Version is a BufferVersion type, checking it's accessible
    let _ = snap.version;
}

#[test]
fn buffer_snapshot_id_default() {
    let snap = BufferSnapshot::empty();
    // Id is a BufferId type, checking it's accessible
    let _ = snap.id;
}

#[test]
fn status_line_position() {
    let mut status = StatusLine::default();
    status.position = "10:5".to_string();
    assert_eq!(status.position, "10:5");
}

#[test]
fn status_line_percentage() {
    let mut status = StatusLine::default();
    status.percentage = "50%".to_string();
    assert_eq!(status.percentage, "50%");
}

#[test]
fn status_line_mode() {
    let mut status = StatusLine::default();
    status.mode = "INSERT".to_string();
    assert_eq!(status.mode, "INSERT");
}

#[test]
fn snapshot_cursor_access() {
    let snap = EditorSnapshot::default();
    assert_eq!(snap.cursor.line(), 0);
    assert_eq!(snap.cursor.col(), 0);
}

#[test]
fn status_line_filename() {
    let mut status = StatusLine::default();
    status.filename = "test.rs".to_string();
    assert_eq!(status.filename, "test.rs");
}

#[test]
fn buffer_snapshot_first_line() {
    let snap = BufferSnapshot::empty();
    assert_eq!(snap.first_line, 0);
}

#[test]
fn snapshot_buffer_lines() {
    let snap = EditorSnapshot::default();
    assert!(!snap.buffer.lines.is_empty());
}

#[test]
fn viewport_first_line_default() {
    let snap = EditorSnapshot::default();
    assert_eq!(snap.viewport.first_line, 0);
}
