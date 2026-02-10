//! Tests for cursor operations and a/A/I behavior.
//!
//! Covers spec requirements:
//! - CUR-01: `a` at non-EOL inserts after cursor grapheme
//! - CUR-02: `a` at EOL inserts after final grapheme
//! - CUR-03: `i` at EOL differs from `a` at EOL
//! - CUR-04: `A` moves to line end before Insert
//! - CUR-05: Repeated `a` and `Esc` never leaves floating cursor

#[cfg(test)]
mod tests {
    use crate::ops::cursor_ops::*;
    use kjxlkj_core_edit::CursorPosition;
    use kjxlkj_core_text::TextBuffer;
    use kjxlkj_core_types::BufferId;

    fn make_buf(text: &str) -> TextBuffer {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, text);
        buf
    }

    /// CUR-01: `a` at non-EOL moves cursor one grapheme right.
    #[test]
    fn test_cursor_append_non_eol() {
        let buf = make_buf("hello\n");
        let mut cur = CursorPosition::new(0, 2); // on 'l'
        cursor_append(&mut cur, &buf);
        // Should move to offset 3 (after 'l')
        assert_eq!(cur.grapheme_offset, 3);
    }

    /// CUR-02: `a` at EOL inserts after final grapheme.
    #[test]
    fn test_cursor_append_eol() {
        let buf = make_buf("hello\n");
        // Cursor on last visible grapheme 'o' (offset 4)
        let mut cur = CursorPosition::new(0, 4);
        cursor_append(&mut cur, &buf);
        // Should move to offset 5 = G (after 'o')
        assert_eq!(cur.grapheme_offset, 5);
    }

    /// CUR-03: `i` and `a` differ at EOL.
    #[test]
    fn test_i_differs_from_a_at_eol() {
        let buf = make_buf("hello\n");
        let i_pos = 4_usize; // `i` stays at 4
        let mut a_pos = CursorPosition::new(0, 4);
        cursor_append(&mut a_pos, &buf);
        // `a` moves to 5, `i` stays at 4
        assert_ne!(i_pos, a_pos.grapheme_offset);
        assert_eq!(a_pos.grapheme_offset, 5);
    }

    /// CUR-04: `A` moves to end-of-line and enter Insert.
    #[test]
    fn test_cursor_append_eol_command() {
        let buf = make_buf("hello world\n");
        let mut cur = CursorPosition::new(0, 2);
        cursor_append_eol(&mut cur, &buf);
        // "hello world\n" graphemes include \n
        // A moves to G (after last grapheme), so cursor goes
        // to the total grapheme count of the line
        let line = buf.line(0).unwrap_or_default();
        let g = kjxlkj_core_text::grapheme::line_graphemes(&line).len();
        assert_eq!(cur.grapheme_offset, g);
    }

    /// Test `I` moves to first non-blank.
    #[test]
    fn test_cursor_first_nonblank() {
        let buf = make_buf("    hello\n");
        let mut cur = CursorPosition::new(0, 7);
        cursor_insert_first_nonblank(&mut cur, &buf);
        // 4 spaces, so first non-blank at offset 4
        assert_eq!(cur.grapheme_offset, 4);
    }

    /// CUR-05: Cursor clamp on leaving Insert.
    #[test]
    fn test_cursor_leave_insert_clamp() {
        let buf = make_buf("hello\n");
        let mut cur = CursorPosition::new(0, 5);
        cursor_leave_insert(&mut cur, &buf);
        // Should go back to 4 (last grapheme in Normal)
        assert_eq!(cur.grapheme_offset, 4);
    }

    /// Test scroll_to_cursor respects scrolloff.
    #[test]
    fn test_scroll_to_cursor() {
        let cur = CursorPosition::new(25, 0);
        let mut top = 0;
        scroll_to_cursor(&cur, &mut top, 20, 3);
        // Cursor at 25, height=20, scrolloff=3
        // bottom = top + 19. Need 25 + 3 <= top + 19
        // top >= 28 - 19 = 9
        assert!(top > 0);
        assert!(top + 20 > 25);
    }
}
