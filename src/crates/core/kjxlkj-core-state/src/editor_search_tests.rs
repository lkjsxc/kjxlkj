//! Integration tests for search features through the handle_key pipeline.
//! Covers *, #, n, N, :nohlsearch, search wrapping, and boundary cases.

use crate::editor::EditorState;
use kjxlkj_core_types::{BufferId, CommandKind, Key, KeyModifiers, Mode};

fn ed() -> EditorState { EditorState::new(80, 24) }
fn m() -> KeyModifiers { KeyModifiers::default() }
fn ed_with(text: &str) -> EditorState {
    let mut s = ed();
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, text).unwrap();
    s
}
fn cursor_col(s: &EditorState) -> usize { s.windows.get(&s.focus.focused).unwrap().cursor.col }
fn cursor_line(s: &EditorState) -> usize { s.windows.get(&s.focus.focused).unwrap().cursor.line }
fn type_ex(s: &mut EditorState, cmd: &str) {
    s.mode = Mode::Command(CommandKind::Ex);
    s.activate_cmdline(CommandKind::Ex);
    for c in cmd.chars() { s.handle_command_input(&Key::Char(c), &m(), CommandKind::Ex); }
    s.handle_command_input(&Key::Enter, &m(), CommandKind::Ex);
}

#[test]
fn star_search_jumps_to_next_word() {
    let mut s = ed_with("foo bar foo baz foo");
    s.handle_key(&Key::Char('*'), &m());
    assert_eq!(cursor_col(&s), 8);
}

#[test]
fn star_search_wraps_around() {
    let mut s = ed_with("foo bar baz");
    // Only one "foo", so * wraps back to col 0.
    s.handle_key(&Key::Char('*'), &m());
    assert_eq!(cursor_col(&s), 0);
}

#[test]
fn hash_search_backward() {
    let mut s = ed_with("foo bar foo");
    // # from col 0 wraps backward to last "foo" at col 8.
    s.handle_key(&Key::Char('#'), &m());
    assert_eq!(cursor_col(&s), 8);
}

#[test]
fn n_repeats_last_search() {
    let mut s = ed_with("aa bb aa cc aa");
    s.handle_key(&Key::Char('*'), &m());
    let c1 = cursor_col(&s);
    s.handle_key(&Key::Char('n'), &m());
    let c2 = cursor_col(&s);
    assert!(c2 != c1 || c2 == 0);
}

#[test]
fn big_n_reverses_search() {
    let mut s = ed_with("aa bb aa cc aa");
    // * forward from col 0 finds next aa (col 6).
    s.handle_key(&Key::Char('*'), &m());
    let c_star = cursor_col(&s);
    // N reverses direction.
    s.handle_key(&Key::Char('N'), &m());
    let c_reverse = cursor_col(&s);
    assert_ne!(c_star, c_reverse);
}

#[test]
fn nohlsearch_clears_highlight() {
    let mut s = ed_with("test word test");
    s.handle_key(&Key::Char('*'), &m());
    assert!(s.search.hlsearch);
    type_ex(&mut s, "noh");
    assert!(!s.search.hlsearch);
}

#[test]
fn new_search_reactivates_highlight() {
    let mut s = ed_with("test word test");
    s.handle_key(&Key::Char('*'), &m());
    type_ex(&mut s, "noh");
    assert!(!s.search.hlsearch);
    // New * search reactivates.
    s.handle_key(&Key::Char('*'), &m());
    assert!(s.search.hlsearch);
}

#[test]
fn star_on_non_word_char_is_noop() {
    let mut s = ed_with("   spaces");
    // Cursor at col 0 which is a space — not a word char.
    s.handle_key(&Key::Char('*'), &m());
    assert_eq!(cursor_col(&s), 0);
}

#[test]
fn star_sets_slash_register() {
    let mut s = ed_with("hello world hello");
    s.handle_key(&Key::Char('*'), &m());
    let reg = s.registers.get('/').unwrap();
    assert!(reg.text.contains("hello"));
}

#[test]
fn search_multiline_forward() {
    let mut s = ed_with("aaa\nbbb\naaa");
    s.handle_key(&Key::Char('*'), &m());
    // Should jump to line 2 col 0.
    assert_eq!(cursor_line(&s), 2);
    assert_eq!(cursor_col(&s), 0);
}

#[test]
fn search_multiline_n_wraps() {
    let mut s = ed_with("aaa\nbbb\naaa");
    s.handle_key(&Key::Char('*'), &m());
    assert_eq!(cursor_line(&s), 2);
    // n wraps back to line 0.
    s.handle_key(&Key::Char('n'), &m());
    assert_eq!(cursor_line(&s), 0);
}

#[test]
fn star_on_empty_buffer_is_noop() {
    let mut s = ed();
    s.handle_key(&Key::Char('*'), &m());
    assert_eq!(cursor_col(&s), 0);
    assert_eq!(cursor_line(&s), 0);
}

#[test]
fn match_count_after_star() {
    let mut s = ed_with("foo bar foo baz foo");
    s.handle_key(&Key::Char('*'), &m());
    let buf = s.buffers.get(&BufferId(0)).unwrap();
    assert_eq!(s.search.match_count(buf), 3);
}
