//! Tests for operator-pending mode dispatch.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode, Operator};

    fn editor_with_text(text: &str) -> EditorState {
        let mut ed = EditorState::new(80, 24);
        ed.open_file("test.txt", text);
        ed
    }

    #[test]
    fn test_dd_deletes_line() {
        let mut ed = editor_with_text("aaa\nbbb\nccc\n");
        ed.handle_key(Key::char('d'));
        assert!(matches!(ed.mode, Mode::OperatorPending(Operator::Delete)));
        ed.handle_key(Key::char('d'));
        assert_eq!(ed.mode, Mode::Normal);
        let text = ed.buffers.current().content.to_string();
        assert!(!text.starts_with("aaa"));
    }

    #[test]
    fn test_yy_yanks_line() {
        let mut ed = editor_with_text("hello\nworld\n");
        ed.handle_key(Key::char('y'));
        assert!(matches!(ed.mode, Mode::OperatorPending(Operator::Yank)));
        ed.handle_key(Key::char('y'));
        assert_eq!(ed.mode, Mode::Normal);
        let reg = ed.registers.get_unnamed();
        assert!(reg.is_some());
    }

    #[test]
    fn test_dw_deletes_word() {
        let mut ed = editor_with_text("hello world\n");
        ed.handle_key(Key::char('d'));
        ed.handle_key(Key::char('w'));
        assert_eq!(ed.mode, Mode::Normal);
        let text = ed.buffers.current().content.to_string();
        assert!(text.starts_with("world"));
    }

    #[test]
    fn test_d_dollar_deletes_to_eol() {
        let mut ed = editor_with_text("abcdef\nline2\n");
        ed.handle_key(Key::char('l')); // move to col 1
        ed.handle_key(Key::char('d'));
        ed.handle_key(Key::char('$'));
        assert_eq!(ed.mode, Mode::Normal);
        let text = ed.buffers.current().content.to_string();
        assert!(text.starts_with('a'));
    }

    #[test]
    fn test_operator_pending_esc_cancels() {
        let mut ed = editor_with_text("hello\n");
        ed.handle_key(Key::char('d'));
        assert!(matches!(ed.mode, Mode::OperatorPending(Operator::Delete)));
        ed.handle_key(Key::esc());
        assert_eq!(ed.mode, Mode::Normal);
    }

    #[test]
    fn test_dgg_deletes_to_top() {
        let mut ed = editor_with_text("line1\nline2\nline3\n");
        // Move to line 2
        ed.handle_key(Key::char('j'));
        ed.handle_key(Key::char('j'));
        ed.handle_key(Key::char('d'));
        ed.handle_key(Key::char('g'));
        ed.handle_key(Key::char('g'));
        assert_eq!(ed.mode, Mode::Normal);
    }

    #[test]
    fn test_cc_changes_line() {
        let mut ed = editor_with_text("hello\nworld\n");
        ed.handle_key(Key::char('c'));
        ed.handle_key(Key::char('c'));
        assert!(matches!(ed.mode, Mode::Insert));
    }

    #[test]
    fn test_count_operator_motion() {
        let mut ed = editor_with_text("one two three four\n");
        // 2dw — delete 2 words
        ed.handle_key(Key::char('2'));
        ed.handle_key(Key::char('d'));
        ed.handle_key(Key::char('w'));
        assert_eq!(ed.mode, Mode::Normal);
        let text = ed.buffers.current().content.to_string();
        assert!(text.starts_with("three"));
    }
}
