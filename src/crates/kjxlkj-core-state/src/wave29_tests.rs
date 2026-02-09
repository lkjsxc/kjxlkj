//! Wave 29 tests: regex backrefs in replacement, echo/echon, visual :s,
//! fuzzy ranked completion, bitwise shift, gw format, snippet auto-exit, commentstring.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.rs", text);
        e
    }

    /// REQ-REGEXBACK-01: Replacement string translates \1-\9 to $1-$9.
    #[test]
    fn test_replacement_backrefs() {
        use crate::ex_substitute::translate_vim_replacement;
        assert_eq!(translate_vim_replacement(r"\1-\2"), "$1-$2");
        assert_eq!(translate_vim_replacement(r"&-\0"), "$0-$0");
        assert_eq!(translate_vim_replacement(r"\n"), "\n");
    }

    /// REQ-ECHO-01: :echo displays message.
    #[test]
    fn test_echo_command() {
        let mut e = editor_with("hello");
        e.execute_ex_command("echo \"test message\"");
        assert!(e.notifications.iter().any(|n| n.message.contains("test message")));
    }

    #[test]
    fn test_echon_command() {
        let mut e = editor_with("hello");
        e.execute_ex_command("echon \"no newline\"");
        assert!(e.notifications.iter().any(|n| n.message.contains("no newline")));
    }

    /// REQ-VISUALSR-01: Visual mode :s substitute within visual range.
    #[test]
    fn test_visual_substitute() {
        let mut e = editor_with("aaa\nbbb\nccc");
        // Simulate visual selection then :'<,'>s/b/X/g
        e.marks.set_visual_start(crate::marks::MarkPosition::new(0, 1, 0));
        e.marks.set_visual_end(crate::marks::MarkPosition::new(0, 1, 2));
        e.execute_ex_command("'<,'>s/b/X/g");
        let buf = e.buffers.get(e.current_buffer_id()).unwrap();
        let text = buf.content.to_string();
        assert!(text.contains("XXX"));
    }

    /// REQ-FUZZYSCORE-01: Fuzzy filter returns ranked results.
    #[test]
    fn test_fuzzy_ranked() {
        use crate::cmdline_completion_ctx::fuzzy_score;
        let s1 = fuzzy_score("wrt", "write").unwrap();
        let s2 = fuzzy_score("wrt", "wquitall").unwrap_or(0);
        // "write" should score higher than "wquitall" for query "wrt"
        assert!(s1 >= s2);
    }

    /// REQ-BITSHIFT-01: lshift/rshift bitwise shift functions.
    #[test]
    fn test_bitshift_operators() {
        use crate::expr_eval::eval_expression;
        assert_eq!(eval_expression("lshift(1, 4)").unwrap(), "16");
        assert_eq!(eval_expression("rshift(16, 2)").unwrap(), "4");
        assert_eq!(eval_expression("lshift(3, 8)").unwrap(), "768");
    }

    /// REQ-GWFORMAT-01: gw format preserves cursor position.
    #[test]
    fn test_gw_format_keeps_cursor() {
        use kjxlkj_core_types::Key;
        let mut e = editor_with("hello world");
        let orig = e.windows.focused().cursor;
        // gw enters op-pending with FormatKeepCursor
        e.handle_key(Key::char('g'));
        e.handle_key(Key::char('w'));
        e.handle_key(Key::char('w')); // doubled = format current line
        let after = e.windows.focused().cursor;
        assert_eq!(orig.line, after.line);
    }

    /// REQ-SNIPPETEXIT-01: Snippet session is_finished at last stop.
    #[test]
    fn test_snippet_session_auto_exit() {
        use crate::snippets::SnippetSession;
        let mut s = SnippetSession {
            stops: vec![5, 10],
            current: 0,
            base_line: 0,
            base_col: 0,
        };
        assert!(!s.is_finished());
        s.advance();
        assert!(s.is_finished());
        assert!(!s.advance()); // no more stops
    }

    /// REQ-COMMENTSTR-01: Filetype commentstring defaults.
    #[test]
    fn test_commentstring_defaults() {
        use crate::config_loader::commentstring_for_filetype;
        assert_eq!(commentstring_for_filetype("rust"), Some("// %s"));
        assert_eq!(commentstring_for_filetype("python"), Some("# %s"));
        assert_eq!(commentstring_for_filetype("html"), Some("<!-- %s -->"));
        assert_eq!(commentstring_for_filetype("lua"), Some("-- %s"));
        assert_eq!(commentstring_for_filetype("unknown"), None);
    }

    #[test]
    fn test_commentstring_set_on_indent_load() {
        let mut e = editor_with("fn main() {}");
        e.load_indent_plugin("python");
        let cs = e.options.get_str("commentstring").to_string();
        assert_eq!(cs, "# %s");
    }
}
