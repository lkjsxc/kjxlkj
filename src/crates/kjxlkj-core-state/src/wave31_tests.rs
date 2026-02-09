#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use crate::regex_translate::translate_vim_to_rust;

    fn editor_with(path: &str, text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file(path, text);
        e
    }

    /// REQ-REGEXVV-01: \%V inside-visual-area atom is accepted (no-op in static regex).
    #[test]
    fn regex_percent_v_visual_atom() {
        let r = translate_vim_to_rust(r"foo\%Vbar");
        // \%V should be silently consumed, producing "foobar".
        assert_eq!(r.pattern, "foobar");
    }

    /// REQ-EXECUTE-01: :execute evaluates string expression and runs as ex command.
    #[test]
    fn execute_eval_and_run() {
        let mut e = editor_with("test.txt", "hello world");
        e.handle_action(kjxlkj_core_types::Action::EnterCommandEx);
        e.cmdline.content = "execute \"echo \\\"from execute\\\"\"".to_string();
        e.execute_cmdline();
        let msg = e.notifications.last().map(|n| n.message.clone()).unwrap_or_default();
        assert!(msg.contains("from execute"), "should run echo: {msg}");
    }

    /// REQ-EXECUTE-01: :exe abbreviation works.
    #[test]
    fn execute_exe_abbreviation() {
        let mut e = editor_with("test.txt", "data");
        e.handle_action(kjxlkj_core_types::Action::EnterCommandEx);
        e.cmdline.content = "exe \"echo \\\"short\\\"\"".to_string();
        e.execute_cmdline();
        let msg = e.notifications.last().map(|n| n.message.clone()).unwrap_or_default();
        assert!(msg.contains("short"), "exe abbreviation: {msg}");
    }

    /// REQ-STRFUNCS-01: toupper/tolower string functions in expressions.
    #[test]
    fn toupper_tolower_functions() {
        let r = crate::expr_eval::eval_expression("toupper(\"hello\")").unwrap();
        assert_eq!(r, "HELLO");
        let r2 = crate::expr_eval::eval_expression("tolower(\"WORLD\")").unwrap();
        assert_eq!(r2, "world");
    }

    /// REQ-RETAB-01: :retab converts tabs to spaces when expandtab is set.
    #[test]
    fn retab_tabs_to_spaces() {
        let mut e = editor_with("test.txt", "\thello\n\t\tworld\n");
        e.options.set("expandtab", crate::options::OptionValue::Bool(true));
        e.options.set("tabstop", crate::options::OptionValue::Int(4));
        e.handle_action(kjxlkj_core_types::Action::EnterCommandEx);
        e.cmdline.content = "retab".to_string();
        e.execute_cmdline();
        let buf_id = e.current_buffer_id();
        let text = e.buffers.get(buf_id).unwrap().content.to_string();
        // Tabs should be replaced with spaces.
        assert!(!text.contains('\t'), "no tabs after retab: {text:?}");
        assert!(text.contains("    hello"), "4 spaces: {text:?}");
    }

    /// REQ-RETAB-01: :retab with new tabstop argument.
    #[test]
    fn retab_with_new_tabstop() {
        let mut e = editor_with("test.txt", "\tline\n");
        e.options.set("expandtab", crate::options::OptionValue::Bool(true));
        e.options.set("tabstop", crate::options::OptionValue::Int(8));
        e.handle_action(kjxlkj_core_types::Action::EnterCommandEx);
        e.cmdline.content = "retab 2".to_string();
        e.execute_cmdline();
        let msg = e.notifications.last().map(|n| n.message.clone()).unwrap_or_default();
        assert!(msg.contains("retab"), "notification: {msg}");
    }

    /// REQ-SPELLSUGGEST-01: z= shows spelling suggestions.
    #[test]
    fn spell_suggest_shows_suggestions() {
        let mut e = editor_with("test.txt", "tset");
        e.spell.add_good("test".to_string());
        e.spell.add_good("best".to_string());
        e.spell_suggest();
        let msg = e.notifications.last().map(|n| n.message.clone()).unwrap_or_default();
        // Should show suggestions including "test" (1 edit distance from "tset").
        assert!(msg.contains("Suggestions") || msg.contains("tset"), "suggest output: {msg}");
    }

    /// REQ-SPELLSUGGEST-01: SpellChecker.suggest returns nearby words.
    #[test]
    fn spell_checker_suggest_edit_distance() {
        let mut sc = crate::spell::SpellChecker::new();
        sc.add_good("hello".to_string());
        sc.add_good("world".to_string());
        let suggestions = sc.suggest("helo", 5);
        assert!(suggestions.iter().any(|s| s == "hello"), "should suggest 'hello': {suggestions:?}");
    }

    /// REQ-MODELINEML-01: Modeline set multiple options separated by colons.
    #[test]
    fn modeline_multiple_options_colon() {
        let text = "// vim: set ts=2:sw=2:et:\ncode here\n";
        let e = editor_with("test.rs", text);
        assert_eq!(e.options.get_int("tabstop"), 2);
        assert_eq!(e.options.get_int("shiftwidth"), 2);
        assert!(e.options.get_bool("expandtab"));
    }

    /// REQ-MODELINEML-01: Modeline set multiple options space-separated.
    #[test]
    fn modeline_multiple_options_spaces() {
        let text = "# vi: set ts=3 sw=3 noet:\ndata\n";
        let e = editor_with("test.py", text);
        assert_eq!(e.options.get_int("tabstop"), 3);
        assert_eq!(e.options.get_int("shiftwidth"), 3);
    }
}
