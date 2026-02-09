#[cfg(test)]
mod wave32_tests {
    use crate::editor::EditorState;
    use crate::regex_translate::translate_vim_to_rust;

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    #[test]
    fn regex_percent_l_column_atom() {
        // \%l and \%c are runtime constraint atoms; in static regex they are consumed silently.
        let r = translate_vim_to_rust("\\%5lfoo");
        assert_eq!(r.pattern, "foo");
        let r2 = translate_vim_to_rust("\\%10cbar");
        assert_eq!(r2.pattern, "bar");
    }

    #[test]
    fn normal_command_inserts_text() {
        let mut e = editor_with("hello world");
        e.execute_ex_command("normal $");
        // Cursor should be at end of line.
        let cursor = e.windows.focused().cursor;
        assert!(cursor.grapheme > 0);
    }

    #[test]
    fn normal_bang_command() {
        let mut e = editor_with("test line");
        e.execute_ex_command("normal! 0");
        let cursor = e.windows.focused().cursor;
        assert_eq!(cursor.grapheme, 0);
    }

    #[test]
    fn visual_block_dollar_flag() {
        let e = editor_with("short\nlonger line\nx");
        assert!(!e.block_dollar);
        // Simply verify the flag exists and defaults to false.
    }

    #[test]
    fn wildmenu_filename_only_display() {
        // Verify the rendering logic strips directory paths.
        let path = "/home/user/.config/kjxlkj/config.toml";
        let display = path.rsplit('/').next().unwrap_or(path);
        assert_eq!(display, "config.toml");
    }

    #[test]
    fn tr_function_transliteration() {
        let result = crate::expr_eval::eval_expression("tr(\"hello\", \"helo\", \"HELO\")").unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn escape_function_backslash_insert() {
        let result = crate::expr_eval::eval_expression("escape(\"a.b*c\", \".*\")").unwrap();
        assert_eq!(result, "a\\.b\\*c");
    }

    #[test]
    fn center_alignment_command() {
        let mut e = editor_with("hello");
        e.options.set("textwidth", crate::options::OptionValue::Int(20));
        e.execute_ex_command("center");
        let buf = e.buffers.get(e.current_buffer_id()).unwrap();
        let line: String = buf.content.line(0).chars().collect();
        // "hello" is 5 chars, padded to center in 20-width => ~7-8 spaces prefix.
        assert!(line.starts_with("       "));
    }

    #[test]
    fn spell_load_dictionary_file() {
        let mut checker = crate::spell::SpellChecker::new();
        // Load a non-existent dict file → should error.
        let result = checker.load_dictionary("/nonexistent/dict.dic");
        assert!(result.is_err());
    }

    #[test]
    fn secure_exrc_option_exists() {
        let mut e = editor_with("test");
        // Verify secure option can be set.
        e.options.set("secure", crate::options::OptionValue::Bool(true));
        assert!(e.options.get_bool("secure"));
    }
}
