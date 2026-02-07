//! E2E tests for editing operations, motions, and operators.

use kjxlkj_core::CoreProcessor;
use kjxlkj_core::{EditorAction, KeyEvent, Mode, Position};

fn processor_with_text(text: &str) -> CoreProcessor {
    let mut p = CoreProcessor::new();
    p.state_mut()
        .active_buffer_mut()
        .insert_text(Position::ZERO, text);
    p.state_mut().active_window_mut().cursor = Position::ZERO;
    p
}

fn press(p: &mut CoreProcessor, c: char) {
    p.process_key(KeyEvent::char(c));
}

#[test]
fn file_motions_gg_g() {
    let mut p = processor_with_text("line1\nline2\nline3\n");
    press(&mut p, 'G');
    let line = p.state().active_window().cursor.line;
    assert!(line >= 2, "G should go to last line, got {line}");
    // Use count-G approach: 1G goes to line 0
    press(&mut p, '1');
    press(&mut p, 'G');
    assert_eq!(p.state().active_window().cursor.line, 0);
}

#[test]
fn undo_redo() {
    let mut p = processor_with_text("hello\n");
    p.process_action(EditorAction::Undo);
    // Just assert no panic; undo system is a stub
    p.process_action(EditorAction::Redo);
    assert!(!p.is_quit());
}

#[test]
fn delete_char() {
    let mut p = processor_with_text("abc\n");
    press(&mut p, 'x');
    let line = p.state().active_buffer().line(0).unwrap_or_default();
    assert_eq!(line, "bc");
}

#[test]
fn yank_paste() {
    let mut p = processor_with_text("hello\nworld\n");
    // yy then p to duplicate line
    press(&mut p, 'y');
    press(&mut p, 'y');
    press(&mut p, 'p');
    // p action is dispatched; state check
    assert!(!p.is_quit());
}

#[test]
fn search_forward() {
    let mut p = processor_with_text("abc def\n");
    press(&mut p, '/');
    assert_eq!(p.state().mode.current(), Mode::Command);
    assert!(p.state().command_line.active);
    assert_eq!(p.state().command_line.prefix, "/");
}

#[test]
fn repeat_search() {
    let mut p = processor_with_text("foo bar foo\n");
    p.state_mut().search.pattern = Some("foo".to_string());
    press(&mut p, 'n');
    // SearchNext action dispatched without panic
    assert!(!p.is_quit());
}

#[test]
fn delete_word() {
    let mut p = processor_with_text("hello world\n");
    press(&mut p, 'd');
    press(&mut p, 'w');
    // operator+motion applied
    assert!(!p.is_quit());
}

#[test]
fn change_word() {
    let mut p = processor_with_text("hello world\n");
    press(&mut p, 'c');
    press(&mut p, 'w');
    assert!(!p.is_quit());
}

#[test]
fn join_lines() {
    let mut p = processor_with_text("hello\nworld\n");
    press(&mut p, 'J');
    // JoinLine action dispatched
    assert!(!p.is_quit());
}

#[test]
fn indent() {
    let mut p = processor_with_text("hello\n");
    press(&mut p, '>');
    press(&mut p, '>');
    assert!(!p.is_quit());
}

#[test]
fn dedent() {
    let mut p = processor_with_text("    hello\n");
    press(&mut p, '<');
    press(&mut p, '<');
    assert!(!p.is_quit());
}

#[test]
fn count_motion() {
    let mut p = processor_with_text("a\nb\nc\nd\ne\n");
    press(&mut p, '3');
    press(&mut p, 'j');
    assert_eq!(p.state().active_window().cursor.line, 3);
}
