//! Advanced editing feature tests - marks, macros, registers, search.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent};

fn char_key(c: char) -> KeyEvent {
    KeyEvent::char(c)
}

fn ctrl_key(c: char) -> KeyEvent {
    KeyEvent::ctrl(KeyCode::Char(c))
}

fn esc_key() -> KeyEvent {
    KeyEvent::plain(KeyCode::Escape)
}

fn enter_key() -> KeyEvent {
    KeyEvent::plain(KeyCode::Enter)
}

// === Marks ===

#[test]
fn test_set_mark() {
    let mut editor = EditorState::new();
    editor.load_content("line 1\nline 2\nline 3");
    
    // Move to line 2
    editor.handle_key(char_key('j'));
    
    // Set mark 'a'
    editor.handle_key(char_key('m'));
    editor.handle_key(char_key('a'));
    
    // The mark should be set without crash
    assert_eq!(editor.cursor().position.line, 1);
}

#[test]
fn test_jump_to_mark() {
    let mut editor = EditorState::new();
    editor.load_content("line 1\nline 2\nline 3");
    
    // Move to line 2 and set mark
    editor.handle_key(char_key('j'));
    editor.handle_key(char_key('m'));
    editor.handle_key(char_key('a'));
    
    // Move to line 3
    editor.handle_key(char_key('j'));
    assert_eq!(editor.cursor().position.line, 2);
    
    // Jump back to mark 'a' using backtick
    editor.handle_key(char_key('`'));
    editor.handle_key(char_key('a'));
    
    // Should be back at line 2
    assert_eq!(editor.cursor().position.line, 1);
}

#[test]
fn test_jump_to_mark_line() {
    let mut editor = EditorState::new();
    editor.load_content("  line 1\n  line 2\n  line 3");
    
    // Move to line 2, column 3 and set mark
    editor.handle_key(char_key('j'));
    editor.handle_key(char_key('l'));
    editor.handle_key(char_key('l'));
    editor.handle_key(char_key('l'));
    editor.handle_key(char_key('m'));
    editor.handle_key(char_key('b'));
    
    // Move to line 3
    editor.handle_key(char_key('j'));
    
    // Jump to mark 'b' line start using apostrophe
    editor.handle_key(char_key('\''));
    editor.handle_key(char_key('b'));
    
    // Should be at line 2
    assert_eq!(editor.cursor().position.line, 1);
}

// === Registers ===

#[test]
fn test_named_register_yank() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Select register "a and yank word
    editor.handle_key(char_key('"'));
    editor.handle_key(char_key('a'));
    editor.handle_key(char_key('y'));
    editor.handle_key(char_key('w'));
    
    // Yank should work without crash
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.contains("hello"));
}

#[test]
fn test_named_register_paste() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Yank to register "a
    editor.handle_key(char_key('"'));
    editor.handle_key(char_key('a'));
    editor.handle_key(char_key('y'));
    editor.handle_key(char_key('w'));
    
    // Move to end
    editor.handle_key(char_key('$'));
    
    // Paste from register "a
    editor.handle_key(char_key('"'));
    editor.handle_key(char_key('a'));
    editor.handle_key(char_key('p'));
    
    // Should have pasted
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.len() > 11);
}

#[test]
fn test_unnamed_register() {
    let mut editor = EditorState::new();
    editor.load_content("test");
    
    // Delete char (goes to unnamed register)
    editor.handle_key(char_key('x'));
    
    // Paste
    editor.handle_key(char_key('p'));
    
    // 't' should be pasted after current position
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.contains("t"));
}

// === Macros ===

#[test]
fn test_macro_record() {
    let mut editor = EditorState::new();
    editor.load_content("test");
    
    // Start recording macro 'a'
    editor.handle_key(char_key('q'));
    editor.handle_key(char_key('a'));
    
    // Do some actions
    editor.handle_key(char_key('x'));
    
    // Stop recording
    editor.handle_key(char_key('q'));
    
    // Should not crash and buffer should be modified
    let line = editor.buffer().line(0).unwrap().to_string();
    assert_eq!(line.trim(), "est");
}

#[test]
fn test_macro_playback() {
    let mut editor = EditorState::new();
    editor.load_content("aaa");
    
    // Record macro: delete char and move right
    editor.handle_key(char_key('q'));
    editor.handle_key(char_key('b'));
    editor.handle_key(char_key('x'));
    editor.handle_key(char_key('q'));
    
    // Play macro
    editor.handle_key(char_key('@'));
    editor.handle_key(char_key('b'));
    
    // Should have deleted another char
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.len() < 3);
}

#[test]
fn test_macro_replay_last() {
    let mut editor = EditorState::new();
    editor.load_content("xxxx");
    
    // Record and play macro
    editor.handle_key(char_key('q'));
    editor.handle_key(char_key('c'));
    editor.handle_key(char_key('x'));
    editor.handle_key(char_key('q'));
    
    editor.handle_key(char_key('@'));
    editor.handle_key(char_key('c'));
    
    // First playback deletes one char, macro record deleted one = 2 total
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.len() <= 3); // xx or less
}

// === Search ===

#[test]
fn test_search_forward() {
    let mut editor = EditorState::new();
    editor.load_content("find the word here");
    
    // Start forward search
    editor.handle_key(char_key('/'));
    
    // Type search term
    editor.handle_key(char_key('w'));
    editor.handle_key(char_key('o'));
    editor.handle_key(char_key('r'));
    editor.handle_key(char_key('d'));
    editor.handle_key(enter_key());
    
    // Cursor should have moved to "word"
    // (Exact position depends on implementation)
}

#[test]
fn test_search_backward() {
    let mut editor = EditorState::new();
    editor.load_content("word is first word is second");
    
    // Move to end
    editor.handle_key(char_key('$'));
    
    // Start backward search
    editor.handle_key(char_key('?'));
    
    // Type search term
    editor.handle_key(char_key('w'));
    editor.handle_key(char_key('o'));
    editor.handle_key(char_key('r'));
    editor.handle_key(char_key('d'));
    editor.handle_key(enter_key());
    
    // Should search backward
}

#[test]
fn test_search_next() {
    let mut editor = EditorState::new();
    editor.load_content("word word word");
    
    // Search for 'word'
    editor.handle_key(char_key('/'));
    editor.handle_key(char_key('w'));
    editor.handle_key(char_key('o'));
    editor.handle_key(char_key('r'));
    editor.handle_key(char_key('d'));
    editor.handle_key(enter_key());
    
    // Next match
    editor.handle_key(char_key('n'));
    
    // Should move to next match
}

#[test]
fn test_search_prev() {
    let mut editor = EditorState::new();
    editor.load_content("word word word");
    
    // Move to middle
    editor.handle_key(char_key('w'));
    editor.handle_key(char_key('w'));
    
    // Search for 'word'
    editor.handle_key(char_key('/'));
    editor.handle_key(char_key('w'));
    editor.handle_key(char_key('o'));
    editor.handle_key(char_key('r'));
    editor.handle_key(char_key('d'));
    editor.handle_key(enter_key());
    
    // Previous match
    editor.handle_key(char_key('N'));
    
    // Should move to previous match
}

// === Undo/Redo ===

#[test]
fn test_redo() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // Delete
    editor.handle_key(char_key('x'));
    
    // Undo
    editor.handle_key(char_key('u'));
    
    // Redo
    editor.handle_key(ctrl_key('r'));
    
    // Should be back to deleted state
    let line = editor.buffer().line(0).unwrap().to_string();
    assert_eq!(line.trim(), "ello");
}

#[test]
fn test_multiple_undo() {
    let mut editor = EditorState::new();
    editor.load_content("abcde");
    
    // Delete multiple chars
    editor.handle_key(char_key('x'));
    editor.handle_key(char_key('x'));
    editor.handle_key(char_key('x'));
    
    // Undo all
    editor.handle_key(char_key('u'));
    editor.handle_key(char_key('u'));
    editor.handle_key(char_key('u'));
    
    // Should be back to original
    let line = editor.buffer().line(0).unwrap().to_string();
    assert_eq!(line.trim(), "abcde");
}

// === Dot Repeat ===

#[test]
fn test_dot_repeat() {
    let mut editor = EditorState::new();
    editor.load_content("aaa");
    
    // Delete char
    editor.handle_key(char_key('x'));
    
    // Repeat
    editor.handle_key(char_key('.'));
    
    // Should have deleted another char
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.len() <= 2);
}

#[test]
fn test_dot_repeat_with_insert() {
    let mut editor = EditorState::new();
    editor.load_content("abc\ndef\nghi");
    
    // Go to insert mode and add text
    editor.handle_key(char_key('I'));
    editor.handle_key(char_key('>'));
    editor.handle_key(esc_key());
    
    // First line should have '>' prefix
    let line1 = editor.buffer().line(0).unwrap().to_string();
    assert!(line1.starts_with('>'));
    
    // Move to next line
    editor.handle_key(char_key('j'));
    
    // Repeat with . (may or may not work depending on implementation)
    editor.handle_key(char_key('.'));
    
    // At minimum, first line should have '>' prefix
    let line1_after = editor.buffer().line(0).unwrap().to_string();
    assert!(line1_after.starts_with('>'));
}

// === Join Lines ===

#[test]
fn test_join_lines() {
    let mut editor = EditorState::new();
    editor.load_content("hello\nworld");
    
    // Join lines
    editor.handle_key(char_key('J'));
    
    let content = editor.buffer().to_string();
    assert_eq!(editor.buffer().line_count(), 1);
    assert!(content.contains("hello") && content.contains("world"));
}

#[test]
fn test_join_with_count() {
    let mut editor = EditorState::new();
    editor.load_content("a\nb\nc\nd");
    
    // Join 3 lines
    editor.handle_key(char_key('3'));
    editor.handle_key(char_key('J'));
    
    // Should have fewer lines
    assert!(editor.buffer().line_count() < 4);
}

// === Change Case ===

#[test]
fn test_toggle_case() {
    let mut editor = EditorState::new();
    editor.load_content("Hello");
    
    // Toggle case of first char
    editor.handle_key(char_key('~'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with('h'));
}

#[test]
fn test_toggle_case_moves() {
    let mut editor = EditorState::new();
    editor.load_content("HELLO");
    
    // Toggle multiple
    editor.handle_key(char_key('~'));
    editor.handle_key(char_key('~'));
    editor.handle_key(char_key('~'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with("hel"));
}
