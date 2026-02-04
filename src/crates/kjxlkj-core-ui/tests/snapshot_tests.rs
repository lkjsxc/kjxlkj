//! Snapshot tests - testing the EditorSnapshot immutability and correctness.

use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine, Viewport};
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, Mode};

fn make_snapshot(content: &str, cursor: Cursor, mode: Mode) -> EditorSnapshot {
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let line_count = lines.len().max(1);
    
    EditorSnapshot {
        buffer: BufferSnapshot {
            id: BufferId::new(1),
            name: BufferName::new("test.txt"),
            version: BufferVersion::new(1),
            line_count,
            lines,
            viewport: Viewport::new(0, 24, 0, 80),
            modified: false,
        },
        cursor,
        selection: None,
        mode,
        status: StatusLine {
            mode: format!("{:?}", mode).to_uppercase(),
            file_name: "test.txt".to_string(),
            modified: false,
            line: cursor.line() + 1,
            col: cursor.col() + 1,
            total_lines: line_count,
            message: None,
        },
        command_line: None,
        search_pattern: None,
        width: 80,
        height: 24,
    }
}

#[test]
fn test_snapshot_immutability() {
    let snap1 = make_snapshot("hello", Cursor::new(0, 0), Mode::Normal);
    let snap2 = snap1.clone();
    
    // Clones should be equal
    assert_eq!(snap1.buffer.lines, snap2.buffer.lines);
    assert_eq!(snap1.cursor.line(), snap2.cursor.line());
    assert_eq!(snap1.mode, snap2.mode);
}

#[test]
fn test_snapshot_cursor_visibility() {
    let snap = make_snapshot("hello\nworld\ntest", Cursor::new(1, 2), Mode::Normal);
    
    // Cursor should be within valid range
    assert!(snap.cursor.line() < snap.buffer.line_count);
    let line = &snap.buffer.lines[snap.cursor.line()];
    assert!(snap.cursor.col() <= line.len());
}

#[test]
fn test_snapshot_status_line() {
    let snap = make_snapshot("hello", Cursor::new(0, 3), Mode::Insert);
    
    assert_eq!(snap.status.line, 1); // 1-indexed
    assert_eq!(snap.status.col, 4);  // 1-indexed
    assert!(snap.status.mode.contains("INSERT"));
}

#[test]
fn test_snapshot_empty_buffer() {
    let snap = make_snapshot("", Cursor::new(0, 0), Mode::Normal);
    
    assert!(snap.buffer.lines.is_empty() || snap.buffer.lines[0].is_empty());
    assert_eq!(snap.cursor.line(), 0);
}

#[test]
fn test_snapshot_multiline() {
    let content = "line 1\nline 2\nline 3\nline 4\nline 5";
    let snap = make_snapshot(content, Cursor::new(2, 3), Mode::Normal);
    
    assert_eq!(snap.buffer.line_count, 5);
    assert_eq!(snap.buffer.lines.len(), 5);
    assert_eq!(snap.cursor.line(), 2);
}

#[test]
fn test_snapshot_mode_transitions() {
    let snap_normal = make_snapshot("hello", Cursor::new(0, 0), Mode::Normal);
    let snap_insert = make_snapshot("hello", Cursor::new(0, 0), Mode::Insert);
    let snap_visual = make_snapshot("hello", Cursor::new(0, 0), Mode::Visual);
    
    assert_eq!(snap_normal.mode, Mode::Normal);
    assert_eq!(snap_insert.mode, Mode::Insert);
    assert_eq!(snap_visual.mode, Mode::Visual);
}

#[test]
fn test_snapshot_viewport_bounds() {
    let snap = make_snapshot("hello", Cursor::new(0, 0), Mode::Normal);
    
    assert!(snap.buffer.viewport.height > 0);
    assert!(snap.buffer.viewport.width > 0);
}

#[test]
fn test_snapshot_modified_flag() {
    let mut snap = make_snapshot("hello", Cursor::new(0, 0), Mode::Normal);
    
    assert!(!snap.buffer.modified);
    
    snap.buffer.modified = true;
    assert!(snap.buffer.modified);
    assert!(!snap.status.modified); // status is separate
}

#[test]
fn test_snapshot_command_line() {
    let mut snap = make_snapshot("hello", Cursor::new(0, 0), Mode::Command);
    snap.command_line = Some(":wq".to_string());
    
    assert_eq!(snap.command_line.as_deref(), Some(":wq"));
}

#[test]
fn test_snapshot_search_pattern() {
    let mut snap = make_snapshot("hello world", Cursor::new(0, 0), Mode::Normal);
    snap.search_pattern = Some("world".to_string());
    
    assert_eq!(snap.search_pattern.as_deref(), Some("world"));
}
