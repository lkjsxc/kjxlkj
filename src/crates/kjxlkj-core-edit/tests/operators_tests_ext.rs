//! Extended tests for block selection and completion menu.

use kjxlkj_core_edit::build_block_edits;
use kjxlkj_core_edit::{BlockOp, BlockSelection};
use kjxlkj_core_edit::{CompletionItem, CompletionMenu, CompletionSource};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Position};

fn buf(text: &str) -> TextBuffer {
    TextBuffer::from_text(BufferId(1), "test".into(), text)
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
    let ops = [
        BlockOp::Insert,
        BlockOp::Append,
        BlockOp::Change,
        BlockOp::Delete,
    ];
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
