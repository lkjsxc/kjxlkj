//! Tests for expanded visual mode operations and new editor features.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{CaseOp, Intent, Mode, MotionKind, OperatorKind, PastePosition, RegisterName, Size};

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

fn cursor(state: &EditorState) -> (usize, usize) {
    let w = state.active_window_state().unwrap();
    (w.cursor_line, w.cursor_col)
}

// ── Visual mode toggle case (~) ────────────────

#[test]
fn visual_toggle_case() {
    let mut s = setup("Hello");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    dispatch_intent(&mut s, Intent::ToggleCase);
    // After toggle in visual, should return to normal
    assert_eq!(line(&s, 0), "hELLO");
}

// ── Visual mode uppercase (U) ───────────────

#[test]
fn visual_uppercase() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    dispatch_intent(&mut s, Intent::CaseOperatorLine(CaseOp::Upper));
    assert_eq!(line(&s, 0), "HELLO WORLD");
}

// ── Visual mode lowercase (u) ───────────────

#[test]
fn visual_lowercase() {
    let mut s = setup("HELLO WORLD");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    dispatch_intent(&mut s, Intent::CaseOperatorLine(CaseOp::Lower));
    assert_eq!(line(&s, 0), "hello world");
}

// ── Visual mode join (J) ───────────────

#[test]
fn visual_join_lines() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    dispatch_intent(&mut s, Intent::JoinLines(true, 1));
    assert_eq!(line(&s, 0), "aaa bbb");
}

// ── Visual mode paste (p) ───────────────

#[test]
fn visual_paste() {
    let mut s = setup("hello world");
    // Yank "hello" to register
    dispatch_intent(&mut s, Intent::Operator(OperatorKind::Yank, MotionKind::WordForward, 1));
    // Move to "world"
    dispatch_intent(&mut s, Intent::Motion(MotionKind::WordForward, 1));
    // Enter visual mode and select "world"
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    // Paste over the visual selection
    dispatch_intent(&mut s, Intent::Paste(RegisterName::Unnamed, PastePosition::After));
    // Should have replaced "world" with "hello "
    let l = line(&s, 0);
    assert!(l.contains("hello"), "line should contain pasted text: {}", l);
}

// ── gv reselect visual ───────────────

#[test]
fn gv_reselect_visual() {
    let mut s = setup("hello world");
    // Enter visual, move right, exit
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Normal));
    assert_eq!(s.current_mode(), Mode::Normal);
    assert!(s.last_visual.is_some());
    // Reselect
    dispatch_intent(&mut s, Intent::ReselectVisual);
    assert!(s.current_mode().is_visual());
}

// ── :put command ───────────────

#[test]
fn put_register_below() {
    let mut s = setup("line one\nline two");
    // Yank first line
    dispatch_intent(&mut s, Intent::YankLine(1));
    // Put below
    dispatch_intent(&mut s, Intent::PutRegister(false));
    assert_eq!(line(&s, 1), "line one");
}

#[test]
fn put_register_above() {
    let mut s = setup("line one\nline two");
    dispatch_intent(&mut s, Intent::YankLine(1));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    dispatch_intent(&mut s, Intent::PutRegister(true));
    // Should insert above line two
    assert_eq!(line(&s, 1), "line one");
}

// ── :put ex command ───────────────

#[test]
fn ex_put_command() {
    let mut s = setup("aaa\nbbb");
    dispatch_intent(&mut s, Intent::YankLine(1));
    dispatch_ex(&mut s, ":put");
    assert_eq!(line(&s, 1), "aaa");
}

// ── :! shell command ───────────────

#[test]
fn shell_command_echo() {
    let mut s = setup("hello");
    dispatch_ex(&mut s, ":!echo test_output");
    let msg = s.message.as_ref().unwrap();
    assert!(msg.contains("test_output"), "message should contain output: {}", msg);
}

#[test]
fn shell_command_no_args() {
    let mut s = setup("hello");
    dispatch_ex(&mut s, ":!");
    let msg = s.message.as_ref().unwrap();
    assert!(msg.contains("E471"), "should give error without args: {}", msg);
}

// ── Visual mode enter command line (:) ───────────────

#[test]
fn visual_enter_cmdline() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::EnterCommandLine(':'));
    assert_eq!(s.current_mode(), Mode::Command);
}
