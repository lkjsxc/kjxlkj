//! Tests for zoom state.

use super::*;
use crate::layout::LayoutTree;
use kjxlkj_core_types::{BufferId, ContentKind, WindowId};

fn buf(id: u64) -> ContentKind { ContentKind::Buffer(BufferId(id)) }

#[test]
fn initial_not_zoomed() {
    let z = ZoomState::new();
    assert!(!z.is_zoomed());
    assert_eq!(z.indicator(), "");
}

#[test]
fn zoom_in_saves_layout() {
    let mut tree = LayoutTree::single(WindowId(0), buf(0));
    tree.split_horizontal(WindowId(0), WindowId(1), buf(1));
    let mut z = ZoomState::new();
    assert!(z.zoom_in(&mut tree, WindowId(0), buf(0)));
    assert!(z.is_zoomed());
    assert_eq!(z.indicator(), "[Z]");
    assert_eq!(tree.window_ids().len(), 1);
}

#[test]
fn zoom_restore() {
    let mut tree = LayoutTree::single(WindowId(0), buf(0));
    tree.split_horizontal(WindowId(0), WindowId(1), buf(1));
    let mut z = ZoomState::new();
    z.zoom_in(&mut tree, WindowId(0), buf(0));
    assert!(z.restore(&mut tree));
    assert!(!z.is_zoomed());
    assert_eq!(tree.window_ids().len(), 2);
}

#[test]
fn toggle_zoom() {
    let mut tree = LayoutTree::single(WindowId(0), buf(0));
    tree.split_horizontal(WindowId(0), WindowId(1), buf(1));
    let mut z = ZoomState::new();
    // Toggle on
    z.toggle(&mut tree, WindowId(0), buf(0));
    assert!(z.is_zoomed());
    assert_eq!(tree.window_ids().len(), 1);
    // Toggle off
    z.toggle(&mut tree, WindowId(0), buf(0));
    assert!(!z.is_zoomed());
    assert_eq!(tree.window_ids().len(), 2);
}

#[test]
fn on_window_closed_removes_from_saved() {
    let mut tree = LayoutTree::single(WindowId(0), buf(0));
    tree.split_horizontal(WindowId(0), WindowId(1), buf(1));
    let mut z = ZoomState::new();
    z.zoom_in(&mut tree, WindowId(0), buf(0));
    z.on_window_closed(WindowId(1));
    z.restore(&mut tree);
    // Window 1 was removed from saved layout
    assert_eq!(tree.window_ids().len(), 1);
    assert_eq!(tree.window_ids()[0], WindowId(0));
}

#[test]
fn double_zoom_in_rejected() {
    let mut tree = LayoutTree::single(WindowId(0), buf(0));
    let mut z = ZoomState::new();
    assert!(z.zoom_in(&mut tree, WindowId(0), buf(0)));
    assert!(!z.zoom_in(&mut tree, WindowId(0), buf(0)));
}

#[test]
fn restore_without_zoom_returns_false() {
    let mut tree = LayoutTree::single(WindowId(0), buf(0));
    let mut z = ZoomState::new();
    assert!(!z.restore(&mut tree));
}
