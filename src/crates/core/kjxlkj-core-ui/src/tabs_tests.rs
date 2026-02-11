//! Tests for tab page model.

use super::*;
use crate::layout::LayoutTree;
use kjxlkj_core_types::{ContentKind, BufferId, WindowId};

fn single_tab_list() -> TabList {
    let tree = LayoutTree::single(WindowId(1), ContentKind::Buffer(BufferId(0)));
    TabList::new(tree, WindowId(1))
}

fn new_tree(wid: u64) -> LayoutTree {
    LayoutTree::single(WindowId(wid), ContentKind::Buffer(BufferId(0)))
}

#[test]
fn initial_state() {
    let tl = single_tab_list();
    assert_eq!(tl.count(), 1);
    assert_eq!(tl.active_number(), 1);
}

#[test]
fn tab_new_inserts_after_current() {
    let mut tl = single_tab_list();
    tl.tab_new(new_tree(10), WindowId(10));
    assert_eq!(tl.count(), 2);
    assert_eq!(tl.active_number(), 2);
}

#[test]
fn tab_close_refuses_last() {
    let mut tl = single_tab_list();
    assert!(!tl.tab_close());
    assert_eq!(tl.count(), 1);
}

#[test]
fn tab_close_removes_current() {
    let mut tl = single_tab_list();
    tl.tab_new(new_tree(10), WindowId(10));
    assert!(tl.tab_close());
    assert_eq!(tl.count(), 1);
}

#[test]
fn tab_next_wraps() {
    let mut tl = single_tab_list();
    tl.tab_new(new_tree(10), WindowId(10));
    assert_eq!(tl.active, 1);
    tl.tab_next();
    assert_eq!(tl.active, 0);
    tl.tab_next();
    assert_eq!(tl.active, 1);
}

#[test]
fn tab_prev_wraps() {
    let mut tl = single_tab_list();
    tl.tab_new(new_tree(10), WindowId(10));
    tl.tab_prev();
    assert_eq!(tl.active, 0);
    tl.tab_prev();
    assert_eq!(tl.active, 1);
}

#[test]
fn tab_goto_one_indexed() {
    let mut tl = single_tab_list();
    tl.tab_new(new_tree(10), WindowId(10));
    tl.tab_new(new_tree(20), WindowId(20));
    assert!(tl.tab_goto(1));
    assert_eq!(tl.active, 0);
    assert!(!tl.tab_goto(0));
    assert!(!tl.tab_goto(4));
}

#[test]
fn tab_only_keeps_current() {
    let mut tl = single_tab_list();
    tl.tab_new(new_tree(10), WindowId(10));
    tl.tab_new(new_tree(20), WindowId(20));
    tl.tab_only();
    assert_eq!(tl.count(), 1);
}

#[test]
fn tab_move_clamps() {
    let mut tl = single_tab_list();
    tl.tab_new(new_tree(10), WindowId(10));
    tl.tab_new(new_tree(20), WindowId(20));
    // Active is 2 (index), move to 0
    tl.tab_move(0);
    assert_eq!(tl.active, 0);
}

#[test]
fn tab_move_relative() {
    let mut tl = single_tab_list();
    tl.tab_new(new_tree(10), WindowId(10));
    tl.tab_new(new_tree(20), WindowId(20));
    tl.tab_first();
    tl.tab_move_relative(2);
    assert_eq!(tl.active, 2);
    tl.tab_move_relative(-1);
    assert_eq!(tl.active, 1);
}

#[test]
fn tabline_visibility() {
    let tl = single_tab_list();
    assert!(!tl.should_show_tabline(false));
    assert!(tl.should_show_tabline(true));
}
