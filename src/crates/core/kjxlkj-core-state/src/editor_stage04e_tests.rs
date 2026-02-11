//! Wave-037 integration tests: mark system and additional coverage.
//!
//! Focus: m{a-z} set, '{a-z} goto line, `{a-z} goto exact, boundary
//! behavior for unset marks, overwrite, persistence across mode changes.

use kjxlkj_core_types::{Action, BufferId, Key, KeyModifiers, Motion};

use crate::editor::EditorState;

fn ed() -> EditorState { EditorState::new(80, 24) }
fn m() -> KeyModifiers { KeyModifiers::default() }
fn k(c: char, s: &mut EditorState) { s.handle_key(&Key::Char(c), &m()); }
fn cur(s: &EditorState) -> (usize, usize) {
    let w = s.windows.get(&s.focus.focused).unwrap();
    (w.cursor.line, w.cursor.col)
}
fn buf(s: &mut EditorState, text: &str) {
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, text).unwrap();
}

#[test] fn mark_set_and_goto_exact() {
    let mut s = ed(); buf(&mut s, "hello\nworld\nfoo");
    k('j', &mut s); k('l', &mut s); k('l', &mut s); // line 1 col 2
    k('m', &mut s); k('a', &mut s); // set mark 'a'
    k('g', &mut s); k('g', &mut s); // go to top
    k('`', &mut s); k('a', &mut s); // goto mark exact
    assert_eq!(cur(&s), (1, 2));
}

#[test] fn mark_set_and_goto_line() {
    let mut s = ed(); buf(&mut s, "  hello\n  world\n  foo");
    k('j', &mut s); k('j', &mut s); // line 2
    k('l', &mut s); k('l', &mut s); k('l', &mut s); k('l', &mut s); // col 4
    k('m', &mut s); k('b', &mut s);
    k('g', &mut s); k('g', &mut s);
    k('\'', &mut s); k('b', &mut s); // goto mark line
    assert_eq!(cur(&s), (2, 2)); // first non-blank after "  "
}

#[test] fn goto_unset_mark_is_noop() {
    let mut s = ed(); buf(&mut s, "hello\nworld");
    k('j', &mut s);
    let before = cur(&s);
    k('`', &mut s); k('z', &mut s); // unset mark
    assert_eq!(cur(&s), before);
}

#[test] fn goto_unset_mark_line_is_noop() {
    let mut s = ed(); buf(&mut s, "hello\nworld");
    k('j', &mut s);
    let before = cur(&s);
    k('\'', &mut s); k('z', &mut s);
    assert_eq!(cur(&s), before);
}

#[test] fn mark_overwrite() {
    let mut s = ed(); buf(&mut s, "aaa\nbbb\nccc");
    k('m', &mut s); k('a', &mut s); // mark 'a' at line 0
    k('j', &mut s); k('j', &mut s);
    k('m', &mut s); k('a', &mut s); // overwrite at line 2
    k('g', &mut s); k('g', &mut s);
    k('`', &mut s); k('a', &mut s);
    assert_eq!(cur(&s).0, 2);
}

#[test] fn mark_persists_across_insert_mode() {
    let mut s = ed(); buf(&mut s, "hello\nworld");
    k('j', &mut s); k('m', &mut s); k('c', &mut s); // mark at line 1
    k('i', &mut s); s.handle_key(&Key::Escape, &m()); // insert/exit
    k('g', &mut s); k('g', &mut s);
    k('`', &mut s); k('c', &mut s);
    assert_eq!(cur(&s).0, 1);
}

#[test] fn mark_set_uppercase_ignored() {
    let mut s = ed(); buf(&mut s, "hello\nworld");
    k('j', &mut s); k('m', &mut s); k('A', &mut s); // uppercase
    k('g', &mut s); k('g', &mut s);
    k('`', &mut s); k('A', &mut s);
    assert_eq!(cur(&s).0, 0); // didn't move
}

#[test] fn multiple_marks_independent() {
    let mut s = ed(); buf(&mut s, "aaa\nbbb\nccc\nddd");
    k('m', &mut s); k('a', &mut s); // mark 'a' at line 0
    k('j', &mut s); k('j', &mut s);
    k('m', &mut s); k('b', &mut s); // mark 'b' at line 2
    k('j', &mut s); // line 3
    k('`', &mut s); k('a', &mut s);
    assert_eq!(cur(&s).0, 0);
    k('`', &mut s); k('b', &mut s);
    assert_eq!(cur(&s).0, 2);
}

#[test] fn mark_goto_exact_clamps_if_line_deleted() {
    let mut s = ed(); buf(&mut s, "aaa\nbbb\nccc");
    k('j', &mut s); k('j', &mut s); // line 2
    k('m', &mut s); k('x', &mut s);
    k('k', &mut s); // line 1
    k('d', &mut s); k('d', &mut s); k('d', &mut s); k('d', &mut s); // delete 2 lines
    k('g', &mut s); k('g', &mut s);
    k('`', &mut s); k('x', &mut s); // goto clamped
    let lc = s.buffers.get(&BufferId(0)).unwrap().line_count();
    assert!(cur(&s).0 < lc);
}

#[test] fn mark_goto_line_first_nonblank_tabs() {
    let mut s = ed(); buf(&mut s, "hello\n\t\tworld");
    k('j', &mut s); k('m', &mut s); k('d', &mut s);
    k('g', &mut s); k('g', &mut s);
    k('\'', &mut s); k('d', &mut s);
    assert_eq!(cur(&s), (1, 2)); // past \t\t
}

#[test] fn mark_action_api_direct() {
    let mut s = ed(); buf(&mut s, "line0\nline1\nline2");
    k('j', &mut s); k('l', &mut s); k('l', &mut s); k('l', &mut s); // line 1 col 3
    s.apply_action(Action::SetMark('e'));
    s.apply_action(Action::Motion(Motion::GotoFirstLine));
    s.apply_action(Action::GotoMarkExact('e'));
    assert_eq!(cur(&s), (1, 3));
}

#[test] fn mark_goto_line_empty_line() {
    let mut s = ed(); buf(&mut s, "hello\n\nworld");
    k('j', &mut s); k('m', &mut s); k('f', &mut s); // mark at empty line 1
    k('g', &mut s); k('g', &mut s);
    k('\'', &mut s); k('f', &mut s);
    assert_eq!(cur(&s), (1, 0)); // empty line: first non-blank is 0
}
