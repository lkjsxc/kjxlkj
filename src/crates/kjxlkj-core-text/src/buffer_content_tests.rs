//! Tests for BufferContent.

use crate::buffer_content::BufferContent;

#[test]
fn empty_buffer() {
    let b = BufferContent::empty();
    assert_eq!(b.line_count(), 1);
    assert_eq!(b.char_count(), 0);
}

#[test]
fn from_str_and_lines() {
    let b = BufferContent::from_str("hello\nworld\n");
    assert_eq!(b.line_count(), 3);
    assert_eq!(b.line_content(0), "hello");
    assert_eq!(b.line_content(1), "world");
}

#[test]
fn insert_text() {
    let mut b = BufferContent::from_str("hello\n");
    b.insert(0, 5, " world");
    assert_eq!(b.line_content(0), "hello world");
}

#[test]
fn delete_text() {
    let mut b = BufferContent::from_str("hello world\n");
    b.delete(0, 5, 0, 11);
    assert_eq!(b.line_content(0), "hello");
}

#[test]
fn cjk_insert() {
    let mut b = BufferContent::from_str("あいう\n");
    b.insert(0, 1, "X");
    assert_eq!(b.line_content(0), "あXいう");
}
