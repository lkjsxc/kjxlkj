//! Tests for command-line mode dispatch.

use kjxlkj_core_state::{handle_cmdline_key, EditorState};
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode, Size};

fn setup() -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("hello");
    s.create_window(bid);
    s.mode.transition(Mode::Command);
    s.cmdline.prefix = ':';
    s
}

#[test]
fn type_char() {
    let mut s = setup();
    let r = handle_cmdline_key(&mut s, &KeyEvent::char('q'));
    assert_eq!(r, Intent::Noop);
    assert_eq!(s.cmdline.text, "q");
    assert_eq!(s.cmdline.cursor, 1);
}

#[test]
fn enter_executes() {
    let mut s = setup();
    handle_cmdline_key(&mut s, &KeyEvent::char('q'));
    let r = handle_cmdline_key(&mut s, &KeyEvent::special(KeyCode::Enter));
    assert_eq!(r, Intent::ExCommand(":q".into()));
}

#[test]
fn escape_cancels() {
    let mut s = setup();
    handle_cmdline_key(&mut s, &KeyEvent::char('q'));
    let r = handle_cmdline_key(&mut s, &KeyEvent::special(KeyCode::Escape));
    assert_eq!(r, Intent::EnterMode(Mode::Normal));
    assert!(s.cmdline.text.is_empty());
}

#[test]
fn backspace_deletes() {
    let mut s = setup();
    handle_cmdline_key(&mut s, &KeyEvent::char('a'));
    handle_cmdline_key(&mut s, &KeyEvent::char('b'));
    handle_cmdline_key(&mut s, &KeyEvent::special(KeyCode::Backspace));
    assert_eq!(s.cmdline.text, "a");
}

#[test]
fn history_navigation() {
    let mut s = setup();
    s.cmdline.history = vec!["first".into(), "second".into()];
    // Use handle_cmdline_key with Up/Down keys
    handle_cmdline_key(&mut s, &KeyEvent::special(KeyCode::Up));
    assert_eq!(s.cmdline.text, "second");
    handle_cmdline_key(&mut s, &KeyEvent::special(KeyCode::Up));
    assert_eq!(s.cmdline.text, "first");
    handle_cmdline_key(&mut s, &KeyEvent::special(KeyCode::Down));
    assert_eq!(s.cmdline.text, "second");
}

#[test]
fn tab_completion() {
    let mut s = setup();
    s.cmdline.text = "wr".into();
    s.cmdline.cursor = 2;
    handle_cmdline_key(&mut s, &KeyEvent::special(KeyCode::Tab));
    assert_eq!(s.cmdline.text, "write");
}

#[test]
fn search_forward() {
    let mut s = setup();
    s.cmdline.prefix = '/';
    handle_cmdline_key(&mut s, &KeyEvent::char('h'));
    handle_cmdline_key(&mut s, &KeyEvent::char('i'));
    let r = handle_cmdline_key(&mut s, &KeyEvent::special(KeyCode::Enter));
    assert_eq!(r, Intent::SearchForward("hi".into()));
}

#[test]
fn search_backward() {
    let mut s = setup();
    s.cmdline.prefix = '?';
    handle_cmdline_key(&mut s, &KeyEvent::char('x'));
    let r = handle_cmdline_key(&mut s, &KeyEvent::special(KeyCode::Enter));
    assert_eq!(r, Intent::SearchBackward("x".into()));
}
