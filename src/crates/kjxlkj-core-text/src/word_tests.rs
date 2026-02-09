//! Tests for word classification and boundary detection.

use crate::word::{
    classify_word_char, find_word_boundary, find_word_end, find_word_start_backward, WordKind,
};

#[test]
fn classify_chars() {
    assert_eq!(classify_word_char('a'), WordKind::Word);
    assert_eq!(classify_word_char('0'), WordKind::Word);
    assert_eq!(classify_word_char('_'), WordKind::Word);
    assert_eq!(classify_word_char(' '), WordKind::Whitespace);
    assert_eq!(classify_word_char('.'), WordKind::Punctuation);
}

#[test]
fn word_forward() {
    let line = "hello world";
    let next = find_word_boundary(line, 0, false);
    assert_eq!(next, Some(6));
}

#[test]
fn word_end() {
    let line = "hello world";
    let end = find_word_end(line, 0, false);
    assert_eq!(end, Some(4));
}

#[test]
fn word_backward() {
    let line = "hello world";
    let start = find_word_start_backward(line, 8, false);
    assert_eq!(start, Some(6));
}
