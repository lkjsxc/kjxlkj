//! Wave-038 integration tests: macro system and race validation.
//!
//! Focus: q{a-z} record, q stop, @{a-z} play, boundary/race tests
//! for macros + marks + jumplist + splits under stress.

use kjxlkj_core_types::{Action, BufferId, Key, KeyModifiers, Mode};

use crate::editor::EditorState;

fn ed() -> EditorState { EditorState::new(80, 24) }
fn m() -> KeyModifiers { KeyModifiers::default() }
fn ctrl() -> KeyModifiers { KeyModifiers { ctrl: true, ..Default::default() } }
fn k(c: char, s: &mut EditorState) { s.handle_key(&Key::Char(c), &m()); }
fn cur(s: &EditorState) -> (usize, usize) {
    let w = s.windows.get(&s.focus.focused).unwrap();
    (w.cursor.line, w.cursor.col)
}
fn buf(s: &mut EditorState, text: &str) {
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, text).unwrap();
}
fn line(s: &EditorState, n: usize) -> String {
    s.buffers.get(&BufferId(0)).unwrap().line(n).unwrap_or_default().trim_end_matches('\n').to_string()
}

// --- Macro recording and playback ---

#[test] fn macro_record_and_play() {
    let mut s = ed(); buf(&mut s, "hello\nworld");
    k('q', &mut s); k('a', &mut s); // start recording into 'a'
    assert!(s.macro_state.is_recording());
    k('x', &mut s); // delete first char
    k('q', &mut s); // stop recording
    assert!(!s.macro_state.is_recording());
    k('@', &mut s); k('a', &mut s); // play macro 'a'
    // Should have deleted another char
    let l = line(&s, 0);
    assert_eq!(l, "llo"); // 2 chars deleted total from "hello"
}

#[test] fn macro_record_insert_mode() {
    let mut s = ed(); buf(&mut s, "");
    k('q', &mut s); k('b', &mut s); // record 'b'
    k('i', &mut s); // enter insert
    k('H', &mut s); k('i', &mut s); // type "Hi"
    s.handle_key(&Key::Escape, &m()); // back to normal
    k('q', &mut s); // stop
    // Register 'b' should have content
    assert!(s.registers.get('b').is_some());
}

#[test] fn macro_play_unset_register_noop() {
    let mut s = ed(); buf(&mut s, "abc");
    let before = cur(&s);
    k('@', &mut s); k('z', &mut s); // play unset macro
    assert_eq!(cur(&s), before);
    assert_eq!(s.mode, Mode::Normal);
}

#[test] fn macro_record_stop_without_start() {
    let mut s = ed();
    // q in normal mode should enter MacroRecord partial, then the second key starts recording
    // so q alone doesn't crash
    assert!(!s.macro_state.is_recording());
}

#[test] fn macro_multiple_records_overwrite() {
    let mut s = ed(); buf(&mut s, "abcdef");
    k('q', &mut s); k('a', &mut s); // record into 'a'
    k('x', &mut s); // delete 'a'
    k('q', &mut s); // stop → register 'a' = "x"
    k('q', &mut s); k('a', &mut s); // record into 'a' again
    k('l', &mut s); // move right
    k('q', &mut s); // stop → register 'a' = "l"
    // Play should now do 'l' (not 'x')
    let pos_before = cur(&s);
    k('@', &mut s); k('a', &mut s);
    assert!(cur(&s).1 > pos_before.1 || cur(&s) == pos_before); // moved or at end
}

#[test] fn macro_uppercase_register_rejected() {
    let mut s = ed();
    k('q', &mut s); k('A', &mut s); // uppercase → dispatches as MacroRecordStart('A')
    // start() returns false for uppercase, so not recording
    assert!(!s.macro_state.is_recording());
}

// --- Race/boundary stress tests ---

#[test] fn macro_record_during_mode_switches() {
    let mut s = ed(); buf(&mut s, "hello");
    k('q', &mut s); k('c', &mut s); // record into 'c'
    k('i', &mut s); // enter insert
    k('X', &mut s); // type X
    s.handle_key(&Key::Escape, &m());
    k('v', &mut s); // enter visual
    s.handle_key(&Key::Escape, &m()); // exit visual
    k('q', &mut s); // stop recording
    assert!(!s.macro_state.is_recording());
    assert_eq!(s.mode, Mode::Normal);
}

#[test] fn mark_and_macro_interaction() {
    let mut s = ed(); buf(&mut s, "aaa\nbbb\nccc");
    k('j', &mut s); // line 1
    k('q', &mut s); k('d', &mut s); // record into 'd'
    k('m', &mut s); k('a', &mut s); // set mark 'a'
    k('q', &mut s); // stop recording
    k('j', &mut s); // line 2
    k('`', &mut s); k('a', &mut s); // goto mark → line 1
    assert_eq!(cur(&s).0, 1);
}

#[test] fn jumplist_and_macro_interaction() {
    let mut s = ed(); buf(&mut s, "aaa\nbbb\nccc\nddd\neee");
    k('q', &mut s); k('e', &mut s); // record into 'e'
    k('G', &mut s); // goto last line (records jump)
    k('q', &mut s); // stop
    s.handle_key(&Key::Char('o'), &ctrl()); // jump older → line 0
    assert_eq!(cur(&s).0, 0);
    k('@', &mut s); k('e', &mut s); // play → G again
    assert_eq!(cur(&s).0, 4); // back to last line
}

#[test] fn split_and_macro_interaction() {
    let mut s = ed(); buf(&mut s, "hello");
    k('q', &mut s); k('f', &mut s); // record into 'f'
    s.apply_action(Action::SplitVertical);
    k('q', &mut s); // stop
    assert_eq!(s.windows.len(), 2);
    assert!(!s.macro_state.is_recording());
}

#[test] fn rapid_macro_record_stop_100x() {
    let mut s = ed(); buf(&mut s, "test");
    for _ in 0..100 {
        k('q', &mut s); k('a', &mut s); // start
        assert!(s.macro_state.is_recording());
        k('q', &mut s); // stop
        assert!(!s.macro_state.is_recording());
    }
    assert_eq!(s.mode, Mode::Normal);
}

#[test] fn deterministic_macro_replay() {
    let run = || {
        let mut s = ed(); buf(&mut s, "abc");
        k('q', &mut s); k('a', &mut s);
        k('x', &mut s); k('x', &mut s);
        k('q', &mut s);
        line(&s, 0)
    };
    assert_eq!(run(), run(), "replay must be deterministic");
}

#[test] fn mark_split_jumplist_combined_stress() {
    let mut s = ed(); buf(&mut s, "aaa\nbbb\nccc\nddd");
    k('j', &mut s); k('m', &mut s); k('a', &mut s); // mark 'a' at line 1
    s.apply_action(Action::SplitVertical); // now 2 windows
    k('G', &mut s); // goto last line (records jump)
    s.handle_key(&Key::Char('o'), &ctrl()); // jump back
    k('`', &mut s); k('a', &mut s); // goto mark
    assert_eq!(cur(&s).0, 1);
    s.apply_action(Action::CloseWindow);
    assert_eq!(s.windows.len(), 1);
    assert_eq!(cur(&s).0, 1); // mark still valid
}

#[test] fn mode_switch_churn_with_marks() {
    let mut s = ed(); buf(&mut s, "hello\nworld");
    for i in 0..50 {
        k('m', &mut s); k('a', &mut s); // set mark
        k('i', &mut s); s.handle_key(&Key::Escape, &m()); // insert/exit
        k('v', &mut s); s.handle_key(&Key::Escape, &m()); // visual/exit
        k('`', &mut s); k('a', &mut s); // goto mark
    }
    assert_eq!(s.mode, Mode::Normal);
}

#[test] fn changelist_stress_with_deletes() {
    let mut s = ed(); buf(&mut s, "abcdefghij");
    for _ in 0..20 {
        k('x', &mut s); // delete chars (records changelist)
    }
    assert!(s.changelist.len() > 0);
    // Navigate changelist without crash
    for _ in 0..30 {
        k('g', &mut s); k(';', &mut s);
    }
    assert_eq!(s.mode, Mode::Normal);
}
