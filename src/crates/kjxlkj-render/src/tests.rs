//! Tests for render crate.

use crate::{Cell, Color, ScreenBuffer, Style};
use kjxlkj_core_ui::Dimensions;

#[test]
fn test_cell_new() {
    let style = Style::new().fg(Color::Red);
    let cell = Cell::new("x", style);
    assert_eq!(cell.content, "x");
    assert_eq!(cell.style.fg, Color::Red);
}

#[test]
fn test_cell_empty() {
    let cell = Cell::empty();
    assert_eq!(cell.content, " ");
}

#[test]
fn test_style_builder() {
    let style = Style::new()
        .fg(Color::Green)
        .bg(Color::Black)
        .bold()
        .reverse();
    assert_eq!(style.fg, Color::Green);
    assert_eq!(style.bg, Color::Black);
    assert!(style.bold);
    assert!(style.reverse);
}

#[test]
fn test_screen_buffer_new() {
    let dims = Dimensions::new(80, 24);
    let buf = ScreenBuffer::new(dims);
    assert_eq!(buf.dimensions().width, 80);
    assert_eq!(buf.dimensions().height, 24);
}

#[test]
fn test_screen_buffer_get_set() {
    let dims = Dimensions::new(10, 10);
    let mut buf = ScreenBuffer::new(dims);
    
    let cell = Cell::new("X", Style::default());
    buf.set(5, 5, cell);
    
    assert_eq!(buf.get(5, 5).unwrap().content, "X");
}

#[test]
fn test_screen_buffer_set_char() {
    let dims = Dimensions::new(10, 10);
    let mut buf = ScreenBuffer::new(dims);
    
    buf.set_char(0, 0, 'A', Style::default());
    assert_eq!(buf.get(0, 0).unwrap().content, "A");
}

#[test]
fn test_screen_buffer_set_string() {
    let dims = Dimensions::new(20, 10);
    let mut buf = ScreenBuffer::new(dims);
    
    buf.set_string(0, 0, "hello", Style::default());
    assert_eq!(buf.get(0, 0).unwrap().content, "h");
    assert_eq!(buf.get(1, 0).unwrap().content, "e");
    assert_eq!(buf.get(4, 0).unwrap().content, "o");
}

#[test]
fn test_screen_buffer_clear() {
    let dims = Dimensions::new(10, 10);
    let mut buf = ScreenBuffer::new(dims);
    
    buf.set_char(0, 0, 'X', Style::default());
    buf.clear();
    
    assert_eq!(buf.get(0, 0).unwrap().content, " ");
}

#[test]
fn test_screen_buffer_resize() {
    let dims = Dimensions::new(10, 10);
    let mut buf = ScreenBuffer::new(dims);
    
    buf.resize(Dimensions::new(20, 20));
    assert_eq!(buf.dimensions().width, 20);
    assert_eq!(buf.dimensions().height, 20);
}

#[test]
fn test_screen_buffer_bounds() {
    let dims = Dimensions::new(10, 10);
    let buf = ScreenBuffer::new(dims);
    
    assert!(buf.get(0, 0).is_some());
    assert!(buf.get(9, 9).is_some());
    assert!(buf.get(10, 10).is_none());
}

#[test]
fn test_color_rgb() {
    let style = Style::new().fg(Color::Rgb(255, 128, 64));
    if let Color::Rgb(r, g, b) = style.fg {
        assert_eq!(r, 255);
        assert_eq!(g, 128);
        assert_eq!(b, 64);
    } else {
        panic!("Expected RGB color");
    }
}
