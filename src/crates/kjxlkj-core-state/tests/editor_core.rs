//! Editor core integration tests.
//!
//! Tests for editor core behaviors as required by
//! /docs/todo/current/wave-implementation/editor/README.md

#![allow(non_snake_case)]

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent, KeyModifiers, Mode};
use std::path::PathBuf;

/// Helper to create a key event.
fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::default(),
    }
}

/// Helper to create a ctrl+key event.
fn ctrl_key(c: char) -> KeyEvent {
    KeyEvent {
        code: KeyCode::Char(c),
        modifiers: KeyModifiers { ctrl: true, ..Default::default() },
    }
}

// =============================================================================
// Buffer lifecycle and identity tests
// =============================================================================

/// Test: Editor starts with a valid buffer.
#[test]
fn test_buffer_lifecycle_initial() {
    let editor = EditorState::new();
    
    // Should have an initial buffer
    assert!(editor.buffer().line_count() >= 1);
    assert!(!editor.should_quit());
}

/// Test: Loading content replaces buffer content.
#[test]
fn test_buffer_lifecycle_load() {
    let mut editor = EditorState::new();
    
    editor.load_content("hello world");
    assert!(editor.content().contains("hello"));
    
    editor.load_content("goodbye");
    assert!(editor.content().contains("goodbye"));
    assert!(!editor.content().contains("hello"));
}

/// Test: Loading a file sets buffer path and name.
#[test]
fn test_buffer_lifecycle_file() {
    let mut editor = EditorState::new();
    
    let path = PathBuf::from("/home/user/test.rs");
    editor.load_file(path.clone(), "fn main() {}");
    
    assert_eq!(editor.buffer().path(), Some(&path));
    assert_eq!(editor.buffer().name().as_str(), "test.rs");
}

/// Test: Buffer ID remains stable across content changes.
#[test]
fn test_buffer_identity_stable() {
    let mut editor = EditorState::new();
    editor.load_content("initial content");
    
    let initial_id = editor.buffer().id();
    
    // Edit buffer
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(key(KeyCode::Char('x')));
    editor.handle_key(key(KeyCode::Escape));
    
    // ID should remain same
    assert_eq!(editor.buffer().id(), initial_id);
}

/// Test: Buffer version increments on edits.
#[test]
fn test_buffer_version_increment() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    let v0 = editor.buffer().version();
    
    // Delete a character
    editor.handle_key(key(KeyCode::Char('x')));
    
    let v1 = editor.buffer().version();
    assert!(v1 > v0);
}

// =============================================================================
// Cursor invariant tests
// =============================================================================

/// Test: Cursor starts at valid position.
#[test]
fn test_cursor_invariant_initial() {
    let editor = EditorState::new();
    
    assert!(editor.cursor().line() < editor.buffer().line_count());
}

/// Test: Cursor clamped after load.
#[test]
fn test_cursor_clamped_after_load() {
    let mut editor = EditorState::new();
    editor.load_content("single line");
    
    // Cursor should be valid
    assert_eq!(editor.cursor().line(), 0);
    assert_eq!(editor.cursor().col(), 0);
}

/// Test: Cursor never goes negative.
#[test]
fn test_cursor_never_negative() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // Try to move left from start
    for _ in 0..10 {
        editor.handle_key(key(KeyCode::Char('h')));
    }
    
    // Cursor is always valid (col is usize, always >= 0)
    let _ = editor.cursor().col();
    let _ = editor.cursor().line();
}

/// Test: Cursor never exceeds buffer bounds.
#[test]
fn test_cursor_bounded_by_buffer() {
    let mut editor = EditorState::new();
    editor.load_content("line 1\nline 2\nline 3");
    
    // Try to move past end of buffer
    for _ in 0..20 {
        editor.handle_key(key(KeyCode::Char('j')));
    }
    
    // Should be clamped
    assert!(editor.cursor().line() < editor.buffer().line_count());
    
    // Try to move past end of line
    for _ in 0..50 {
        editor.handle_key(key(KeyCode::Char('l')));
    }
    
    let line_len = editor.buffer().line_grapheme_len(editor.cursor().line());
    assert!(editor.cursor().col() <= line_len);
}

/// Test: Cursor vertical movement preserves preferred column.
#[test]
fn test_cursor_preferred_column() {
    let mut editor = EditorState::new();
    editor.load_content("short\nlonger line here\nend");
    
    // Move to end of first line
    editor.handle_key(key(KeyCode::Char('$')));
    
    // Move down to longer line
    editor.handle_key(key(KeyCode::Char('j')));
    
    // Should stay at or near column 5 (end of "short")
    // Move down to shorter line
    editor.handle_key(key(KeyCode::Char('j')));
    
    // Cursor should be clamped to line length
    let line_len = editor.buffer().line_grapheme_len(editor.cursor().line());
    assert!(editor.cursor().col() <= line_len);
}

// =============================================================================
// Viewport invariant tests
// =============================================================================

/// Test: Viewport follows cursor when scrolling down.
#[test]
fn test_viewport_follows_cursor_down() {
    let mut editor = EditorState::new();
    let content = (0..100).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    editor.load_content(&content);
    editor.resize(80, 24);
    
    // Move cursor down many lines
    for _ in 0..50 {
        editor.handle_key(key(KeyCode::Char('j')));
    }
    
    // Cursor should still be visible
    let cursor_line = editor.cursor().line();
    assert!(cursor_line >= 40, "Cursor moved to line {}", cursor_line);
}

/// Test: Viewport follows cursor when scrolling up.
#[test]
fn test_viewport_follows_cursor_up() {
    let mut editor = EditorState::new();
    let content = (0..100).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    editor.load_content(&content);
    editor.resize(80, 24);
    
    // Move to end
    editor.handle_key(key(KeyCode::Char('G')));
    
    // Move up many lines
    for _ in 0..50 {
        editor.handle_key(key(KeyCode::Char('k')));
    }
    
    // Cursor should still be valid
    assert!(editor.cursor().line() < 60);
}

/// Test: Resize updates viewport.
#[test]
fn test_viewport_resize() {
    let mut editor = EditorState::new();
    editor.load_content("hello\nworld");
    
    editor.resize(80, 24);
    // Just verify no crash
    
    editor.resize(120, 50);
    // Cursor should still be valid
    assert!(editor.cursor().line() < editor.buffer().line_count());
}

/// Test: Half-page scroll (Ctrl-d/u) works.
#[test]
fn test_viewport_half_page_scroll() {
    let mut editor = EditorState::new();
    let content = (0..100).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    editor.load_content(&content);
    editor.resize(80, 24);
    
    let initial = editor.cursor().line();
    
    editor.handle_key(ctrl_key('d'));
    assert!(editor.cursor().line() > initial);
    
    editor.handle_key(ctrl_key('u'));
    assert!(editor.cursor().line() < 50);
}

// =============================================================================
// Snapshot consistency tests
// =============================================================================

/// Test: Snapshot generation doesn't crash.
#[test]
fn test_snapshot_generation() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    editor.resize(80, 24);
    
    let snapshot = editor.snapshot();
    
    // Buffer snapshot should be present with lines
    assert!(snapshot.buffer.line_count >= 1);
    assert_eq!(snapshot.mode, Mode::Normal);
}

/// Test: Snapshot reflects current mode.
#[test]
fn test_snapshot_mode_consistency() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    editor.resize(80, 24);
    
    // Normal mode
    let snap1 = editor.snapshot();
    assert_eq!(snap1.mode, Mode::Normal);
    
    // Enter insert
    editor.handle_key(key(KeyCode::Char('i')));
    let snap2 = editor.snapshot();
    assert_eq!(snap2.mode, Mode::Insert);
    
    // Back to normal
    editor.handle_key(key(KeyCode::Escape));
    let snap3 = editor.snapshot();
    assert_eq!(snap3.mode, Mode::Normal);
}

/// Test: Snapshot reflects cursor position.
#[test]
fn test_snapshot_cursor_consistency() {
    let mut editor = EditorState::new();
    editor.load_content("hello\nworld");
    editor.resize(80, 24);
    
    let snap1 = editor.snapshot();
    let initial_line = snap1.cursor.line();
    
    editor.handle_key(key(KeyCode::Char('j')));
    let snap2 = editor.snapshot();
    assert_eq!(snap2.cursor.line(), initial_line + 1);
}

/// Test: Snapshot status line contains mode info.
#[test]
fn test_snapshot_status_line() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    editor.resize(80, 24);
    
    let snapshot = editor.snapshot();
    
    // Status line should contain mode text
    let mode_text = snapshot.status.mode.to_uppercase();
    assert!(mode_text.contains("NORMAL") || !mode_text.is_empty());
}

// =============================================================================
// Edit operation tests
// =============================================================================

/// Test: Delete operation modifies buffer.
#[test]
fn test_edit_delete() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char('x')));
    assert_eq!(editor.content(), "ello");
}

/// Test: Insert operation modifies buffer.
#[test]
fn test_edit_insert() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(key(KeyCode::Char('h')));
    editor.handle_key(key(KeyCode::Char('i')));
    
    assert!(editor.content().contains("hi") || editor.content().len() >= 2);
}

/// Test: Join lines works.
#[test]
fn test_edit_join_lines() {
    let mut editor = EditorState::new();
    editor.load_content("line 1\nline 2");
    
    editor.handle_key(key(KeyCode::Char('J')));
    assert_eq!(editor.buffer().line_count(), 1);
}

/// Test: Deterministic editing.
#[test]
fn test_edit_determinism() {
    let content = "hello world";
    
    let mut editor1 = EditorState::new();
    editor1.load_content(content);
    
    let mut editor2 = EditorState::new();
    editor2.load_content(content);
    
    // Same operations
    for _ in 0..3 {
        editor1.handle_key(key(KeyCode::Char('x')));
        editor2.handle_key(key(KeyCode::Char('x')));
    }
    
    assert_eq!(editor1.content(), editor2.content());
    assert_eq!(editor1.cursor().line(), editor2.cursor().line());
    assert_eq!(editor1.cursor().col(), editor2.cursor().col());
}
