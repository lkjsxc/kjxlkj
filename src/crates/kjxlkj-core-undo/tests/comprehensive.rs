//! Comprehensive tests for the undo/redo tree.

use kjxlkj_core_undo::*;
use std::time::Instant;

fn entry(fwd: &str, rev: &str) -> UndoEntry {
    UndoEntry {
        forward: fwd.as_bytes().to_vec(),
        reverse: rev.as_bytes().to_vec(),
        timestamp: Instant::now(),
    }
}

// ──────────── Empty tree ────────────

#[test]
fn new_tree_is_empty() {
    let tree = UndoTree::new();
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
}

#[test]
fn default_tree_is_empty() {
    let tree = UndoTree::default();
    assert!(tree.is_empty());
}

#[test]
fn empty_tree_cannot_undo() {
    let mut tree = UndoTree::new();
    assert!(!tree.can_undo());
    assert!(tree.undo().is_none());
}

#[test]
fn empty_tree_cannot_redo() {
    let mut tree = UndoTree::new();
    assert!(!tree.can_redo());
    assert!(tree.redo().is_none());
}

// ──────────── Single push ────────────

#[test]
fn push_one_entry() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    assert_eq!(tree.len(), 1);
    assert!(!tree.is_empty());
}

#[test]
fn push_one_can_undo() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    assert!(tree.can_undo());
}

#[test]
fn push_one_cannot_redo() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    assert!(!tree.can_redo());
}

// ──────────── Undo basics ────────────

#[test]
fn undo_returns_entry() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    let e = tree.undo().unwrap();
    assert_eq!(e.forward, b"a");
    assert_eq!(e.reverse, b"A");
}

#[test]
fn after_undo_cannot_undo() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.undo();
    assert!(!tree.can_undo());
}

#[test]
fn after_undo_can_redo() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.undo();
    assert!(tree.can_redo());
}

// ──────────── Redo basics ────────────

#[test]
fn redo_returns_entry() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.undo();
    let e = tree.redo().unwrap();
    assert_eq!(e.forward, b"a");
}

#[test]
fn redo_then_no_more_redo() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.undo();
    tree.redo();
    assert!(!tree.can_redo());
}

#[test]
fn redo_then_can_undo_again() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.undo();
    tree.redo();
    assert!(tree.can_undo());
}

// ──────────── Multi-entry ────────────

#[test]
fn push_three_entries() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.push(entry("b", "B"));
    tree.push(entry("c", "C"));
    assert_eq!(tree.len(), 3);
}

#[test]
fn undo_three_in_order() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.push(entry("b", "B"));
    tree.push(entry("c", "C"));
    assert_eq!(tree.undo().unwrap().forward, b"c");
    assert_eq!(tree.undo().unwrap().forward, b"b");
    assert_eq!(tree.undo().unwrap().forward, b"a");
    assert!(tree.undo().is_none());
}

#[test]
fn redo_three_in_order() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.push(entry("b", "B"));
    tree.push(entry("c", "C"));
    tree.undo();
    tree.undo();
    tree.undo();
    assert_eq!(tree.redo().unwrap().forward, b"a");
    assert_eq!(tree.redo().unwrap().forward, b"b");
    assert_eq!(tree.redo().unwrap().forward, b"c");
    assert!(tree.redo().is_none());
}

// ──────────── Truncation (push after undo) ────────────

#[test]
fn push_after_undo_truncates_redo() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.push(entry("b", "B"));
    tree.push(entry("c", "C"));
    tree.undo(); // c undone
    tree.undo(); // b undone
    tree.push(entry("d", "D"));
    assert_eq!(tree.len(), 2); // a, d
    assert!(!tree.can_redo());
}

#[test]
fn push_after_undo_correct_content() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.push(entry("b", "B"));
    tree.undo();
    tree.push(entry("c", "C"));
    // Undo should give 'c', then 'a'
    assert_eq!(tree.undo().unwrap().forward, b"c");
    assert_eq!(tree.undo().unwrap().forward, b"a");
    assert!(tree.undo().is_none());
}

// ──────────── Interleaved undo/redo ────────────

#[test]
fn interleaved_undo_redo() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.push(entry("b", "B"));
    tree.undo();
    tree.redo();
    tree.undo();
    assert_eq!(tree.undo().unwrap().forward, b"a");
}

#[test]
fn undo_all_then_redo_all() {
    let mut tree = UndoTree::new();
    for i in 0..5 {
        tree.push(entry(&format!("{}", i), &format!("R{}", i)));
    }
    for _ in 0..5 { tree.undo(); }
    assert!(!tree.can_undo());
    for _ in 0..5 { tree.redo(); }
    assert!(!tree.can_redo());
    assert!(tree.can_undo());
}

// ──────────── Timestamps ────────────

#[test]
fn entries_have_timestamps() {
    let before = Instant::now();
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    let e = tree.undo().unwrap();
    assert!(e.timestamp >= before);
}

// ──────────── UndoNodeId ────────────

#[test]
fn undo_node_id_eq() {
    let a = UndoNodeId(1);
    let b = UndoNodeId(1);
    let c = UndoNodeId(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn undo_node_id_copy() {
    let a = UndoNodeId(42);
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn undo_node_id_debug() {
    let a = UndoNodeId(7);
    assert!(format!("{:?}", a).contains("7"));
}

// ──────────── UndoEntry fields ────────────

#[test]
fn entry_stores_forward_reverse() {
    let e = entry("insert_x", "delete_x");
    assert_eq!(e.forward, b"insert_x");
    assert_eq!(e.reverse, b"delete_x");
}

#[test]
fn entry_clone() {
    let e = entry("a", "b");
    let e2 = e.clone();
    assert_eq!(e.forward, e2.forward);
    assert_eq!(e.reverse, e2.reverse);
}

// ──────────── Edge cases ────────────

#[test]
fn push_empty_data() {
    let mut tree = UndoTree::new();
    tree.push(entry("", ""));
    assert_eq!(tree.len(), 1);
    let e = tree.undo().unwrap();
    assert!(e.forward.is_empty());
}

#[test]
fn many_entries() {
    let mut tree = UndoTree::new();
    for i in 0..100 {
        tree.push(entry(&format!("{}", i), "r"));
    }
    assert_eq!(tree.len(), 100);
    for _ in 0..100 {
        assert!(tree.undo().is_some());
    }
    assert!(tree.undo().is_none());
}

#[test]
fn push_after_full_undo() {
    let mut tree = UndoTree::new();
    tree.push(entry("a", "A"));
    tree.push(entry("b", "B"));
    tree.undo();
    tree.undo();
    tree.push(entry("x", "X"));
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.undo().unwrap().forward, b"x");
}
