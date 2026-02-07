//! Tests for Rect, layout, and theme.
use kjxlkj_core_ui::layout::{standard_layout, ComponentKind, LayoutNode, Rect};
use kjxlkj_core_ui::theme::ThemeRegistry;
use kjxlkj_core_ui::theme_builtin::{theme_dark, theme_gruvbox, theme_light};

#[test]
fn rect_new_and_contains() {
    let r = Rect::new(10, 20, 30, 40);
    assert!(r.contains(10, 20));
    assert!(r.contains(39, 59));
    assert!(!r.contains(40, 20));
}
#[test]
fn rect_split_horizontal() {
    let r = Rect::new(0, 0, 80, 24);
    let (top, bot) = r.split_horizontal(12);
    assert_eq!(top.h, 12);
    assert_eq!(bot.h, 12);
    assert_eq!(bot.y, 12);
}
#[test]
fn rect_split_vertical() {
    let r = Rect::new(0, 0, 80, 24);
    let (left, right) = r.split_vertical(30);
    assert_eq!(left.w, 30);
    assert_eq!(right.w, 50);
    assert_eq!(right.x, 30);
}
#[test]
fn rect_overlaps() {
    let a = Rect::new(0, 0, 10, 10);
    let b = Rect::new(5, 5, 10, 10);
    let c = Rect::new(20, 20, 5, 5);
    assert!(a.overlaps(&b));
    assert!(!a.overlaps(&c));
}
#[test]
fn rect_area() {
    assert_eq!(Rect::new(0, 0, 10, 20).area(), 200);
}
#[test]
fn standard_layout_has_all_components() {
    let nodes = standard_layout(80, 24);
    assert!(nodes.iter().any(|n| n.kind == ComponentKind::TabLine));
    assert!(nodes.iter().any(|n| n.kind == ComponentKind::BufferView));
    assert!(nodes.iter().any(|n| n.kind == ComponentKind::StatusLine));
    assert!(nodes.iter().any(|n| n.kind == ComponentKind::CommandLine));
}
#[test]
fn standard_layout_viewport_heights() {
    let nodes = standard_layout(120, 40);
    let buf = nodes
        .iter()
        .find(|n| n.kind == ComponentKind::BufferView)
        .unwrap();
    assert_eq!(buf.rect.h, 37);
}
#[test]
fn layout_node_construction() {
    let node = LayoutNode {
        id: 42,
        kind: ComponentKind::BufferView,
        rect: Rect::new(0, 0, 80, 24),
        visible: true,
    };
    assert_eq!(node.id, 42);
    assert!(node.visible);
}
#[test]
fn theme_register_and_switch() {
    let mut reg = ThemeRegistry::new();
    reg.register(theme_dark());
    reg.register(theme_light());
    assert_eq!(reg.active, "dark");
    assert!(reg.set_active("light"));
    assert_eq!(reg.active_theme().unwrap().name, "light");
}
#[test]
fn theme_get_by_name() {
    let mut reg = ThemeRegistry::new();
    reg.register(theme_gruvbox());
    assert!(reg.get("gruvbox").is_some());
    assert!(reg.get("nonexistent").is_none());
}
#[test]
fn theme_set_missing_returns_false() {
    let mut reg = ThemeRegistry::new();
    assert!(!reg.set_active("missing"));
}
#[test]
fn theme_palette_colors() {
    let t = theme_dark();
    assert!(!t.palette.fg.is_empty());
    assert!(!t.palette.bg.is_empty());
}
