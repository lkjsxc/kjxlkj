//! Tests for :execute and :normal commands.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{Intent, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

fn dispatch_ex(state: &mut EditorState, cmd: &str) {
    dispatch_intent(state, Intent::ExCommand(cmd.into()));
}

fn line(state: &EditorState, idx: usize) -> String {
    state.active_buffer().unwrap().text.line_to_string(idx)
}

fn line_count(state: &EditorState) -> usize {
    state.active_buffer().unwrap().text.line_count()
}

fn cursor(state: &EditorState) -> (usize, usize) {
    let w = state.active_window_state().unwrap();
    (w.cursor_line, w.cursor_col)
}

// ── :execute ─────────────────────────────────

#[test]
fn execute_simple_command() {
    let mut s = setup("hello world");
    dispatch_ex(&mut s, ":execute \"set number\"");
    assert!(s.options.number);
}

#[test]
fn execute_without_quotes() {
    let mut s = setup("hello world");
    dispatch_ex(&mut s, ":execute set number");
    assert!(s.options.number);
}

#[test]
fn execute_no_arg_gives_error() {
    let mut s = setup("hello");
    dispatch_ex(&mut s, ":execute");
    assert!(s.message.as_ref().unwrap().contains("E471"));
}

#[test]
fn exe_alias_works() {
    let mut s = setup("hello");
    dispatch_ex(&mut s, ":exe \"set number\"");
    assert!(s.options.number);
}

#[test]
fn execute_nested_ex_commands() {
    let mut s = setup("aaa\nbbb\nccc");
    // Execute a quit command — should set should_quit
    dispatch_ex(&mut s, ":execute \"q!\"");
    assert!(s.should_quit);
}

// ── :normal ─────────────────────────────────

#[test]
fn normal_delete_char() {
    let mut s = setup("hello world");
    dispatch_ex(&mut s, ":normal x");
    assert_eq!(line(&s, 0), "ello world");
}

#[test]
fn normal_word_forward() {
    let mut s = setup("hello world");
    dispatch_ex(&mut s, ":normal w");
    assert_eq!(cursor(&s), (0, 6));
}

#[test]
fn normal_append_a_key() {
    // `dd` deletes a line
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_ex(&mut s, ":normal dd");
    assert_eq!(line_count(&s), 2);
    assert_eq!(line(&s, 0), "bbb");
}

#[test]
fn normal_with_range() {
    // Add text at beginning of each line using I (enter insert at first non-blank)
    // Since :normal feeds keys through normal parser, we test with 'x' on each line
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_ex(&mut s, ":1,3normal x");
    assert_eq!(line(&s, 0), "aa");
    assert_eq!(line(&s, 1), "bb");
    assert_eq!(line(&s, 2), "cc");
}

#[test]
fn normal_bang_works() {
    let mut s = setup("hello");
    dispatch_ex(&mut s, ":normal! x");
    assert_eq!(line(&s, 0), "ello");
}

#[test]
fn normal_goto_end_of_line() {
    let mut s = setup("hello world");
    dispatch_ex(&mut s, ":normal $");
    assert_eq!(cursor(&s).1, 10); // last char index
}

#[test]
fn normal_replace_char() {
    let mut s = setup("hello");
    dispatch_ex(&mut s, ":normal rX");
    assert_eq!(line(&s, 0), "Xello");
}

#[test]
fn normal_no_args_gives_error() {
    let mut s = setup("hello");
    dispatch_ex(&mut s, ":normal");
    assert!(s.message.as_ref().unwrap().contains("E471"));
}

#[test]
fn normal_on_range_deletes_chars() {
    let mut s = setup("aaa\nbbb\nccc\nddd");
    dispatch_ex(&mut s, ":2,3normal x");
    assert_eq!(line(&s, 0), "aaa");
    assert_eq!(line(&s, 1), "bb");
    assert_eq!(line(&s, 2), "cc");
    assert_eq!(line(&s, 3), "ddd");
}

#[test]
fn normal_yy_dd_sequence() {
    let mut s = setup("aaa\nbbb\nccc");
    // Delete second line using :2normal dd
    dispatch_ex(&mut s, ":2normal dd");
    assert_eq!(line_count(&s), 2);
    assert_eq!(line(&s, 0), "aaa");
    assert_eq!(line(&s, 1), "ccc");
}

// ── :execute + :normal combined ──────────────

#[test]
fn execute_runs_normal() {
    let mut s = setup("hello world");
    dispatch_ex(&mut s, ":execute \"normal x\"");
    assert_eq!(line(&s, 0), "ello world");
}
