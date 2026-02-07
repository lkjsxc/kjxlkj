//! Extended UI tests: view tree, search highlights, floating windows.
use kjxlkj_core_types::Position;
use kjxlkj_core_ui::floating::{
    compute_preset_rects, FloatAnchor, FloatBorder, FloatConfig, LayoutPreset, ZoomState,
};
use kjxlkj_core_ui::layout::Rect;
use kjxlkj_core_ui::search_highlight::SearchHighlights;
use kjxlkj_core_ui::view_tree::{FocusTarget, ViewNode, ViewTree};

#[test]
fn view_tree_focus_push_pop() {
    let root = ViewNode {
        id: 0,
        kind: "root".into(),
        rect: Rect::new(0, 0, 80, 24),
        focusable: false,
        children: vec![],
    };
    let mut tree = ViewTree::new(root);
    assert_eq!(tree.current_focus(), FocusTarget::Editor);
    tree.push_focus(FocusTarget::CommandLine);
    assert_eq!(tree.current_focus(), FocusTarget::CommandLine);
    tree.pop_focus();
    assert_eq!(tree.current_focus(), FocusTarget::Editor);
}
#[test]
fn view_tree_pop_at_bottom_stays() {
    let root = ViewNode {
        id: 0,
        kind: "root".into(),
        rect: Rect::new(0, 0, 80, 24),
        focusable: false,
        children: vec![],
    };
    let mut tree = ViewTree::new(root);
    tree.pop_focus();
    assert_eq!(tree.focus_stack.len(), 1);
}
#[test]
fn view_tree_from_splits() {
    let rects = vec![(1, Rect::new(0, 0, 40, 24)), (2, Rect::new(40, 0, 40, 24))];
    let tree = ViewTree::from_splits(rects);
    assert_eq!(tree.root.children.len(), 2);
    assert_eq!(tree.root.rect.w, 80);
}
#[test]
fn search_highlights_add_and_next() {
    let mut hl = SearchHighlights::new();
    hl.add_match(Position::new(0, 0), Position::new(0, 3));
    hl.add_match(Position::new(1, 0), Position::new(1, 3));
    hl.next_match();
    assert_eq!(hl.current_idx, Some(0));
    hl.next_match();
    assert_eq!(hl.current_idx, Some(1));
}
#[test]
fn search_highlights_prev_wraps() {
    let mut hl = SearchHighlights::new();
    hl.add_match(Position::new(0, 0), Position::new(0, 1));
    hl.add_match(Position::new(1, 0), Position::new(1, 1));
    hl.next_match(); // idx=0
    hl.prev_match(); // wraps to 1
    assert_eq!(hl.current_idx, Some(1));
}
#[test]
fn search_highlights_clear() {
    let mut hl = SearchHighlights::new();
    hl.add_match(Position::new(0, 0), Position::new(0, 1));
    hl.clear();
    assert!(hl.matches.is_empty());
}
#[test]
fn float_config_construction() {
    let fc = FloatConfig {
        anchor: FloatAnchor::Center,
        width: 40,
        height: 10,
        border: FloatBorder::Rounded,
        row_offset: 0,
        col_offset: 0,
        zindex: 50,
    };
    assert_eq!(fc.width, 40);
}
#[test]
fn zoom_state_toggle() {
    let normal = ZoomState::Normal;
    let zoomed = ZoomState::Zoomed {
        restore_w: 40,
        restore_h: 12,
    };
    assert_eq!(normal, ZoomState::Normal);
    assert_ne!(zoomed, ZoomState::Normal);
}
#[test]
fn layout_preset_single() {
    let rects = compute_preset_rects(LayoutPreset::Single, 80, 24, 1);
    assert_eq!(rects.len(), 1);
    assert_eq!(rects[0], Rect::new(0, 0, 80, 24));
}
#[test]
fn layout_preset_equal_v() {
    let rects = compute_preset_rects(LayoutPreset::EqualV, 80, 24, 2);
    assert_eq!(rects.len(), 2);
    assert_eq!(rects[0].w, 40);
}
#[test]
fn layout_preset_grid() {
    let rects = compute_preset_rects(LayoutPreset::Grid, 80, 24, 4);
    assert_eq!(rects.len(), 4);
}
#[test]
fn layout_preset_main_left() {
    let rects = compute_preset_rects(LayoutPreset::MainLeft, 80, 24, 3);
    assert_eq!(rects.len(), 3);
    assert_eq!(rects[0].w, 40);
}
