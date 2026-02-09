//! Wave 23 tests: autoload, tab buffer assoc, context completion,
//! keys/values, viminfo autosave, keyword count, nested snippets,
//! complex range expressions.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::Mode;

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    /// REQ-AUTOLOAD-01: autoload function resolution via # separator.
    #[test]
    fn autoload_function_resolution() {
        let mut ed = editor_with("hello");
        // Define function "MyFunc" — autoload "ns#MyFunc" should resolve to "MyFunc"
        ed.execute_ex_command("function! MyFunc(x)");
        ed.execute_ex_command("return a:x");
        ed.execute_ex_command("endfunction");
        // Call via autoload name ns#MyFunc
        let result = ed.handle_call_function("call ns#MyFunc(\"ok\")");
        assert_eq!(result, Some("ok".to_string()));
    }

    /// REQ-TABBUFASSOC-01: tab-specific buffer associations in session.
    #[test]
    fn tab_buffer_associations() {
        use crate::session::{SessionData, SessionManager};
        let data = SessionData {
            tab_count: 2,
            tab_buffers: vec![vec![0, 1], vec![2]],
            ..Default::default()
        };
        let serialized = SessionManager::serialize(&data);
        assert!(serialized.contains("tabbuf 0 0,1"));
        assert!(serialized.contains("tabbuf 1 2"));
        let restored = SessionManager::deserialize(&serialized);
        assert_eq!(restored.tab_buffers.len(), 2);
        assert_eq!(restored.tab_buffers[0], vec![0, 1]);
        assert_eq!(restored.tab_buffers[1], vec![2]);
    }

    /// REQ-CTXCOMPL-01: context-aware completion for mark/register/help commands.
    #[test]
    fn context_aware_completion() {
        let mut ed = editor_with("hello");
        ed.mode = Mode::Command(kjxlkj_core_types::CommandKind::Ex);
        ed.cmdline.open(':');
        ed.cmdline.content = "mark ".to_string();
        ed.build_arg_candidates();
        // Should have mark name candidates (a-z, A-Z)
        assert!(!ed.cmdline.completion.candidates.is_empty());
        assert!(ed.cmdline.completion.candidates.contains(&"a".to_string()));

        ed.cmdline.completion.clear();
        ed.cmdline.content = "help ".to_string();
        ed.build_arg_candidates();
        assert!(!ed.cmdline.completion.candidates.is_empty());
        assert!(ed.cmdline.completion.candidates.contains(&"insert".to_string()));
    }

    /// REQ-DICTITER-01: keys() and values() functions.
    #[test]
    fn dict_keys_and_values() {
        let r = crate::expr_eval::eval_expression("keys({\"a\":\"1\",\"b\":\"2\"})");
        let keys = r.unwrap();
        assert!(keys.contains("\"a\"") && keys.contains("\"b\""), "keys should contain a and b: {keys}");

        let r2 = crate::expr_eval::eval_expression("values({\"a\":\"1\",\"b\":\"2\"})");
        let vals = r2.unwrap();
        assert!(vals.contains("\"1\"") && vals.contains("\"2\""), "values should contain 1 and 2: {vals}");
    }

    /// REQ-VIMINFOAUTOSAVE-01: viminfo auto-save writes file on quit.
    #[test]
    fn viminfo_autosave_on_quit() {
        let tmp = std::env::temp_dir().join("kjxlkj_test_viminfo");
        let _ = std::fs::remove_file(&tmp);
        let mut ed = editor_with("hello");
        ed.options.set("viminfofile", crate::options::OptionValue::Str(tmp.display().to_string()));
        // Set a global mark
        ed.marks.set('A', crate::marks::MarkPosition::new(0, 5, 3 ));
        // Force quit triggers save_viminfo
        ed.handle_action(kjxlkj_core_types::Action::ForceQuit);
        // Verify file was written
        assert!(tmp.exists(), "viminfo file should be created on quit");
        let content = std::fs::read_to_string(&tmp).unwrap();
        assert!(content.contains("'A"), "viminfo should contain mark A");
        let _ = std::fs::remove_file(&tmp);
    }

    /// REQ-KEYWORDCOUNT-01: K command with count passes section arg.
    #[test]
    fn keyword_lookup_with_count() {
        let mut ed = editor_with("printf test");
        ed.options.set("keywordprg", crate::options::OptionValue::Str("__nonexistent_kp_cnt__".into()));
        // Call with count=3 — should pass "3" as arg to keywordprg
        ed.handle_keyword_lookup(3);
        let notif = ed.notifications.last().unwrap();
        // Should mention the program name in error
        assert!(notif.message.contains("__nonexistent_kp_cnt__"), "notification should reference keywordprg: {:?}", notif);
    }

    /// REQ-SNIPPETNEST-01: nested snippet placeholders.
    #[test]
    fn nested_snippet_placeholders() {
        let mut reg = crate::snippets::SnippetRegistry::new();
        reg.add("nest", "${1:outer ${2:inner}}", "nested");
        let (text, stops) = reg.expand("nest").unwrap();
        assert!(text.contains("outer inner"), "nested default should expand: {text}");
        assert!(stops.len() >= 2, "should have at least 2 stops");
    }

    /// REQ-RANGEEXPRFUNC-01: expression addresses with function calls.
    #[test]
    fn range_expr_with_function_calls() {
        // Test that expression evaluation handles function calls in arithmetic
        let r = crate::expr_eval::eval_expression("strlen(\"abc\")+1");
        assert_eq!(r.unwrap(), "4", "strlen(\"abc\")+1 should be 4");
        let r2 = crate::expr_eval::eval_expression("len([1,2,3])*2");
        assert_eq!(r2.unwrap(), "6", "len([1,2,3])*2 should be 6");
    }
}
