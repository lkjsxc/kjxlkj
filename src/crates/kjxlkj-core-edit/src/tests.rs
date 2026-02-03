//! Tests for edit module.

use super::*;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Cursor, CursorDirection, Position};

mod motion_tests {
    use super::*;

    #[test]
    fn test_move_left_right() {
        let buf = TextBuffer::from_text("hello");
        let mut cursor = Cursor::new(0, 2);

        execute_motion(&buf, &mut cursor, CursorDirection::Left, 1);
        assert_eq!(cursor.col(), 1);

        execute_motion(&buf, &mut cursor, CursorDirection::Right, 1);
        assert_eq!(cursor.col(), 2);
    }

    #[test]
    fn test_move_up_down() {
        let buf = TextBuffer::from_text("line1\nline2\nline3");
        let mut cursor = Cursor::new(1, 2);

        execute_motion(&buf, &mut cursor, CursorDirection::Up, 1);
        assert_eq!(cursor.line(), 0);

        execute_motion(&buf, &mut cursor, CursorDirection::Down, 2);
        assert_eq!(cursor.line(), 2);
    }

    #[test]
    fn test_move_with_count() {
        let buf = TextBuffer::from_text("hello world");
        let mut cursor = Cursor::new(0, 0);

        execute_motion(&buf, &mut cursor, CursorDirection::Right, 3);
        assert_eq!(cursor.col(), 3);
    }

    #[test]
    fn test_line_end_start() {
        let buf = TextBuffer::from_text("hello");
        let mut cursor = Cursor::new(0, 2);

        move_to_line_end(&buf, &mut cursor);
        assert_eq!(cursor.col(), 4);

        move_to_line_start(&mut cursor);
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn test_first_non_blank() {
        let buf = TextBuffer::from_text("   hello");
        let mut cursor = Cursor::new(0, 0);

        move_to_first_non_blank(&buf, &mut cursor);
        assert_eq!(cursor.col(), 3);
    }
}

mod operator_tests {
    use super::*;

    #[test]
    fn test_delete_char() {
        let mut buf = TextBuffer::from_text("hello");
        let cursor = Cursor::new(0, 2);

        let deleted = delete_char(&mut buf, &cursor).unwrap();
        assert_eq!(deleted, "l");
        assert_eq!(buf.line(0), Some("helo".to_string()));
    }

    #[test]
    fn test_delete_line() {
        let mut buf = TextBuffer::from_text("line1\nline2\nline3");
        let cursor = Cursor::new(1, 0);

        let content = delete_line(&mut buf, &cursor).unwrap();
        assert!(content.text.contains("line2"));
        assert_eq!(buf.line_count(), 2);
    }

    #[test]
    fn test_insert_text() {
        let mut buf = TextBuffer::from_text("hllo");
        let mut cursor = Cursor::new(0, 1);

        insert_text(&mut buf, &mut cursor, "e").unwrap();
        assert_eq!(buf.line(0), Some("hello".to_string()));
        assert_eq!(cursor.col(), 2);
    }

    #[test]
    fn test_delete_backward() {
        let mut buf = TextBuffer::from_text("hello");
        let mut cursor = Cursor::new(0, 3);

        delete_backward(&mut buf, &mut cursor).unwrap();
        assert_eq!(buf.line(0), Some("helo".to_string()));
        assert_eq!(cursor.col(), 2);
    }

    #[test]
    fn test_yank_line() {
        let buf = TextBuffer::from_text("hello\nworld");
        let cursor = Cursor::new(0, 0);

        let content = yank_line(&buf, &cursor).unwrap();
        assert_eq!(content.text, "hello\n");
    }
}

mod transaction_tests {
    use super::*;

    #[test]
    fn test_edit_op_inverse() {
        let insert = EditOp::Insert {
            pos: Position::new(0, 0),
            text: "hello".to_string(),
        };
        let inverse = insert.inverse();

        match inverse {
            EditOp::Delete { range, deleted_text } => {
                assert_eq!(range.start, Position::new(0, 0));
                assert_eq!(deleted_text, "hello");
            }
            _ => panic!("Expected Delete"),
        }
    }

    #[test]
    fn test_transaction_inverse() {
        use kjxlkj_core_types::BufferVersion;

        let mut tx = Transaction::new(BufferVersion::new(0), Position::new(0, 0));
        tx.push(EditOp::Insert {
            pos: Position::new(0, 0),
            text: "a".to_string(),
        });
        tx.push(EditOp::Insert {
            pos: Position::new(0, 1),
            text: "b".to_string(),
        });

        let inverse = tx.inverse();
        assert_eq!(inverse.ops.len(), 2);
        // Inverse should be in reverse order
        match &inverse.ops[0] {
            EditOp::Delete { .. } => {}
            _ => panic!("Expected Delete"),
        }
    }
}
