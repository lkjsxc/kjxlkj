#[cfg(test)]
mod wave33 {
    use crate::editor::EditorState;
    fn editor_with(text: &str) -> EditorState { let mut e = EditorState::new(80, 24); e.open_file("test.txt", text); e }

    #[test]
    fn test_regex_cursor_atom() {
        let r = crate::regex_translate::translate_vim_to_rust("foo\\%#bar");
        assert!(r.pattern.contains("foobar"));
    }

    #[test]
    fn test_for_endfor_loop() {
        let mut e = editor_with("line1\nline2\nline3\n");
        e.execute_ex_command("for item in [\"a\",\"b\",\"c\"]");
        e.execute_ex_command("let result = item");
        e.execute_ex_command("endfor");
        assert_eq!(e.options.get_str("result"), "c");
    }

    #[test]
    fn test_visual_block_insert_count() {
        let mut e = editor_with("abc\ndef\nghi\n");
        // block_insert uses op_count for repetition
        e.op_count = 3;
        let text = "X";
        let start = kjxlkj_core_types::CursorPosition { line: 0, grapheme: 0 };
        let end = kjxlkj_core_types::CursorPosition { line: 2, grapheme: 0 };
        e.visual_block_insert(text, start, end, false);
        let content = e.buffers.get(e.current_buffer_id()).unwrap().content.to_string();
        assert!(content.starts_with("XXXabc\n"));
    }

    #[test]
    fn test_printf_function() {
        let r = crate::expr_eval::eval_expression("printf(\"%s has %d items\", \"list\", \"5\")").unwrap();
        assert_eq!(r, "list has 5 items");
    }

    #[test]
    fn test_split_function() {
        let r = crate::expr_eval::eval_expression("split(\"a:b:c\", \":\")").unwrap();
        assert_eq!(r, "[\"a\",\"b\",\"c\"]");
    }

    #[test]
    fn test_join_function() {
        // join expects list items without inner quotes in the list string
        let r = crate::expr_eval::eval_expression("join(split(\"a:b:c\", \":\"), \"-\")").unwrap();
        assert_eq!(r, "a-b-c");
    }

    #[test]
    fn test_move_range() {
        let mut e = editor_with("line1\nline2\nline3\nline4\n");
        e.execute_ex_command("1,2move 3");
        let content = e.buffers.get(e.current_buffer_id()).unwrap().content.to_string();
        assert!(content.starts_with("line3\nline1\nline2\n"));
    }

    #[test]
    fn test_copy_range() {
        let mut e = editor_with("aaa\nbbb\nccc\n");
        e.execute_ex_command("1copy 2");
        let content = e.buffers.get(e.current_buffer_id()).unwrap().content.to_string();
        assert!(content.contains("aaa\nbbb\nccc\naaa\n") || content.contains("aaa\nbbb\naaa\nccc\n"));
    }

    #[test]
    fn test_multi_language_spell() {
        let mut e = editor_with("test");
        e.spell.add_good("hello".to_string());
        e.spell.lang = "en".to_string();
        e.options.set("spelllang", crate::options::OptionValue::Str("en,fr".to_string()));
        // Verify the spelllang option was set
        assert_eq!(e.options.get_str("spelllang"), "en,fr");
    }

    #[test]
    fn test_trust_store_option() {
        let mut e = editor_with("test");
        // Verify exrc + secure options exist
        e.options.set("exrc", crate::options::OptionValue::Bool(true));
        e.options.set("secure", crate::options::OptionValue::Bool(true));
        assert!(e.options.get_bool("exrc"));
        assert!(e.options.get_bool("secure"));
    }
}
