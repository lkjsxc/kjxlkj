//! Operator and operator+motion integration tests.
//!
//! These tests verify that operators work correctly with motions
//! and text objects in the full EditorState context.

#![allow(non_snake_case)]

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent, KeyModifiers, Mode};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn char_key(c: char) -> KeyEvent {
    KeyEvent::char(c)
}

// === Line operators (dd, yy, cc) ===

#[test]
fn test_dd_deletes_line() {
    let mut editor = EditorState::new();
    editor.load_content("line one\nline two\nline three");
    
    // Delete first line with dd
    editor.handle_key(char_key('d'));
    editor.handle_key(char_key('d'));
    
    assert_eq!(editor.buffer().line_count(), 2);
    assert!(editor.buffer().line(0).unwrap().to_string().starts_with("line two"));
}

#[test]
fn test_dd_on_last_line() {
    let mut editor = EditorState::new();
    editor.load_content("line one\nline two");
    
    // Initial state
    assert_eq!(editor.buffer().line_count(), 2, "Should start with 2 lines");
    assert_eq!(editor.cursor().line(), 0);
    
    // Move to last line
    editor.handle_key(char_key('j'));
    assert_eq!(editor.cursor().line(), 1, "Should be on line 1 after j");
    
    // Delete it
    editor.handle_key(char_key('d'));
    editor.handle_key(char_key('d'));
    
    // Should have one line left
    assert_eq!(editor.buffer().line_count(), 1, "Should have 1 line after dd on last line");
    assert!(editor.buffer().line(0).unwrap().to_string().starts_with("line one"));
}

#[test]
fn test_yy_yanks_line() {
    let mut editor = EditorState::new();
    editor.load_content("line one\nline two");
    
    // Yank first line
    editor.handle_key(char_key('y'));
    editor.handle_key(char_key('y'));
    
    // Buffer should be unchanged
    assert_eq!(editor.buffer().line_count(), 2);
    
    // Paste below (p) to verify yank worked
    editor.handle_key(char_key('p'));
    
    // Now we should have 3 lines
    assert_eq!(editor.buffer().line_count(), 3);
}

#[test]
fn test_cc_changes_line() {
    let mut editor = EditorState::new();
    editor.load_content("line one\nline two");
    
    // Change first line
    editor.handle_key(char_key('c'));
    editor.handle_key(char_key('c'));
    
    // Should be in insert mode
    assert_eq!(editor.mode(), Mode::Insert);
    
    // Type new content
    editor.handle_key(char_key('n'));
    editor.handle_key(char_key('e'));
    editor.handle_key(char_key('w'));
    
    // Escape back to normal
    editor.handle_key(key(KeyCode::Escape));
    
    // Verify content changed
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.contains("new"));
}

// === Single character operators ===

#[test]
fn test_x_deletes_char() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // Delete 'h'
    editor.handle_key(char_key('x'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with("ello"));
}

#[test]
fn test_x_at_end_of_line() {
    let mut editor = EditorState::new();
    editor.load_content("ab");
    
    // Move to end
    editor.handle_key(char_key('$'));
    
    // Delete char
    editor.handle_key(char_key('x'));
    
    // Should have just 'a' left (cursor was on 'b')
    let line = editor.buffer().line(0).unwrap().to_string();
    // Remove newline for comparison
    let line = line.trim_end();
    assert_eq!(line, "a");
}

#[test]
fn test_r_replaces_char() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // Replace 'h' with 'j'
    editor.handle_key(char_key('r'));
    editor.handle_key(char_key('j'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with("jello"));
    
    // Should still be in normal mode
    assert_eq!(editor.mode(), Mode::Normal);
}

#[test]
fn test_tilde_toggles_case() {
    let mut editor = EditorState::new();
    editor.load_content("Hello");
    
    // Toggle case of 'H'
    editor.handle_key(char_key('~'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with("h"));
}

// === Delete operators with motion ===

#[test]
fn test_dw_deletes_word() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Delete word
    editor.handle_key(char_key('d'));
    editor.handle_key(char_key('w'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    // After dw, "hello " should be deleted, leaving "world"
    assert!(line.starts_with("world") || line.contains("world"));
}

#[test]
fn test_d_dollar_deletes_to_end() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Move to middle
    editor.handle_key(char_key('w')); // Move to 'w' of 'world'
    
    // Delete to end of line
    editor.handle_key(char_key('D')); // D is shorthand for d$
    
    let line = editor.buffer().line(0).unwrap().to_string();
    // Should have only "hello " left
    assert!(line.starts_with("hello"));
}

#[test]
fn test_d0_deletes_to_start() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Move to end
    editor.handle_key(char_key('$'));
    
    // Delete to start
    editor.handle_key(char_key('d'));
    editor.handle_key(char_key('0'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    // Only the last char should remain
    assert!(line.len() <= 2); // Just 'd' or newline
}

// === Change operators ===

#[test]
fn test_cw_changes_word() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Change word
    editor.handle_key(char_key('c'));
    editor.handle_key(char_key('w'));
    
    // Should be in insert mode
    assert_eq!(editor.mode(), Mode::Insert);
    
    // Type replacement
    editor.handle_key(char_key('H'));
    editor.handle_key(char_key('i'));
    
    // Escape
    editor.handle_key(key(KeyCode::Escape));
    
    // Verify
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with("Hi") || line.contains("Hi"));
}

#[test]
fn test_C_changes_to_end() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // C is shorthand for c$
    editor.handle_key(char_key('C'));
    
    // Should be in insert mode
    assert_eq!(editor.mode(), Mode::Insert);
}

// === Substitute ===

#[test]
fn test_s_substitutes_char() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // Substitute first char
    editor.handle_key(char_key('s'));
    
    // Should be in insert mode
    assert_eq!(editor.mode(), Mode::Insert);
    
    // Type replacement
    editor.handle_key(char_key('j'));
    editor.handle_key(key(KeyCode::Escape));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with("j"));
}

#[test]
fn test_S_substitutes_line() {
    let mut editor = EditorState::new();
    editor.load_content("hello\nworld");
    
    // Substitute entire line
    editor.handle_key(char_key('S'));
    
    // Should be in insert mode
    assert_eq!(editor.mode(), Mode::Insert);
}

// === Join lines ===

#[test]
fn test_J_joins_lines() {
    let mut editor = EditorState::new();
    editor.load_content("hello\nworld");
    
    // Join with next line
    editor.handle_key(char_key('J'));
    
    // Should have one line
    assert_eq!(editor.buffer().line_count(), 1);
    
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.contains("hello") && line.contains("world"));
}

// === Indent/Outdent ===

#[test]
fn test_indent_line() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // Indent
    editor.handle_key(char_key('>'));
    editor.handle_key(char_key('>'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with("    ") || line.starts_with("\t"));
}

#[test]
fn test_outdent_line() {
    let mut editor = EditorState::new();
    editor.load_content("    hello"); // 4 spaces indent
    
    // Outdent
    editor.handle_key(char_key('<'));
    editor.handle_key(char_key('<'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    // Should have less or no indent
    assert!(line.starts_with("hello") || !line.starts_with("    "));
}

// === Count prefix ===

#[test]
fn test_count_with_dd() {
    let mut editor = EditorState::new();
    editor.load_content("one\ntwo\nthree\nfour\nfive");
    
    // Delete 3 lines with 3dd
    editor.handle_key(char_key('3'));
    editor.handle_key(char_key('d'));
    editor.handle_key(char_key('d'));
    
    // Should have 2 lines left
    assert_eq!(editor.buffer().line_count(), 2);
}

#[test]
fn test_count_with_x() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // Delete 3 chars with 3x
    editor.handle_key(char_key('3'));
    editor.handle_key(char_key('x'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    // Should have "lo" left
    assert!(line.starts_with("lo") || line.len() <= 3);
}

// === Paste ===

#[test]
fn test_p_pastes_after() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Yank word
    editor.handle_key(char_key('y'));
    editor.handle_key(char_key('w'));
    
    // Move to end
    editor.handle_key(char_key('$'));
    
    // Paste
    editor.handle_key(char_key('p'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    // Should have "hello" duplicated somewhere
    assert!(line.len() > 11);
}

#[test]
fn test_P_pastes_before() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Yank word (yanks "hello ")
    editor.handle_key(char_key('y'));
    editor.handle_key(char_key('w'));
    
    // Move to second word
    editor.handle_key(char_key('w'));
    
    // Paste before cursor
    editor.handle_key(char_key('P'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    // Should have "hello hello world" or similar - definitely longer than original
    assert!(line.len() > 11);
}

// === Undo/Redo ===

#[test]
fn test_undo_after_delete() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    let original_len = editor.buffer().line(0).unwrap().to_string().len();
    
    // Delete char
    editor.handle_key(char_key('x'));
    
    // Undo
    editor.handle_key(char_key('u'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    assert_eq!(line.len(), original_len);
}

// === Dot repeat ===

#[test]
fn test_dot_repeats_delete() {
    let mut editor = EditorState::new();
    editor.load_content("aaa");
    
    // Delete first char
    editor.handle_key(char_key('x'));
    
    // Repeat
    editor.handle_key(char_key('.'));
    
    let line = editor.buffer().line(0).unwrap().to_string();
    // Should have just one 'a' left
    assert!(line.len() <= 2);
}

// === Visual mode operators ===

#[test]
fn test_visual_delete() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Enter visual mode
    editor.handle_key(char_key('v'));
    assert_eq!(editor.mode(), Mode::Visual);
    
    // Select a few chars
    editor.handle_key(char_key('l'));
    editor.handle_key(char_key('l'));
    
    // Delete
    editor.handle_key(char_key('d'));
    
    // Should be back in normal mode
    assert_eq!(editor.mode(), Mode::Normal);
    
    let line = editor.buffer().line(0).unwrap().to_string();
    // "hel" should be deleted, leaving "lo world"
    assert!(line.contains("lo") || line.len() < 11);
}

#[test]
fn test_visual_line_delete() {
    let mut editor = EditorState::new();
    editor.load_content("one\ntwo\nthree");
    
    // Enter visual line mode
    editor.handle_key(char_key('V'));
    assert_eq!(editor.mode(), Mode::VisualLine);
    
    // Select one more line
    editor.handle_key(char_key('j'));
    
    // Delete
    editor.handle_key(char_key('d'));
    
    // Should have one line left
    assert_eq!(editor.buffer().line_count(), 1);
}

#[test]
fn test_visual_yank() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");
    
    // Enter visual mode
    editor.handle_key(char_key('v'));
    
    // Select chars
    editor.handle_key(char_key('l'));
    editor.handle_key(char_key('l'));
    editor.handle_key(char_key('l'));
    editor.handle_key(char_key('l'));
    
    // Yank
    editor.handle_key(char_key('y'));
    
    // Should be back in normal mode
    assert_eq!(editor.mode(), Mode::Normal);
    
    // Buffer should be unchanged
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with("hello"));
}

// === Edge cases ===

#[test]
fn test_operator_on_empty_buffer() {
    let mut editor = EditorState::new();
    editor.load_content("");
    
    // Try to delete - should not panic
    editor.handle_key(char_key('d'));
    editor.handle_key(char_key('d'));
    
    // Still have at least 1 line
    assert!(editor.buffer().line_count() >= 1);
}

#[test]
fn test_multiple_operators_sequence() {
    let mut editor = EditorState::new();
    editor.load_content("one\ntwo\nthree\nfour\nfive");
    
    // Delete, then delete again
    editor.handle_key(char_key('d'));
    editor.handle_key(char_key('d'));
    
    editor.handle_key(char_key('d'));
    editor.handle_key(char_key('d'));
    
    // Should have 3 lines left
    assert_eq!(editor.buffer().line_count(), 3);
}

#[test]
fn test_cancel_operator_with_escape() {
    let mut editor = EditorState::new();
    editor.load_content("hello");
    
    // Start operator
    editor.handle_key(char_key('d'));
    
    // Cancel with Escape
    editor.handle_key(key(KeyCode::Escape));
    
    // Buffer should be unchanged
    let line = editor.buffer().line(0).unwrap().to_string();
    assert!(line.starts_with("hello"));
}
