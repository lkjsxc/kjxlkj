//! Tests for autopairs: auto-insert matching brackets and skip-over closing brackets.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{Intent, Mode, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

fn setup_autopairs(text: &str) -> EditorState {
    let mut s = setup(text);
    s.options.autopairs = true;
    s.mode.transition(Mode::Insert);
    s
}

// ── Autopairs disabled by default ──

#[test]
fn autopairs_disabled_by_default() {
    let s = setup("hello");
    assert!(!s.options.autopairs);
}

// ── :set autopairs / :set noautopairs ──

#[test]
fn set_autopairs_on() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":set autopairs".into()));
    assert!(s.options.autopairs);
    assert!(s.message.as_ref().unwrap().contains("on"));
}

#[test]
fn set_noautopairs() {
    let mut s = setup("hello");
    s.options.autopairs = true;
    dispatch_intent(&mut s, Intent::ExCommand(":set noautopairs".into()));
    assert!(!s.options.autopairs);
    assert!(s.message.as_ref().unwrap().contains("off"));
}

// ── Open bracket auto-inserts close ──

#[test]
fn paren_auto_close() {
    let mut s = setup_autopairs("");
    dispatch_intent(&mut s, Intent::InsertChar('('));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "()");
    let win = s.active_window_state().unwrap();
    assert_eq!(win.cursor_col, 1); // cursor between ( and )
}

#[test]
fn bracket_auto_close() {
    let mut s = setup_autopairs("");
    dispatch_intent(&mut s, Intent::InsertChar('['));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "[]");
}

#[test]
fn brace_auto_close() {
    let mut s = setup_autopairs("");
    dispatch_intent(&mut s, Intent::InsertChar('{'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "{}");
}

// ── Close bracket skips over ──

#[test]
fn close_paren_skips() {
    let mut s = setup_autopairs("");
    dispatch_intent(&mut s, Intent::InsertChar('('));
    // Cursor should be between ( and )
    dispatch_intent(&mut s, Intent::InsertChar(')'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "()"); // no extra )
    let win = s.active_window_state().unwrap();
    assert_eq!(win.cursor_col, 2); // cursor after )
}

#[test]
fn close_bracket_skips() {
    let mut s = setup_autopairs("");
    dispatch_intent(&mut s, Intent::InsertChar('['));
    dispatch_intent(&mut s, Intent::InsertChar(']'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "[]");
}

// ── Without autopairs, no auto-close ──

#[test]
fn no_auto_close_when_disabled() {
    let mut s = setup("");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertChar('('));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "(");
}

// ── Typing inside autopairs ──

#[test]
fn type_inside_autopairs() {
    let mut s = setup_autopairs("");
    dispatch_intent(&mut s, Intent::InsertChar('('));
    dispatch_intent(&mut s, Intent::InsertChar('x'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "(x)");
}

// ── Nested autopairs ──

#[test]
fn nested_pairs() {
    let mut s = setup_autopairs("");
    dispatch_intent(&mut s, Intent::InsertChar('('));
    dispatch_intent(&mut s, Intent::InsertChar('['));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "([])");
}
