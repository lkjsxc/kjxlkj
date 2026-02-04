//! Integration tests for core-undo.

use crate::*;
use kjxlkj_core_edit::EditOp;
use kjxlkj_core_types::{BufferVersion, CharOffset, Cursor};

#[test]
fn transaction_cursor_tracking() {
    let mut tx = Transaction::new(Cursor::origin(), BufferVersion::default());
    tx.push(EditOp::insert(CharOffset::new(0), "hello"));

    let new_cursor = Cursor::at(kjxlkj_core_types::LineCol::new(0, 5));
    tx.set_cursor_after(new_cursor);

    assert_eq!(tx.cursor_after.position.col, 5);
}

#[test]
fn history_max_size() {
    let mut history = UndoHistory::with_max_size(5);

    for i in 0..10 {
        history.begin(Cursor::origin(), BufferVersion::new(i));
        history.current_mut().unwrap().push(EditOp::insert(CharOffset::new(0), "x"));
        history.commit();
    }

    assert_eq!(history.undo_count(), 5);
}
