//! Wave 22 tests: script-local vars, tab layouts, popup insert,
//! dict access/has_key, viminfo marks, keywordprg, snippet mirrors,
//! range expression evaluation.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, KeyCode, Mode, Modifier};

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    /// REQ-SCRIPTLOCAL-01: s: variable namespace and function() reference.
    #[test]
    fn script_local_vars_and_funcref() {
        use std::collections::HashMap;
        let mut vars = HashMap::new();
        vars.insert("s:myvar".to_string(), "hello".to_string());
        let r = crate::expr_eval::eval_expression_with_vars("s:myvar", &vars);
        assert_eq!(r.unwrap(), "hello");
        // function() reference
        let r2 = crate::expr_eval::eval_expression("function(\"MyFunc\")");
        assert_eq!(r2.unwrap(), "MyFunc");
    }

    /// REQ-TABWINLAYOUT-01: per-tab window layouts in session.
    #[test]
    fn tab_page_window_layouts() {
        use crate::session::{SessionData, SessionLayout, SessionManager};
        let data = SessionData {
            tab_count: 2,
            active_tab: 0,
            tab_layouts: vec![
                SessionLayout::Single,
                SessionLayout::Hsplit(Vec::new(), vec![0.5, 0.5]),
            ],
            ..Default::default()
        };
        let serialized = SessionManager::serialize(&data);
        assert!(serialized.contains("tablayout 0 single"));
        assert!(serialized.contains("tablayout 1 hsplit:"));
        let restored = SessionManager::deserialize(&serialized);
        assert_eq!(restored.tab_layouts.len(), 2);
    }

    /// REQ-POPUPINSERT-01: Enter on popup selection inserts and stays in command mode.
    #[test]
    fn popup_menu_enter_inserts() {
        let mut ed = editor_with("hello");
        ed.mode = Mode::Command(kjxlkj_core_types::CommandKind::Ex);
        ed.cmdline.open(':');
        ed.cmdline.insert_char('s');
        ed.cmdline_complete_next();
        assert!(!ed.cmdline.completion.candidates.is_empty());
        // Press Enter: should insert selection and stay in Command mode.
        ed.handle_key(Key::new(KeyCode::Enter, Modifier::NONE));
        // Should still be in command mode (not executed)
        assert!(matches!(ed.mode, Mode::Command(_)), "should stay in command mode after popup enter");
        assert!(ed.cmdline.completion.candidates.is_empty(), "completion should be cleared");
    }

    /// REQ-DICTACCESS-01: dict["key"] access and has_key() function.
    #[test]
    fn dict_key_access_and_has_key() {
        let r = crate::expr_eval::eval_expression("{\"name\":\"alice\"}[\"name\"]");
        assert_eq!(r.unwrap(), "alice");
        let r2 = crate::expr_eval::eval_expression("has_key({\"a\":\"1\"}, \"a\")");
        assert_eq!(r2.unwrap(), "1");
        let r3 = crate::expr_eval::eval_expression("has_key({\"a\":\"1\"}, \"b\")");
        assert_eq!(r3.unwrap(), "0");
    }

    /// REQ-VIMINFOMARKS-01: viminfo mark serialization roundtrip.
    #[test]
    fn viminfo_marks_roundtrip() {
        let mut marks = crate::marks::MarkFile::new();
        let mp = crate::marks::MarkPosition::new(0, 10, 5 );
        marks.set('A', mp);
        marks.set('B', crate::marks::MarkPosition::new(1, 20, 3 ));
        let viminfo = marks.serialize_viminfo();
        assert!(viminfo.contains("'A"));
        assert!(viminfo.contains("'B"));
        let mut marks2 = crate::marks::MarkFile::new();
        marks2.load_viminfo(&viminfo);
        let a = marks2.get('A', 0).unwrap();
        assert_eq!(a.line, 10);
        assert_eq!(a.col, 5);
        let b = marks2.get('B', 1).unwrap();
        assert_eq!(b.line, 20);
    }

    /// REQ-KEYWORDPRG-01: K command dispatches LookupKeyword action.
    #[test]
    fn keyword_lookup_action() {
        let mut ed = editor_with("hello world");
        // Set a non-existent program to exercise error path
        ed.options.set("keywordprg", crate::options::OptionValue::Str("__nonexistent_kp__".into()));
        ed.handle_keyword_lookup(1);
        let notif = ed.notifications.last();
        assert!(notif.is_some(), "should have notification about keyword lookup");
    }

    /// REQ-SNIPPETMIRROR-01: snippet mirror tab-stops repeat text.
    #[test]
    fn snippet_mirror_tab_stops() {
        let mut reg = crate::snippets::SnippetRegistry::new();
        // ${1:name} defines default, then $1 mirrors it
        reg.add("pair", "${1:tag}</${1}>", "paired tags");
        let (text, stops) = reg.expand("pair").unwrap();
        // Both occurrences should contain "tag"
        assert!(text.contains("tag</tag>"), "mirror should repeat default text: {text}");
        assert_eq!(stops.len(), 2, "should have 2 stops (both $1 positions)");
    }

    /// REQ-RANGEEXPR-01: expression address (expr) evaluates to line number.
    #[test]
    fn range_expression_evaluation() {
        let mut ed = editor_with("aaa\nbbb\nccc\nddd\n");
        // (2+1) should evaluate to line 3 (1-based → 0-based: line 2)
        ed.execute_ex_command("(2+1)d");
        let buf = ed.buffers.get(ed.current_buffer_id()).unwrap();
        let text: String = buf.content.chars().collect();
        // "ccc" line should be deleted
        assert!(!text.contains("ccc"), "line 3 (ccc) should be deleted: {text}");
        assert!(text.contains("bbb"), "line 2 should remain");
    }
}
