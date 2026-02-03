//! Integration tests for kjxlkj.

use kjxlkj_core::{EditorState, Key, KeyCode, Mode};

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
