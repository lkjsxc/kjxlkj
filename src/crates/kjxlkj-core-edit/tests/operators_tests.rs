//! Tests for operators, block selection, and completion menu.

use kjxlkj_core_edit::{
    apply_operator, change_range, delete_range, indent_range, lower_case_range, outdent_range,
    toggle_case_range, upper_case_range, yank_range,
};
use kjxlkj_core_edit::{BlockOp, BlockSelection};
use kjxlkj_core_edit::{build_block_edits, extend_to_eol};
use kjxlkj_core_edit::{CompletionItem, CompletionMenu, CompletionSource};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Operator, Position, Range};

fn buf(text: &str) -> TextBuffer {
    TextBuffer::from_text(BufferId(1), "test".into(), text)
}

// --- Operator functions ---

#[test]
fn delete_range_removes_text() {
    let mut b = buf("hello world");
    let deleted = delete_range(&mut b, Range::new(Position::new(0, 0), Position::new(0, 5)));
    assert_eq!(deleted, "hello");
    assert_eq!(b.line(0).unwrap(), " world");
}

#[test]
fn yank_range_preserves_buffer() {
    let b = buf("hello world");
    let text = yank_range(&b, Range::new(Position::new(0, 6), Position::new(0, 11)));
    assert_eq!(text, "world");
    assert_eq!(b.line(0).unwrap(), "hello world");
}

#[test]
fn change_range_deletes_and_returns() {
    let mut b = buf("abcdef");
    let deleted = change_range(&mut b, Range::new(Position::new(0, 2), Position::new(0, 4)));
    assert_eq!(deleted, "cd");
    assert_eq!(b.line(0).unwrap(), "abef");
}

#[test]
fn indent_range_adds_space() {
    let mut b = buf("hello\nworld\n");
    indent_range(&mut b, Range::new(Position::new(0, 0), Position::new(1, 3)), 1);
    let l0 = b.line(0).unwrap();
    assert!(l0.starts_with("    ") || l0.starts_with('\t'));
}

#[test]
fn outdent_range_removes_indent() {
    let mut b = buf("        hello\n        world\n");
    outdent_range(&mut b, Range::new(Position::new(0, 0), Position::new(1, 5)), 1);
    let l0 = b.line(0).unwrap();
    let old_spaces = "        ".len();
    assert!(l0.len() < old_spaces + "hello".len());
}

#[test]
fn upper_case_range_converts() {
    let mut b = buf("hello");
    upper_case_range(&mut b, Range::new(Position::new(0, 0), Position::new(0, 5)));
    assert_eq!(b.line(0).unwrap(), "HELLO");
}

#[test]
fn lower_case_range_converts() {
    let mut b = buf("HELLO");
    lower_case_range(&mut b, Range::new(Position::new(0, 0), Position::new(0, 5)));
    assert_eq!(b.line(0).unwrap(), "hello");
}

#[test]
fn toggle_case_range_swaps() {
    let mut b = buf("HeLLo");
    toggle_case_range(&mut b, Range::new(Position::new(0, 0), Position::new(0, 5)));
    assert_eq!(b.line(0).unwrap(), "hEllO");
}

#[test]
fn apply_operator_delete() {
    let mut b = buf("abc def");
    let res = apply_operator(&mut b, Operator::Delete, Range::new(Position::new(0, 0), Position::new(0, 3)));
    assert_eq!(res.deleted_text, Some("abc".into()));
    assert!(!res.entered_insert);
}

#[test]
fn apply_operator_change_enters_insert() {
    let mut b = buf("abc def");
    let res = apply_operator(&mut b, Operator::Change, Range::new(Position::new(0, 0), Position::new(0, 3)));
    assert!(res.entered_insert);
}

#[test]
fn apply_operator_yank_no_modify() {
    let b_text = "abc def";
    let mut b = buf(b_text);
    let res = apply_operator(&mut b, Operator::Yank, Range::new(Position::new(0, 0), Position::new(0, 3)));
    assert_eq!(res.deleted_text, Some("abc".into()));
    assert_eq!(b.line(0).unwrap(), b_text);
}

// --- BlockSelection ---

#[test]
fn block_selection_dimensions() {
    let sel = BlockSelection::new(Position::new(2, 3), Position::new(5, 8));
    assert_eq!(sel.height(), 4);
    assert_eq!(sel.width(), 6);
}

#[test]
fn block_selection_line_range() {
    let sel = BlockSelection::new(Position::new(0, 0), Position::new(9, 5));
    assert_eq!(sel.height(), 10);
}

#[test]
fn block_op_variants() {
    let ops = [BlockOp::Insert, BlockOp::Append, BlockOp::Change, BlockOp::Delete];
    assert_eq!(ops.len(), 4);
    assert_eq!(ops[0], BlockOp::Insert);
}

#[test]
fn build_block_edits_count() {
    let b = buf("line1\nline2\nline3\n");
    let sel = BlockSelection::new(Position::new(0, 0), Position::new(2, 3));
    let edits = build_block_edits(&sel, &b);
    assert_eq!(edits.len(), 3);
}

// --- CompletionMenu ---

#[test]
fn completion_menu_open() {
    let items = vec![item("alpha"), item("beta")];
    let menu = CompletionMenu::open(items);
    assert!(menu.is_active());
    assert_eq!(menu.current().unwrap().label, "alpha");
}

#[test]
fn completion_menu_select_next_prev() {
    let mut menu = CompletionMenu::open(vec![item("a"), item("b"), item("c")]);
    menu.select_next();
    assert_eq!(menu.current().unwrap().label, "b");
    menu.select_prev();
    assert_eq!(menu.current().unwrap().label, "a");
    menu.select_prev(); // wraps
    assert_eq!(menu.current().unwrap().label, "c");
}

#[test]
fn completion_menu_filter() {
    let mut menu = CompletionMenu::open(vec![item("apple"), item("banana"), item("avocado")]);
    menu.filter("a");
    assert_eq!(menu.items.len(), 2);
    assert_eq!(menu.selected_index, 0);
}

#[test]
fn completion_menu_close() {
    let mut menu = CompletionMenu::open(vec![item("x")]);
    menu.close();
    assert!(!menu.is_active());
    assert!(menu.current().is_none());
}

fn item(label: &str) -> CompletionItem {
    CompletionItem {
        label: label.into(),
        detail: None,
        kind: None,
        source: CompletionSource::Buffer,
    }
}
