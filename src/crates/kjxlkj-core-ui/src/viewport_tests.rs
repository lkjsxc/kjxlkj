//! Tests for viewport tracking.

use crate::Viewport;

#[test]
fn viewport_visible_lines() {
    let vp = Viewport::new(80, 24);
    assert_eq!(vp.visible_lines(), 24);
}

#[test]
fn viewport_is_line_visible() {
    let vp = Viewport::new(80, 24);
    assert!(vp.is_line_visible(0));
    assert!(vp.is_line_visible(23));
    assert!(!vp.is_line_visible(24));
}

#[test]
fn viewport_ensure_visible() {
    let mut vp = Viewport::new(80, 24);
    vp.ensure_visible(30, 3);
    assert!(vp.is_line_visible(30));
}

#[test]
fn viewport_center() {
    let mut vp = Viewport::new(80, 24);
    vp.center_on_line(50, 100);
    assert!(vp.is_line_visible(50));
}

#[test]
fn viewport_last_line() {
    let vp = Viewport::new(80, 24);
    assert_eq!(vp.last_line(), 23);
}

#[test]
fn viewport_last_line_after_scroll() {
    let mut vp = Viewport::new(80, 24);
    vp.first_line = 10;
    assert_eq!(vp.last_line(), 33);
}

#[test]
fn viewport_ensure_visible_scrolls_down() {
    let mut vp = Viewport::new(80, 10);
    vp.ensure_visible(15, 2);
    assert!(vp.first_line > 0);
    assert!(vp.is_line_visible(15));
}

#[test]
fn viewport_ensure_visible_scrolls_up() {
    let mut vp = Viewport::new(80, 10);
    vp.first_line = 20;
    vp.ensure_visible(5, 2);
    assert!(vp.first_line <= 5);
    assert!(vp.is_line_visible(5));
}

#[test]
fn viewport_resize_preserves_first_line() {
    let mut vp = Viewport::new(80, 24);
    vp.first_line = 100;
    vp.resize(120, 40);
    assert_eq!(vp.first_line, 100);
    assert_eq!(vp.width, 120);
    assert_eq!(vp.height, 40);
}

#[test]
fn viewport_scroll_to_top() {
    let mut vp = Viewport::new(80, 24);
    vp.scroll_to_top(50);
    assert_eq!(vp.first_line, 50);
}

#[test]
fn viewport_scroll_to_bottom() {
    let mut vp = Viewport::new(80, 24);
    vp.scroll_to_bottom(50, 100);
    assert!(vp.is_line_visible(50));
    assert_eq!(vp.last_line(), 50);
}

#[test]
fn viewport_default() {
    let vp = Viewport::default();
    assert_eq!(vp.width, 80);
    assert_eq!(vp.height, 24);
}

#[test]
fn viewport_first_col_default() {
    let vp = Viewport::new(80, 24);
    assert_eq!(vp.first_col, 0);
}

#[test]
fn viewport_center_at_start() {
    let mut vp = Viewport::new(80, 24);
    vp.center_on_line(0, 100);
    assert_eq!(vp.first_line, 0);
}

#[test]
fn viewport_center_at_end() {
    let mut vp = Viewport::new(80, 24);
    vp.center_on_line(99, 100);
    assert!(vp.first_line <= 99 - vp.visible_lines() / 2 + 10);
}

#[test]
fn viewport_equality() {
    let vp1 = Viewport::new(80, 24);
    let vp2 = Viewport::new(80, 24);
    assert_eq!(vp1, vp2);
}

#[test]
fn viewport_clone() {
    let vp = Viewport::new(120, 40);
    let cloned = vp.clone();
    assert_eq!(vp, cloned);
}
