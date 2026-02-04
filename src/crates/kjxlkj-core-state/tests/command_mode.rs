//! Command line mode integration tests.
//!
//! Tests for Command mode behaviors as required by
//! /docs/reference/CONFORMANCE_COMMANDS_TESTING.md

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent, KeyModifiers, Mode};

/// Helper to create a key event.
fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::default(),
    }
}

/// Helper to type a character.
fn char_key(c: char) -> KeyEvent {
    KeyEvent {
        code: KeyCode::Char(c),
        modifiers: KeyModifiers::default(),
    }
}

/// Test: Enter Command mode with colon.
#[test]
fn test_command_enter() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char(':')));
    assert_eq!(editor.mode(), Mode::Command);
}

/// Test: Exit Command mode with Escape.
#[test]
fn test_command_escape() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char(':')));
    assert_eq!(editor.mode(), Mode::Command);
    
    editor.handle_key(key(KeyCode::Escape));
    assert_eq!(editor.mode(), Mode::Normal);
}

/// Test: Execute command with Enter.
#[test]
fn test_command_execute() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char(':')));
    // Type a command
    editor.handle_key(char_key('q'));
    editor.handle_key(key(KeyCode::Enter));
    
    // Should return to normal mode or quit
    assert!(editor.mode() == Mode::Normal || editor.should_quit());
}

/// Test: :q command sets quit flag.
#[test]
fn test_command_quit() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char(':')));
    editor.handle_key(char_key('q'));
    editor.handle_key(key(KeyCode::Enter));
    
    assert!(editor.should_quit());
}

/// Test: :q! command (forced quit).
#[test]
fn test_command_force_quit() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char(':')));
    editor.handle_key(char_key('q'));
    editor.handle_key(char_key('!'));
    editor.handle_key(key(KeyCode::Enter));
    
    assert!(editor.should_quit());
}

/// Test: Type command characters.
#[test]
fn test_command_typing() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char(':')));
    
    // Type "echo"
    for c in "echo".chars() {
        editor.handle_key(char_key(c));
        assert_eq!(editor.mode(), Mode::Command);
    }
}

/// Test: Backspace in command line.
#[test]
fn test_command_backspace() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char(':')));
    editor.handle_key(char_key('a'));
    editor.handle_key(char_key('b'));
    editor.handle_key(key(KeyCode::Backspace));
    
    // Should still be in command mode
    assert_eq!(editor.mode(), Mode::Command);
}

/// Test: Command doesn't modify buffer content directly.
#[test]
fn test_command_no_buffer_modification() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    let before = editor.content();
    
    editor.handle_key(key(KeyCode::Char(':')));
    editor.handle_key(char_key('x'));
    editor.handle_key(char_key('y'));
    editor.handle_key(char_key('z'));
    editor.handle_key(key(KeyCode::Escape));
    
    // Buffer shouldn't have xyz in it from command typing
    assert_eq!(editor.content(), before);
}

/// Test: Unknown command doesn't crash.
#[test]
fn test_command_unknown() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char(':')));
    for c in "unknowncommand".chars() {
        editor.handle_key(char_key(c));
    }
    editor.handle_key(key(KeyCode::Enter));
    
    // Should return to normal mode without crashing
    assert_eq!(editor.mode(), Mode::Normal);
}

/// Test: Multiple command entries.
#[test]
fn test_command_multiple_entries() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    for _ in 0..3 {
        editor.handle_key(key(KeyCode::Char(':')));
        assert_eq!(editor.mode(), Mode::Command);
        editor.handle_key(key(KeyCode::Escape));
        assert_eq!(editor.mode(), Mode::Normal);
    }
}

/// Test: Command mode persistence.
#[test]
fn test_command_persistence() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char(':')));
    
    // Typing should stay in command mode
    for c in "test command".chars() {
        editor.handle_key(char_key(c));
        assert_eq!(editor.mode(), Mode::Command);
    }
}
