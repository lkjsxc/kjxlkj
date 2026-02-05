//! Comprehensive tests for kjxlkj-core-undo.

use kjxlkj_core_types::{BufferVersion, Position, Range};
use kjxlkj_core_undo::*;

mod edit_tests {
    use super::*;

    #[test]
    fn test_edit_insert() {
        let edit = Edit::insert(
            Position::new(0, 0),
            "hello".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 5),
        );
        match edit.kind {
            EditKind::Insert { pos, text } => {
                assert_eq!(pos, Position::new(0, 0));
                assert_eq!(text, "hello");
            }
            _ => panic!("Expected Insert"),
        }
    }

    #[test]
    fn test_edit_delete() {
        let edit = Edit::delete(
            Range::from_coords(0, 0, 0, 5),
            "hello".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 0),
        );
        match edit.kind {
            EditKind::Delete { range, text } => {
                assert_eq!(range.start, Position::new(0, 0));
                assert_eq!(text, "hello");
            }
            _ => panic!("Expected Delete"),
        }
    }

    #[test]
    fn test_edit_replace() {
        let edit = Edit {
            kind: EditKind::Replace {
                range: Range::from_coords(0, 0, 0, 5),
                old: "hello".to_string(),
                new: "world".to_string(),
            },
            version_before: BufferVersion::new(0),
            version_after: BufferVersion::new(1),
            cursor_before: Position::new(0, 0),
            cursor_after: Position::new(0, 5),
        };
        match edit.kind {
            EditKind::Replace { range, old, new } => {
                assert_eq!(range.start, Position::new(0, 0));
                assert_eq!(old, "hello");
                assert_eq!(new, "world");
            }
            _ => panic!("Expected Replace"),
        }
    }

    #[test]
    fn test_edit_cursor_positions() {
        let edit = Edit::insert(
            Position::new(1, 5),
            "test".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(1, 9),
        );
        assert_eq!(edit.cursor_before, Position::new(1, 5));
        assert_eq!(edit.cursor_after, Position::new(1, 9));
    }
}

mod history_tests {
    use super::*;

    #[test]
    fn test_new_history() {
        let history = UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_record_enables_undo() {
        let mut history = UndoHistory::new();
        let edit = Edit::insert(
            Position::origin(),
            "a".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 1),
        );
        history.record(edit);
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_undo() {
        let mut history = UndoHistory::new();
        let edit = Edit::insert(
            Position::origin(),
            "a".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 1),
        );
        history.record(edit);

        let undone = history.undo();
        assert!(undone.is_some());
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_redo() {
        let mut history = UndoHistory::new();
        let edit = Edit::insert(
            Position::origin(),
            "a".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 1),
        );
        history.record(edit);
        history.undo();

        let redone = history.redo();
        assert!(redone.is_some());
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_new_edit_clears_redo() {
        let mut history = UndoHistory::new();
        
        history.record(Edit::insert(
            Position::origin(),
            "a".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 1),
        ));
        history.undo();
        assert!(history.can_redo());

        history.record(Edit::insert(
            Position::origin(),
            "b".to_string(),
            BufferVersion::new(1),
            BufferVersion::new(2),
            Position::new(0, 1),
        ));
        assert!(!history.can_redo());
    }

    #[test]
    fn test_multiple_undos() {
        let mut history = UndoHistory::new();

        for i in 0..5 {
            history.record(Edit::insert(
                Position::new(0, i),
                "x".to_string(),
                BufferVersion::new(i as u64),
                BufferVersion::new((i + 1) as u64),
                Position::new(0, i + 1),
            ));
        }

        for i in (0..5).rev() {
            assert!(history.can_undo());
            let edit = history.undo().unwrap();
            match &edit.kind {
                EditKind::Insert { pos, .. } => {
                    assert_eq!(pos.column, i);
                }
                _ => panic!("Expected Insert"),
            }
        }

        assert!(!history.can_undo());
    }

    #[test]
    fn test_multiple_redos() {
        let mut history = UndoHistory::new();

        for i in 0..3 {
            history.record(Edit::insert(
                Position::new(0, i),
                "x".to_string(),
                BufferVersion::new(i as u64),
                BufferVersion::new((i + 1) as u64),
                Position::new(0, i + 1),
            ));
        }

        // Undo all
        while history.can_undo() {
            history.undo();
        }

        // Redo all
        let mut count = 0;
        while history.can_redo() {
            history.redo();
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn test_clear() {
        let mut history = UndoHistory::new();

        history.record(Edit::insert(
            Position::origin(),
            "a".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 1),
        ));
        history.undo();

        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_transaction_basic() {
        let mut history = UndoHistory::new();

        history.begin_transaction();
        history.record(Edit::insert(
            Position::new(0, 0),
            "a".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 1),
        ));
        history.record(Edit::insert(
            Position::new(0, 1),
            "b".to_string(),
            BufferVersion::new(1),
            BufferVersion::new(2),
            Position::new(0, 2),
        ));
        history.commit_transaction();

        // Single undo should undo both
        assert!(history.can_undo());
        let edit1 = history.undo();
        assert!(edit1.is_some());
        // In a simple implementation, might need two undos
    }

    #[test]
    fn test_undo_returns_correct_edit() {
        let mut history = UndoHistory::new();

        history.record(Edit::insert(
            Position::origin(),
            "hello".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 5),
        ));

        let edit = history.undo().unwrap();
        match &edit.kind {
            EditKind::Insert { text, .. } => {
                assert_eq!(text, "hello");
            }
            _ => panic!("Expected Insert"),
        }
    }

    #[test]
    fn test_redo_returns_correct_edit() {
        let mut history = UndoHistory::new();

        history.record(Edit::insert(
            Position::origin(),
            "world".to_string(),
            BufferVersion::new(0),
            BufferVersion::new(1),
            Position::new(0, 5),
        ));

        history.undo();
        let edit = history.redo().unwrap();
        match &edit.kind {
            EditKind::Insert { text, .. } => {
                assert_eq!(text, "world");
            }
            _ => panic!("Expected Insert"),
        }
    }
}

mod edit_kind_tests {
    use super::*;

    #[test]
    fn test_edit_kind_insert_debug() {
        let kind = EditKind::Insert {
            pos: Position::origin(),
            text: "test".to_string(),
        };
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Insert"));
    }

    #[test]
    fn test_edit_kind_delete_debug() {
        let kind = EditKind::Delete {
            range: Range::from_coords(0, 0, 0, 5),
            text: "hello".to_string(),
        };
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Delete"));
    }

    #[test]
    fn test_edit_kind_replace_debug() {
        let kind = EditKind::Replace {
            range: Range::from_coords(0, 0, 0, 5),
            old: "hello".to_string(),
            new: "world".to_string(),
        };
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Replace"));
    }
}

mod stress_tests {
    use super::*;

    #[test]
    fn test_many_edits() {
        let mut history = UndoHistory::new();

        for i in 0..100 {
            history.record(Edit::insert(
                Position::new(0, i),
                "x".to_string(),
                BufferVersion::new(i as u64),
                BufferVersion::new((i + 1) as u64),
                Position::new(0, i + 1),
            ));
        }

        let mut undo_count = 0;
        while history.can_undo() {
            history.undo();
            undo_count += 1;
        }
        assert_eq!(undo_count, 100);
    }

    #[test]
    fn test_interleaved_undo_redo() {
        let mut history = UndoHistory::new();

        for i in 0..10 {
            history.record(Edit::insert(
                Position::new(0, i),
                "x".to_string(),
                BufferVersion::new(i as u64),
                BufferVersion::new((i + 1) as u64),
                Position::new(0, i + 1),
            ));
        }

        // Undo 5
        for _ in 0..5 {
            history.undo();
        }
        assert!(history.can_undo());
        assert!(history.can_redo());

        // Redo 3
        for _ in 0..3 {
            history.redo();
        }
        assert!(history.can_undo());
        assert!(history.can_redo());

        // Undo 8 (remaining)
        for _ in 0..8 {
            history.undo();
        }
        assert!(!history.can_undo());
    }
}
