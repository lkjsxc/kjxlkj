//! Tests for the mark system.
use crate::marks::{MarkFile, MarkPosition};

#[test]
fn test_set_and_get_local_mark() {
    let mut mf = MarkFile::new();
    let pos = MarkPosition::new(1, 10, 5);
    mf.set('a', pos);
    assert_eq!(mf.get('a', 1), Some(&pos));
    assert_eq!(mf.get('a', 2), None);
}

#[test]
fn test_global_mark() {
    let mut mf = MarkFile::new();
    let pos = MarkPosition::new(1, 20, 0);
    mf.set('A', pos);
    assert_eq!(mf.get('A', 1), Some(&pos));
    assert_eq!(mf.get('A', 2), Some(&pos));
}

#[test]
fn test_delete_mark() {
    let mut mf = MarkFile::new();
    mf.set(
        'a',
        MarkPosition::new(1, 5, 0),
    );
    assert!(mf.delete('a', 1));
    assert_eq!(mf.get('a', 1), None);
}

#[test]
fn test_special_marks() {
    let mut mf = MarkFile::new();
    let pos = MarkPosition::new(1, 15, 3);
    mf.set_last_change(pos);
    assert_eq!(mf.get('.', 1), Some(&pos));
}

#[test]
fn test_adjust_for_insert() {
    let mut mf = MarkFile::new();
    mf.set(
        'a',
        MarkPosition::new(1, 10, 0),
    );
    mf.adjust_for_edit(1, 5, 3);
    assert_eq!(mf.get('a', 1).unwrap().line, 13);
}

#[test]
fn test_adjust_for_delete() {
    let mut mf = MarkFile::new();
    mf.set(
        'a',
        MarkPosition::new(1, 10, 0),
    );
    mf.adjust_for_edit(1, 5, -3);
    assert_eq!(mf.get('a', 1).unwrap().line, 7);
}

#[test]
fn test_list_for_buffer() {
    let mut mf = MarkFile::new();
    mf.set(
        'a',
        MarkPosition::new(1, 1, 0),
    );
    mf.set(
        'b',
        MarkPosition::new(1, 2, 0),
    );
    let list = mf.list_for_buffer(1);
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].0, 'a');
    assert_eq!(list[1].0, 'b');
}
