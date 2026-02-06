//! Tests for viewport scrolloff, cursor-follow, and zz/zt/zb.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{Intent, MotionKind, ScrollKind, Size};

fn setup(lines: usize) -> EditorState {
    let text = (0..lines)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(&text);
    s.create_window(bid);
    s
}

fn dispatch_ex(state: &mut EditorState, cmd: &str) {
    dispatch_intent(state, Intent::ExCommand(cmd.into()));
}

// ── Vertical cursor-follow ─────────────────

#[test]
fn scrolloff_keeps_margin_when_moving_down() {
    let mut s = setup(100);
    // Move cursor down 30 lines
    for _ in 0..30 {
        dispatch_intent(
            &mut s,
            Intent::Motion(MotionKind::Down, 1),
        );
    }
    let win = s.active_window_state().unwrap();
    assert_eq!(win.cursor_line, 30);
    // cursor_line - top_line should leave scrolloff margin
    let bottom_room = win.top_line + win.height - 1 - win.cursor_line;
    assert!(
        bottom_room >= win.scrolloff.min((win.height - 1) / 2),
        "bottom_room={}, scrolloff={}",
        bottom_room, win.scrolloff
    );
}

#[test]
fn scrolloff_keeps_margin_when_moving_up() {
    let mut s = setup(100);
    // Move down then back up
    for _ in 0..30 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    }
    for _ in 0..10 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Up, 1));
    }
    let win = s.active_window_state().unwrap();
    assert_eq!(win.cursor_line, 20);
    let top_room = win.cursor_line - win.top_line;
    let v_margin = win.scrolloff.min((win.height - 1) / 2);
    assert!(
        top_room >= v_margin,
        "top_room={}, v_margin={}", top_room, v_margin
    );
}

// ── zz/zt/zb ────────────────────────────────

#[test]
fn zz_centers_cursor() {
    let mut s = setup(100);
    for _ in 0..50 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    }
    dispatch_intent(
        &mut s,
        Intent::Scroll(ScrollKind::CursorCenter),
    );
    let win = s.active_window_state().unwrap();
    let center_pos = win.cursor_line - win.top_line;
    assert_eq!(center_pos, win.height / 2);
}

#[test]
fn zt_puts_cursor_at_top() {
    let mut s = setup(100);
    for _ in 0..50 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    }
    dispatch_intent(
        &mut s,
        Intent::Scroll(ScrollKind::CursorTop),
    );
    let win = s.active_window_state().unwrap();
    assert_eq!(win.top_line, win.cursor_line);
}

#[test]
fn zb_puts_cursor_at_bottom() {
    let mut s = setup(100);
    for _ in 0..50 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    }
    dispatch_intent(
        &mut s,
        Intent::Scroll(ScrollKind::CursorBottom),
    );
    let win = s.active_window_state().unwrap();
    let offset = win.cursor_line - win.top_line;
    assert_eq!(offset, win.height - 1);
}

// ── :set scrolloff ──────────────────────────

#[test]
fn set_scrolloff_changes_option() {
    let mut s = setup(10);
    dispatch_ex(&mut s, ":set scrolloff=5");
    assert_eq!(s.options.scrolloff, 5);
}

#[test]
fn set_sidescrolloff_changes_option() {
    let mut s = setup(10);
    dispatch_ex(&mut s, ":set sidescrolloff=10");
    assert_eq!(s.options.sidescrolloff, 10);
}

// ── Wrap interaction ────────────────────────

#[test]
fn wrap_on_forces_left_col_zero() {
    let mut s = setup(10);
    // Ensure wrap=true → left_col always 0
    let win = s.active_window_state().unwrap();
    assert!(win.wrap);
    assert_eq!(win.left_col, 0);
}

#[test]
fn set_nowrap() {
    let mut s = setup(10);
    dispatch_ex(&mut s, ":set nowrap");
    assert!(!s.options.wrap);
}

// ── Page scroll ─────────────────────────────

#[test]
fn half_page_down_moves_cursor() {
    let mut s = setup(100);
    dispatch_intent(
        &mut s,
        Intent::Scroll(ScrollKind::HalfPageDown),
    );
    let win = s.active_window_state().unwrap();
    assert!(win.cursor_line > 0);
}

#[test]
fn full_page_down_up_returns() {
    let mut s = setup(100);
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::FullPageDown));
    let line_after_down = s.cursor().line;
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::FullPageUp));
    let line_after_up = s.cursor().line;
    assert!(line_after_up < line_after_down);
}

// ── Window dimensions ───────────────────────

#[test]
fn window_inherits_editor_size() {
    let mut s = EditorState::new(Size::new(120, 40));
    let bid = s.create_buffer_from_text("test");
    s.create_window(bid);
    let win = s.active_window_state().unwrap();
    assert_eq!(win.width, 120);
    assert_eq!(win.height, 38); // 40 - 2 for statusline+cmdline
}

#[test]
fn window_inherits_scrolloff() {
    let mut s = EditorState::new(Size::new(80, 24));
    s.options.scrolloff = 7;
    let bid = s.create_buffer_from_text("test");
    s.create_window(bid);
    let win = s.active_window_state().unwrap();
    assert_eq!(win.scrolloff, 7);
}
