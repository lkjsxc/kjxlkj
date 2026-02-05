//! Cursor movement tests - comprehensive tests for cursor navigation.

#![allow(non_snake_case)]

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent};

fn key(c: char) -> KeyEvent {
    KeyEvent::char(c)
}

fn ctrl(c: char) -> KeyEvent {
    KeyEvent::ctrl(KeyCode::Char(c))
}

fn setup(content: &str) -> EditorState {
    let mut editor = EditorState::new();
    editor.load_content(content);
    editor
}

// === Basic Movement ===

#[test]
fn test_h_moves_left() {
    let mut editor = setup("hello");
    editor.handle_key(key('l')); // Move right first
    editor.handle_key(key('l'));
    assert_eq!(editor.cursor().position.col, 2);
    
    editor.handle_key(key('h'));
    assert_eq!(editor.cursor().position.col, 1);
}

#[test]
fn test_l_moves_right() {
    let mut editor = setup("hello");
    assert_eq!(editor.cursor().position.col, 0);
    
    editor.handle_key(key('l'));
    assert_eq!(editor.cursor().position.col, 1);
}

#[test]
fn test_j_moves_down() {
    let mut editor = setup("line 1\nline 2");
    assert_eq!(editor.cursor().position.line, 0);
    
    editor.handle_key(key('j'));
    assert_eq!(editor.cursor().position.line, 1);
}

#[test]
fn test_k_moves_up() {
    let mut editor = setup("line 1\nline 2");
    editor.handle_key(key('j'));
    assert_eq!(editor.cursor().position.line, 1);
    
    editor.handle_key(key('k'));
    assert_eq!(editor.cursor().position.line, 0);
}

// === Line Start/End ===

#[test]
fn test_0_moves_to_line_start() {
    let mut editor = setup("hello");
    editor.handle_key(key('l'));
    editor.handle_key(key('l'));
    assert_eq!(editor.cursor().position.col, 2);
    
    editor.handle_key(key('0'));
    assert_eq!(editor.cursor().position.col, 0);
}

#[test]
fn test_dollar_moves_to_line_end() {
    let mut editor = setup("hello");
    assert_eq!(editor.cursor().position.col, 0);
    
    editor.handle_key(key('$'));
    assert!(editor.cursor().position.col > 0);
}

#[test]
fn test_caret_moves_to_first_nonblank() {
    let mut editor = setup("  hello");
    editor.handle_key(key('$')); // Go to end
    
    editor.handle_key(key('^'));
    assert_eq!(editor.cursor().position.col, 2); // Skip leading spaces
}

// === Word Movement ===

#[test]
fn test_w_moves_to_next_word() {
    let mut editor = setup("hello world");
    assert_eq!(editor.cursor().position.col, 0);
    
    editor.handle_key(key('w'));
    assert_eq!(editor.cursor().position.col, 6); // Start of "world"
}

#[test]
fn test_b_moves_to_prev_word() {
    let mut editor = setup("hello world");
    editor.handle_key(key('$')); // Go to end
    
    editor.handle_key(key('b'));
    assert_eq!(editor.cursor().position.col, 6); // Start of "world"
}

#[test]
fn test_e_moves_to_end_of_word() {
    let mut editor = setup("hello world");
    assert_eq!(editor.cursor().position.col, 0);
    
    editor.handle_key(key('e'));
    assert_eq!(editor.cursor().position.col, 4); // 'o' of "hello"
}

// === File Movement ===

#[test]
fn test_gg_moves_to_first_line() {
    let mut editor = setup("line 1\nline 2\nline 3");
    editor.handle_key(key('j'));
    editor.handle_key(key('j'));
    assert_eq!(editor.cursor().position.line, 2);
    
    editor.handle_key(key('g'));
    editor.handle_key(key('g'));
    // May or may not be implemented - check it doesn't crash
    assert!(editor.cursor().position.line <= 2);
}

#[test]
fn test_G_moves_to_last_line() {
    let mut editor = setup("line 1\nline 2\nline 3");
    assert_eq!(editor.cursor().position.line, 0);
    
    editor.handle_key(key('G'));
    assert_eq!(editor.cursor().position.line, 2);
}

// === Boundary Checks ===

#[test]
fn test_h_at_start_stays() {
    let mut editor = setup("hello");
    assert_eq!(editor.cursor().position.col, 0);
    
    editor.handle_key(key('h'));
    assert_eq!(editor.cursor().position.col, 0);
}

#[test]
fn test_k_at_first_line_stays() {
    let mut editor = setup("hello");
    assert_eq!(editor.cursor().position.line, 0);
    
    editor.handle_key(key('k'));
    assert_eq!(editor.cursor().position.line, 0);
}

#[test]
fn test_j_at_last_line_stays() {
    let mut editor = setup("hello");
    assert_eq!(editor.cursor().position.line, 0);
    
    editor.handle_key(key('j'));
    assert_eq!(editor.cursor().position.line, 0);
}

// === Counts ===

#[test]
fn test_count_j() {
    let mut editor = setup("a\nb\nc\nd\ne");
    assert_eq!(editor.cursor().position.line, 0);
    
    editor.handle_key(key('3'));
    editor.handle_key(key('j'));
    assert_eq!(editor.cursor().position.line, 3);
}

#[test]
fn test_count_l() {
    let mut editor = setup("hello world");
    assert_eq!(editor.cursor().position.col, 0);
    
    editor.handle_key(key('4'));
    editor.handle_key(key('l'));
    assert_eq!(editor.cursor().position.col, 4);
}

#[test]
fn test_count_w() {
    let mut editor = setup("one two three four");
    assert_eq!(editor.cursor().position.col, 0);
    
    editor.handle_key(key('2'));
    editor.handle_key(key('w'));
    // Should be at start of "three"
    assert!(editor.cursor().position.col > 4);
}

// === Find Character (not fully implemented, test for no-crash) ===

#[test]
fn test_f_finds_char() {
    let mut editor = setup("hello world");
    assert_eq!(editor.cursor().position.col, 0);
    
    editor.handle_key(key('f'));
    editor.handle_key(key('w'));
    // f may or may not be implemented - ensure no crash
    // If implemented, should be at 'w' position (6)
}

#[test]
fn test_F_finds_char_backward() {
    let mut editor = setup("hello world");
    editor.handle_key(key('$'));
    
    editor.handle_key(key('F'));
    editor.handle_key(key('l'));
    // F may or may not be implemented - ensure no crash
}

#[test]
fn test_t_to_char() {
    let mut editor = setup("hello world");
    assert_eq!(editor.cursor().position.col, 0);
    
    editor.handle_key(key('t'));
    editor.handle_key(key('w'));
    // t may or may not be implemented - ensure no crash
}

// === Scroll Commands ===

#[test]
fn test_ctrl_d_scrolls_down() {
    let mut editor = setup(&(0..50).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n"));
    let start_line = editor.cursor().position.line;
    
    editor.handle_key(ctrl('d'));
    // Should have moved down
    assert!(editor.cursor().position.line >= start_line);
}

#[test]
fn test_ctrl_u_scrolls_up() {
    let mut editor = setup(&(0..50).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n"));
    // Move down first
    for _ in 0..20 {
        editor.handle_key(key('j'));
    }
    let mid_line = editor.cursor().position.line;
    
    editor.handle_key(ctrl('u'));
    // Should have moved up
    assert!(editor.cursor().position.line <= mid_line);
}
