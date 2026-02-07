//! Tests for UndoTree, BranchingUndoTree, WindowOptions, CloseGuard, WindowSnapshot.

use kjxlkj_core_types::{BufferId, Position, WindowId};
use kjxlkj_core_undo::{
    can_close, BranchingUndoTree, ChangeEntry, ChangeKind, CloseGuard, SignColumn, TextChange,
    UndoEntry, UndoTree, WindowOptionStore, WindowOptions, WindowSnapshot,
};

fn entry(ts: u64) -> UndoEntry {
    UndoEntry {
        changes: vec![TextChange {
            kind: ChangeKind::Insert,
            position: Position::new(0, 0),
            text: format!("t{ts}"),
        }],
        cursor_before: Position::ZERO,
        cursor_after: Position::new(0, 2),
        timestamp: ts,
    }
}

fn change_entry(label: &str) -> ChangeEntry {
    ChangeEntry {
        forward: vec![TextChange {
            kind: ChangeKind::Insert,
            position: Position::ZERO,
            text: label.into(),
        }],
        reverse: vec![TextChange {
            kind: ChangeKind::Delete,
            position: Position::ZERO,
            text: label.into(),
        }],
        cursor_before: Position::ZERO,
        cursor_after: Position::new(0, label.len()),
    }
}

// --- UndoTree ---

#[test]
fn undo_tree_push_and_undo() {
    let mut tree = UndoTree::new();
    tree.push(entry(1));
    tree.push(entry(2));
    assert_eq!(tree.entry_count(), 2);
    let e = tree.undo().unwrap();
    assert_eq!(e.timestamp, 2);
}

#[test]
fn undo_tree_redo() {
    let mut tree = UndoTree::new();
    tree.push(entry(1));
    tree.undo();
    assert!(tree.can_redo());
    let e = tree.redo().unwrap();
    assert_eq!(e.timestamp, 1);
}

#[test]
fn undo_tree_push_truncates_redo() {
    let mut tree = UndoTree::new();
    tree.push(entry(1));
    tree.push(entry(2));
    tree.undo();
    tree.push(entry(3));
    assert_eq!(tree.entry_count(), 2);
    assert!(!tree.can_redo());
}

#[test]
fn undo_tree_empty_undo() {
    let mut tree = UndoTree::new();
    assert!(tree.undo().is_none());
    assert!(!tree.can_undo());
}

#[test]
fn undo_tree_clear() {
    let mut tree = UndoTree::new();
    tree.push(entry(1));
    tree.clear();
    assert_eq!(tree.entry_count(), 0);
}

// --- BranchingUndoTree ---

#[test]
fn branching_push_and_undo() {
    let mut tree = BranchingUndoTree::new();
    tree.push(change_entry("a"));
    tree.push(change_entry("b"));
    assert_eq!(tree.node_count(), 3);
    let e = tree.undo().unwrap();
    assert_eq!(e.forward[0].text, "b");
}

#[test]
fn branching_redo() {
    let mut tree = BranchingUndoTree::new();
    tree.push(change_entry("a"));
    tree.undo();
    tree.push(change_entry("b"));
    tree.undo();
    assert_eq!(tree.branches_at_current(), 2);
    let e = tree.redo(0).unwrap();
    assert_eq!(e.forward[0].text, "a");
}

#[test]
fn branching_undo_at_root() {
    let mut tree = BranchingUndoTree::new();
    assert!(tree.undo().is_none());
}

#[test]
fn branching_redo_bad_index() {
    let mut tree = BranchingUndoTree::new();
    tree.push(change_entry("x"));
    assert!(tree.redo(99).is_none());
}

// --- WindowOptions ---

#[test]
fn window_options_defaults() {
    let opts = WindowOptions::default();
    assert!(opts.wrap);
    assert!(!opts.number);
    assert!(!opts.relative_number);
    assert_eq!(opts.sign_column, SignColumn::Auto);
    assert_eq!(opts.scrolloff, 0);
}

// --- CloseGuard ---

#[test]
fn close_guard_needs_save() {
    assert_eq!(can_close(true, false), CloseGuard::NeedsSave);
}

#[test]
fn close_guard_last_window() {
    assert_eq!(can_close(false, true), CloseGuard::LastWindow);
}

#[test]
fn close_guard_allow() {
    assert_eq!(can_close(false, false), CloseGuard::Allow);
}

// --- WindowSnapshot ---

#[test]
fn window_snapshot_creation() {
    let snap = WindowSnapshot {
        id: WindowId(1),
        buffer_id: BufferId(2),
        options: WindowOptions::default(),
        cursor: Position::new(5, 10),
        viewport_top: 0,
    };
    assert_eq!(snap.id, WindowId(1));
    assert_eq!(snap.cursor, Position::new(5, 10));
}

// --- WindowOptionStore ---

#[test]
fn option_store_get_default() {
    let store = WindowOptionStore::default();
    assert_eq!(store.get(WindowId(42)), WindowOptions::default());
}
