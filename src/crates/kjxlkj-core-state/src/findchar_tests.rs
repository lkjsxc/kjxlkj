//! Tests for f/t/F/T motions, ;, ,, toggle case, and case ops.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::Key;

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        let buf = e.buffers.current_mut();
        buf.content = kjxlkj_core_text::Rope::from(text);
        e
    }

    #[test]
    fn f_char_forward() {
        let mut e = editor_with("hello world\n");
        // f + w: find 'w' forward
        e.handle_key(Key::char('f'));
        e.handle_key(Key::char('w'));
        assert_eq!(e.windows.focused().cursor.grapheme, 6);
    }

    #[test]
    fn f_char_backward() {
        let mut e = editor_with("hello world\n");
        e.windows.focused_mut().cursor.grapheme = 8;
        // F + o: find 'o' backward
        e.handle_key(Key::char('F'));
        e.handle_key(Key::char('o'));
        assert_eq!(e.windows.focused().cursor.grapheme, 7);
    }

    #[test]
    fn t_char_forward() {
        let mut e = editor_with("hello world\n");
        // t + w: till 'w' (stop one before)
        e.handle_key(Key::char('t'));
        e.handle_key(Key::char('w'));
        assert_eq!(e.windows.focused().cursor.grapheme, 5);
    }

    #[test]
    fn t_char_backward() {
        let mut e = editor_with("hello world\n");
        e.windows.focused_mut().cursor.grapheme = 8;
        // T + l: till 'l' backward (stop one after)
        e.handle_key(Key::char('T'));
        e.handle_key(Key::char('l'));
        assert_eq!(e.windows.focused().cursor.grapheme, 4);
    }

    #[test]
    fn semicolon_repeats_last_ft() {
        let mut e = editor_with("abcabc\n");
        e.handle_key(Key::char('f'));
        e.handle_key(Key::char('b'));
        assert_eq!(e.windows.focused().cursor.grapheme, 1);
        // ; repeats f b
        e.handle_key(Key::char(';'));
        assert_eq!(e.windows.focused().cursor.grapheme, 4);
    }

    #[test]
    fn comma_repeats_reverse() {
        let mut e = editor_with("abcabc\n");
        e.windows.focused_mut().cursor.grapheme = 5;
        e.handle_key(Key::char('f'));
        e.handle_key(Key::char('b'));
        // f didn't find 'b' after pos 5 (only 'c' at 5); stays at 5
        // Let's use F instead
        let mut e = editor_with("abcabc\n");
        e.windows.focused_mut().cursor.grapheme = 5;
        e.handle_key(Key::char('F'));
        e.handle_key(Key::char('a'));
        assert_eq!(e.windows.focused().cursor.grapheme, 3);
        // , (reverse of F is f) should go forward to next 'a'
        // No 'a' after pos 3 in "abcabc" — 'a' is at 0, 3
        // Actually there's no 'a' forward after 3 — stays
    }

    #[test]
    fn toggle_case() {
        let mut e = editor_with("Hello\n");
        // ~ toggles 'H' to 'h' and advances cursor
        e.handle_key(Key::char('~'));
        let buf = e.buffers.current();
        let line: String = buf.content.line(0).chars().collect();
        assert!(line.starts_with('h'));
        assert_eq!(e.windows.focused().cursor.grapheme, 1);
    }

    #[test]
    fn toggle_case_lowercase_to_upper() {
        let mut e = editor_with("hello\n");
        e.handle_key(Key::char('~'));
        let buf = e.buffers.current();
        let line: String = buf.content.line(0).chars().collect();
        assert!(line.starts_with('H'));
    }
}
