//! Motion tests.

use super::apply_motion;
use kjxlkj_core::{BufferId, EditorState, Motion, MotionKind, Position, TextBuffer};

fn state_with_text(text: &str) -> EditorState {
    let mut state = EditorState::new();
    state.buffer = TextBuffer::from_str(BufferId::new(0), text);
    state
}

#[test]
fn motion_left() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 3;
    apply_motion(&mut state, Motion::new(MotionKind::Left));
    assert_eq!(state.cursor.col(), 2);
}

#[test]
fn motion_left_at_start() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 0;
    apply_motion(&mut state, Motion::new(MotionKind::Left));
    assert_eq!(state.cursor.col(), 0); // Should not go negative
}

#[test]
fn motion_right() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 1;
    apply_motion(&mut state, Motion::new(MotionKind::Right));
    assert_eq!(state.cursor.col(), 2);
}

#[test]
fn motion_right_at_end() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 4;
    apply_motion(&mut state, Motion::new(MotionKind::Right));
    assert_eq!(state.cursor.col(), 4); // Normal mode: stops at last char
}

#[test]
fn motion_up() {
    let mut state = state_with_text("line1\nline2\nline3");
    state.cursor.position.line = 2;
    apply_motion(&mut state, Motion::new(MotionKind::Up));
    assert_eq!(state.cursor.line(), 1);
}

#[test]
fn motion_down() {
    let mut state = state_with_text("line1\nline2\nline3");
    state.cursor.position.line = 0;
    apply_motion(&mut state, Motion::new(MotionKind::Down));
    assert_eq!(state.cursor.line(), 1);
}

#[test]
fn motion_line_start() {
    let mut state = state_with_text("  hello");
    state.cursor.position.col = 5;
    apply_motion(&mut state, Motion::new(MotionKind::LineStart));
    assert_eq!(state.cursor.col(), 0);
}

#[test]
fn motion_first_non_blank() {
    let mut state = state_with_text("   hello");
    state.cursor.position.col = 7;
    apply_motion(&mut state, Motion::new(MotionKind::FirstNonBlank));
    assert_eq!(state.cursor.col(), 3);
}

#[test]
fn motion_line_end() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 0;
    apply_motion(&mut state, Motion::new(MotionKind::LineEnd));
    assert_eq!(state.cursor.col(), 4); // Last char index
}

#[test]
fn motion_file_start() {
    let mut state = state_with_text("line1\nline2\nline3");
    state.cursor.position.line = 2;
    state.cursor.position.col = 3;
    apply_motion(&mut state, Motion::new(MotionKind::FileStart));
    assert_eq!(state.cursor.position, Position::new(0, 0));
}

#[test]
fn motion_file_end() {
    let mut state = state_with_text("line1\nline2\nline3");
    state.cursor.position.line = 0;
    apply_motion(&mut state, Motion::new(MotionKind::FileEnd));
    assert_eq!(state.cursor.line(), 2);
}

#[test]
fn motion_with_count() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 0;
    apply_motion(&mut state, Motion::new(MotionKind::Right).with_count(3));
    assert_eq!(state.cursor.col(), 3);
}

#[test]
fn motion_word_end() {
    let mut state = state_with_text("hello world");
    state.cursor.position.col = 0;
    apply_motion(&mut state, Motion::new(MotionKind::WordEnd));
    assert!(state.cursor.col() >= 4);
}

#[test]
fn motion_down_twice() {
    let mut state = state_with_text("a\nb\nc");
    state.cursor.position.line = 0;
    apply_motion(&mut state, Motion::new(MotionKind::Down).with_count(2));
    assert_eq!(state.cursor.line(), 2);
}

#[test]
fn motion_up_twice() {
    let mut state = state_with_text("a\nb\nc");
    state.cursor.position.line = 2;
    apply_motion(&mut state, Motion::new(MotionKind::Up).with_count(2));
    assert_eq!(state.cursor.line(), 0);
}

#[test]
fn motion_left_at_boundary() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 0;
    apply_motion(&mut state, Motion::new(MotionKind::Left));
    assert_eq!(state.cursor.col(), 0);
}

#[test]
fn motion_line_end_cursor() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 0;
    apply_motion(&mut state, Motion::new(MotionKind::LineEnd));
    assert!(state.cursor.col() >= 4);
}

#[test]
fn motion_line_start_cursor() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 3;
    apply_motion(&mut state, Motion::new(MotionKind::LineStart));
    assert_eq!(state.cursor.col(), 0);
}

#[test]
fn motion_right_one() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 0;
    apply_motion(&mut state, Motion::new(MotionKind::Right));
    assert_eq!(state.cursor.col(), 1);
}

#[test]
fn motion_left_one() {
    let mut state = state_with_text("hello");
    state.cursor.position.col = 2;
    apply_motion(&mut state, Motion::new(MotionKind::Left));
    assert_eq!(state.cursor.col(), 1);
}

#[test]
fn motion_down_one() {
    let mut state = state_with_text("a\nb\nc");
    state.cursor.position.line = 0;
    apply_motion(&mut state, Motion::new(MotionKind::Down));
    assert_eq!(state.cursor.line(), 1);
}

#[test]
fn motion_up_one() {
    let mut state = state_with_text("a\nb\nc");
    state.cursor.position.line = 1;
    apply_motion(&mut state, Motion::new(MotionKind::Up));
    assert_eq!(state.cursor.line(), 0);
}

#[test]
fn motion_first_line() {
    let mut state = state_with_text("a\nb\nc");
    state.cursor.position.line = 2;
    apply_motion(&mut state, Motion::new(MotionKind::FileStart));
    assert_eq!(state.cursor.line(), 0);
}
