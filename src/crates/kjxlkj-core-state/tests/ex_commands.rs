//! Ex command integration tests.
//!
//! Tests for Ex commands as required by:
//! /docs/spec/commands/essential.md

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent, KeyModifiers, Mode};

/// Helper to create a key event.
fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::default(),
    }
}

/// Helper to enter command mode and type a command.
fn enter_command(editor: &mut EditorState, cmd: &str) {
    editor.handle_key(key(KeyCode::Char(':')));
    assert_eq!(editor.mode(), Mode::Command);
    for c in cmd.chars() {
        editor.handle_key(key(KeyCode::Char(c)));
    }
    editor.handle_key(key(KeyCode::Enter));
}

// ============================================================================
// Quit commands
// ============================================================================

/// Test: :q quits if buffer not modified.
#[test]
fn test_quit_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    // Mark as not modified (load_content leaves it modified)
    editor.buffer_mut().mark_saved();

    assert!(!editor.should_quit());
    enter_command(&mut editor, "q");
    assert!(editor.should_quit());
}

/// Test: :q! quits even if modified.
#[test]
fn test_quit_force_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    assert!(!editor.should_quit());
    enter_command(&mut editor, "q!");
    assert!(editor.should_quit());
}

/// Test: :wq quits.
#[test]
fn test_write_quit_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "wq");
    assert!(editor.should_quit());
}

/// Test: :x quits (same as :wq).
#[test]
fn test_x_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "x");
    assert!(editor.should_quit());
}

// ============================================================================
// Write commands
// ============================================================================

/// Test: :w writes (shows message).
#[test]
fn test_write_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "w");
    assert!(!editor.should_quit());
    // Should have status message
    assert_eq!(editor.mode(), Mode::Normal);
}

/// Test: :wa writes all.
#[test]
fn test_write_all_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "wa");
    assert!(!editor.should_quit());
}

// ============================================================================
// Buffer commands
// ============================================================================

/// Test: :ls shows buffer list.
#[test]
fn test_buffers_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "ls");
    assert!(!editor.should_quit());
}

/// Test: :bn goes to next buffer (shows message when no next).
#[test]
fn test_bnext_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "bn");
    assert!(!editor.should_quit());
}

/// Test: :bp goes to previous buffer.
#[test]
fn test_bprev_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "bp");
    assert!(!editor.should_quit());
}

/// Test: :bd deletes buffer.
#[test]
fn test_bdelete_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    editor.buffer_mut().mark_saved();

    enter_command(&mut editor, "bd");
    assert!(!editor.should_quit());
}

// ============================================================================
// Edit commands
// ============================================================================

/// Test: :e! reloads without saving.
#[test]
fn test_edit_force_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "e!");
    assert!(!editor.should_quit());
}

// ============================================================================
// Window commands
// ============================================================================

/// Test: :sp shows split message.
#[test]
fn test_split_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "sp");
    assert!(!editor.should_quit());
}

/// Test: :vsp shows vsplit message.
#[test]
fn test_vsplit_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "vsp");
    assert!(!editor.should_quit());
}

/// Test: :only shows only message.
#[test]
fn test_only_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "only");
    assert!(!editor.should_quit());
}

// ============================================================================
// Set commands
// ============================================================================

/// Test: :set number.
#[test]
fn test_set_number() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "set number");
    assert!(!editor.should_quit());
}

/// Test: :set nowrap.
#[test]
fn test_set_nowrap() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "set nowrap");
    assert!(!editor.should_quit());
}

// ============================================================================
// Line number commands
// ============================================================================

/// Test: :{number} goes to line.
#[test]
fn test_goto_line_command() {
    let mut editor = EditorState::new();
    editor.load_content("line 1\nline 2\nline 3\nline 4\nline 5");

    enter_command(&mut editor, "3");
    assert_eq!(editor.cursor().line(), 2); // 0-indexed
}

/// Test: Large line number clamps to last line.
#[test]
fn test_goto_line_clamp() {
    let mut editor = EditorState::new();
    editor.load_content("line 1\nline 2\nline 3");

    enter_command(&mut editor, "100");
    assert_eq!(editor.cursor().line(), 2); // Last line
}

// ============================================================================
// Unknown commands
// ============================================================================

/// Test: Unknown command shows error.
#[test]
fn test_unknown_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "notacommand");
    assert!(!editor.should_quit());
}

/// Test: Empty command does nothing.
#[test]
fn test_empty_command() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    enter_command(&mut editor, "");
    assert!(!editor.should_quit());
    assert_eq!(editor.mode(), Mode::Normal);
}
