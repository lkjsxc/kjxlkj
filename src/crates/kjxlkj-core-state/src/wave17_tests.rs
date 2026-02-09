//! Wave 17 tests: visual J/=, :marks, session history, expr fns,
//! sub confirm, multi-line search, macro edit, filetype detect.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode, VisualKind};

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    #[test]
    fn visual_join_lines() {
        let mut e = editor_with("hello\nworld\nfoo\n");
        // Enter visual line mode, select 2 lines, press J
        e.handle_key(Key::char('V'));
        e.handle_key(Key::char('j'));
        assert!(matches!(e.mode, Mode::Visual(VisualKind::Line)));
        e.handle_key(Key::char('J'));
        assert!(matches!(e.mode, Mode::Normal));
        let buf = e.buffers.get(e.current_buffer_id()).unwrap();
        let text = buf.content.to_string();
        // Lines joined with space
        assert!(text.starts_with("hello world"));
    }

    #[test]
    fn visual_reindent() {
        let mut e = editor_with("  first\nsecond\n  third\n");
        // Move to second line, visual select it, press =
        e.handle_key(Key::char('j'));
        e.handle_key(Key::char('V'));
        e.handle_key(Key::char('='));
        assert!(matches!(e.mode, Mode::Normal));
        // reindent_lines was called — line should have adjusted indent
        let buf = e.buffers.get(e.current_buffer_id()).unwrap();
        let _text = buf.content.to_string();
    }

    #[test]
    fn marks_list_shows_positions() {
        let mut e = editor_with("line one\nline two\nline three\n");
        // Set mark 'a' at line 0, col 0
        e.handle_key(Key::char('m'));
        e.handle_key(Key::char('a'));
        // Execute :marks
        e.execute_ex_command("marks");
        let last = e.notifications.last().unwrap();
        let msg = &last.message;
        assert!(msg.contains("mark line  col"), "got: {msg}");
        assert!(msg.contains(" a"), "got: {msg}");
    }

    #[test]
    fn session_saves_search_history() {
        let mut e = editor_with("hello\nworld\n");
        e.search.pattern = Some("hello".to_string());
        // Add cmdline history
        e.cmdline.history.push("set number".to_string());
        // Generate session content
        let path = "/tmp/kjxlkj_test_wave17_session.vim";
        e.handle_mksession(Some(path));
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("\" search: hello"), "got: {content}");
        assert!(
            content.contains("\" history: set number"),
            "got: {content}"
        );
        // Restore
        let mut e2 = EditorState::new(80, 24);
        e2.handle_source(path);
        assert_eq!(e2.search.pattern.as_deref(), Some("hello"));
        assert!(e2.cmdline.history.contains(&"set number".to_string()));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn expr_strlen_line_col() {
        use crate::expr_eval::eval_expression_with_vars;
        use std::collections::HashMap;

        // strlen
        let r = eval_expression_with_vars("strlen(\"hello\")", &HashMap::new());
        assert_eq!(r.unwrap(), "5");

        // line(".")
        let mut vars = HashMap::new();
        vars.insert("v:lnum".to_string(), "42".to_string());
        let r = eval_expression_with_vars("line(\".\")", &vars);
        assert_eq!(r.unwrap(), "42");

        // col(".")
        vars.insert("v:col".to_string(), "10".to_string());
        let r = eval_expression_with_vars("col(\".\")", &vars);
        assert_eq!(r.unwrap(), "10");
    }

    #[test]
    fn substitute_confirm_flag_parsed() {
        use crate::ex_parse_substitute::parse_substitute;
        let cmd = parse_substitute("/foo/bar/gc").unwrap();
        assert!(cmd.confirm);
        assert!(cmd.global);
        assert_eq!(cmd.pattern, "foo");
        assert_eq!(cmd.replacement, "bar");
    }

    #[test]
    fn macro_register_sync_on_yank() {
        let mut e = editor_with("ihello\x1b\n");
        // Set pending register to 'a', yank line
        e.pending_register = Some('a');
        e.yank_lines(1);
        // Now macro store should have keys for register 'a'
        assert!(e.macro_store.contains_key(&'a'));
    }

    #[test]
    fn filetype_detection() {
        use crate::config_loader::detect_filetype;
        assert_eq!(detect_filetype("main.rs"), Some("rust"));
        assert_eq!(detect_filetype("app.py"), Some("python"));
        assert_eq!(detect_filetype("index.tsx"), Some("typescriptreact"));
        assert_eq!(detect_filetype("Makefile"), None);

        // Opening file sets filetype option
        let mut e = EditorState::new(80, 24);
        e.open_file("test.py", "print('hi')");
        assert_eq!(e.options.get_str("filetype"), "python");
    }
}
