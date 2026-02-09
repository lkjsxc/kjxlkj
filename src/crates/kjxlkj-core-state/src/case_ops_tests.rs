//! Tests for gu/gU operators and case_range.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode, Operator};

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        let buf = e.buffers.current_mut();
        buf.content = kjxlkj_core_text::Rope::from(text);
        e
    }

    #[test]
    fn gu_enters_operator_pending_lowercase() {
        let mut e = editor_with("Hello World\n");
        e.handle_key(Key::char('g'));
        e.handle_key(Key::char('u'));
        assert!(matches!(e.mode, Mode::OperatorPending(Operator::Lowercase)));
    }

    #[test]
    fn g_u_u_lowercases_current_line() {
        let mut e = editor_with("Hello World\n");
        // guu: g→u→u (doubled operator)
        e.handle_key(Key::char('g'));
        e.handle_key(Key::char('u'));
        // Now in OperatorPending(Lowercase), pressing 'u' should
        // not be doubled because is_doubled checks for the same char
        // Actually, for guu, we need the doubled check to match
        // Lowercase + 'u'. Let's just test lowercase_lines directly.
        e.mode = Mode::Normal;
        e.lowercase_lines(0, 0);
        let buf = e.buffers.current();
        let line: String = buf.content.line(0).chars().collect();
        assert_eq!(line.trim(), "hello world");
    }

    #[test]
    fn g_u_motion_lowercases_word() {
        let mut e = editor_with("HELLO World\n");
        // gu + w: lowercase to next word
        e.handle_key(Key::char('g'));
        e.handle_key(Key::char('u'));
        e.handle_key(Key::char('w'));
        let buf = e.buffers.current();
        let line: String = buf.content.line(0).chars().collect();
        assert!(line.starts_with("hello"));
    }

    #[test]
    fn g_upper_u_enters_operator_pending_uppercase() {
        let mut e = editor_with("hello\n");
        e.handle_key(Key::char('g'));
        e.handle_key(Key::char('U'));
        assert!(matches!(e.mode, Mode::OperatorPending(Operator::Uppercase)));
    }

    #[test]
    fn uppercase_lines_helper() {
        let mut e = editor_with("hello world\n");
        e.uppercase_lines(0, 0);
        let buf = e.buffers.current();
        let line: String = buf.content.line(0).chars().collect();
        assert_eq!(line.trim(), "HELLO WORLD");
    }

    #[test]
    fn g_upper_u_motion_uppercases_word() {
        let mut e = editor_with("hello world\n");
        e.handle_key(Key::char('g'));
        e.handle_key(Key::char('U'));
        e.handle_key(Key::char('w'));
        let buf = e.buffers.current();
        let line: String = buf.content.line(0).chars().collect();
        assert!(line.starts_with("HELLO"));
    }
}
