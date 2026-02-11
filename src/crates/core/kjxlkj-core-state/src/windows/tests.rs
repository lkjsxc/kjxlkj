use super::{Axis, Direction, Rect, WindowKind, WindowTree};
use std::collections::HashSet;

fn area() -> Rect {
    Rect {
        row: 0,
        col: 0,
        rows: 20,
        cols: 80,
    }
}

#[test]
fn split_close_only_keeps_single_valid_focus() {
    let mut tree = WindowTree::new();
    tree.split_focused(Axis::Horizontal, WindowKind::Explorer);
    tree.split_focused(Axis::Vertical, WindowKind::Terminal);
    assert!(tree.geometry_invariants_hold(area()));
    assert!(tree.close_focused());
    assert!(tree.geometry_invariants_hold(area()));
    tree.only();
    assert!(tree.geometry_invariants_hold(area()));
    assert_eq!(tree.focused(), 2);
}

#[test]
fn directional_focus_is_deterministic_for_asymmetric_layout() {
    let mut tree = WindowTree::new();
    tree.split_focused(Axis::Vertical, WindowKind::Explorer);
    tree.split_focused(Axis::Horizontal, WindowKind::Terminal);
    tree.focus_direction(Direction::Left, area());
    let left_focus = tree.focused();
    tree.focus_direction(Direction::Down, area());
    let down_focus = tree.focused();
    assert_eq!(left_focus, 1);
    assert_eq!(down_focus, 1);
}

#[test]
fn cyclic_and_previous_focus_are_stable() {
    let mut t1 = WindowTree::new();
    t1.split_focused(Axis::Vertical, WindowKind::Explorer);
    t1.split_focused(Axis::Vertical, WindowKind::Terminal);
    let mut t2 = t1.clone();

    let mut trace1 = Vec::new();
    let mut trace2 = Vec::new();
    for _ in 0..2 {
        t1.cycle_next();
        trace1.push(t1.focused());
        t1.cycle_prev();
        trace1.push(t1.focused());
        t1.focus_previous();
        trace1.push(t1.focused());

        t2.cycle_next();
        trace2.push(t2.focused());
        t2.cycle_prev();
        trace2.push(t2.focused());
        t2.focus_previous();
        trace2.push(t2.focused());
    }
    assert_eq!(trace1, trace2);
}

#[test]
fn top_and_bottom_targets_are_deterministic() {
    let mut tree = WindowTree::new();
    tree.split_focused(Axis::Horizontal, WindowKind::Explorer);
    tree.split_focused(Axis::Vertical, WindowKind::Terminal);
    tree.focus_top_left(area());
    let top = tree.focused();
    tree.focus_bottom_right(area());
    let bottom = tree.focused();
    assert_eq!(top, 1);
    assert_eq!(bottom, 3);
}

#[test]
fn session_dump_roundtrip_restores_tree_focus_and_kinds() {
    let mut original = WindowTree::new();
    original.split_focused(Axis::Horizontal, WindowKind::Explorer);
    original.split_focused(Axis::Vertical, WindowKind::Terminal);
    original.focus_top_left(area());
    let snapshot = original.session_dump();

    let mut restored = WindowTree::new();
    restored
        .restore_session(&snapshot)
        .expect("session dump should restore");
    assert_eq!(restored.focused(), original.focused());
    assert_eq!(restored.focused_kind(), original.focused_kind());
    assert_eq!(restored.session_dump(), snapshot);
    assert!(restored.geometry_invariants_hold(area()));
}

#[test]
fn focus_history_pointer_stays_valid_through_split_close_churn() {
    let mut tree = WindowTree::new();
    for step in 0..12 {
        let axis = if step % 2 == 0 {
            Axis::Horizontal
        } else {
            Axis::Vertical
        };
        let kind = if step % 3 == 0 {
            WindowKind::Explorer
        } else {
            WindowKind::Terminal
        };
        tree.split_focused(axis, kind);
        tree.cycle_prev();
        tree.focus_previous();
        let _ = tree.close_focused();
        assert!(tree.leaf_ids().contains(&tree.focused()));
        if let Some(prev) = tree.previous {
            assert!(tree.leaf_ids().contains(&prev));
        }
        assert!(tree.geometry_invariants_hold(area()));
    }
}

#[test]
fn nested_rebalance_keeps_unique_leaf_ids_and_full_coverage() {
    let mut tree = WindowTree::new();
    tree.split_focused(Axis::Horizontal, WindowKind::Explorer);
    tree.split_focused(Axis::Vertical, WindowKind::Terminal);
    tree.split_focused(Axis::Horizontal, WindowKind::Buffer);
    tree.split_focused(Axis::Vertical, WindowKind::Explorer);
    for _ in 0..3 {
        assert!(tree.close_focused());
        tree.cycle_next();
        let ids = tree.leaf_ids();
        let uniq: HashSet<u64> = ids.iter().copied().collect();
        assert_eq!(uniq.len(), ids.len());
        assert!(ids.contains(&tree.focused()));
        assert!(tree.geometry_invariants_hold(area()));
    }
}
