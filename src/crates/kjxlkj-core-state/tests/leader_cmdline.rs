//! Tests for leader chords, command history, macro persistence, and long-line slicing.

use kjxlkj_core_state::{dispatch_intent, handle_cmdline_key, EditorState, MacroStore, KeyStroke};
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode, MotionKind, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

fn key(c: char) -> KeyEvent { KeyEvent::char(c) }
fn ctrl(c: char) -> KeyEvent { KeyEvent::ctrl(c) }
fn special(code: KeyCode) -> KeyEvent { KeyEvent::special(code) }

// --- Leader chord parsing ---

#[test]
fn leader_e_triggers_explorer() {
    let mut s = setup("hello");
    // Space enters leader pending
    use kjxlkj_core_mode::KeyParser;
    let mut parser = KeyParser::new();
    let i1 = parser.parse_normal(&key(' '));
    assert_eq!(i1, Intent::Noop); // pending
    let i2 = parser.parse_normal(&key('e'));
    assert_eq!(i2, Intent::ExCommand(":explorer".into()));
}

#[test]
fn leader_t_triggers_terminal() {
    use kjxlkj_core_mode::KeyParser;
    let mut parser = KeyParser::new();
    let _ = parser.parse_normal(&key(' '));
    let i = parser.parse_normal(&key('t'));
    assert_eq!(i, Intent::ExCommand(":terminal".into()));
}

#[test]
fn leader_f_triggers_find() {
    use kjxlkj_core_mode::KeyParser;
    let mut parser = KeyParser::new();
    let _ = parser.parse_normal(&key(' '));
    let i = parser.parse_normal(&key('f'));
    assert_eq!(i, Intent::ExCommand(":find".into()));
}

#[test]
fn leader_g_triggers_livegrep() {
    use kjxlkj_core_mode::KeyParser;
    let mut parser = KeyParser::new();
    let _ = parser.parse_normal(&key(' '));
    let i = parser.parse_normal(&key('g'));
    assert_eq!(i, Intent::ExCommand(":livegrep".into()));
}

#[test]
fn leader_b_triggers_buffers() {
    use kjxlkj_core_mode::KeyParser;
    let mut parser = KeyParser::new();
    let _ = parser.parse_normal(&key(' '));
    let i = parser.parse_normal(&key('b'));
    assert_eq!(i, Intent::ExCommand(":ls".into()));
}

#[test]
fn leader_unknown_is_noop() {
    use kjxlkj_core_mode::KeyParser;
    let mut parser = KeyParser::new();
    let _ = parser.parse_normal(&key(' '));
    let i = parser.parse_normal(&key('z'));
    assert_eq!(i, Intent::Noop);
}

// --- Command history navigation ---

#[test]
fn cmdline_history_up_down() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterCommandLine(':'));
    // Type and execute "set number"
    for c in "set number".chars() {
        handle_cmdline_key(&mut s, &key(c));
    }
    let intent = handle_cmdline_key(&mut s, &special(KeyCode::Enter));
    assert!(matches!(intent, Intent::ExCommand(_)));
    // Enter command mode again
    dispatch_intent(&mut s, Intent::EnterCommandLine(':'));
    // Press Up to recall "set number"
    handle_cmdline_key(&mut s, &special(KeyCode::Up));
    assert_eq!(s.cmdline.text, "set number");
    // Press Down to go back to empty
    handle_cmdline_key(&mut s, &special(KeyCode::Down));
    assert_eq!(s.cmdline.text, "");
}

#[test]
fn cmdline_ctrl_u_clears_to_start() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterCommandLine(':'));
    for c in "hello world".chars() {
        handle_cmdline_key(&mut s, &key(c));
    }
    // Move cursor to middle
    for _ in 0..5 { handle_cmdline_key(&mut s, &special(KeyCode::Left)); }
    handle_cmdline_key(&mut s, &ctrl('u'));
    assert_eq!(s.cmdline.text, "world");
}

#[test]
fn cmdline_ctrl_w_deletes_word() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterCommandLine(':'));
    for c in "set number".chars() {
        handle_cmdline_key(&mut s, &key(c));
    }
    handle_cmdline_key(&mut s, &ctrl('w'));
    assert_eq!(s.cmdline.text, "set ");
}

#[test]
fn cmdline_tab_completion() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterCommandLine(':'));
    for c in "qu".chars() {
        handle_cmdline_key(&mut s, &key(c));
    }
    handle_cmdline_key(&mut s, &special(KeyCode::Tab));
    assert!(s.cmdline.text.starts_with("quit"));
}

// --- Macro persistence ---

#[test]
fn macro_store_roundtrip() {
    let mut store = MacroStore::new();
    store.store('a', vec![
        KeyStroke { code: "j".into(), ctrl: false, alt: false, shift: false },
        KeyStroke { code: "d".into(), ctrl: false, alt: false, shift: false },
    ]);
    store.store('b', vec![
        KeyStroke { code: "x".into(), ctrl: false, alt: false, shift: false },
    ]);
    assert_eq!(store.len(), 2);
    assert_eq!(store.registers(), vec!['a', 'b']);
    let a = store.get('a').unwrap();
    assert_eq!(a.len(), 2);
    assert_eq!(a[0].code, "j");
}

// --- Snapshot line slicing for long lines ---

#[test]
fn snapshot_line_slice_basic() {
    use kjxlkj_core_text::{TextBuffer, BufferSnapshot};
    use kjxlkj_core_types::Position;
    let long = "x".repeat(1000);
    let buf = TextBuffer::from_text(&long);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 1, Position::new(0, 0));
    let slice = snap.line_slice(0, 100, 80).unwrap();
    assert_eq!(slice.len(), 80);
}

#[test]
fn snapshot_line_slice_at_end() {
    use kjxlkj_core_text::{TextBuffer, BufferSnapshot};
    use kjxlkj_core_types::Position;
    let buf = TextBuffer::from_text("short");
    let snap = BufferSnapshot::from_buffer(&buf, 0, 1, Position::new(0, 0));
    let slice = snap.line_slice(0, 3, 80).unwrap();
    assert_eq!(slice, "rt");
}

#[test]
fn snapshot_line_slice_no_overflow() {
    use kjxlkj_core_text::{TextBuffer, BufferSnapshot};
    use kjxlkj_core_types::Position;
    let buf = TextBuffer::from_text("abc");
    let snap = BufferSnapshot::from_buffer(&buf, 0, 1, Position::new(0, 0));
    let slice = snap.line_slice(0, 10, 80).unwrap();
    assert_eq!(slice, ""); // past end of line
}
