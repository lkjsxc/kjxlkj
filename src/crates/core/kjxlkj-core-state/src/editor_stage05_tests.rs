//! Integration tests for wave-046 insert sub-state features.

use crate::editor::EditorState;
use kjxlkj_core_types::{Action, Key, KeyModifiers, Mode};
use kjxlkj_core_mode::PendingState;

fn ed() -> EditorState {
    EditorState::new(80, 24)
}

fn ctrl() -> KeyModifiers { KeyModifiers { ctrl: true, ..KeyModifiers::default() } }

#[test]
fn insert_digraph_lookup_hit() {
    let mut s = ed();
    s.mode = Mode::Insert;
    // InsertDigraph('a', ':') should insert ä (U+00E4)
    s.apply_action(Action::InsertDigraph('a', ':'));
    let buf = s.buffers.values().next().unwrap();
    let line = buf.line(0).unwrap();
    assert!(line.contains('\u{00E4}'), "Expected ä in: {}", line);
}

#[test]
fn insert_digraph_fallback() {
    let mut s = ed();
    s.mode = Mode::Insert;
    // Unknown digraph ('z','z') → inserts second char literally
    s.apply_action(Action::InsertDigraph('z', 'z'));
    let buf = s.buffers.values().next().unwrap();
    let line = buf.line(0).unwrap();
    assert!(line.contains('z'), "Expected z in: {}", line);
}

#[test]
fn insert_literal_char() {
    let mut s = ed();
    s.mode = Mode::Insert;
    s.apply_action(Action::InsertLiteral('\x01'));
    let buf = s.buffers.values().next().unwrap();
    let line = buf.line(0).unwrap();
    assert!(line.contains('\x01'));
}

#[test]
fn insert_register_unnamed() {
    let mut s = ed();
    s.mode = Mode::Insert;
    // Yank some text into unnamed register
    s.apply_action(Action::InsertChar('A'));
    s.apply_action(Action::InsertChar('B'));
    s.apply_action(Action::ExitToNormal);
    // Yank current line
    s.apply_action(Action::YankCurrentLine);
    // Enter insert and paste from unnamed register
    s.mode = Mode::Insert;
    s.apply_action(Action::InsertRegister('"'));
    let buf = s.buffers.values().next().unwrap();
    let text = buf.line(0).unwrap();
    // Should have "AB" duplicated (original + register insert)
    assert!(text.contains("AB"), "Expected AB in: {}", text);
}

#[test]
fn insert_register_empty() {
    let mut s = ed();
    s.mode = Mode::Insert;
    // Insert from unset register — should do nothing
    s.apply_action(Action::InsertRegister('z'));
    let buf = s.buffers.values().next().unwrap();
    let line = buf.line(0).unwrap();
    assert_eq!(line.trim_end_matches('\n'), "");
}

#[test]
fn completion_actions_no_panic() {
    let mut s = ed();
    s.mode = Mode::Insert;
    // These are dispatched but currently deferred — should not panic
    s.apply_action(Action::CompletionTrigger(None));
    s.apply_action(Action::CompletionNext);
    s.apply_action(Action::CompletionPrev);
    s.apply_action(Action::CompletionAccept);
    s.apply_action(Action::CompletionCancel);
}

#[test]
fn snippet_actions_no_panic() {
    let mut s = ed();
    s.apply_action(Action::SnippetNext);
    s.apply_action(Action::SnippetPrev);
    s.apply_action(Action::SnippetCancel);
}

#[test]
fn insert_ctrl_n_emits_completion_next() {
    // Verify mode handler maps Ctrl-n to CompletionNext
    let (action, mode) = kjxlkj_core_mode::dispatch_key(
        Mode::Insert, &Key::Char('n'), &ctrl(), &mut PendingState::default(),
    );
    assert_eq!(action, Action::CompletionNext);
    assert_eq!(mode, None);
}

#[test]
fn insert_ctrl_p_emits_completion_prev() {
    let (action, _) = kjxlkj_core_mode::dispatch_key(
        Mode::Insert, &Key::Char('p'), &ctrl(), &mut PendingState::default(),
    );
    assert_eq!(action, Action::CompletionPrev);
}

#[test]
fn insert_ctrl_y_emits_completion_accept() {
    let (action, _) = kjxlkj_core_mode::dispatch_key(
        Mode::Insert, &Key::Char('y'), &ctrl(), &mut PendingState::default(),
    );
    assert_eq!(action, Action::CompletionAccept);
}

#[test]
fn insert_ctrl_e_emits_completion_cancel() {
    let (action, _) = kjxlkj_core_mode::dispatch_key(
        Mode::Insert, &Key::Char('e'), &ctrl(), &mut PendingState::default(),
    );
    assert_eq!(action, Action::CompletionCancel);
}

#[test]
fn insert_ctrl_x_emits_completion_trigger() {
    let (action, _) = kjxlkj_core_mode::dispatch_key(
        Mode::Insert, &Key::Char('x'), &ctrl(), &mut PendingState::default(),
    );
    assert_eq!(action, Action::CompletionTrigger(None));
}
