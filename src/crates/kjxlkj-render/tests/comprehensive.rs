//! Comprehensive tests for the render pipeline.

use kjxlkj_render::*;
use kjxlkj_core_types::Size;
use kjxlkj_core_ui::UiModel;

// ──────────── Cell ────────────

#[test]
fn cell_default() {
    let c = Cell::default();
    assert_eq!(c.ch, ' ');
    assert_eq!(c.fg, Color::Default);
    assert_eq!(c.bg, Color::Default);
    assert!(!c.bold);
    assert!(!c.underline);
    assert!(!c.reverse);
}

#[test]
fn cell_with_char() {
    let c = Cell {
        ch: 'X',
        ..Cell::default()
    };
    assert_eq!(c.ch, 'X');
}

#[test]
fn cell_with_color() {
    let c = Cell {
        fg: Color::Rgb(255, 0, 0),
        bg: Color::Indexed(42),
        ..Cell::default()
    };
    assert_eq!(c.fg, Color::Rgb(255, 0, 0));
    assert_eq!(c.bg, Color::Indexed(42));
}

#[test]
fn cell_with_attributes() {
    let c = Cell {
        bold: true,
        underline: true,
        reverse: true,
        ..Cell::default()
    };
    assert!(c.bold);
    assert!(c.underline);
    assert!(c.reverse);
}

#[test]
fn cell_equality() {
    let a = Cell::default();
    let b = Cell::default();
    assert_eq!(a, b);
}

#[test]
fn cell_inequality() {
    let a = Cell::default();
    let mut b = Cell::default();
    b.ch = '!';
    assert_ne!(a, b);
}

#[test]
fn cell_clone() {
    let c = Cell {
        ch: 'A',
        fg: Color::Rgb(1, 2, 3),
        bg: Color::Default,
        bold: true,
        underline: false,
        reverse: false,
    };
    let c2 = c.clone();
    assert_eq!(c, c2);
}

// ──────────── Color ────────────

#[test]
fn color_default() {
    assert_eq!(Color::Default, Color::Default);
}

#[test]
fn color_rgb() {
    assert_eq!(Color::Rgb(255, 128, 0), Color::Rgb(255, 128, 0));
    assert_ne!(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 255));
}

#[test]
fn color_indexed() {
    assert_eq!(Color::Indexed(0), Color::Indexed(0));
    assert_ne!(Color::Indexed(0), Color::Indexed(255));
}

#[test]
fn color_different_kinds() {
    assert_ne!(Color::Default, Color::Rgb(0, 0, 0));
    assert_ne!(Color::Default, Color::Indexed(0));
    assert_ne!(Color::Rgb(0, 0, 0), Color::Indexed(0));
}

// ──────────── RenderFrame ────────────

#[test]
fn render_frame_new() {
    let f = RenderFrame::new(Size::new(80, 24));
    // Just verify it doesn't panic
    let _ = f;
}

#[test]
fn render_frame_resize() {
    let mut f = RenderFrame::new(Size::new(80, 24));
    f.resize(Size::new(120, 40));
    // No panic
}

#[test]
fn render_frame_swap() {
    let mut f = RenderFrame::new(Size::new(10, 5));
    f.swap();
    f.swap();
    // No panic on multiple swaps
}

// ──────────── Renderer ────────────

#[test]
fn renderer_new() {
    let _r = Renderer::new(Size::new(80, 24));
}

#[test]
fn renderer_render_empty_model() {
    let mut r = Renderer::new(Size::new(80, 24));
    let model = UiModel::empty(Size::new(80, 24));
    assert!(r.render(&model).is_ok());
}

#[test]
fn renderer_resize() {
    let mut r = Renderer::new(Size::new(80, 24));
    r.resize(Size::new(120, 40));
    let model = UiModel::empty(Size::new(120, 40));
    assert!(r.render(&model).is_ok());
}

#[test]
fn renderer_multiple_renders() {
    let mut r = Renderer::new(Size::new(80, 24));
    let model = UiModel::empty(Size::new(80, 24));
    for _ in 0..10 {
        assert!(r.render(&model).is_ok());
    }
}
