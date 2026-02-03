//! E2E and script-based tests for kjxlkj.

use std::fs;
use std::path::PathBuf;

use kjxlkj_core::{EditorState, Key, KeyCode};
use kjxlkj_host::run_headless;

/// Helper to create a temporary script file.
fn create_temp_script(content: &str) -> PathBuf {
    let dir = std::env::temp_dir();
    let path = dir.join(format!("kjxlkj_test_{}.json", std::process::id()));
    fs::write(&path, content).unwrap();
    path
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
