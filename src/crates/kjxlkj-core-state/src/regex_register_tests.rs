//! Tests for regex translation and register-aware yank/put.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use crate::regex_translate::{compile_vim_pattern, translate_vim_to_rust};
    use kjxlkj_core_edit::RegisterName;
    use kjxlkj_core_types::Key;

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        let buf = e.buffers.current_mut();
        buf.content = kjxlkj_core_text::Rope::from(text);
        e
    }

    // --- Regex translation tests ---

    #[test]
    fn translate_vim_plus() {
        let r = translate_vim_to_rust(r"foo\+");
        assert_eq!(r.pattern, "foo+");
    }

    #[test]
    fn translate_vim_group() {
        let r = translate_vim_to_rust(r"\(abc\)");
        assert_eq!(r.pattern, "(abc)");
    }

    #[test]
    fn translate_vim_word_boundary() {
        let r = translate_vim_to_rust(r"\<word\>");
        assert_eq!(r.pattern, r"\bword\b");
    }

    #[test]
    fn translate_vim_case_insensitive() {
        let r = translate_vim_to_rust(r"\cfoo");
        assert_eq!(r.case_override, Some(false));
        assert_eq!(r.pattern, "foo");
    }

    #[test]
    fn compile_basic_pattern() {
        let re = compile_vim_pattern("hello", true);
        assert!(re.is_some());
        let re = re.unwrap();
        assert!(re.is_match("hello world"));
        assert!(!re.is_match("HELLO world"));
    }

    #[test]
    fn compile_case_insensitive() {
        let re = compile_vim_pattern("hello", false);
        assert!(re.is_some());
        let re = re.unwrap();
        assert!(re.is_match("HELLO world"));
    }

    // --- Register-aware yank/put tests ---

    #[test]
    fn yank_to_named_register() {
        let mut e = editor_with("hello\nworld\n");
        // "ayy — yank line into register 'a'
        e.handle_key(Key::char('"'));
        e.handle_key(Key::char('a'));
        // Now pending_register is Some('a')
        assert_eq!(e.pending_register, Some('a'));
        // Yank a line
        e.yank_lines(1);
        // Register 'a' should have content
        let reg = e.registers.get(RegisterName::Named('a'));
        assert!(reg.is_some());
        assert!(reg.unwrap().content.contains("hello"));
    }

    #[test]
    fn put_from_named_register() {
        let mut e = editor_with("hello\nworld\n");
        // Yank into register 'b'
        e.pending_register = Some('b');
        e.yank_lines(1);
        // Move to line 2
        e.move_cursor_down(1);
        // Put from register 'b'
        e.pending_register = Some('b');
        e.put_after();
        let buf = e.buffers.current();
        assert!(buf.content.len_lines() > 2);
    }

    #[test]
    fn delete_stores_in_named_register() {
        let mut e = editor_with("line1\nline2\nline3\n");
        e.pending_register = Some('c');
        e.delete_lines(1);
        let reg = e.registers.get(RegisterName::Named('c'));
        assert!(reg.is_some());
        assert!(reg.unwrap().content.contains("line1"));
    }

    #[test]
    fn default_yank_uses_unnamed() {
        let mut e = editor_with("test\n");
        e.yank_lines(1);
        let reg = e.registers.get(RegisterName::Unnamed);
        assert!(reg.is_some());
    }
}
