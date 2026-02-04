//! Tests for transaction and edit types.

use crate::{Edit, EditKind, Transaction};
use kjxlkj_core_types::Position;

#[test]
fn edit_invert() {
    let insert = Edit::insert(Position::new(0, 0), "hello".to_string());
    let inverted = insert.invert();
    assert_eq!(inverted.kind, EditKind::Delete);
    assert_eq!(inverted.text, "hello");
}

#[test]
fn transaction_invert() {
    let mut tx = Transaction::new(Position::new(0, 0));
    tx.push(Edit::insert(Position::new(0, 0), "a".to_string()));
    tx.push(Edit::insert(Position::new(0, 1), "b".to_string()));
    tx.set_cursor_after(Position::new(0, 2));
    let inverted = tx.invert();
    assert_eq!(inverted.edits.len(), 2);
    assert_eq!(inverted.edits[0].kind, EditKind::Delete);
    assert_eq!(inverted.edits[0].text, "b");
    assert_eq!(inverted.cursor_before, Position::new(0, 2));
    assert_eq!(inverted.cursor_after, Position::new(0, 0));
}

#[test]
fn transaction_is_empty() {
    let tx = Transaction::new(Position::new(0, 0));
    assert!(tx.is_empty());
}

#[test]
fn transaction_not_empty() {
    let mut tx = Transaction::new(Position::new(0, 0));
    tx.push(Edit::insert(Position::new(0, 0), "x".to_string()));
    assert!(!tx.is_empty());
}

#[test]
fn edit_delete() {
    let del = Edit::delete(Position::new(1, 5), "test".to_string());
    assert_eq!(del.kind, EditKind::Delete);
    assert_eq!(del.position, Position::new(1, 5));
    assert_eq!(del.text, "test");
}

#[test]
fn edit_delete_invert() {
    let del = Edit::delete(Position::new(1, 0), "abc".to_string());
    let inverted = del.invert();
    assert_eq!(inverted.kind, EditKind::Insert);
}

#[test]
fn transaction_double_invert() {
    let mut tx = Transaction::new(Position::new(5, 5));
    tx.push(Edit::insert(Position::new(5, 5), "hello".to_string()));
    tx.set_cursor_after(Position::new(5, 10));
    let inverted = tx.invert();
    let restored = inverted.invert();
    assert_eq!(restored.cursor_before, tx.cursor_before);
    assert_eq!(restored.cursor_after, tx.cursor_after);
}

#[test]
fn edit_insert_position() {
    let edit = Edit::insert(Position::new(3, 7), "x".to_string());
    assert_eq!(edit.position, Position::new(3, 7));
}

#[test]
fn edit_equality() {
    let e1 = Edit::insert(Position::new(0, 0), "a".to_string());
    let e2 = Edit::insert(Position::new(0, 0), "a".to_string());
    assert_eq!(e1, e2);
}

#[test]
fn edit_inequality() {
    let e1 = Edit::insert(Position::new(0, 0), "a".to_string());
    let e2 = Edit::insert(Position::new(0, 0), "b".to_string());
    assert_ne!(e1, e2);
}

#[test]
fn transaction_cursor_positions() {
    let mut tx = Transaction::new(Position::new(1, 1));
    tx.set_cursor_after(Position::new(2, 2));
    assert_eq!(tx.cursor_before, Position::new(1, 1));
    assert_eq!(tx.cursor_after, Position::new(2, 2));
}

#[test]
fn edit_clone() {
    let edit = Edit::insert(Position::new(0, 0), "test".to_string());
    let cloned = edit.clone();
    assert_eq!(edit, cloned);
}
