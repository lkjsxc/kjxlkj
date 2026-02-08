//! Tests for operator execution.

use crate::cursor::CursorPosition;
use crate::operator_helpers::normalize_range;

#[test]
fn normalize_range_order() {
    let a = CursorPosition::new(0, 5);
    let b = CursorPosition::new(0, 2);
    let (s, e) = normalize_range(a, b);
    assert_eq!(s.grapheme_offset, 2);
    assert_eq!(e.grapheme_offset, 5);
}
