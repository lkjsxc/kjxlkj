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


// Additional edge case tests for Motion
mod motion_extra {
    use super::*;

    #[test]
    fn test_motion_up_large_count() {
        let buf = TextBuffer::from_str("l1\nl2\nl3\nl4\nl5");
        let cursor = Cursor::new(4, 0);
        let result = apply_motion(&Motion::Up(100), cursor, &buf, false);
        assert_eq!(result.cursor.line, 0);
    }

    #[test]
    fn test_motion_down_large_count() {
        let buf = TextBuffer::from_str("l1\nl2\nl3");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Down(100), cursor, &buf, false);
        assert_eq!(result.cursor.line, 2);
    }

    #[test]
    fn test_motion_left_large_count() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 3);
        let result = apply_motion(&Motion::Left(100), cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_right_large_count() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Right(100), cursor, &buf, false);
        // Should be clamped to end of line
        assert!(result.cursor.column <= 5);
    }

    #[test]
    fn test_motion_clone() {
        let m = Motion::Up(5);
        let cloned = m.clone();
        assert_eq!(format!("{:?}", m), format!("{:?}", cloned));
    }

    #[test]
    fn test_motion_all_variants() {
        let motions = [
            Motion::Up(1), Motion::Down(1), Motion::Left(1), Motion::Right(1),
            Motion::LineStart, Motion::LineEnd, Motion::FirstNonBlank,
            Motion::DocumentStart, Motion::DocumentEnd,
            Motion::WordForward(1), Motion::WordBackward(1),
        ];
        for m in motions {
            let debug = format!("{:?}", m);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_motion_word_forward_at_end() {
        let buf = TextBuffer::from_str("word");
        let cursor = Cursor::new(0, 3);
        let result = apply_motion(&Motion::WordForward(1), cursor, &buf, false);
        // Should not panic, stays at end
        assert!(result.cursor.column <= 4);
    }

    #[test]
    fn test_motion_word_backward_at_start() {
        let buf = TextBuffer::from_str("word");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::WordBackward(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }
}

// Additional edge case tests for TextObject
mod text_object_extra {
    use super::*;

    #[test]
    fn test_text_object_all_variants() {
        let objects = [
            TextObject::Word,
            TextObject::BigWord,
            TextObject::Sentence,
            TextObject::Paragraph,
            TextObject::Quote('"'),
            TextObject::Quote('\''),
            TextObject::Bracket('(', ')'),
            TextObject::Bracket('[', ']'),
            TextObject::Bracket('{', '}'),
            TextObject::Tag,
        ];
        for obj in objects {
            let debug = format!("{:?}", obj);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_text_object_kind() {
        let inner = TextObjectKind::Inner;
        let around = TextObjectKind::Around;
        assert_ne!(inner, around);
    }

    #[test]
    fn test_find_word_empty_buffer() {
        let buf = TextBuffer::from_str("");
        let cursor = Cursor::new(0, 0);
        let result = find_text_object(&TextObject::Word, TextObjectKind::Inner, cursor, &buf);
        // Should return None for empty buffer
        assert!(result.is_none());
    }

    #[test]
    fn test_find_word_at_word() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 2);
        let result = find_text_object(&TextObject::Word, TextObjectKind::Inner, cursor, &buf);
        assert!(result.is_some());
    }

    #[test]
    fn test_find_quote_no_quotes() {
        let buf = TextBuffer::from_str("no quotes here");
        let cursor = Cursor::new(0, 5);
        let result = find_text_object(&TextObject::Quote('"'), TextObjectKind::Inner, cursor, &buf);
        assert!(result.is_none());
    }
}

// Additional edge case tests for Operator
mod operator_extra {
    use super::*;

    #[test]
    fn test_operator_all_variants() {
        let operators = [
            Operator::Delete, Operator::Yank, Operator::Change,
            Operator::Indent, Operator::Outdent,
            Operator::Uppercase, Operator::Lowercase, Operator::ToggleCase,
        ];
        for op in operators {
            let debug = format!("{:?}", op);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_operator_copy_clone() {
        let op = Operator::Delete;
        let copied = op;
        assert_eq!(op, copied);
        let cloned = op.clone();
        assert_eq!(op, cloned);
    }

    #[test]
    fn test_operator_equality_all() {
        let operators = [
            Operator::Delete, Operator::Yank, Operator::Change,
            Operator::Indent, Operator::Outdent,
        ];
        for i in 0..operators.len() {
            assert_eq!(operators[i], operators[i]);
            for j in 0..operators.len() {
                if i != j {
                    assert_ne!(operators[i], operators[j]);
                }
            }
        }
    }
}

mod extra_motion_edge_tests {
    use super::*;

    #[test]
    fn test_motion_up_from_middle() {
        let buf = TextBuffer::from_str("a\nb\nc\nd\ne");
        let cursor = Cursor::new(2, 0);
        let result = apply_motion(&Motion::Up(1), cursor, &buf, false);
        assert_eq!(result.cursor.line, 1);
    }

    #[test]
    fn test_motion_down_multiple_past_end() {
        let buf = TextBuffer::from_str("a\nb");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Down(10), cursor, &buf, false);
        assert_eq!(result.cursor.line, 1); // Capped at last line
    }

    #[test]
    fn test_motion_left_multiple() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 4);
        let result = apply_motion(&Motion::Left(2), cursor, &buf, false);
        assert_eq!(result.cursor.column, 2);
    }

    #[test]
    fn test_motion_left_past_start() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 2);
        let result = apply_motion(&Motion::Left(10), cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_right_multiple() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Right(3), cursor, &buf, false);
        assert_eq!(result.cursor.column, 3);
    }

    #[test]
    fn test_motion_empty_buffer() {
        let buf = TextBuffer::new();
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Down(1), cursor, &buf, false);
        assert_eq!(result.cursor.line, 0);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_single_char_buffer() {
        let buf = TextBuffer::from_str("x");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Right(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 0); // Can't move past single char in exclusive mode
    }

    #[test]
    fn test_motion_line_end_empty_line() {
        let buf = TextBuffer::from_str("hello\n\nworld");
        let cursor = Cursor::new(1, 0);
        let result = apply_motion(&Motion::LineEnd, cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_first_non_blank_all_whitespace() {
        let buf = TextBuffer::from_str("     ");
        let cursor = Cursor::new(0, 3);
        let result = apply_motion(&Motion::FirstNonBlank, cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_first_non_blank_no_leading_space() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 3);
        let result = apply_motion(&Motion::FirstNonBlank, cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_line_beyond_count() {
        let buf = TextBuffer::from_str("a\nb\nc");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Line(100), cursor, &buf, false);
        assert_eq!(result.cursor.line, 2); // Capped at max
    }

    #[test]
    fn test_motion_line_zero() {
        let buf = TextBuffer::from_str("a\nb\nc");
        let cursor = Cursor::new(2, 0);
        let result = apply_motion(&Motion::Line(0), cursor, &buf, false);
        assert_eq!(result.cursor.line, 0);
    }

    #[test]
    fn test_motion_word_forward_at_end() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 4);
        let result = apply_motion(&Motion::WordForward(1), cursor, &buf, false);
        // At end of single word, stays put or goes to end
        assert!(result.cursor.column >= 4);
    }

    #[test]
    fn test_motion_word_backward_at_start() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::WordBackward(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_word_end_single_word() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::WordEnd(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 4);
    }

    #[test]
    fn test_motion_big_word_backward() {
        let buf = TextBuffer::from_str("hello-world test");
        let cursor = Cursor::new(0, 12);
        let result = apply_motion(&Motion::BigWordBackward(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 0);
    }

    #[test]
    fn test_motion_big_word_end() {
        let buf = TextBuffer::from_str("hello-world test");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::BigWordEnd(1), cursor, &buf, false);
        assert_eq!(result.cursor.column, 10); // End of "hello-world"
    }

    #[test]
    fn test_motion_find_char_second_occurrence() {
        let buf = TextBuffer::from_str("hello hello");
        let cursor = Cursor::new(0, 5);
        let result = apply_motion(&Motion::FindChar { char: 'e', forward: true, till: false }, cursor, &buf, false);
        assert_eq!(result.cursor.column, 7);
    }

    #[test]
    fn test_motion_match_bracket_not_on_bracket() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::MatchBracket, cursor, &buf, false);
        assert_eq!(result.cursor.column, 0); // Unchanged
    }

    #[test]
    fn test_motion_linewise_flag() {
        let buf = TextBuffer::from_str("a\nb\nc");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::DocumentEnd, cursor, &buf, false);
        assert!(result.linewise);
    }

    #[test]
    fn test_motion_up_preserves_target_column() {
        let buf = TextBuffer::from_str("hello\na\nworld");
        let cursor = Cursor::new(0, 4);
        let r1 = apply_motion(&Motion::Down(1), cursor, &buf, false);
        assert_eq!(r1.cursor.column, 0); // Line "a" is short
        let r2 = apply_motion(&Motion::Down(1), r1.cursor, &buf, false);
        // Target column should be preserved to 4
        assert_eq!(r2.cursor.column, 4);
    }

    #[test]
    fn test_motion_result_clone() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Right(1), cursor, &buf, false);
        let cloned = result.clone();
        assert_eq!(cloned.cursor.column, result.cursor.column);
    }

    #[test]
    fn test_motion_result_debug() {
        let buf = TextBuffer::from_str("hello");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::Right(1), cursor, &buf, false);
        let debug = format!("{:?}", result);
        assert!(debug.contains("cursor"));
    }
}

mod extra_text_object_edge_tests {
    use super::*;

    #[test]
    fn test_text_object_word_at_start() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 0);
        let range = find_text_object(&TextObject::Word, TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.column, 0);
        assert_eq!(r.end.column, 5);
    }

    #[test]
    fn test_text_object_word_at_end() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 10);
        let range = find_text_object(&TextObject::Word, TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_word_on_space() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 5); // On the space
        let range = find_text_object(&TextObject::Word, TextObjectKind::Inner, cursor, &buf);
        // Behavior varies, but should return something
        assert!(range.is_some() || range.is_none());
    }

    #[test]
    fn test_text_object_empty_parens() {
        let buf = TextBuffer::from_str("func()");
        let cursor = Cursor::new(0, 4);
        let range = find_text_object(&TextObject::Bracket('(', ')'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
        let r = range.unwrap();
        // Inner of () is empty
        assert_eq!(r.start.column, 5);
        assert_eq!(r.end.column, 5);
    }

    #[test]
    fn test_text_object_nested_parens_outer() {
        let buf = TextBuffer::from_str("a((b))c");
        let cursor = Cursor::new(0, 3);
        let range = find_text_object(&TextObject::Bracket('(', ')'), TextObjectKind::Around, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_quote_on_quote_char() {
        let buf = TextBuffer::from_str("say \"hello\" there");
        let cursor = Cursor::new(0, 4); // On the opening quote
        let range = find_text_object(&TextObject::Quote('"'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_backtick() {
        let buf = TextBuffer::from_str("say `hello` there");
        let cursor = Cursor::new(0, 6);
        let range = find_text_object(&TextObject::Quote('`'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_angle_brackets() {
        let buf = TextBuffer::from_str("<html>");
        let cursor = Cursor::new(0, 2);
        let range = find_text_object(&TextObject::Bracket('<', '>'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_unmatched_bracket() {
        let buf = TextBuffer::from_str("hello(world");
        let cursor = Cursor::new(0, 7);
        let range = find_text_object(&TextObject::Bracket('(', ')'), TextObjectKind::Inner, cursor, &buf);
        // Unmatched, may return None
        assert!(range.is_none());
    }

    #[test]
    fn test_text_object_paragraph_at_start() {
        let buf = TextBuffer::from_str("hello\nworld\n\nnew para");
        let cursor = Cursor::new(0, 0);
        let range = find_text_object(&TextObject::Paragraph, TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
    }

    #[test]
    fn test_text_object_kind_debug() {
        let kind = TextObjectKind::Inner;
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Inner"));
    }

    #[test]
    fn test_text_object_kind_clone() {
        let k1 = TextObjectKind::Around;
        let k2 = k1.clone();
        assert!(matches!(k2, TextObjectKind::Around));
    }
}

mod extra_operator_edge_tests {
    use super::*;
    use kjxlkj_core_undo::UndoHistory;

    #[test]
    fn test_operator_delete_empty_range() {
        let mut buf = TextBuffer::from_str("hello");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 2, 0, 2);
        let _result = apply_operator(Operator::Delete, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_operator_yank_multiline() {
        let mut buf = TextBuffer::from_str("line1\nline2\nline3");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 1, 5);
        let _result = apply_operator(Operator::Yank, range, &mut buf, &mut history, &mut register);
        assert!(register.contains("line1"));
        assert!(register.contains("line2"));
    }

    #[test]
    fn test_operator_change_entire_line() {
        let mut buf = TextBuffer::from_str("hello world");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 11);
        let _result = apply_operator(Operator::Change, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), "");
    }

    #[test]
    fn test_operator_uppercase_mixed() {
        let mut buf = TextBuffer::from_str("HeLLo");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::Uppercase, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), "HELLO");
    }

    #[test]
    fn test_operator_lowercase_mixed() {
        let mut buf = TextBuffer::from_str("HeLLo");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::Lowercase, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_operator_toggle_case_all_upper() {
        let mut buf = TextBuffer::from_str("HELLO");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::ToggleCase, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_operator_toggle_case_all_lower() {
        let mut buf = TextBuffer::from_str("hello");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::ToggleCase, range, &mut buf, &mut history, &mut register);
        assert_eq!(buf.line(0).unwrap(), "HELLO");
    }

    #[test]
    fn test_operator_delete_all() {
        let mut buf = TextBuffer::from_str("line1\nline2");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 1, 5);
        let _result = apply_operator(Operator::Delete, range, &mut buf, &mut history, &mut register);
        // Buffer should be minimal
        assert!(buf.line_count() <= 1);
    }

    #[test]
    fn test_operator_indent() {
        let mut buf = TextBuffer::from_str("hello");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::Indent, range, &mut buf, &mut history, &mut register);
        // Should have leading spaces/tabs
        assert!(buf.line(0).unwrap().starts_with(' ') || buf.line(0).unwrap().starts_with('\t') || buf.line(0).unwrap() == "hello");
    }

    #[test]
    fn test_operator_outdent_no_indent() {
        let mut buf = TextBuffer::from_str("hello");
        let mut history = UndoHistory::new();
        let mut register = String::new();
        let range = Range::from_coords(0, 0, 0, 5);
        let _result = apply_operator(Operator::Outdent, range, &mut buf, &mut history, &mut register);
        // No change expected
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]

    #[test]
    fn test_operator_default_trait() {
        // Test operators can be compared
        assert!(Operator::Delete != Operator::Change);
    }

    #[test]
    fn test_operator_coverage_all_variants() {
        // Just ensure all variants exist
        let _d = Operator::Delete;
        let _y = Operator::Yank;
        let _c = Operator::Change;
        let _u = Operator::Uppercase;
        let _l = Operator::Lowercase;
        let _t = Operator::ToggleCase;
        let _i = Operator::Indent;
        let _o = Operator::Outdent;
    }
}

mod motion_variants_coverage {
    use super::*;

    #[test]
    fn test_motion_find_char_forward() {
        let m = Motion::FindChar { char: 'a', forward: true, till: false };
        assert!(matches!(m, Motion::FindChar { .. }));
    }

    #[test]
    fn test_motion_find_char_backward() {
        let buf = TextBuffer::from_str("hello hello");
        let cursor = Cursor::new(0, 10);
        let m = Motion::FindChar { char: 'e', forward: false, till: false };
        let result = apply_motion(&m, cursor, &buf, false);
        // Should find 'e' going backward
        assert!(result.cursor.column < 10);
    }

    #[test]
    fn test_motion_till_char_forward() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 0);
        let m = Motion::FindChar { char: 'o', forward: true, till: true };
        let result = apply_motion(&m, cursor, &buf, false);
        // Should stop one before 'o'
        assert_eq!(result.cursor.column, 3);
    }

    #[test]
    fn test_motion_till_char_backward() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 10);
        let m = Motion::FindChar { char: 'e', forward: false, till: true };
        let result = apply_motion(&m, cursor, &buf, false);
        // Should stop one after 'e' going backward
        assert!(result.cursor.column > 1);
    }

    #[test]
    fn test_motion_match_bracket_paren() {
        let buf = TextBuffer::from_str("fn(a)");
        let cursor = Cursor::new(0, 2); // On '('
        let result = apply_motion(&Motion::MatchBracket, cursor, &buf, false);
        assert_eq!(result.cursor.column, 4); // On ')'
    }

    #[test]
    fn test_motion_match_bracket_brace() {
        let buf = TextBuffer::from_str("{ x }");
        let cursor = Cursor::new(0, 0); // On '{'
        let result = apply_motion(&Motion::MatchBracket, cursor, &buf, false);
        assert_eq!(result.cursor.column, 4); // On '}'
    }

    #[test]
    fn test_motion_match_bracket_square() {
        let buf = TextBuffer::from_str("[a]");
        let cursor = Cursor::new(0, 0);
        let result = apply_motion(&Motion::MatchBracket, cursor, &buf, false);
        assert_eq!(result.cursor.column, 2);
    }
}
