//! Tests for mode state machine, cursor rendering, popups, and transitions.
use kjxlkj_core_mode::{
    blink_state, can_transition, cursor_for_mode, cursor_shape_escape, cursor_visible,
    validate_transition, BlinkState, CursorShape, ModeState, OverlayManager, PopupKind,
    PopupState, TransitionError,
};
use kjxlkj_core_types::Mode;

#[test]
fn mode_state_starts_normal() {
    let ms = ModeState::new();
    assert_eq!(ms.current(), Mode::Normal);
}
#[test]
fn mode_state_is_insert_after_transition() {
    let mut ms = ModeState::new();
    ms.transition(Mode::Insert);
    assert_eq!(ms.current(), Mode::Insert);
    assert_eq!(ms.previous(), Mode::Normal);
}
#[test]
fn mode_state_invalid_transition_stays() {
    let mut ms = ModeState::new();
    ms.transition(Mode::Insert);
    ms.transition(Mode::Visual); // invalid from insert
    assert_eq!(ms.current(), Mode::Insert);
}
#[test]
fn mode_state_visual_modes() {
    let mut ms = ModeState::new();
    ms.transition(Mode::Visual);
    assert_eq!(ms.current(), Mode::Visual);
    ms.transition(Mode::VisualLine);
    assert_eq!(ms.current(), Mode::VisualLine);
    ms.transition(Mode::VisualBlock);
    assert_eq!(ms.current(), Mode::VisualBlock);
}
#[test]
fn mode_state_command_mode() {
    let mut ms = ModeState::new();
    ms.transition(Mode::Command);
    assert_eq!(ms.current(), Mode::Command);
    ms.transition(Mode::Normal);
    assert_eq!(ms.current(), Mode::Normal);
}
#[test]
fn can_transition_normal_to_all() {
    assert!(can_transition(Mode::Normal, Mode::Insert));
    assert!(can_transition(Mode::Normal, Mode::Visual));
    assert!(can_transition(Mode::Normal, Mode::Command));
    assert!(can_transition(Mode::Normal, Mode::Replace));
    assert!(can_transition(Mode::Normal, Mode::Terminal));
}
#[test]
fn all_modes_escape_to_normal() {
    for mode in [Mode::Insert, Mode::Visual, Mode::VisualLine, Mode::VisualBlock,
                 Mode::Command, Mode::Replace, Mode::OperatorPending, Mode::Terminal] {
        assert!(can_transition(mode, Mode::Normal), "{mode} -> Normal");
    }
}
#[test]
fn validate_transition_error() {
    let err = validate_transition(Mode::Insert, Mode::Replace);
    assert!(matches!(err, Err(TransitionError::InvalidTransition { .. })));
}
#[test]
fn validate_transition_ok() {
    assert!(validate_transition(Mode::Normal, Mode::Insert).is_ok());
}
#[test]
fn cursor_normal_block() {
    let cfg = cursor_for_mode("Normal");
    assert_eq!(cfg.shape, CursorShape::Block);
    assert!(!cfg.blink_enabled);
}
#[test]
fn cursor_insert_line() {
    let cfg = cursor_for_mode("Insert");
    assert_eq!(cfg.shape, CursorShape::Line);
    assert!(cfg.blink_enabled);
}
#[test]
fn cursor_replace_underline() {
    let cfg = cursor_for_mode("Replace");
    assert_eq!(cfg.shape, CursorShape::Underline);
}
#[test]
fn cursor_escape_block_steady() {
    assert_eq!(cursor_shape_escape(CursorShape::Block, false), "\x1b[2 q");
}
#[test]
fn cursor_escape_line_blink() {
    assert_eq!(cursor_shape_escape(CursorShape::Line, true), "\x1b[5 q");
}
#[test]
fn cursor_escape_hidden() {
    assert_eq!(cursor_shape_escape(CursorShape::Hidden, true), "\x1b[?25l");
}
#[test]
fn blink_state_on_off_cycle() {
    assert_eq!(blink_state(0, 500), BlinkState::On);
    assert_eq!(blink_state(500, 500), BlinkState::Off);
    assert_eq!(blink_state(1000, 500), BlinkState::On);
}
#[test]
fn blink_zero_interval_always_on() {
    assert_eq!(blink_state(999, 0), BlinkState::On);
}
#[test]
fn cursor_visible_hidden() {
    assert!(!cursor_visible(CursorShape::Hidden, BlinkState::On));
}
#[test]
fn cursor_visible_on_off() {
    assert!(cursor_visible(CursorShape::Block, BlinkState::On));
    assert!(!cursor_visible(CursorShape::Block, BlinkState::Off));
}
#[test]
fn popup_show_hide() {
    let mut p = popup();
    p.hide();
    assert!(!p.visible);
    p.show(vec!["x".into()]);
    assert!(p.visible);
    assert_eq!(p.items.len(), 1);
}
#[test]
fn popup_select_next_scrolls() {
    let mut p = popup();
    p.select_next();
    p.select_next();
    p.select_next();
    assert_eq!(p.selected, 3);
    assert!(p.scroll_offset > 0);
}
#[test]
fn popup_select_prev() {
    let mut p = popup();
    p.selected = 2;
    p.select_prev();
    assert_eq!(p.selected, 1);
}
#[test]
fn popup_visible_items() {
    let p = popup();
    assert_eq!(p.visible_items().len(), 3);
}
#[test]
fn overlay_open_close() {
    let mut mgr = OverlayManager::default();
    mgr.open(popup());
    assert_eq!(mgr.top().unwrap().kind, PopupKind::Completion);
    mgr.close_kind(PopupKind::Completion);
    assert!(mgr.top().is_none());
}
#[test]
fn overlay_stack_behavior() {
    let mut mgr = OverlayManager::default();
    mgr.open(popup());
    let mut h = popup();
    h.kind = PopupKind::Hover;
    mgr.open(h);
    assert_eq!(mgr.top().unwrap().kind, PopupKind::Hover);
    mgr.close_all();
    assert!(mgr.top().is_none());
}

fn popup() -> PopupState {
    PopupState {
        kind: PopupKind::Completion,
        items: vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()],
        selected: 0,
        visible: true,
        max_visible: 3,
        scroll_offset: 0,
    }
}
