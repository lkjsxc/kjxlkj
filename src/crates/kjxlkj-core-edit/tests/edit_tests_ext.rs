//! Extended tests: visual selection, auto-pairs, and extended text objects.
use kjxlkj_core_edit::visual::{VisualKind, VisualSelection};
use kjxlkj_core_edit::{default_pairs, should_auto_close, should_skip_over, AutoPairConfig};
use kjxlkj_core_edit::{
    find_argument, find_entire_buffer, find_indent_level, find_number, TextRange,
};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Position};

fn buf(text: &str) -> TextBuffer {
    TextBuffer::from_text(BufferId(1), "t".into(), text)
}

#[test]
fn visual_new_and_contains() {
    let sel = VisualSelection::new(VisualKind::Char, Position::new(0, 2), Position::new(0, 8));
    assert!(sel.contains(Position::new(0, 5)));
    assert!(!sel.contains(Position::new(0, 9)));
}
#[test]
fn visual_line_contains_whole_line() {
    let sel = VisualSelection::new(VisualKind::Line, Position::new(1, 0), Position::new(3, 0));
    assert!(sel.contains(Position::new(2, 99)));
    assert!(!sel.contains(Position::new(0, 0)));
}
#[test]
fn visual_swap_ends() {
    let mut sel = VisualSelection::new(VisualKind::Char, Position::new(0, 0), Position::new(0, 10));
    sel.swap_ends();
    assert_eq!(sel.anchor, Position::new(0, 10));
    assert_eq!(sel.cursor, Position::new(0, 0));
}
#[test]
fn visual_block_cols() {
    let sel = VisualSelection::new(VisualKind::Block, Position::new(0, 10), Position::new(3, 3));
    let (l, r) = sel.block_cols();
    assert_eq!(l, 3);
    assert_eq!(r, 11);
}
#[test]
fn visual_extract_line() {
    let b = buf("aaa\nbbb\nccc\n");
    let sel = VisualSelection::new(VisualKind::Line, Position::new(0, 0), Position::new(1, 0));
    let lines = sel.extract_selection(&b);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "aaa");
}
#[test]
fn auto_close_bracket() {
    let cfg = AutoPairConfig::default();
    assert_eq!(should_auto_close('[', &cfg), Some(']'));
    assert_eq!(should_auto_close('{', &cfg), Some('}'));
}
#[test]
fn skip_over_closing_bracket() {
    let cfg = AutoPairConfig::default();
    assert!(should_skip_over(']', Some(']'), &cfg));
    assert!(!should_skip_over(']', Some('a'), &cfg));
}
#[test]
fn default_pairs_count() {
    assert_eq!(default_pairs().len(), 3);
}
#[test]
fn find_argument_inner() {
    let r = find_argument(&["fn(aaa, bbb, ccc)"], Position::new(0, 9), true).unwrap();
    assert_eq!(r.start.col, 8);
    assert_eq!(r.end.col, 11);
}
#[test]
fn find_indent_level_basic() {
    let lines = ["if true:", "    foo", "    bar", "end"];
    let r = find_indent_level(&lines.map(|s| s), Position::new(1, 0), true).unwrap();
    assert_eq!(r.start.line, 1);
    assert_eq!(r.end.line, 2);
}
#[test]
fn find_entire_buffer_range() {
    let r = find_entire_buffer(5, false);
    assert_eq!(r.start, Position::ZERO);
    assert_eq!(r.end.line, 4);
}
#[test]
fn find_number_decimal() {
    let r = find_number("val = 42;", 6).unwrap();
    assert_eq!(r.start.col, 6);
    assert_eq!(r.end.col, 8);
}
#[test]
fn text_range_empty() {
    let tr = TextRange::new(Position::new(0, 5), Position::new(0, 5));
    assert!(tr.is_empty());
}
