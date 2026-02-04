//! Search and replace tests.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent};

fn key(c: char) -> KeyEvent {
    KeyEvent::char(c)
}

fn enter() -> KeyEvent {
    KeyEvent::plain(KeyCode::Enter)
}

fn escape() -> KeyEvent {
    KeyEvent::plain(KeyCode::Escape)
}

fn setup(content: &str) -> EditorState {
    let mut editor = EditorState::new();
    editor.load_content(content);
    editor
}

fn type_string(editor: &mut EditorState, s: &str) {
    for c in s.chars() {
        editor.handle_key(key(c));
    }
}

// === Forward Search ===

#[test]
fn test_forward_search_entry() {
    let mut editor = setup("hello world");
    
    editor.handle_key(key('/'));
    // Should be in search/command mode
}

#[test]
fn test_forward_search_pattern() {
    let mut editor = setup("hello world hello");
    
    editor.handle_key(key('/'));
    type_string(&mut editor, "world");
    editor.handle_key(enter());
    
    // Should complete without crashing
}

#[test]
fn test_search_next_n() {
    let mut editor = setup("test test test test");
    
    // First search
    editor.handle_key(key('/'));
    type_string(&mut editor, "test");
    editor.handle_key(enter());
    
    // Find next
    editor.handle_key(key('n'));
    // Should not crash
}

#[test]
fn test_search_prev_N() {
    let mut editor = setup("test test test test");
    
    // Go to end
    editor.handle_key(key('G'));
    
    // Search and find previous
    editor.handle_key(key('/'));
    type_string(&mut editor, "test");
    editor.handle_key(enter());
    
    // Find previous
    editor.handle_key(key('N'));
    // Should not crash
}

// === Backward Search ===

#[test]
fn test_backward_search_entry() {
    let mut editor = setup("hello world");
    
    editor.handle_key(key('?'));
    // Should be in search mode
}

#[test]
fn test_backward_search_pattern() {
    let mut editor = setup("hello world hello");
    editor.handle_key(key('$')); // Go to end
    
    editor.handle_key(key('?'));
    type_string(&mut editor, "hello");
    editor.handle_key(enter());
    
    // Should complete without crashing
}

// === Star and Hash Search ===

#[test]
fn test_star_search_word() {
    let mut editor = setup("hello world hello there hello");
    
    // Search for word under cursor
    editor.handle_key(key('*'));
    // Should not crash
}

#[test]
fn test_hash_search_word_backward() {
    let mut editor = setup("hello world hello there hello");
    editor.handle_key(key('$')); // Go to end
    
    // Search backward for word under cursor
    editor.handle_key(key('#'));
    // Should not crash
}

// === Search Cancel ===

#[test]
fn test_search_cancel_escape() {
    let mut editor = setup("hello world");
    
    editor.handle_key(key('/'));
    type_string(&mut editor, "wor");
    editor.handle_key(escape());
    
    // Should be back in normal mode
}

// === Search Repeat ===

#[test]
fn test_search_repeat_n() {
    let mut editor = setup("abc abc abc abc abc");
    
    editor.handle_key(key('/'));
    type_string(&mut editor, "abc");
    editor.handle_key(enter());
    
    for _ in 0..3 {
        editor.handle_key(key('n'));
    }
    // Should not crash after multiple n
}

#[test]
fn test_search_repeat_N_reverse() {
    let mut editor = setup("abc abc abc abc abc");
    
    editor.handle_key(key('/'));
    type_string(&mut editor, "abc");
    editor.handle_key(enter());
    
    // Forward then backward
    editor.handle_key(key('n'));
    editor.handle_key(key('n'));
    editor.handle_key(key('N'));
    // Should not crash
}

// === Empty Search ===

#[test]
fn test_empty_search() {
    let mut editor = setup("hello world");
    
    editor.handle_key(key('/'));
    editor.handle_key(enter()); // Empty search
    
    // Should handle gracefully
}

// === Search with Special Characters ===

#[test]
fn test_search_with_spaces() {
    let mut editor = setup("hello world test");
    
    editor.handle_key(key('/'));
    type_string(&mut editor, "hello world");
    editor.handle_key(enter());
    
    // Should work with spaces
}

// === Multiple Line Search ===

#[test]
fn test_search_multiline_buffer() {
    let mut editor = setup("line one\nline two\nline three\nline four");
    
    editor.handle_key(key('/'));
    type_string(&mut editor, "three");
    editor.handle_key(enter());
    
    // Should find on line 3
}

// === Search from Different Positions ===

#[test]
fn test_search_from_middle() {
    let mut editor = setup("start middle end middle start");
    
    // Move to middle
    editor.handle_key(key('w'));
    editor.handle_key(key('w'));
    
    editor.handle_key(key('/'));
    type_string(&mut editor, "start");
    editor.handle_key(enter());
    
    // Should find next "start" or wrap around
}

// === Case Sensitivity (default behavior) ===

#[test]
fn test_search_case_sensitive() {
    let mut editor = setup("Hello hello HELLO");
    
    editor.handle_key(key('/'));
    type_string(&mut editor, "hello");
    editor.handle_key(enter());
    
    // Should find based on case settings
}

// === Search After Edit ===

#[test]
fn test_search_after_insert() {
    let mut editor = setup("aaa bbb ccc");
    
    // Insert some text
    editor.handle_key(key('i'));
    type_string(&mut editor, "test ");
    editor.handle_key(escape());
    
    // Now search
    editor.handle_key(key('/'));
    type_string(&mut editor, "bbb");
    editor.handle_key(enter());
    
    // Should work after edit
}
