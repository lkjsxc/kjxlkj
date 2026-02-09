//! Tests for macro recording and playback.

use kjxlkj_core_types::Key;

use crate::editor::EditorState;

fn ed() -> EditorState {
    let mut e = EditorState::new(80, 24);
    e.open_file("test.txt", "hello world\nfoo bar\nbaz qux\n");
    e
}

#[test]
fn start_and_stop_recording() {
    let mut e = ed();
    // q a starts recording into 'a'
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('a'));
    assert!(e.is_recording());
    assert_eq!(e.recording_macro, Some('a'));
    // q stops recording
    e.handle_key(Key::char('q'));
    assert!(!e.is_recording());
    assert!(e.macro_store.contains_key(&'a'));
}

#[test]
fn macro_records_keys() {
    let mut e = ed();
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('a'));
    // Type some keys: j then j
    e.handle_key(Key::char('j'));
    e.handle_key(Key::char('j'));
    e.handle_key(Key::char('q'));
    let keys = e.macro_store.get(&'a').unwrap();
    assert_eq!(keys.len(), 2);
}

#[test]
fn macro_playback_replays_keys() {
    let mut e = ed();
    // Record macro: j (move down)
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('a'));
    e.handle_key(Key::char('j'));
    e.handle_key(Key::char('q'));
    assert_eq!(e.windows.focused().cursor.line, 1);
    // Play back @a
    e.handle_key(Key::char('@'));
    e.handle_key(Key::char('a'));
    assert_eq!(e.windows.focused().cursor.line, 2);
}

#[test]
fn macro_replay_last_with_at_at() {
    let mut e = ed();
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('b'));
    e.handle_key(Key::char('j'));
    e.handle_key(Key::char('q'));
    assert_eq!(e.windows.focused().cursor.line, 1);
    // @b then @@
    e.handle_key(Key::char('@'));
    e.handle_key(Key::char('b'));
    assert_eq!(e.windows.focused().cursor.line, 2);
    // Wrap around: @@
    e.handle_key(Key::char('@'));
    e.handle_key(Key::char('@'));
    // Should have replayed last macro (b → j)
    assert_eq!(e.last_macro, Some('b'));
}

#[test]
fn macro_dd_deletes_line() {
    let mut e = ed();
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('c'));
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('q'));
    // dd should have deleted the first line
    let text = e.buffers.current().content.to_string();
    assert!(!text.starts_with("hello"));
    // Play back: another line deleted
    let lines_before = e.buffers.current().line_count();
    e.handle_key(Key::char('@'));
    e.handle_key(Key::char('c'));
    let lines_after = e.buffers.current().line_count();
    assert_eq!(lines_after, lines_before - 1);
}

#[test]
fn q_not_recording_does_not_crash() {
    let mut e = ed();
    // Just pressing q then Esc shouldn't crash
    e.handle_key(Key::char('q'));
    e.handle_key(Key::esc());
    assert!(!e.is_recording());
}
