//! Comprehensive undo/redo tests.

use kjxlkj_core_undo::{Edit, Transaction, UndoHistory};
use kjxlkj_core_types::Position;

// === UndoHistory Tests ===

#[test]
fn test_new_history_empty() {
    let history = UndoHistory::new();
    assert!(!history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(history.undo_count(), 0);
    assert_eq!(history.redo_count(), 0);
}

#[test]
fn test_push_makes_undoable() {
    let mut history = UndoHistory::new();
    
    let mut tx = Transaction::new();
    tx.push(Edit::insert(Position::new(0, 0), "hello"));
    history.push(tx);
    
    assert!(history.can_undo());
    assert_eq!(history.undo_count(), 1);
}

#[test]
fn test_undo_returns_inverse() {
    let mut history = UndoHistory::new();
    
    let mut tx = Transaction::new();
    tx.push(Edit::insert(Position::new(0, 0), "hello"));
    history.push(tx);
    
    let inverse = history.undo();
    assert!(inverse.is_some());
    let inverse = inverse.unwrap();
    assert!(!inverse.is_empty());
}

#[test]
fn test_undo_enables_redo() {
    let mut history = UndoHistory::new();
    
    let mut tx = Transaction::new();
    tx.push(Edit::insert(Position::new(0, 0), "hello"));
    history.push(tx);
    
    history.undo();
    
    assert!(!history.can_undo());
    assert!(history.can_redo());
    assert_eq!(history.redo_count(), 1);
}

#[test]
fn test_redo_returns_transaction() {
    let mut history = UndoHistory::new();
    
    let mut tx = Transaction::new();
    tx.push(Edit::insert(Position::new(0, 0), "hello"));
    history.push(tx);
    
    history.undo();
    let redo_tx = history.redo();
    
    assert!(redo_tx.is_some());
    assert!(history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_new_change_clears_redo() {
    let mut history = UndoHistory::new();
    
    // First change
    let mut tx1 = Transaction::new();
    tx1.push(Edit::insert(Position::new(0, 0), "hello"));
    history.push(tx1);
    
    // Undo it
    history.undo();
    assert!(history.can_redo());
    
    // Make a new change
    let mut tx2 = Transaction::new();
    tx2.push(Edit::insert(Position::new(0, 0), "world"));
    history.push(tx2);
    
    // Redo should be cleared
    assert!(!history.can_redo());
}

#[test]
fn test_multiple_undo_redo() {
    let mut history = UndoHistory::new();
    
    for i in 0..5 {
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, i), format!("edit{}", i)));
        history.push(tx);
    }
    
    assert_eq!(history.undo_count(), 5);
    
    // Undo all
    for _ in 0..5 {
        history.undo();
    }
    
    assert_eq!(history.undo_count(), 0);
    assert_eq!(history.redo_count(), 5);
    
    // Redo all
    for _ in 0..5 {
        history.redo();
    }
    
    assert_eq!(history.undo_count(), 5);
    assert_eq!(history.redo_count(), 0);
}

#[test]
fn test_clear_removes_all() {
    let mut history = UndoHistory::new();
    
    let mut tx = Transaction::new();
    tx.push(Edit::insert(Position::new(0, 0), "hello"));
    history.push(tx);
    history.undo();
    
    history.clear();
    
    assert!(!history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_empty_transaction_not_pushed() {
    let mut history = UndoHistory::new();
    
    let tx = Transaction::new();
    history.push(tx);
    
    assert!(!history.can_undo());
    assert_eq!(history.undo_count(), 0);
}

// === Transaction Tests ===

#[test]
fn test_transaction_new_empty() {
    let tx = Transaction::new();
    assert!(tx.is_empty());
}

#[test]
fn test_transaction_with_edits_not_empty() {
    let mut tx = Transaction::new();
    tx.push(Edit::insert(Position::new(0, 0), "hello"));
    assert!(!tx.is_empty());
}

#[test]
fn test_transaction_inverse() {
    let mut tx = Transaction::new();
    tx.push(Edit::insert(Position::new(0, 0), "hello"));
    
    let inverse = tx.inverse();
    assert!(!inverse.is_empty());
}

// === Edit Tests ===

#[test]
fn test_edit_insert() {
    let edit = Edit::insert(Position::new(1, 5), "text");
    // Should create without panicking
    match edit {
        Edit::Insert { position, text } => {
            assert_eq!(position.line, 1);
            assert_eq!(position.col, 5);
            assert_eq!(text, "text");
        }
        _ => panic!("Expected Insert variant"),
    }
}

#[test]
fn test_edit_delete() {
    let edit = Edit::delete(Position::new(0, 0), "hello");
    // Should create without panicking
    match edit {
        Edit::Delete { position, text } => {
            assert_eq!(position.line, 0);
            assert_eq!(position.col, 0);
            assert_eq!(text, "hello");
        }
        _ => panic!("Expected Delete variant"),
    }
}

#[test]
fn test_edit_inverse() {
    let edit = Edit::insert(Position::new(0, 0), "hello");
    let inverse = edit.inverse();
    
    // Insert inverse should be a delete
    assert!(matches!(inverse, Edit::Delete { .. }));
}
