//! Undo/redo system for the editor.
//!
//! Provides transactional undo with support for grouping multiple edits.

mod history;
mod transaction;

pub use history::UndoHistory;
pub use transaction::{Edit, Transaction};

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Position;

    #[test]
    fn test_module_exports() {
        let _history = UndoHistory::new();
        let _tx = Transaction::new();
        let _edit = Edit::insert(Position::new(0, 0), "test");
    }
}
