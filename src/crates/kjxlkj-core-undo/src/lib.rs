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

    #[test]
    fn test_edit_insert_create() {
        let edit = Edit::insert(Position::new(0, 0), "hello");
        assert!(matches!(edit, Edit::Insert { .. }));
    }

    #[test]
    fn test_edit_delete_create() {
        let edit = Edit::delete(Position::new(1, 5), "world");
        assert!(matches!(edit, Edit::Delete { .. }));
    }

    #[test]
    fn test_transaction_push_edit() {
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "a"));
        tx.push(Edit::insert(Position::new(0, 1), "b"));
        assert_eq!(tx.edits().len(), 2);
    }

    #[test]
    fn test_history_push_pop() {
        let mut history = UndoHistory::new();
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "test"));
        history.push(tx);
        assert!(history.can_undo());
    }

    #[test]
    fn test_history_empty() {
        let history = UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }
}
