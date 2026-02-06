//! Viewport scroll and resize probe tests — cursor visibility invariants.

use kjxlkj_core_state::{EditorState, WindowState};
use kjxlkj_core_types::{Intent, MotionKind, ScrollKind, Size};

fn editor_with_lines(n: usize) -> EditorState {
    let text: String = (0..n).map(|i| format!("Line {i}\n")).collect();
    let mut st = EditorState::new(Size::new(80, 24));
    let bid = st.create_buffer_from_text(&text);
    st.create_window(bid); st
}
fn active_win(st: &EditorState) -> &WindowState { st.active_window_state().unwrap() }
fn cursor_in_viewport(st: &EditorState) -> bool {
    let w = active_win(st);
    w.cursor_line >= w.top_line && w.cursor_line < w.top_line + w.height
}
fn scroll_and_check(st: &mut EditorState, kind: ScrollKind) {
    kjxlkj_core_state::dispatch_intent(st, Intent::Scroll(kind));
    if let Some(w) = st.active_window_mut() { w.ensure_cursor_visible(); }
}
fn move_down(st: &mut EditorState, n: usize) {
    for _ in 0..n { kjxlkj_core_state::dispatch_intent(st, Intent::Motion(MotionKind::Down, 1)); }
}

#[test]
fn cursor_visible_after_page_down() {
    let mut st = editor_with_lines(200);
    for _ in 0..10 {
        scroll_and_check(&mut st, ScrollKind::FullPageDown);
        assert!(cursor_in_viewport(&st), "cursor not visible after page down");
    }
}

#[test]
fn cursor_visible_after_page_up() {
    let mut st = editor_with_lines(200);
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::Motion(MotionKind::FileEnd, 1));
    for _ in 0..10 {
        scroll_and_check(&mut st, ScrollKind::FullPageUp);
        assert!(cursor_in_viewport(&st), "cursor not visible after page up");
    }
}

#[test]
fn cursor_visible_after_half_page_scroll() {
    let mut st = editor_with_lines(200);
    for _ in 0..12 {
        scroll_and_check(&mut st, ScrollKind::HalfPageDown);
        assert!(cursor_in_viewport(&st));
    }
    for _ in 0..12 {
        scroll_and_check(&mut st, ScrollKind::HalfPageUp);
        assert!(cursor_in_viewport(&st));
    }
}

#[test]
fn scrolloff_respected_after_down_motions() {
    let mut st = editor_with_lines(200);
    st.options.scrolloff = 5;
    if let Some(w) = st.active_window_mut() { w.scrolloff = 5; }
    for _ in 0..50 {
        kjxlkj_core_state::dispatch_intent(&mut st, Intent::Motion(MotionKind::Down, 1));
        let w = active_win(&st);
        if w.cursor_line + w.scrolloff < 200 {
            assert!(w.cursor_line + w.scrolloff < w.top_line + w.height);
        }
    }
}

#[test]
fn resize_smaller_keeps_cursor_visible() {
    let mut st = editor_with_lines(100);
    move_down(&mut st, 50);
    if let Some(w) = st.active_window_mut() { w.height = 10; w.ensure_cursor_visible(); }
    assert!(cursor_in_viewport(&st));
}

#[test]
fn resize_larger_keeps_cursor_visible() {
    let mut st = editor_with_lines(100);
    move_down(&mut st, 30);
    if let Some(w) = st.active_window_mut() { w.height = 50; w.ensure_cursor_visible(); }
    assert!(cursor_in_viewport(&st));
}

#[test]
fn wrap_toggle_keeps_cursor_visible() {
    let long = "x".repeat(300) + "\n";
    let text: String = (0..20).map(|_| long.clone()).collect();
    let mut st = EditorState::new(Size::new(80, 24));
    let bid = st.create_buffer_from_text(&text);
    st.create_window(bid);
    if let Some(w) = st.active_window_mut() { w.cursor_col = 200; }
    if let Some(w) = st.active_window_mut() { w.wrap = false; w.ensure_cursor_visible(); }
    assert!(cursor_in_viewport(&st));
    if let Some(w) = st.active_window_mut() { w.wrap = true; w.ensure_cursor_visible(); }
    assert!(cursor_in_viewport(&st));
}

#[test]
fn center_cursor_zz() {
    let mut st = editor_with_lines(200);
    move_down(&mut st, 100);
    if let Some(w) = st.active_window_mut() { w.center_cursor(); }
    let w = active_win(&st);
    assert!((w.cursor_line as isize - (w.top_line + w.height / 2) as isize).unsigned_abs() <= 1);
}

#[test]
fn cursor_to_top_zt() {
    let mut st = editor_with_lines(200);
    move_down(&mut st, 50);
    if let Some(w) = st.active_window_mut() { w.cursor_to_top(); }
    let w = active_win(&st);
    assert_eq!(w.top_line, w.cursor_line);
}

#[test]
fn cursor_to_bottom_zb() {
    let mut st = editor_with_lines(200);
    move_down(&mut st, 50);
    if let Some(w) = st.active_window_mut() { w.cursor_to_bottom(); }
    let w = active_win(&st);
    assert_eq!(w.top_line + w.height.saturating_sub(1), w.cursor_line);
}

#[test]
fn horizontal_scroll_nowrap() {
    let line = "x".repeat(500) + "\n";
    let mut st = EditorState::new(Size::new(80, 24));
    let bid = st.create_buffer_from_text(&line);
    st.create_window(bid);
    if let Some(w) = st.active_window_mut() {
        w.wrap = false;
        w.cursor_col = 400;
        w.ensure_cursor_visible();
    }
    let w = active_win(&st);
    assert!(w.left_col > 0, "should have scrolled horizontally");
    assert!(w.cursor_col >= w.left_col && w.cursor_col < w.left_col + w.width);
}

#[test]
fn rapid_resize_storm() {
    let mut st = editor_with_lines(500);
    move_down(&mut st, 30);
    for &h in &[5, 100, 3, 80, 10, 24, 1, 50, 15, 200u16] {
        if let Some(w) = st.active_window_mut() { w.height = h as usize; w.ensure_cursor_visible(); }
        assert!(cursor_in_viewport(&st), "cursor not visible after resize to height={h}");
    }
}

#[test]
fn sidescrolloff_respected() {
    let line = "x".repeat(500) + "\n";
    let mut st = EditorState::new(Size::new(80, 24));
    let bid = st.create_buffer_from_text(&line);
    st.create_window(bid);
    if let Some(w) = st.active_window_mut() {
        w.wrap = false;
        w.sidescrolloff = 10;
        w.cursor_col = 200;
        w.ensure_cursor_visible();
    }
    let w = active_win(&st);
    let right_edge = w.left_col + w.width;
    assert!(w.cursor_col + w.sidescrolloff <= right_edge || w.cursor_col + w.sidescrolloff >= 500);
}
