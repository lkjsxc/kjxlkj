//! Integration tests for core-text.

use crate::*;
use kjxlkj_core_types::CharOffset;

#[test]
fn rope_text_basic_operations() {
    let mut text = RopeText::from_str("Line 1\nLine 2\nLine 3");
    assert_eq!(text.len_lines(), 3);

    text.insert(CharOffset::new(0), "Start: ");
    assert!(text.to_string().starts_with("Start: "));
}

#[test]
fn unicode_handling() {
    let text = RopeText::from_str("日本語テスト");
    assert_eq!(text.len_chars(), 6);
    assert_eq!(display_width("日本語テスト"), 12);
}
