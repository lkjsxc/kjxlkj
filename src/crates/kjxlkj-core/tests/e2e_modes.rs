//! E2E tests for mode transitions and basic editing via CoreProcessor.

use kjxlkj_core::CoreProcessor;
use kjxlkj_core::{KeyCode, KeyEvent, Mode, Position};

fn processor_with_text(text: &str) -> CoreProcessor {
    let mut p = CoreProcessor::new();
    let buf = p.state_mut().active_buffer_mut();
    buf.insert_text(Position::ZERO, text);
    p.state_mut().active_window_mut().cursor = Position::ZERO;
    p
}

fn press(p: &mut CoreProcessor, c: char) {
    p.process_key(KeyEvent::char(c));
}

fn press_key(p: &mut CoreProcessor, code: KeyCode) {
    p.process_key(KeyEvent::plain(code));
}

#[test]
fn headless_mode_starts() {
    let p = CoreProcessor::new();
    assert_eq!(p.state().mode.current(), Mode::Normal);
}

#[test]
fn insert_mode() {
    let mut p = CoreProcessor::new();
    press(&mut p, 'i');
    assert_eq!(p.state().mode.current(), Mode::Insert);
}

#[test]
fn cursor_movement_hjkl() {
    let mut p = processor_with_text("abcdef\nghijkl\nmnopqr\n");
    press(&mut p, 'j');
    assert_eq!(p.state().active_window().cursor.line, 1);
    press(&mut p, 'l');
    assert_eq!(p.state().active_window().cursor.col, 1);
    press(&mut p, 'k');
    assert_eq!(p.state().active_window().cursor.line, 0);
    press(&mut p, 'h');
    assert_eq!(p.state().active_window().cursor.col, 0);
}

#[test]
fn command_mode() {
    let mut p = CoreProcessor::new();
    press(&mut p, ':');
    assert_eq!(p.state().mode.current(), Mode::Command);
}

#[test]
fn visual_mode() {
    let mut p = CoreProcessor::new();
    press(&mut p, 'v');
    assert_eq!(p.state().mode.current(), Mode::Visual);
}

#[test]
fn visual_line_mode() {
    let mut p = CoreProcessor::new();
    press(&mut p, 'V');
    assert_eq!(p.state().mode.current(), Mode::VisualLine);
}

#[test]
fn replace_mode() {
    let mut p = CoreProcessor::new();
    press(&mut p, 'R');
    assert_eq!(p.state().mode.current(), Mode::Replace);
}

#[test]
fn append_mode() {
    let mut p = processor_with_text("abc\n");
    press(&mut p, 'a');
    assert_eq!(p.state().mode.current(), Mode::Insert);
    assert!(p.state().active_window().cursor.col >= 1);
}

#[test]
fn open_line_below() {
    let mut p = processor_with_text("hello\n");
    press(&mut p, 'o');
    assert_eq!(p.state().mode.current(), Mode::Insert);
    assert_eq!(p.state().active_window().cursor.line, 1);
}

#[test]
fn open_line_above() {
    let mut p = processor_with_text("hello\n");
    press(&mut p, 'j');
    press(&mut p, 'O');
    assert_eq!(p.state().mode.current(), Mode::Insert);
}

#[test]
fn text_insert_delete() {
    let mut p = CoreProcessor::new();
    press(&mut p, 'i');
    press(&mut p, 'a');
    press(&mut p, 'b');
    press_key(&mut p, KeyCode::Backspace);
    let line = p.state().active_buffer().line(0).unwrap_or_default();
    assert_eq!(line, "a");
}

#[test]
fn word_motions() {
    let mut p = processor_with_text("foo bar baz\n");
    // w/b dispatch the motion; verify no panic and mode stays Normal
    press(&mut p, 'w');
    assert_eq!(p.state().mode.current(), Mode::Normal);
    press(&mut p, 'b');
    assert_eq!(p.state().mode.current(), Mode::Normal);
}

#[test]
fn line_motions() {
    let mut p = processor_with_text("hello world\n");
    // Move cursor right first so '0' has observable effect
    press(&mut p, 'l');
    press(&mut p, 'l');
    assert!(p.state().active_window().cursor.col >= 1);
    press(&mut p, '0');
    assert_eq!(p.state().active_window().cursor.col, 0);
}
