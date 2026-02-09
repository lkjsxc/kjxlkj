//! Wave 21 tests: local vars/return, tab session, popup nav,
//! dict/type, numbered marks, equalprg, snippet placeholders,
//! backwards range notification.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, KeyCode, Mode, Modifier};

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    /// REQ-LOCALVAR-01: let l:var = expr and return value.
    #[test]
    fn function_local_vars_and_return() {
        let mut ed = editor_with("hello");
        ed.execute_ex_command("function! Add(x)");
        ed.execute_ex_command("let l:result = a:x");
        ed.execute_ex_command("return l:result");
        ed.execute_ex_command("endfunction");
        let ret = ed.handle_call_function("call Add(\"42\")");
        // l:result should be set in options
        let val = ed.options.get_str("l:result").to_string();
        assert_eq!(val, "42");
        assert_eq!(ret, Some("42".into()));
    }

    /// REQ-TABSESS-01: tab count in session serialization.
    #[test]
    fn tab_pages_in_session() {
        use crate::session::{SessionData, SessionManager};
        let data = SessionData {
            tab_count: 3,
            active_tab: 1,
            ..Default::default()
        };
        let serialized = SessionManager::serialize(&data);
        assert!(serialized.contains("tabs 3 1"));
        let restored = SessionManager::deserialize(&serialized);
        assert_eq!(restored.tab_count, 3);
        assert_eq!(restored.active_tab, 1);
    }

    /// REQ-POPUPNAV-01: Ctrl-N/Ctrl-P navigate popup candidates.
    #[test]
    fn popup_menu_ctrl_n_p() {
        let mut ed = editor_with("hello");
        ed.mode = Mode::Command(kjxlkj_core_types::CommandKind::Ex);
        ed.cmdline.open(':');
        ed.cmdline.insert_char('s');
        // Trigger completion
        ed.cmdline_complete_next();
        assert!(!ed.cmdline.completion.candidates.is_empty());
        let first = ed.cmdline.completion.index;
        // Ctrl-N advances
        ed.dispatch_command_key(Key::new(KeyCode::Char('n'), Modifier::CTRL));
        let second = ed.cmdline.completion.index;
        assert_ne!(first, second);
        // Ctrl-P goes back
        ed.dispatch_command_key(Key::new(KeyCode::Char('p'), Modifier::CTRL));
        assert_eq!(ed.cmdline.completion.index, first);
    }

    /// REQ-DICTLIT-01: dict literals and type() function.
    #[test]
    fn dict_literal_and_type() {
        let r = crate::expr_eval::eval_expression("{\"a\":\"1\"}");
        assert_eq!(r.unwrap(), "{\"a\":\"1\"}");
        let t = crate::expr_eval::eval_expression("type(42)");
        assert_eq!(t.unwrap(), "0"); // number
        let t2 = crate::expr_eval::eval_expression("type(\"hello\")");
        assert_eq!(t2.unwrap(), "1"); // string
        let t3 = crate::expr_eval::eval_expression("type([1,2])");
        assert_eq!(t3.unwrap(), "3"); // list
        let t4 = crate::expr_eval::eval_expression("type({\"k\":\"v\"})");
        assert_eq!(t4.unwrap(), "4"); // dict
    }

    /// REQ-NUMMARKS-01: numbered marks rotate on jump.
    #[test]
    fn numbered_marks_rotate() {
        let mut ed = editor_with("line1\nline2\nline3\nline4\n");
        let bid = ed.current_buffer_id().0 as usize;
        // First jump:
        ed.push_jumplist();
        let m0 = ed.marks.get('0', bid);
        assert!(m0.is_some(), "mark 0 should be set after jump");
        // Second jump from different line:
        ed.windows.focused_mut().cursor.line = 2;
        ed.push_jumplist();
        let m1 = ed.marks.get('1', bid);
        assert!(m1.is_some(), "mark 1 should exist (rotated from 0)");
    }

    /// REQ-EQUALPRG-01: equalprg pipes through external program.
    #[test]
    fn equalprg_option() {
        let mut ed = editor_with("hello world\n");
        ed.options.set("equalprg", crate::options::OptionValue::Str("__no_such_eq__".into()));
        ed.reindent_lines(0, 0);
        // Should produce error notification about the program
        let notif = ed.notifications.last();
        assert!(notif.is_some());
    }

    /// REQ-SNIPPLACEHOLDER-01: snippet ${1:default} includes default text.
    #[test]
    fn snippet_placeholder_default() {
        let mut reg = crate::snippets::SnippetRegistry::new();
        reg.add("fn", "fn ${1:name}(${2:args}) {\n  $0\n}", "function");
        let (text, stops) = reg.expand("fn").unwrap();
        assert!(text.contains("name"), "default text 'name' should be in expansion: {text}");
        assert!(text.contains("args"), "default text 'args' should be in expansion: {text}");
        assert_eq!(stops.len(), 3); // $1, $2, $0
    }

    /// REQ-BACKRANGE-01: backwards range auto-swaps and notifies.
    #[test]
    fn backwards_range_notification() {
        let mut ed = editor_with("aaa\nbbb\nccc\n");
        ed.execute_ex_command("3,1d");
        // Should have a "Backwards range corrected" notification
        let has = ed.notifications.iter().any(|n| n.message.contains("Backwards"));
        assert!(has, "should notify about backwards range correction");
    }
}
