//! Render system integration tests.
//!
//! Tests for render diff and terminal output as required by
//! /docs/spec/architecture/runtime.md

use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine, Viewport};
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, Mode};
use kjxlkj_render::RenderDiff;

/// Create a test snapshot.
fn test_snapshot(content: &str) -> EditorSnapshot {
    let viewport = Viewport::new(0, 24, 0, 80);
    EditorSnapshot {
        buffer: BufferSnapshot {
            id: BufferId::new(1),
            name: BufferName::new("test.txt"),
            version: BufferVersion::new(1),
            line_count: content.lines().count().max(1),
            lines: content.lines().map(|s| s.to_string()).collect(),
            viewport,
            modified: false,
        },
        cursor: Cursor::new(0, 0),
        selection: None,
        mode: Mode::Normal,
        status: StatusLine {
            mode: "NORMAL".to_string(),
            file_name: "test.txt".to_string(),
            modified: false,
            line: 1,
            col: 1,
            total_lines: content.lines().count().max(1),
            message: None,
        },
        command_line: None,
        search_pattern: None,
        width: 80,
        height: 24,
    }
}

// =============================================================================
// RenderDiff tests
// =============================================================================

/// Test: Diff between identical snapshots is minimal.
#[test]
fn test_render_diff_identical() {
    let snap1 = test_snapshot("hello");
    let snap2 = snap1.clone();
    
    let diff = RenderDiff::compute(&snap1, &snap2);
    
    // Should have minimal changes
    assert!(diff.changed_lines.is_empty());
    assert!(!diff.cursor_changed);
    assert!(!diff.mode_changed);
    assert!(!diff.status_changed);
}

/// Test: Diff detects cursor movement.
#[test]
fn test_render_diff_cursor_moved() {
    let snap1 = test_snapshot("hello");
    let mut snap2 = snap1.clone();
    snap2.cursor = Cursor::new(0, 2);
    
    let diff = RenderDiff::compute(&snap1, &snap2);
    
    assert!(diff.cursor_changed);
}

/// Test: Diff detects mode change.
#[test]
fn test_render_diff_mode_changed() {
    let snap1 = test_snapshot("hello");
    let mut snap2 = snap1.clone();
    snap2.mode = Mode::Insert;
    snap2.status.mode = "INSERT".to_string();
    
    let diff = RenderDiff::compute(&snap1, &snap2);
    
    assert!(diff.mode_changed);
}

/// Test: Diff detects line changes.
#[test]
fn test_render_diff_line_changed() {
    let snap1 = test_snapshot("line 1\nline 2");
    let mut snap2 = snap1.clone();
    snap2.buffer.lines[0] = "modified".to_string();
    
    let diff = RenderDiff::compute(&snap1, &snap2);
    
    assert!(!diff.changed_lines.is_empty());
    assert!(diff.changed_lines.contains(&0));
}

/// Test: Diff detects status line change.
#[test]
fn test_render_diff_status_changed() {
    let snap1 = test_snapshot("hello");
    let mut snap2 = snap1.clone();
    snap2.status.line = 2;
    snap2.status.col = 5;
    
    let diff = RenderDiff::compute(&snap1, &snap2);
    
    assert!(diff.status_changed);
}

/// Test: Diff detects viewport scroll.
#[test]
fn test_render_diff_viewport_scroll() {
    let content = "line\n".repeat(100);
    let snap1 = test_snapshot(&content);
    let mut snap2 = snap1.clone();
    snap2.buffer.viewport.top_line = 10;
    
    let diff = RenderDiff::compute(&snap1, &snap2);
    
    // Viewport scroll causes all lines to be marked as changed
    assert!(!diff.changed_lines.is_empty());
}

// =============================================================================
// Snapshot structure tests
// =============================================================================

/// Test: Snapshot buffer contains lines.
#[test]
fn test_snapshot_buffer_lines() {
    let snap = test_snapshot("line 1\nline 2\nline 3");
    
    assert_eq!(snap.buffer.lines.len(), 3);
    assert_eq!(snap.buffer.lines[0], "line 1");
    assert_eq!(snap.buffer.lines[1], "line 2");
    assert_eq!(snap.buffer.lines[2], "line 3");
}

/// Test: Snapshot status line has required fields.
#[test]
fn test_snapshot_status_line() {
    let snap = test_snapshot("hello");
    
    assert!(!snap.status.mode.is_empty());
    assert!(!snap.status.file_name.is_empty());
    assert!(snap.status.line >= 1);
}

/// Test: Snapshot viewport.
#[test]
fn test_snapshot_viewport() {
    let snap = test_snapshot("hello");
    
    assert_eq!(snap.buffer.viewport.top_line, 0);
    assert!(snap.buffer.viewport.height > 0);
    assert!(snap.buffer.viewport.width > 0);
}

/// Test: Snapshot mode.
#[test]
fn test_snapshot_mode() {
    let snap = test_snapshot("hello");
    assert_eq!(snap.mode, Mode::Normal);
}

/// Test: Snapshot cursor.
#[test]
fn test_snapshot_cursor() {
    let snap = test_snapshot("hello");
    assert_eq!(snap.cursor.line(), 0);
    assert_eq!(snap.cursor.col(), 0);
}

// =============================================================================
// RenderDiff edge cases
// =============================================================================

/// Test: Diff with empty snapshots.
#[test]
fn test_render_diff_empty() {
    let snap1 = test_snapshot("");
    let snap2 = snap1.clone();
    
    let diff = RenderDiff::compute(&snap1, &snap2);
    
    // Should not crash - just check computation succeeded
    let _ = diff.changed_lines.is_empty();
}

/// Test: Diff with added lines.
#[test]
fn test_render_diff_added_lines() {
    let snap1 = test_snapshot("line 1");
    let mut snap2 = snap1.clone();
    snap2.buffer.lines.push("line 2".to_string());
    snap2.buffer.line_count = 2;
    
    let diff = RenderDiff::compute(&snap1, &snap2);
    
    // Should detect the change
    assert!(!diff.changed_lines.is_empty());
}

/// Test: Diff with removed lines.
#[test]
fn test_render_diff_removed_lines() {
    let snap1 = test_snapshot("line 1\nline 2");
    let mut snap2 = snap1.clone();
    snap2.buffer.lines = vec!["line 1".to_string()];
    snap2.buffer.line_count = 1;
    
    let diff = RenderDiff::compute(&snap1, &snap2);
    
    // Should detect the change
    assert!(!diff.changed_lines.is_empty());
}

/// Test: Diff determinism.
#[test]
fn test_render_diff_determinism() {
    let snap1 = test_snapshot("hello world");
    let mut snap2 = snap1.clone();
    snap2.cursor = Cursor::new(0, 5);
    
    let diff1 = RenderDiff::compute(&snap1, &snap2);
    let diff2 = RenderDiff::compute(&snap1, &snap2);
    
    assert_eq!(diff1.cursor_changed, diff2.cursor_changed);
    assert_eq!(diff1.mode_changed, diff2.mode_changed);
}
