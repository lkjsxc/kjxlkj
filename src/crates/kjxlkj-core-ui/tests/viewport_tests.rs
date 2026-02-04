//! Window and viewport integration tests.
//!
//! Tests for window identity, viewport, and option model as required by
//! /docs/todo/current/wave-implementation/editor/windows/README.md

use kjxlkj_core_ui::Viewport;

/// Test: Create a viewport.
#[test]
fn test_viewport_creation() {
    let vp = Viewport::new(0, 24, 0, 80);
    assert_eq!(vp.top_line, 0);
    assert_eq!(vp.height, 24);
    assert_eq!(vp.left_col, 0);
    assert_eq!(vp.width, 80);
}

/// Test: Viewport bottom line calculation.
#[test]
fn test_viewport_bottom_line() {
    let vp = Viewport::new(10, 20, 0, 80);
    assert_eq!(vp.bottom_line(), 30);
}

/// Test: Line visibility check.
#[test]
fn test_viewport_line_visibility() {
    let vp = Viewport::new(10, 20, 0, 80);
    
    // Visible lines
    assert!(vp.is_line_visible(10)); // Top line
    assert!(vp.is_line_visible(20)); // Middle
    assert!(vp.is_line_visible(29)); // Last visible line
    
    // Not visible lines
    assert!(!vp.is_line_visible(9));  // Just above
    assert!(!vp.is_line_visible(30)); // Just below
    assert!(!vp.is_line_visible(0));  // Far above
    assert!(!vp.is_line_visible(100)); // Far below
}

/// Test: Scroll to make line visible (line above viewport).
#[test]
fn test_viewport_scroll_up() {
    let mut vp = Viewport::new(50, 20, 0, 80);
    vp.scroll_to_line(25, 100);
    
    assert!(vp.is_line_visible(25));
    assert_eq!(vp.top_line, 25);
}

/// Test: Scroll to make line visible (line below viewport).
#[test]
fn test_viewport_scroll_down() {
    let mut vp = Viewport::new(0, 20, 0, 80);
    vp.scroll_to_line(30, 100);
    
    assert!(vp.is_line_visible(30));
}

/// Test: Scroll to line already visible (no change).
#[test]
fn test_viewport_scroll_noop() {
    let mut vp = Viewport::new(10, 20, 0, 80);
    let original_top = vp.top_line;
    
    vp.scroll_to_line(15, 100);
    
    assert_eq!(vp.top_line, original_top);
}

/// Test: Scroll respects buffer bounds.
#[test]
fn test_viewport_scroll_bounds() {
    let mut vp = Viewport::new(0, 20, 0, 80);
    
    // Try to scroll beyond buffer
    vp.scroll_to_line(95, 100);
    
    // Should clamp to keep viewport within buffer
    assert!(vp.top_line <= 80); // max_top = 100 - 20 = 80
}

/// Test: Scroll down by lines.
#[test]
fn test_viewport_scroll_down_lines() {
    let mut vp = Viewport::new(0, 20, 0, 80);
    
    vp.scroll_down(5, 100);
    assert_eq!(vp.top_line, 5);
    
    vp.scroll_down(10, 100);
    assert_eq!(vp.top_line, 15);
}

/// Test: Scroll up by lines.
#[test]
fn test_viewport_scroll_up_lines() {
    let mut vp = Viewport::new(20, 20, 0, 80);
    
    vp.scroll_up(5);
    assert_eq!(vp.top_line, 15);
    
    vp.scroll_up(10);
    assert_eq!(vp.top_line, 5);
}

/// Test: Scroll up doesn't go below zero.
#[test]
fn test_viewport_scroll_up_bounds() {
    let mut vp = Viewport::new(5, 20, 0, 80);
    vp.scroll_up(10);
    assert_eq!(vp.top_line, 0);
}

/// Test: Scroll down doesn't exceed buffer.
#[test]
fn test_viewport_scroll_down_bounds() {
    let mut vp = Viewport::new(70, 20, 0, 80);
    vp.scroll_down(20, 100);
    assert_eq!(vp.top_line, 80); // max_top = 100 - 20
}

/// Test: Center on line.
#[test]
fn test_viewport_center() {
    let mut vp = Viewport::new(0, 20, 0, 80);
    
    vp.center_on_line(50, 100);
    
    assert!(vp.is_line_visible(50));
    // Line should be near the middle of the viewport
    let middle_offset = 50_i32 - vp.top_line as i32;
    assert!(middle_offset >= 8 && middle_offset <= 12);
}

/// Test: Center near start of buffer.
#[test]
fn test_viewport_center_near_start() {
    let mut vp = Viewport::new(50, 20, 0, 80);
    
    vp.center_on_line(5, 100);
    
    assert!(vp.is_line_visible(5));
    assert_eq!(vp.top_line, 0); // Can't go above 0
}

/// Test: Center near end of buffer.
#[test]
fn test_viewport_center_near_end() {
    let mut vp = Viewport::new(0, 20, 0, 80);
    
    vp.center_on_line(95, 100);
    
    assert!(vp.is_line_visible(95));
    assert_eq!(vp.top_line, 80); // max_top = 100 - 20
}

/// Test: Cursor to top.
#[test]
fn test_viewport_cursor_to_top() {
    let mut vp = Viewport::new(0, 20, 0, 80);
    
    vp.cursor_to_top(30);
    
    assert_eq!(vp.top_line, 30);
    assert!(vp.is_line_visible(30));
}

/// Test: Cursor to bottom.
#[test]
fn test_viewport_cursor_to_bottom() {
    let mut vp = Viewport::new(0, 20, 0, 80);
    
    vp.cursor_to_bottom(30);
    
    assert_eq!(vp.top_line, 11); // 30 - (20 - 1) = 11
    assert!(vp.is_line_visible(30));
}

/// Test: Default viewport.
#[test]
fn test_viewport_default() {
    let vp = Viewport::default();
    assert_eq!(vp.top_line, 0);
    assert_eq!(vp.height, 0);
    assert_eq!(vp.left_col, 0);
    assert_eq!(vp.width, 0);
}

/// Test: Viewport determinism.
#[test]
fn test_viewport_determinism() {
    let mut vp1 = Viewport::new(0, 20, 0, 80);
    let mut vp2 = Viewport::new(0, 20, 0, 80);
    
    // Same operations should produce identical results
    vp1.scroll_to_line(50, 100);
    vp1.scroll_down(10, 100);
    vp1.scroll_up(5);
    
    vp2.scroll_to_line(50, 100);
    vp2.scroll_down(10, 100);
    vp2.scroll_up(5);
    
    assert_eq!(vp1.top_line, vp2.top_line);
}

/// Test: Viewport clone equality.
#[test]
fn test_viewport_clone() {
    let vp1 = Viewport::new(10, 20, 5, 80);
    let vp2 = vp1.clone();
    
    assert_eq!(vp1, vp2);
}

/// Test: Small viewport.
#[test]
fn test_viewport_small() {
    let mut vp = Viewport::new(0, 1, 0, 80);
    
    // With height 1, scrolling should work
    vp.scroll_to_line(50, 100);
    assert_eq!(vp.top_line, 50);
    assert!(vp.is_line_visible(50));
    assert!(!vp.is_line_visible(51));
}

/// Test: Large buffer with viewport.
#[test]
fn test_viewport_large_buffer() {
    let mut vp = Viewport::new(0, 20, 0, 80);
    let buffer_lines = 100_000;
    
    vp.scroll_to_line(50_000, buffer_lines);
    assert!(vp.is_line_visible(50_000));
    
    vp.center_on_line(75_000, buffer_lines);
    assert!(vp.is_line_visible(75_000));
}
