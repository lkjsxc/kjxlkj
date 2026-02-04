//! Tests for intent types.

use super::intent::*;
use kjxlkj_core_types::Mode;

#[test]
fn intent_none() {
    let intent = Intent::None;
    assert_eq!(intent, Intent::None);
}

#[test]
fn intent_change_mode() {
    let intent = Intent::ChangeMode(Mode::Insert);
    if let Intent::ChangeMode(mode) = intent {
        assert_eq!(mode, Mode::Insert);
    } else {
        panic!("Expected ChangeMode");
    }
}

#[test]
fn intent_quit() {
    let intent = Intent::Quit { force: false };
    assert!(matches!(intent, Intent::Quit { .. }));
}

#[test]
fn intent_scroll() {
    let intent = Intent::Scroll(ScrollIntent::HalfPageDown);
    assert!(matches!(intent, Intent::Scroll(_)));
}

#[test]
fn intent_enter_insert() {
    let intent = Intent::EnterInsert { at_line_end: true, after_cursor: false };
    assert!(matches!(intent, Intent::EnterInsert { .. }));
}

#[test]
fn intent_undo_redo() {
    let undo = Intent::Undo;
    let redo = Intent::Redo;
    assert_eq!(undo, Intent::Undo);
    assert_eq!(redo, Intent::Redo);
}

#[test]
fn scroll_intent_variants() {
    let _down = ScrollIntent::HalfPageDown;
    let _up = ScrollIntent::HalfPageUp;
    let _full_down = ScrollIntent::FullPageDown;
    let _full_up = ScrollIntent::FullPageUp;
    let _line_down = ScrollIntent::LineDown;
    let _line_up = ScrollIntent::LineUp;
}

#[test]
fn center_kind_variants() {
    let _center = CenterKind::Center;
    let _top = CenterKind::Top;
    let _bottom = CenterKind::Bottom;
}

#[test]
fn intent_insert_text() {
    let intent = Intent::InsertText("hello".to_string());
    assert!(matches!(intent, Intent::InsertText(_)));
}

#[test]
fn intent_delete_char() {
    let intent = Intent::DeleteChar;
    assert_eq!(intent, Intent::DeleteChar);
}

#[test]
fn intent_delete_char_before() {
    let intent = Intent::DeleteCharBefore;
    assert_eq!(intent, Intent::DeleteCharBefore);
}

#[test]
fn intent_open_line_below() {
    let intent = Intent::OpenLineBelow;
    assert_eq!(intent, Intent::OpenLineBelow);
}

#[test]
fn intent_open_line_above() {
    let intent = Intent::OpenLineAbove;
    assert_eq!(intent, Intent::OpenLineAbove);
}

#[test]
fn intent_join_lines() {
    let intent = Intent::JoinLines { with_space: true };
    assert!(matches!(intent, Intent::JoinLines { with_space: true }));
}

#[test]
fn intent_paste() {
    let intent = Intent::Paste { before: false, cursor_at_end: true };
    assert!(matches!(intent, Intent::Paste { .. }));
}

#[test]
fn intent_start_macro() {
    let intent = Intent::StartMacro('q');
    assert!(matches!(intent, Intent::StartMacro('q')));
}

#[test]
fn intent_stop_macro() {
    let intent = Intent::StopMacro;
    assert_eq!(intent, Intent::StopMacro);
}

#[test]
fn intent_play_macro() {
    let intent = Intent::PlayMacro('a');
    assert!(matches!(intent, Intent::PlayMacro('a')));
}

#[test]
fn intent_set_mark() {
    let intent = Intent::SetMark('m');
    assert!(matches!(intent, Intent::SetMark('m')));
}

#[test]
fn intent_jump_to_mark() {
    let intent = Intent::JumpToMark { mark: 'a', line_start: true };
    assert!(matches!(intent, Intent::JumpToMark { .. }));
}

#[test]
fn intent_search_forward() {
    let intent = Intent::SearchForward;
    assert_eq!(intent, Intent::SearchForward);
}

#[test]
fn intent_search_backward() {
    let intent = Intent::SearchBackward;
    assert_eq!(intent, Intent::SearchBackward);
}

#[test]
fn intent_next_match() {
    let intent = Intent::NextMatch;
    assert_eq!(intent, Intent::NextMatch);
}

#[test]
fn intent_prev_match() {
    let intent = Intent::PrevMatch;
    assert_eq!(intent, Intent::PrevMatch);
}

#[test]
fn intent_cancel() {
    let intent = Intent::Cancel;
    assert_eq!(intent, Intent::Cancel);
}

#[test]
fn intent_write() {
    let intent = Intent::Write { path: None };
    assert!(matches!(intent, Intent::Write { .. }));
}

#[test]
fn intent_write_quit() {
    let intent = Intent::WriteQuit { path: Some("/tmp/test".to_string()) };
    assert!(matches!(intent, Intent::WriteQuit { .. }));
}

#[test]
fn intent_repeat_change() {
    let intent = Intent::RepeatChange;
    assert_eq!(intent, Intent::RepeatChange);
}

#[test]
fn intent_enter_command() {
    let intent = Intent::EnterCommand;
    assert_eq!(intent, Intent::EnterCommand);
}

#[test]
fn intent_enter_replace() {
    let intent = Intent::EnterReplace;
    assert_eq!(intent, Intent::EnterReplace);
}
