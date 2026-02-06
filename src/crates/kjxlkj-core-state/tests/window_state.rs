use kjxlkj_core_state::WindowState;
use kjxlkj_core_types::{BufferId, Position, WindowId};

#[test]
fn new_window_state() {
    let ws = WindowState::new(WindowId(1), BufferId(1));
    assert_eq!(ws.cursor(), Position::new(0, 0));
    assert_eq!(ws.top_line, 0);
    assert!(ws.wrap);
}

#[test]
fn set_cursor() {
    let mut ws = WindowState::new(WindowId(1), BufferId(1));
    ws.set_cursor(Position::new(5, 10));
    assert_eq!(ws.cursor_line, 5);
    assert_eq!(ws.cursor_col, 10);
}

#[test]
fn vertical_follow_scrolls_down() {
    let mut ws = WindowState::new(WindowId(1), BufferId(1));
    ws.height = 10;
    ws.scrolloff = 3;
    ws.cursor_line = 20;
    ws.ensure_cursor_visible();
    assert!(ws.cursor_line >= ws.top_line);
    assert!(ws.cursor_line < ws.top_line + ws.height);
}

#[test]
fn vertical_follow_scrolls_up() {
    let mut ws = WindowState::new(WindowId(1), BufferId(1));
    ws.height = 10;
    ws.scrolloff = 3;
    ws.top_line = 20;
    ws.cursor_line = 15;
    ws.ensure_cursor_visible();
    assert!(ws.top_line <= ws.cursor_line);
}

#[test]
fn scrolloff_clamped_to_half() {
    let mut ws = WindowState::new(WindowId(1), BufferId(1));
    ws.height = 5;
    ws.scrolloff = 10;
    ws.cursor_line = 0;
    ws.top_line = 5;
    ws.ensure_cursor_visible();
    assert!(ws.cursor_line >= ws.top_line);
}

#[test]
fn horizontal_follow_no_wrap() {
    let mut ws = WindowState::new(WindowId(1), BufferId(1));
    ws.wrap = false;
    ws.width = 20;
    ws.sidescrolloff = 5;
    ws.cursor_col = 50;
    ws.ensure_cursor_visible();
    assert!(ws.left_col > 0);
    assert!(ws.cursor_col >= ws.left_col);
    assert!(ws.cursor_col < ws.left_col + ws.width);
}

#[test]
fn horizontal_forced_zero_when_wrap() {
    let mut ws = WindowState::new(WindowId(1), BufferId(1));
    ws.wrap = true;
    ws.left_col = 10;
    ws.ensure_cursor_visible();
    assert_eq!(ws.left_col, 0);
}

#[test]
fn center_cursor_zz() {
    let mut ws = WindowState::new(WindowId(1), BufferId(1));
    ws.height = 20;
    ws.cursor_line = 50;
    ws.center_cursor();
    assert_eq!(ws.top_line, 40);
}

#[test]
fn cursor_to_top_zt() {
    let mut ws = WindowState::new(WindowId(1), BufferId(1));
    ws.cursor_line = 30;
    ws.cursor_to_top();
    assert_eq!(ws.top_line, 30);
}

#[test]
fn cursor_to_bottom_zb() {
    let mut ws = WindowState::new(WindowId(1), BufferId(1));
    ws.height = 20;
    ws.cursor_line = 50;
    ws.cursor_to_bottom();
    assert_eq!(ws.top_line, 31);
}
