//! Wave 20 tests: function params, sort, popup menu, list/len,
//! local marks in session, formatexpr, sub-confirm cursor,
//! op-pending highlight.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Mode, Operator};

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    /// REQ-FUNCPARAM-01: :call FuncName(arg) binds a:param.
    #[test]
    fn call_function_with_args() {
        let mut ed = editor_with("hello");
        ed.execute_ex_command("function! Greet(name)");
        ed.execute_ex_command("endfunction");
        ed.execute_ex_command("call Greet(\"world\")");
        let val = ed.options.get_str("a:name").to_string();
        assert_eq!(val, "world");
    }

    /// REQ-VSORT-01: :sort sorts lines alphabetically.
    #[test]
    fn sort_lines() {
        let mut ed = editor_with("cherry\napple\nbanana\n");
        ed.execute_ex_command("1,3sort");
        let buf = ed.buffers.get(ed.current_buffer_id()).unwrap();
        let l0: std::borrow::Cow<str> = buf.content.line(0).into();
        let l1: std::borrow::Cow<str> = buf.content.line(1).into();
        let l2: std::borrow::Cow<str> = buf.content.line(2).into();
        assert!(l0.starts_with("apple"), "first line should be apple, got: {l0}");
        assert!(l1.starts_with("banana"), "second: {l1}");
        assert!(l2.starts_with("cherry"), "third: {l2}");
    }

    /// REQ-POPUP-01: PopupMenu snapshot includes position fields.
    #[test]
    fn popup_menu_position_fields() {
        let mut ed = editor_with("hello");
        // Add completion candidates to trigger popup.
        ed.cmdline.completion.candidates = vec!["abc".into(), "def".into()];
        ed.cmdline.completion.index = Some(0);
        let snap = ed.snapshot();
        let pm = snap.popup_menu.expect("popup should exist");
        assert_eq!(pm.items.len(), 2);
        assert!(pm.max_visible > 0);
        // row should be near bottom of screen.
        assert!(pm.row > 0);
    }

    /// REQ-LISTLIT-01: list literal and len() function.
    #[test]
    fn list_literal_and_len() {
        let r = crate::expr_eval::eval_expression("[1,2,3]");
        assert_eq!(r.unwrap(), "[1,2,3]");
        let r2 = crate::expr_eval::eval_expression("len([10,20,30])");
        assert_eq!(r2.unwrap(), "3");
        let r3 = crate::expr_eval::eval_expression("len([])");
        assert_eq!(r3.unwrap(), "0");
    }

    /// REQ-MARKPERSIST-01: local marks serialized in session file.
    #[test]
    fn local_marks_in_session() {
        use crate::session::{SessionData, SessionFile, SessionManager};
        use std::path::PathBuf;
        let data = SessionData {
            files: vec![SessionFile {
                path: PathBuf::from("/tmp/a.rs"),
                cursor_line: 0,
                cursor_col: 0,
                was_modified: false,
                local_marks: vec![('a', 5, 3), ('b', 10, 0)],
            }],
            ..Default::default()
        };
        let serialized = SessionManager::serialize(&data);
        assert!(serialized.contains("localmark a 5 3"));
        assert!(serialized.contains("localmark b 10 0"));
        let restored = SessionManager::deserialize(&serialized);
        assert_eq!(restored.files[0].local_marks.len(), 2);
        assert_eq!(restored.files[0].local_marks[0], ('a', 5, 3));
    }

    /// REQ-FMTEXPR-01: formatexpr calls function on gq.
    #[test]
    fn formatexpr_calls_function() {
        let mut ed = editor_with("hello world\n");
        ed.execute_ex_command("function! MyFmt()");
        ed.execute_ex_command("endfunction");
        ed.options.set("formatexpr", crate::options::OptionValue::Str("MyFmt()".into()));
        // Should not crash — formatexpr invokes function.
        ed.format_lines(0, 0);
    }

    /// REQ-SUBCONFIRM-01: :s///c enters confirm mode and positions cursor.
    #[test]
    fn sub_confirm_cursor_position() {
        let mut ed = editor_with("bbb\naaa\nccc\nbbb\n");
        ed.execute_ex_command("1,4s/bbb/xxx/c");
        assert!(ed.sub_confirm.is_some());
        // Cursor should be on line 0 (first line of range, which matches).
        assert_eq!(ed.windows.focused().cursor.line, 0);
    }

    /// REQ-OPHIGHLIGHT-01: search highlights shown in op-pending.
    #[test]
    fn op_pending_search_highlight() {
        let mut ed = editor_with("foo bar foo\n");
        ed.search.pattern = Some("foo".into());
        ed.search.active = true;
        ed.mode = Mode::OperatorPending(Operator::Delete);
        let snap = ed.snapshot();
        assert!(!snap.search.highlight_ranges.is_empty(),
            "search highlights should appear in operator-pending mode");
    }
}
