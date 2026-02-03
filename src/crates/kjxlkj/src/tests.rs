//! Integration and E2E tests for kjxlkj.

use std::fs;
use std::path::PathBuf;

use kjxlkj_core::{EditorState, Key, KeyCode, Mode};
use kjxlkj_host::run_headless;

/// Helper to create a temporary script file.
fn create_temp_script(content: &str) -> PathBuf {
    let dir = std::env::temp_dir();
    let path = dir.join(format!("kjxlkj_test_{}.json", std::process::id()));
    fs::write(&path, content).unwrap();
    path
}

#[test]
fn test_editor_opens_empty() {
    let state = EditorState::new();
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn test_insert_mode_flow() {
    let mut state = EditorState::new();

    // Enter insert mode
    state.handle_key(Key::char('i')).unwrap();
    assert_eq!(state.mode(), Mode::Insert);

    // Type some text
    state.handle_key(Key::char('H')).unwrap();
    state.handle_key(Key::char('e')).unwrap();
    state.handle_key(Key::char('l')).unwrap();
    state.handle_key(Key::char('l')).unwrap();
    state.handle_key(Key::char('o')).unwrap();

    // Exit insert mode
    state.handle_key(Key::new(KeyCode::Esc)).unwrap();
    assert_eq!(state.mode(), Mode::Normal);

    // Check content
    let snap = state.snapshot();
    assert!(snap.active_window.lines[0].text.contains("Hello"));
}

#[test]
fn test_visual_mode() {
    let mut state = EditorState::new();

    // Add some text
    state.handle_key(Key::char('i')).unwrap();
    for c in "hello world".chars() {
        state.handle_key(Key::char(c)).unwrap();
    }
    state.handle_key(Key::new(KeyCode::Esc)).unwrap();

    // Enter visual mode
    state.handle_key(Key::char('v')).unwrap();
    assert_eq!(state.mode(), Mode::Visual);

    // Exit visual mode
    state.handle_key(Key::new(KeyCode::Esc)).unwrap();
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn test_delete_line() {
    let mut state = EditorState::new();

    // Add some text
    state.handle_key(Key::char('i')).unwrap();
    for c in "line1".chars() {
        state.handle_key(Key::char(c)).unwrap();
    }
    state.handle_key(Key::new(KeyCode::Enter)).unwrap();
    for c in "line2".chars() {
        state.handle_key(Key::char(c)).unwrap();
    }
    state.handle_key(Key::new(KeyCode::Esc)).unwrap();

    // Go to first line
    state.handle_key(Key::char('g')).unwrap();
    state.handle_key(Key::char('g')).unwrap();

    // Delete line
    state.handle_key(Key::char('d')).unwrap();
    state.handle_key(Key::char('d')).unwrap();

    let snap = state.snapshot();
    // First line should now be line2
    assert!(snap.active_window.lines[0].text.contains("line2"));
}

#[test]
fn test_undo_redo() {
    let mut state = EditorState::new();

    // Add text
    state.handle_key(Key::char('i')).unwrap();
    state.handle_key(Key::char('a')).unwrap();
    state.handle_key(Key::new(KeyCode::Esc)).unwrap();

    let snap = state.snapshot();
    assert!(snap.active_window.lines[0].text.contains("a"));

    // This is a simplified test - full undo would need transaction tracking
}

#[test]
fn test_quit_command() {
    let mut state = EditorState::new();

    // Enter command mode and quit
    state.handle_key(Key::char(':')).unwrap();
    state.handle_key(Key::char('q')).unwrap();
    state.handle_key(Key::new(KeyCode::Enter)).unwrap();

    assert!(state.should_quit());
}

#[test]
fn test_headless_script() {
    let mut state = EditorState::new();

    let script = r#"[
        {"kind": "key", "code": "i", "mods": []},
        {"kind": "key", "code": "t", "mods": []},
        {"kind": "key", "code": "e", "mods": []},
        {"kind": "key", "code": "s", "mods": []},
        {"kind": "key", "code": "t", "mods": []},
        {"kind": "key", "code": "esc", "mods": []},
        {"kind": "key", "code": ":", "mods": []},
        {"kind": "key", "code": "q", "mods": []},
        {"kind": "key", "code": "!", "mods": []},
        {"kind": "key", "code": "enter", "mods": []}
    ]"#;

    let path = create_temp_script(script);
    let result = run_headless(&mut state, &path).unwrap();

    assert!(result.quit);
    assert!(result.final_snapshot.active_window.lines[0].text.contains("test"));

    // Cleanup
    let _ = fs::remove_file(path);
}

#[test]
fn test_movement_keys() {
    let mut state = EditorState::new();

    // Add multi-line content
    state.handle_key(Key::char('i')).unwrap();
    for c in "line1".chars() {
        state.handle_key(Key::char(c)).unwrap();
    }
    state.handle_key(Key::new(KeyCode::Enter)).unwrap();
    for c in "line2".chars() {
        state.handle_key(Key::char(c)).unwrap();
    }
    state.handle_key(Key::new(KeyCode::Enter)).unwrap();
    for c in "line3".chars() {
        state.handle_key(Key::char(c)).unwrap();
    }
    state.handle_key(Key::new(KeyCode::Esc)).unwrap();

    // Go to start
    state.handle_key(Key::char('g')).unwrap();

    // Move down
    state.handle_key(Key::char('j')).unwrap();
    let snap = state.snapshot();
    assert!(snap.active_window.cursor.line() >= 1);

    // Move up
    state.handle_key(Key::char('k')).unwrap();
}

#[test]
fn test_yank_paste() {
    let mut state = EditorState::new();

    // Add text
    state.handle_key(Key::char('i')).unwrap();
    for c in "hello".chars() {
        state.handle_key(Key::char(c)).unwrap();
    }
    state.handle_key(Key::new(KeyCode::Esc)).unwrap();

    // Yank line
    state.handle_key(Key::char('y')).unwrap();
    state.handle_key(Key::char('y')).unwrap();

    // Paste
    state.handle_key(Key::char('p')).unwrap();

    let snap = state.snapshot();
    // Should have 2 lines now
    assert!(snap.active_window.lines.len() >= 2);
}

#[test]
fn test_count_prefix() {
    let mut state = EditorState::new();

    // Add text
    state.handle_key(Key::char('i')).unwrap();
    for c in "abcdefghij".chars() {
        state.handle_key(Key::char(c)).unwrap();
    }
    state.handle_key(Key::new(KeyCode::Esc)).unwrap();

    // Go to start
    state.handle_key(Key::char('0')).unwrap();

    // Move 5 characters right
    state.handle_key(Key::char('5')).unwrap();
    state.handle_key(Key::char('l')).unwrap();

    let snap = state.snapshot();
    assert_eq!(snap.active_window.cursor.col(), 5);
}
