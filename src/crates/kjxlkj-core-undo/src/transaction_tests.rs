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

#[test]
fn transaction_clone() {
    let mut tx = Transaction::new(Position::new(1, 1));
    tx.push(Edit::insert(Position::new(1, 1), "x".to_string()));
    let cloned = tx.clone();
    assert_eq!(tx.cursor_before, cloned.cursor_before);
    assert_eq!(tx.edits.len(), cloned.edits.len());
}

#[test]
fn edit_debug_format() {
    let edit = Edit::insert(Position::new(0, 0), "abc".to_string());
    let debug = format!("{:?}", edit);
    assert!(debug.contains("Insert"));
}

#[test]
fn transaction_multiple_edits() {
    let mut tx = Transaction::new(Position::new(0, 0));
    tx.push(Edit::insert(Position::new(0, 0), "a".to_string()));
    tx.push(Edit::insert(Position::new(0, 1), "b".to_string()));
    tx.push(Edit::insert(Position::new(0, 2), "c".to_string()));
    assert_eq!(tx.edits.len(), 3);
}

#[test]
fn edit_kind_equality() {
    assert_eq!(EditKind::Insert, EditKind::Insert);
    assert_eq!(EditKind::Delete, EditKind::Delete);
    assert_ne!(EditKind::Insert, EditKind::Delete);
}

#[test]
fn transaction_cursor_accessors() {
    let mut tx = Transaction::new(Position::new(5, 10));
    tx.set_cursor_after(Position::new(6, 12));
    assert_eq!(tx.cursor_before.line, 5);
    assert_eq!(tx.cursor_after.line, 6);
}

#[test]
fn edit_delete_position() {
    let edit = Edit::delete(Position::new(3, 7), "test".to_string());
    assert_eq!(edit.position.line, 3);
    assert_eq!(edit.position.col, 7);
    assert_eq!(edit.text, "test");
}

#[test]
fn transaction_invert_empty() {
    let tx = Transaction::new(Position::new(0, 0));
    let inverted = tx.invert();
    assert!(inverted.edits.is_empty());
}

#[test]
fn edit_insert_kind() {
    let edit = Edit::insert(Position::new(0, 0), "x".to_string());
    assert_eq!(edit.kind, EditKind::Insert);
}

#[test]
fn edit_delete_kind() {
    let edit = Edit::delete(Position::new(0, 0), "y".to_string());
    assert_eq!(edit.kind, EditKind::Delete);
}

#[test]
fn transaction_has_edits() {
    let mut tx = Transaction::new(Position::new(0, 0));
    tx.push(Edit::insert(Position::new(0, 0), "z".to_string()));
    assert!(!tx.is_empty());
}

#[test]
fn edit_text_content() {
    let edit = Edit::insert(Position::new(1, 2), "content".to_string());
    assert_eq!(edit.text, "content");
}

#[test]
fn transaction_single_edit() {
    let mut tx = Transaction::new(Position::new(0, 0));
    tx.push(Edit::insert(Position::new(0, 0), "a".to_string()));
    assert_eq!(tx.edits.len(), 1);
}

#[test]
fn transaction_two_edits() {
    let mut tx = Transaction::new(Position::new(0, 0));
    tx.push(Edit::insert(Position::new(0, 0), "a".to_string()));
    tx.push(Edit::insert(Position::new(0, 1), "b".to_string()));
    assert_eq!(tx.edits.len(), 2);
}
