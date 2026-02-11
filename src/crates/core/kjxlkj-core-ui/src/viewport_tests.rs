use super::*;

#[test]
fn new_defaults() {
    let v = ViewportState::new(24, 80);
    assert_eq!(v.scrolloff, 5);
    assert_eq!(v.text_rows, 24);
    assert!(!v.wrap);
}

#[test]
fn ensure_visible_scrolls_down() {
    let mut v = ViewportState::new(10, 80);
    v.scrolloff = 2;
    v.ensure_visible(15, 0, 100);
    assert!(v.is_line_visible(15));
}

#[test]
fn ensure_visible_scrolls_up() {
    let mut v = ViewportState::new(10, 80);
    v.scrolloff = 2;
    v.top_line = 20;
    v.ensure_visible(5, 0, 100);
    assert!(v.is_line_visible(5));
    assert!(v.top_line <= 5);
}

#[test]
fn scroll_center_positions() {
    let mut v = ViewportState::new(20, 80);
    v.scroll_center(50, 100);
    assert_eq!(v.top_line, 40);
    assert!(v.is_line_visible(50));
}

#[test]
fn scroll_top_positions() {
    let mut v = ViewportState::new(20, 80);
    v.scroll_top(30, 100);
    assert_eq!(v.top_line, 30);
}

#[test]
fn scroll_bottom_positions() {
    let mut v = ViewportState::new(20, 80);
    v.scroll_bottom(30, 100);
    assert_eq!(v.top_line, 11);
}

#[test]
fn horizontal_scroll_nowrap() {
    let mut v = ViewportState::new(10, 20);
    v.sidescrolloff = 3;
    v.ensure_visible(0, 30, 10);
    assert!(v.left_col > 0);
}

#[test]
fn clamp_top_line() {
    let mut v = ViewportState::new(10, 80);
    v.scroll_top(100, 50);
    assert_eq!(v.top_line, 49);
}
