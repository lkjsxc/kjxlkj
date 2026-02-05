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

    #[test]
    fn test_transaction_is_empty() {
        let tx = Transaction::new();
        assert!(tx.is_empty());
    }

    #[test]
    fn test_transaction_not_empty_after_push() {
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "x"));
        assert!(!tx.is_empty());
    }

    #[test]
    fn test_history_undo_enables_redo() {
        let mut history = UndoHistory::new();
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "test"));
        history.push(tx);
        let _ = history.undo();
        assert!(history.can_redo());
    }

    #[test]
    fn test_history_redo_after_undo() {
        let mut history = UndoHistory::new();
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "test"));
        history.push(tx);
        let _ = history.undo();
        let _ = history.redo();
        assert!(history.can_undo());
    }

    #[test]
    fn test_history_multiple_transactions() {
        let mut history = UndoHistory::new();
        for i in 0..5 {
            let mut tx = Transaction::new();
            tx.push(Edit::insert(Position::new(0, i), "x"));
            history.push(tx);
        }
        assert!(history.can_undo());
    }

    #[test]
    fn test_history_clear() {
        let mut history = UndoHistory::new();
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "test"));
        history.push(tx);
        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_edit_position() {
        let edit = Edit::insert(Position::new(5, 10), "hi");
        if let Edit::Insert { position, .. } = edit {
            assert_eq!(position.line, 5);
            assert_eq!(position.col, 10);
        } else {
            panic!("Expected Insert");
        }
    }

    #[test]
    fn test_edit_content() {
        let edit = Edit::insert(Position::new(0, 0), "content");
        if let Edit::Insert { text, .. } = edit {
            assert_eq!(text, "content");
        } else {
            panic!("Expected Insert");
        }
    }

    #[test]
    fn test_transaction_edits_order() {
        let mut tx = Transaction::new();
        tx.push(Edit::insert(Position::new(0, 0), "first"));
        tx.push(Edit::insert(Position::new(0, 5), "second"));
        let edits = tx.edits();
        assert!(matches!(&edits[0], Edit::Insert { text, .. } if text == "first"));
        assert!(matches!(&edits[1], Edit::Insert { text, .. } if text == "second"));
    }

    #[test]
    fn test_history_default() {
        let history = UndoHistory::default();
        assert!(!history.can_undo());
    }

    #[test]
    fn test_transaction_default() {
        let tx = Transaction::default();
        assert!(tx.is_empty());
    }
}
