//! Tests for undo module.

use super::*;
use kjxlkj_core_edit::{EditOp, Transaction};
use kjxlkj_core_types::{BufferVersion, Position};

#[test]
fn test_undo_history_push() {
    let mut history = UndoHistory::new();

    let mut tx = Transaction::new(BufferVersion::new(0), Position::new(0, 0));
    tx.push(EditOp::Insert {
        pos: Position::new(0, 0),
        text: "hello".to_string(),
    });

    history.push(tx);

    assert!(history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(history.undo_count(), 1);
}

#[test]
fn test_undo_redo_cycle() {
    let mut history = UndoHistory::new();

    let mut tx = Transaction::new(BufferVersion::new(0), Position::new(0, 0));
    tx.push(EditOp::Insert {
        pos: Position::new(0, 0),
        text: "a".to_string(),
    });
    history.push(tx);

    // Undo
    let inverse = history.undo().unwrap();
    assert!(!history.can_undo());
    assert!(history.can_redo());

    // The inverse should be a delete
    match &inverse.ops[0] {
        EditOp::Delete { .. } => {}
        _ => panic!("Expected delete"),
    }

    // Redo
    let redo_inverse = history.redo().unwrap();
    assert!(history.can_undo());
    assert!(!history.can_redo());

    match &redo_inverse.ops[0] {
        EditOp::Insert { .. } => {}
        _ => panic!("Expected insert"),
    }
}

#[test]
fn test_push_clears_redo() {
    let mut history = UndoHistory::new();

    let mut tx1 = Transaction::new(BufferVersion::new(0), Position::new(0, 0));
    tx1.push(EditOp::Insert {
        pos: Position::new(0, 0),
        text: "a".to_string(),
    });
    history.push(tx1);

    history.undo();
    assert!(history.can_redo());

    // Push new transaction
    let mut tx2 = Transaction::new(BufferVersion::new(1), Position::new(0, 0));
    tx2.push(EditOp::Insert {
        pos: Position::new(0, 0),
        text: "b".to_string(),
    });
    history.push(tx2);

    // Redo should be cleared
    assert!(!history.can_redo());
}

#[test]
fn test_empty_transaction_not_pushed() {
    let mut history = UndoHistory::new();
    let tx = Transaction::new(BufferVersion::new(0), Position::new(0, 0));
    history.push(tx);
    assert!(!history.can_undo());
}
