//! Comprehensive tests for kjxlkj-core-edit.

use kjxlkj_core_edit::*;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Cursor, Position, Range};

mod motion_tests {
    use super::*;

    #[test]
    fn test_motion_up() {
        let buf = TextBuffer::from_str("line1\nline2\nline3");
        let cursor = Cursor::new(2, 0);
        let result = apply_motion(&Motion::Up(1), cursor, &buf, false);
        assert_eq!(result.cursor.line, 1);
    }

    #[test]
    fn test_motion_up_at_top() {
        let buf = TextBuffer::from_str("line1\nline2");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Up(1), cursor, &buf, false);
        assert_eq!(result.cursor.line, 0);
    }

    #[test]
    fn test_motion_up_multiple() {
        let buf = TextBuffer::from_str("l1\nl2\nl3\nl4\nl5");
        let cursor = Cursor::new(4, 0);
        let result = apply_motion(&Motion::Up(3), cursor, &buf, false);
        assert_eq!(result.cursor.line, 1);
    }

    #[test]
    fn test_motion_down() {
        let buf = TextBuffer::from_str("line1\nline2\nline3");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Down(1), cursor, &buf, false);
        assert_eq!(result.cursor.line, 1);
    }

    #[test]
    fn test_motion_down_at_bottom() {
        let buf = TextBuffer::from_str("line1\nline2");
        let cursor = Cursor::new(1, 0);
        let result = apply_motion(&Motion::Down(1), cursor, &buf, false);
        assert_eq!(result.cursor.line, 1);
    }

    #[test]
    fn test_motion_left() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 3);
        let result = apply_motion(&Motion::Left(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 2);
    }

    #[test]
    fn test_motion_left_at_start() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Left(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_right() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 2);
        let result = apply_motion(&Motion::Right(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 3);
    }

    #[test]
    fn test_motion_right_at_end_exclusive() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 4);
        let result = apply_motion(&Motion::Right(1), cursor, &buf, false);
        // In end-exclusive mode (normal), can't go past last char
        assert_eq!(result.cursor.column, 4);
    }

    #[test]
    fn test_motion_right_at_end_inclusive() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 4);
        let result = apply_motion(&Motion::Right(1), cursor, &buf, true);
        // In end-inclusive mode (insert), can go to column 5
        assert_eq!(result.cursor.column, 5);
    }

    #[test]
    fn test_motion_line_start() {
        let buf = TextBuffer::from_str("  hello");
        let cursor = Cursor::new(0, 5);
        let result = apply_motion(&Motion::LineStart, cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_first_non_blank() {
        let buf = TextBuffer::from_str("  hello");
        let cursor = Cursor::new(0, 5);
        let result = apply_motion(&Motion::FirstNonBlank, cursor, &buf, false);
        assert_eq!(result.cursor.column, 2);
    }

    #[test]
    fn test_motion_line_end() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::LineEnd, cursor, &buf, false);
        // In end-exclusive, should be at last char (4)
        assert_eq!(result.cursor.column, 4);
    }

    #[test]
    fn test_motion_document_start() {
        let buf = TextBuffer::from_str("line1\nline2\nline3");
        let cursor = Cursor::new(2, 3);
        let result = apply_motion(&Motion::DocumentStart, cursor, &buf, false);
        assert_eq!(result.cursor.line, 0);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_document_end() {
        let buf = TextBuffer::from_str("line1\nline2\nline3");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::DocumentEnd, cursor, &buf, false);
        assert_eq!(result.cursor.line, 2);
    }

    #[test]
    fn test_motion_line() {
        let buf = TextBuffer::from_str("l1\nl2\nl3\nl4\nl5");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Line(2), cursor, &buf, false);
        // Line 3 is index 2
        assert_eq!(result.cursor.line, 2);
    }

    #[test]
    fn test_motion_word_forward() {
        let buf = TextBuffer::from_str("hello world test");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::WordForward(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 6); // Start of "world"
    }

    #[test]
    fn test_motion_word_forward_multiple() {
        let buf = TextBuffer::from_str("one two three");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::WordForward(2), cursor, &buf, false);
        assert_eq!(result.cursor.column, 8); // Start of "three"
    }

    #[test]
    fn test_motion_word_backward() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 8);
        let result = apply_motion(&Motion::WordBackward(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 6); // Start of "world"
    }

    #[test]
    fn test_motion_word_end() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::WordEnd(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 4); // End of "hello"
    }

    #[test]
    fn test_motion_big_word_forward() {
        let buf = TextBuffer::from_str("hello-world test");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::BigWordForward(1), cursor, &buf, false);
        // "hello-world" is one WORD
        assert_eq!(result.cursor.column, 12); // Start of "test"
    }

    #[test]
    fn test_motion_find_char() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::FindChar { char: 'o', forward: true, till: false }, cursor, &buf, false);
        assert_eq!(result.cursor.column, 4); // First 'o'
    }

    #[test]
    fn test_motion_find_char_not_found() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::FindChar { char: 'z', forward: true, till: false }, cursor, &buf, false);
        assert_eq!(result.cursor.column, 0); // Unchanged
    }

    #[test]
    fn test_motion_till_char() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::FindChar { char: 'o', forward: true, till: true }, cursor, &buf, false);
        assert_eq!(result.cursor.column, 3); // One before 'o'
    }
}

mod text_object_tests {
    use super::*;

    #[test]
    fn test_text_object_inner_word() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 7); // In "world"
        let range = find_text_object(&TextObject::Word, TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.column, 6);
        assert_eq!(r.end.column, 11);
    }

    #[test]
    fn test_text_object_around_word() {
        let buf = TextBuffer::from_str("hello world test");
        let cursor = Cursor::new(0, 7);
        let range = find_text_object(&TextObject::Word, TextObjectKind::Around, cursor, &buf);
        assert!(range.is_some());
        // Should include trailing space
    }

    #[test]
    fn test_text_object_inner_paren() {
        let buf = TextBuffer::from_str("func(arg)");
        let cursor = Cursor::new(0, 6);
        let range = find_text_object(&TextObject::Bracket('(', ')'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_around_paren() {
        let buf = TextBuffer::from_str("func(arg)");
        let cursor = Cursor::new(0, 6);
        let range = find_text_object(&TextObject::Bracket('(', ')'), TextObjectKind::Around, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_inner_bracket() {
        let buf = TextBuffer::from_str("arr[idx]");
        let cursor = Cursor::new(0, 5);
        let range = find_text_object(&TextObject::Bracket('[', ']'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_inner_brace() {
        let buf = TextBuffer::from_str("{ code }");
        let cursor = Cursor::new(0, 3);
        let range = find_text_object(&TextObject::Bracket('{', '}'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_inner_double_quote() {
        let buf = TextBuffer::from_str("say \"hello\"");
        let cursor = Cursor::new(0, 6);
        let range = find_text_object(&TextObject::Quote('"'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.column, 5);
        assert_eq!(r.end.column, 10);
    }

    #[test]
    fn test_text_object_inner_single_quote() {
        let buf = TextBuffer::from_str("say 'hello'");
        let cursor = Cursor::new(0, 6);
        let range = find_text_object(&TextObject::Quote('\''), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_inner_paragraph() {
        let buf = TextBuffer::from_str("para1\npara1\n\npara2");
        let cursor = Cursor::new(0, 0);
        let range = find_text_object(&TextObject::Paragraph, TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_no_match() {
        let buf = TextBuffer::from_str("no quotes here");
        let cursor = Cursor::new(0, 5);
        let range = find_text_object(&TextObject::Quote('"'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_none());
    }
}

mod operator_tests {
    use super::*;
    use kjxlkj_core_undo::UndoHistory;

    #[test]
    fn test_operator_delete() {
        let mut buf = TextBuffer::from_str("hello world");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 6);
        let result = apply_operator(Operator::Delete, range, &mut buf, &mut history, &mut register);
        assert!(result.is_some());
        assert_eq!(buf.line(0).unwrap(), "world");
    }

    #[test]
    fn test_operator_yank() {
        let mut buf = TextBuffer::from_str("hello world");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::Yank, range, &mut buf, &mut history, &mut register);
        assert_eq!(register, "hello");
        // Buffer should be unchanged
        assert_eq!(buf.line(0).unwrap(), "hello world");
    }

    #[test]
    fn test_operator_change() {
        let mut buf = TextBuffer::from_str("hello world");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::Change, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), " world");
    }

    #[test]
    fn test_operator_uppercase() {
        let mut buf = TextBuffer::from_str("hello");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::Uppercase, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), "HELLO");
    }

    #[test]
    fn test_operator_lowercase() {
        let mut buf = TextBuffer::from_str("HELLO");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::Lowercase, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_operator_toggle_case() {
        let mut buf = TextBuffer::from_str("HeLLo");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::ToggleCase, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), "hEllO");
    }

    #[test]
    fn test_operator_delete_multiline() {
        let mut buf = TextBuffer::from_str("line1\nline2\nline3");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 2, 0);
        let _result = apply_operator(Operator::Delete, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.line(0).unwrap(), "line3");
    }
}

mod motion_enum_tests {
    use super::*;

    #[test]
    fn test_motion_debug() {
        let m = Motion::Up(5);
        let debug = format!("{:?}", m);
        assert!(debug.contains("Up"));
    }

    #[test]
    fn test_motion_clone() {
        let m1 = Motion::Right(3);
        let m2 = m1.clone();
        assert!(matches!(m2, Motion::Right(3)));
    }

    #[test]
    fn test_motion_equality() {
        assert_eq!(Motion::Up(1), Motion::Up(1));
        assert_ne!(Motion::Up(1), Motion::Up(2));
        assert_ne!(Motion::Up(1), Motion::Down(1));
    }
}

mod text_object_enum_tests {
    use super::*;

    #[test]
    fn test_text_object_debug() {
        let to = TextObject::Word;
        let debug = format!("{:?}", to);
        assert!(debug.contains("Word"));
    }

    #[test]
    fn test_text_object_clone() {
        let to1 = TextObject::Word;
        let to2 = to1.clone();
        assert!(matches!(to2, TextObject::Word));
    }
}

mod operator_enum_tests {
    use super::*;

    #[test]
    fn test_operator_debug() {
        let op = Operator::Delete;
        let debug = format!("{:?}", op);
        assert!(debug.contains("Delete"));
    }

    #[test]
    fn test_operator_clone() {
        let op1 = Operator::Yank;
        let op2 = op1.clone();
        assert!(matches!(op2, Operator::Yank));
    }

    #[test]
    fn test_operator_equality() {
        assert_eq!(Operator::Delete, Operator::Delete);
        assert_ne!(Operator::Delete, Operator::Yank);
    }
}

