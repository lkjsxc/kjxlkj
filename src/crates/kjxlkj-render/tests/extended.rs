//! Extended render tests.

use kjxlkj_render::*;
use kjxlkj_core_types::Size;
use kjxlkj_core_ui::UiModel;

// ──────────── Cell ────────────

#[test]
fn cell_default_space() {
    let c = Cell::default();
    assert_eq!(c.ch, ' ');
    assert!(!c.bold);
    assert!(!c.underline);
    assert!(!c.reverse);
}

#[test]
fn cell_custom() {
    let c = Cell {
        ch: 'A',
        fg: Color::Rgb(255, 0, 0),
        bg: Color::Indexed(8),
        bold: true,
        underline: true,
        reverse: false,
    };
    assert_eq!(c.ch, 'A');
    assert!(c.bold);
    assert!(c.underline);
}

#[test]
fn cell_eq() {
    let a = Cell::default();
    let b = Cell::default();
    assert_eq!(a, b);
}

#[test]
fn cell_ne() {
    let a = Cell::default();
    let mut b = Cell::default();
    b.ch = 'x';
    assert_ne!(a, b);
}

#[test]
fn cell_clone() {
    let c = Cell {
        ch: 'Z',
        fg: Color::Rgb(1, 2, 3),
        bg: Color::Default,
        bold: false,
        underline: false,
        reverse: true,
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
fn color_rgb_eq() {
    assert_eq!(Color::Rgb(10, 20, 30), Color::Rgb(10, 20, 30));
    assert_ne!(Color::Rgb(10, 20, 30), Color::Rgb(10, 20, 31));
}

#[test]
fn color_indexed() {
    assert_eq!(Color::Indexed(42), Color::Indexed(42));
    assert_ne!(Color::Indexed(1), Color::Indexed(2));
}

#[test]
fn color_copy() {
    let c = Color::Rgb(100, 200, 50);
    let c2 = c;
    assert_eq!(c, c2);
}

// ──────────── RenderFrame ────────────

#[test]
fn frame_new() {
    let f = RenderFrame::new(Size::new(80, 24));
    // Just verify it doesn't panic
    let _ = f;
}

#[test]
fn frame_resize() {
    let mut f = RenderFrame::new(Size::new(80, 24));
    f.resize(Size::new(120, 40));
}

#[test]
fn frame_swap() {
    let mut f = RenderFrame::new(Size::new(10, 5));
    f.swap();
    f.swap();
}

#[test]
fn frame_zero_size() {
    let f = RenderFrame::new(Size::new(0, 0));
    let _ = f;
}

// ──────────── Renderer ────────────

#[test]
fn renderer_new() {
    let r = Renderer::new(Size::new(80, 24));
    let _ = r;
}

#[test]
fn renderer_render_empty() {
    let mut r = Renderer::new(Size::new(80, 24));
    let model = UiModel::empty(Size::new(80, 24));
    assert!(r.render(&model).is_ok());
}

#[test]
fn renderer_resize() {
    let mut r = Renderer::new(Size::new(80, 24));
    r.resize(Size::new(120, 40));
}

#[test]
fn renderer_render_after_resize() {
    let mut r = Renderer::new(Size::new(80, 24));
    r.resize(Size::new(100, 30));
    let model = UiModel::empty(Size::new(100, 30));
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
