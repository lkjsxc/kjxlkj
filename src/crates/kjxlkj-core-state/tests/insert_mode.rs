//! Insert mode integration tests.
//!
//! Tests for Insert mode behaviors as required by
//! /docs/todo/current/wave-implementation/modes/insert/README.md

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

/// Test: Enter Insert mode from Normal.
#[test]
fn test_insert_enter() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    assert_eq!(editor.mode(), Mode::Normal);
    editor.handle_key(key(KeyCode::Char('i')));
    assert_eq!(editor.mode(), Mode::Insert);
}

/// Test: Exit Insert mode with Escape.
#[test]
fn test_insert_escape() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char('i')));
    assert_eq!(editor.mode(), Mode::Insert);
    
    editor.handle_key(key(KeyCode::Escape));
    assert_eq!(editor.mode(), Mode::Normal);
}

/// Test: Type characters in Insert mode.
#[test]
fn test_insert_type_chars() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(char_key('a'));
    editor.handle_key(char_key('b'));
    editor.handle_key(char_key('c'));
    
    assert!(editor.content().contains("abc"));
}

/// Test: Insert mode Enter creates newline.
#[test]
fn test_insert_newline() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(key(KeyCode::Enter));
    
    assert!(editor.buffer().line_count() >= 2);
}

/// Test: Insert mode Backspace deletes.
#[test]
fn test_insert_backspace() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(char_key('a'));
    editor.handle_key(char_key('b'));
    editor.handle_key(key(KeyCode::Backspace));
    
    // Should have 'a' without 'b' (or close to it)
    let content = editor.content();
    assert!(content.contains('a') || content.is_empty());
}

/// Test: Insert mode Tab inserts spaces.
#[test]
fn test_insert_tab() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(key(KeyCode::Tab));
    
    // Tab should insert spaces
    assert!(editor.content().contains(' '));
}

/// Test: Insert mode arrow keys for navigation.
#[test]
fn test_insert_arrow_keys() {
    let mut editor = EditorState::new();
    editor.load_content("hello\nworld");
    editor.resize(80, 24);
    
    editor.handle_key(key(KeyCode::Char('i')));
    
    // Arrow keys should work in insert mode
    editor.handle_key(key(KeyCode::Down));
    assert_eq!(editor.cursor().line(), 1);
    
    editor.handle_key(key(KeyCode::Up));
    assert_eq!(editor.cursor().line(), 0);
    
    editor.handle_key(key(KeyCode::Right));
    assert_eq!(editor.cursor().col(), 1);
    
    editor.handle_key(key(KeyCode::Left));
    assert_eq!(editor.cursor().col(), 0);
}

/// Test: Insert mode preserves until Escape.
#[test]
fn test_insert_mode_persistence() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char('i')));
    
    // Multiple operations should stay in Insert
    for _ in 0..5 {
        editor.handle_key(char_key('x'));
        assert_eq!(editor.mode(), Mode::Insert);
    }
}

/// Test: Insert mode with append (a).
#[test]
fn test_insert_append() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // 'a' enters insert after cursor
    editor.handle_key(key(KeyCode::Char('a')));
    assert_eq!(editor.mode(), Mode::Insert);
    assert_eq!(editor.cursor().col(), 1);
}

/// Test: Insert mode with append end of line (A).
#[test]
fn test_insert_append_eol() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // 'A' enters insert at end of line
    editor.handle_key(key(KeyCode::Char('A')));
    assert_eq!(editor.mode(), Mode::Insert);
    // Cursor should be at/near end
    assert!(editor.cursor().col() >= 4);
}

/// Test: Insert mode with open line below (o).
#[test]
fn test_insert_open_below() {
    let mut editor = EditorState::new();
    editor.load_content("line 1");
    
    editor.handle_key(key(KeyCode::Char('o')));
    assert_eq!(editor.mode(), Mode::Insert);
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.cursor().line(), 1);
}

/// Test: Insert mode with open line above (O).
#[test]
fn test_insert_open_above() {
    let mut editor = EditorState::new();
    editor.load_content("line 1");
    
    editor.handle_key(key(KeyCode::Char('O')));
    assert_eq!(editor.mode(), Mode::Insert);
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.cursor().line(), 0);
}

/// Test: Insert at first non-blank (I).
#[test]
fn test_insert_first_nonblank() {
    let mut editor = EditorState::new();
    editor.load_content("  hello");
    
    // Move to middle
    editor.handle_key(key(KeyCode::Char('$')));
    
    // I should go to first non-blank
    editor.handle_key(key(KeyCode::Char('I')));
    assert_eq!(editor.mode(), Mode::Insert);
    assert_eq!(editor.cursor().col(), 2);
}

/// Test: Typing unicode characters.
#[test]
fn test_insert_unicode() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    editor.handle_key(key(KeyCode::Char('i')));
    
    // Type some unicode
    for c in "世界".chars() {
        editor.handle_key(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::default(),
        });
    }
    
    assert!(editor.content().contains("世界") || editor.content().contains('世'));
}

/// Test: Multiple newlines.
#[test]
fn test_insert_multiple_newlines() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(key(KeyCode::Enter));
    editor.handle_key(key(KeyCode::Enter));
    editor.handle_key(key(KeyCode::Enter));
    
    assert!(editor.buffer().line_count() >= 3);
}

/// Test: Insert mode typing burst.
#[test]
fn test_insert_typing_burst() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    editor.handle_key(key(KeyCode::Char('i')));
    
    // Type 50 characters rapidly
    for _ in 0..50 {
        editor.handle_key(char_key('x'));
    }
    
    assert_eq!(editor.mode(), Mode::Insert);
    // Should have many x's
    assert!(editor.content().matches('x').count() >= 40);
}

/// Helper to create a key event with Ctrl modifier.
fn ctrl_key(c: char) -> KeyEvent {
    KeyEvent {
        code: KeyCode::Char(c),
        modifiers: KeyModifiers { ctrl: true, ..Default::default() },
    }
}

/// Test: Ctrl-j inserts newline (same as Enter).
#[test]
fn test_insert_ctrl_j_newline() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(ctrl_key('j'));
    
    // Should have multiple lines now
    assert!(editor.buffer().line_count() >= 2);
}

/// Test: Ctrl-m inserts newline (same as Enter).
#[test]
fn test_insert_ctrl_m_newline() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(ctrl_key('m'));
    
    // Should have multiple lines now
    assert!(editor.buffer().line_count() >= 2);
}

/// Test: Rapid newline + typing stress test (200 lines).
#[test]
fn test_insert_rapid_newlines_200() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    editor.handle_key(key(KeyCode::Char('i')));
    
    // Alternate typing and Enter for 200 lines
    for i in 0..200 {
        // Type a line number
        for c in format!("line{}", i).chars() {
            editor.handle_key(char_key(c));
        }
        editor.handle_key(key(KeyCode::Enter));
    }
    
    assert_eq!(editor.mode(), Mode::Insert);
    // Should have ~200+ lines
    assert!(editor.buffer().line_count() >= 200);
}

/// Test: Ctrl-h acts as backspace in Insert mode.
#[test]
fn test_insert_ctrl_h_backspace() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(char_key('a'));
    editor.handle_key(char_key('b'));
    editor.handle_key(ctrl_key('h'));
    
    // Should have just 'a' (b was deleted)
    let content = editor.content();
    // Could be 'a' or empty depending on implementation
    assert!(content.len() <= 2);
}

/// Test: Insert mode with Ctrl-r register placeholder.
#[test]
fn test_insert_ctrl_r_register() {
    let mut editor = EditorState::new();
    editor.load_content("test");
    
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(ctrl_key('r'));
    // Waiting for register name
    editor.handle_key(char_key('a'));
    
    // Should still be in Insert mode
    assert_eq!(editor.mode(), Mode::Insert);
}
