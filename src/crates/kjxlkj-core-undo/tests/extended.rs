//! Extended undo/redo tree tests.

use kjxlkj_core_undo::*;
use std::time::Instant;

fn entry(fwd: &[u8], rev: &[u8]) -> UndoEntry {
    UndoEntry {
        forward: fwd.to_vec(),
        reverse: rev.to_vec(),
        timestamp: Instant::now(),
    }
}

// ──────────── Construction ────────────

#[test]
fn new_tree_empty() {
    let t = UndoTree::new();
    assert!(t.is_empty());
    assert_eq!(t.len(), 0);
}

#[test]
fn default_tree_empty() {
    let t = UndoTree::default();
    assert!(t.is_empty());
}

#[test]
fn new_tree_cannot_undo() {
    let t = UndoTree::new();
    assert!(!t.can_undo());
}

#[test]
fn new_tree_cannot_redo() {
    let t = UndoTree::new();
    assert!(!t.can_redo());
}

// ──────────── Push ────────────

#[test]
fn push_one() {
    let mut t = UndoTree::new();
    t.push(entry(b"a", b"A"));
    assert_eq!(t.len(), 1);
    assert!(!t.is_empty());
}

#[test]
fn push_multiple() {
    let mut t = UndoTree::new();
    for i in 0..10 {
        t.push(entry(&[i], &[i + 100]));
    }
    assert_eq!(t.len(), 10);
}

#[test]
fn push_can_undo() {
    let mut t = UndoTree::new();
    t.push(entry(b"x", b"X"));
    assert!(t.can_undo());
}

#[test]
fn push_cannot_redo() {
    let mut t = UndoTree::new();
    t.push(entry(b"x", b"X"));
    assert!(!t.can_redo());
}

// ──────────── Undo ────────────

#[test]
fn undo_returns_entry() {
    let mut t = UndoTree::new();
    t.push(entry(b"hello", b"HELLO"));
    let e = t.undo().unwrap();
    assert_eq!(e.forward, b"hello");
    assert_eq!(e.reverse, b"HELLO");
}

#[test]
fn undo_makes_redo_available() {
    let mut t = UndoTree::new();
    t.push(entry(b"x", b"X"));
    t.undo();
    assert!(t.can_redo());
}

#[test]
fn undo_on_empty_returns_none() {
    let mut t = UndoTree::new();
    assert!(t.undo().is_none());
}

#[test]
fn double_undo_on_single_entry() {
    let mut t = UndoTree::new();
    t.push(entry(b"x", b"X"));
    assert!(t.undo().is_some());
    assert!(t.undo().is_none());
}

#[test]
fn undo_all_entries() {
    let mut t = UndoTree::new();
    for i in 0..5 {
        t.push(entry(&[i], &[i]));
    }
    for _ in 0..5 {
        assert!(t.undo().is_some());
    }
    assert!(t.undo().is_none());
}

// ──────────── Redo ────────────

#[test]
fn redo_returns_entry() {
    let mut t = UndoTree::new();
    t.push(entry(b"abc", b"ABC"));
    t.undo();
    let e = t.redo().unwrap();
    assert_eq!(e.forward, b"abc");
}

#[test]
fn redo_on_empty_returns_none() {
    let mut t = UndoTree::new();
    assert!(t.redo().is_none());
}

#[test]
fn redo_without_undo_returns_none() {
    let mut t = UndoTree::new();
    t.push(entry(b"x", b"X"));
    assert!(t.redo().is_none());
}

#[test]
fn undo_redo_roundtrip() {
    let mut t = UndoTree::new();
    t.push(entry(b"test", b"TEST"));
    t.undo();
    t.redo();
    assert!(t.can_undo());
    assert!(!t.can_redo());
}

// ──────────── Truncation ────────────

#[test]
fn push_after_undo_truncates() {
    let mut t = UndoTree::new();
    t.push(entry(b"a", b"A"));
    t.push(entry(b"b", b"B"));
    t.undo(); // current = 1
    t.push(entry(b"c", b"C"));
    assert_eq!(t.len(), 2); // b was truncated
    assert!(!t.can_redo());
}

#[test]
fn push_after_full_undo_replaces_all() {
    let mut t = UndoTree::new();
    t.push(entry(b"a", b"A"));
    t.push(entry(b"b", b"B"));
    t.undo();
    t.undo();
    t.push(entry(b"c", b"C"));
    assert_eq!(t.len(), 1);
}

#[test]
fn redo_after_push_is_gone() {
    let mut t = UndoTree::new();
    t.push(entry(b"a", b"A"));
    t.push(entry(b"b", b"B"));
    t.undo();
    t.push(entry(b"new", b"NEW"));
    assert!(t.redo().is_none());
}

// ──────────── Interleaved operations ────────────

#[test]
fn undo_redo_undo_cycle() {
    let mut t = UndoTree::new();
    t.push(entry(b"1", b"R1"));
    t.push(entry(b"2", b"R2"));
    t.push(entry(b"3", b"R3"));
    t.undo(); // back to 2
    t.undo(); // back to 1
    t.redo(); // forward to 2
    assert!(t.can_undo());
    assert!(t.can_redo());
}

#[test]
fn undo_preserves_len() {
    let mut t = UndoTree::new();
    t.push(entry(b"a", b"A"));
    t.push(entry(b"b", b"B"));
    t.undo();
    assert_eq!(t.len(), 2); // len unchanged
}

#[test]
fn timestamp_is_set() {
    let before = Instant::now();
    let e = entry(b"x", b"X");
    assert!(e.timestamp >= before);
}

#[test]
fn entry_data_preserved() {
    let e = entry(b"forward_data", b"reverse_data");
    assert_eq!(e.forward, b"forward_data");
    assert_eq!(e.reverse, b"reverse_data");
}

#[test]
fn large_undo_tree() {
    let mut t = UndoTree::new();
    for i in 0u32..1000 {
        t.push(entry(&i.to_le_bytes(), &i.to_be_bytes()));
    }
    assert_eq!(t.len(), 1000);
    for _ in 0..500 {
        t.undo();
    }
    assert!(t.can_undo());
    assert!(t.can_redo());
    assert_eq!(t.len(), 1000);
}
