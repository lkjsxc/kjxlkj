use kjxlkj_core_text::{is_word_char, word_start_forward, word_start_backward, word_end_forward, TextBuffer};
use kjxlkj_core_types::Position;

#[test]
fn word_forward_simple() {
    let buf = TextBuffer::from_text("hello world foo");
    assert_eq!(word_start_forward(&buf, Position::new(0, 0)), Position::new(0, 6));
}

#[test]
fn word_backward_simple() {
    let buf = TextBuffer::from_text("hello world");
    assert_eq!(word_start_backward(&buf, Position::new(0, 8)), Position::new(0, 6));
}

#[test]
fn word_end_simple() {
    let buf = TextBuffer::from_text("hello world");
    assert_eq!(word_end_forward(&buf, Position::new(0, 0)), Position::new(0, 4));
}

#[test]
fn word_forward_across_lines() {
    let buf = TextBuffer::from_text("hello\nworld");
    assert_eq!(word_start_forward(&buf, Position::new(0, 0)), Position::new(1, 0));
}

#[test]
fn word_char_classification() {
    assert!(is_word_char('a'));
    assert!(is_word_char('_'));
    assert!(is_word_char('5'));
    assert!(!is_word_char('.'));
    assert!(!is_word_char(' '));
}
