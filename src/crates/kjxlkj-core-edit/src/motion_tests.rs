//! Tests for motion definitions.

use crate::{Motion, MotionKind};

#[test]
fn motion_with_count() {
    let m = Motion::new(MotionKind::Right).with_count(5);
    assert_eq!(m.count, 5);
    assert_eq!(m.kind, MotionKind::Right);
}

#[test]
fn motion_inclusive() {
    let m = Motion::new(MotionKind::WordEnd).inclusive();
    assert!(m.inclusive);
}

#[test]
fn motion_default_count() {
    let m = Motion::new(MotionKind::Left);
    assert_eq!(m.count, 1);
}

#[test]
fn motion_count_minimum_one() {
    let m = Motion::new(MotionKind::Right).with_count(0);
    assert_eq!(m.count, 1);
}

#[test]
fn motion_find_char() {
    let m = Motion::new(MotionKind::FindChar('x'));
    assert_eq!(m.kind, MotionKind::FindChar('x'));
}

#[test]
fn motion_goto_line() {
    let m = Motion::new(MotionKind::GoToLine(42));
    assert_eq!(m.kind, MotionKind::GoToLine(42));
}

#[test]
fn motion_kind_equality() {
    assert_eq!(MotionKind::Left, MotionKind::Left);
    assert_ne!(MotionKind::Left, MotionKind::Right);
}

#[test]
fn motion_chained_builder() {
    let m = Motion::new(MotionKind::Down).with_count(3).inclusive();
    assert_eq!(m.count, 3);
    assert!(m.inclusive);
}

#[test]
fn motion_screen_positions() {
    let _ = Motion::new(MotionKind::ScreenTop);
    let _ = Motion::new(MotionKind::ScreenMiddle);
    let _ = Motion::new(MotionKind::ScreenBottom);
}

#[test]
fn motion_paragraph() {
    let m = Motion::new(MotionKind::ParagraphForward);
    assert_eq!(m.kind, MotionKind::ParagraphForward);
}

#[test]
fn motion_sentence() {
    let m = Motion::new(MotionKind::SentenceForward);
    assert_eq!(m.kind, MotionKind::SentenceForward);
}

#[test]
fn motion_goto_percent() {
    let m = Motion::new(MotionKind::GoToPercent(50));
    assert_eq!(m.kind, MotionKind::GoToPercent(50));
}

#[test]
fn motion_till_char() {
    let m = Motion::new(MotionKind::TillChar('a'));
    assert_eq!(m.kind, MotionKind::TillChar('a'));
}

#[test]
fn motion_clone() {
    let m = Motion::new(MotionKind::WordStart);
    let cloned = m.clone();
    assert_eq!(m, cloned);
}

#[test]
fn motion_sentence_backward() {
    let m = Motion::new(MotionKind::SentenceBackward);
    assert_eq!(m.kind, MotionKind::SentenceBackward);
}

#[test]
fn motion_paragraph_backward() {
    let m = Motion::new(MotionKind::ParagraphBackward);
    assert_eq!(m.kind, MotionKind::ParagraphBackward);
}

#[test]
fn motion_till_char_backward() {
    let m = Motion::new(MotionKind::TillCharBackward('z'));
    assert_eq!(m.kind, MotionKind::TillCharBackward('z'));
}

#[test]
fn motion_repeat_find() {
    let m = Motion::new(MotionKind::RepeatFind);
    assert_eq!(m.kind, MotionKind::RepeatFind);
}

#[test]
fn motion_repeat_find_reverse() {
    let m = Motion::new(MotionKind::RepeatFindReverse);
    assert_eq!(m.kind, MotionKind::RepeatFindReverse);
}

#[test]
fn motion_screen_top() {
    let m = Motion::new(MotionKind::ScreenTop);
    assert_eq!(m.kind, MotionKind::ScreenTop);
}

#[test]
fn motion_screen_middle() {
    let m = Motion::new(MotionKind::ScreenMiddle);
    assert_eq!(m.kind, MotionKind::ScreenMiddle);
}

#[test]
fn motion_screen_bottom() {
    let m = Motion::new(MotionKind::ScreenBottom);
    assert_eq!(m.kind, MotionKind::ScreenBottom);
}
