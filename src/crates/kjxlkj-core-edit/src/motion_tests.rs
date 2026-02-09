//! Tests for motion execution.

#[cfg(test)]
mod tests {
    use kjxlkj_core_text::BufferContent;
    use kjxlkj_core_types::Motion;

    use crate::cursor::CursorPosition;
    use crate::motion_exec::execute_motion;

    #[test]
    fn move_left_right() {
        let content = BufferContent::from_str("hello\n");
        let mut cursor = CursorPosition::new(0, 2);
        execute_motion(&mut cursor, &Motion::Left, 1, &content);
        assert_eq!(cursor.grapheme_offset, 1);
        execute_motion(&mut cursor, &Motion::Right, 1, &content);
        assert_eq!(cursor.grapheme_offset, 2);
    }

    #[test]
    fn move_down_up() {
        let content = BufferContent::from_str("abc\ndef\n");
        let mut cursor = CursorPosition::new(0, 0);
        execute_motion(&mut cursor, &Motion::Down, 1, &content);
        assert_eq!(cursor.line, 1);
        execute_motion(&mut cursor, &Motion::Up, 1, &content);
        assert_eq!(cursor.line, 0);
    }

    #[test]
    fn goto_lines() {
        let content = BufferContent::from_str("a\nb\nc\nd\n");
        let mut cursor = CursorPosition::new(0, 0);
        execute_motion(&mut cursor, &Motion::GotoLastLine, 1, &content);
        assert!(cursor.line >= 3);
    }

    #[test]
    fn find_char_forward() {
        let content = BufferContent::from_str("hello world\n");
        let mut cursor = CursorPosition::new(0, 0);
        execute_motion(&mut cursor, &Motion::FindCharForward('o'), 1, &content);
        assert_eq!(cursor.grapheme_offset, 4);
    }

    #[test]
    fn find_char_backward() {
        let content = BufferContent::from_str("hello world\n");
        let mut cursor = CursorPosition::new(0, 7);
        execute_motion(&mut cursor, &Motion::FindCharBackward('l'), 1, &content);
        assert_eq!(cursor.grapheme_offset, 3);
    }

    #[test]
    fn matching_bracket() {
        let content = BufferContent::from_str("(hello)\n");
        let mut cursor = CursorPosition::new(0, 0);
        execute_motion(&mut cursor, &Motion::MatchingBracket, 1, &content);
        assert_eq!(cursor.grapheme_offset, 6);
    }

    #[test]
    fn star_forward() {
        let content = BufferContent::from_str("foo bar\nfoo baz\n");
        let mut cursor = CursorPosition::new(0, 0);
        execute_motion(&mut cursor, &Motion::StarForward, 1, &content);
        assert_eq!(cursor.line, 1);
        assert_eq!(cursor.grapheme_offset, 0);
    }

    #[test]
    fn screen_positions() {
        let content = BufferContent::from_str("a\nb\nc\nd\ne\n");
        let mut cursor = CursorPosition::new(0, 0);
        execute_motion(&mut cursor, &Motion::ScreenBottom, 1, &content);
        // Should go to last line
        assert!(cursor.line >= 4);
    }
}
