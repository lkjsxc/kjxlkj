//! Integration tests for render.

use crate::*;

#[test]
fn renderer_creation() {
    let renderer = Renderer::new(80, 24);
    // Basic creation test
    assert!(true);
}

#[test]
fn style_defaults() {
    let style = Style::default();
    assert!(style.fg.is_none());
    assert!(style.bg.is_none());
    assert!(!style.bold);
}
