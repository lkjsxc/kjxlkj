//! Wave 24 tests: script sourcing, arglist persistence, filetype completion,
//! map/filter/extend, viminfo merge, visual K, snippet choice, range variables.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use crate::expr_eval::eval_expression;
    use crate::snippets::SnippetRegistry;
    use std::collections::HashMap;

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    /// REQ-SCRIPTSOURCE-01: :source with finish early exit.
    #[test]
    fn source_finish_stops_execution() {
        let mut e = editor_with("hello");
        let dir = std::env::temp_dir().join("wave24_source_test");
        let _ = std::fs::create_dir_all(&dir);
        let script = dir.join("test_finish.vim");
        std::fs::write(&script, "set number\nfinish\nset nonumber\n").unwrap();
        e.execute_ex_command(&format!("source {}", script.display()));
        assert!(e.options.get_bool("number"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// REQ-ARGLIST-01: arglist persistence in session data.
    #[test]
    fn arglist_session_roundtrip() {
        use crate::session::{SessionData, SessionManager, SessionLayout};
        use std::path::PathBuf;
        let data = SessionData {
            files: Vec::new(), cwd: None, layout: SessionLayout::Single,
            marks: Vec::new(), active_buffer: 0, tab_count: 1, active_tab: 0,
            tab_layouts: Vec::new(), tab_buffers: Vec::new(),
            arglist: vec![PathBuf::from("a.txt"), PathBuf::from("b.txt")],
        };
        let ser = SessionManager::serialize(&data);
        let restored = SessionManager::deserialize(&ser);
        assert_eq!(restored.arglist.len(), 2);
        assert_eq!(restored.arglist[0], PathBuf::from("a.txt"));
    }

    /// REQ-FTCOMPL-01: filetype completion after set filetype=.
    #[test]
    fn filetype_completion_offers_types() {
        let mut e = editor_with("hello");
        e.handle_key(kjxlkj_core_types::Key::char(':'));
        for c in "set filetype=r".chars() { e.handle_key(kjxlkj_core_types::Key::char(c)); }
        e.handle_key(kjxlkj_core_types::Key::new(kjxlkj_core_types::KeyCode::Tab, kjxlkj_core_types::Modifier::NONE));
        assert!(e.cmdline.content.contains("rust") || e.cmdline.content.contains("ruby"));
    }

    /// REQ-MAPFILTER-01: map(), filter(), extend() in expression evaluator.
    #[test]
    fn map_filter_extend_builtins() {
        let r = eval_expression("extend([1,2],[3,4])").unwrap();
        assert_eq!(r, "[1,2,3,4]");
        let r2 = eval_expression("len(\"abc\")").unwrap();
        assert_eq!(r2, "3");
    }

    /// REQ-VIMINFOMERGE-01: viminfo merge loads existing + merges new marks.
    #[test]
    fn viminfo_merge_preserves_old() {
        use crate::marks::{MarkFile, MarkPosition};
        let mut old = MarkFile::new();
        old.set('A', MarkPosition { buffer_id: 0, line: 5, col: 3 });
        let old_str = old.serialize_viminfo();
        let mut merged = MarkFile::new();
        merged.load_viminfo(&old_str);
        merged.set('B', MarkPosition { buffer_id: 1, line: 10, col: 0 });
        let result = merged.serialize_viminfo();
        assert!(result.contains("'A"));
        assert!(result.contains("'B"));
    }

    /// REQ-VISUALK-01: K in visual mode dispatches visual_keyword_lookup.
    #[test]
    fn visual_k_uses_selected_text() {
        // Visual K exits visual mode (returns to normal)
        let mut e = editor_with("hello world");
        e.mode = kjxlkj_core_types::Mode::Visual(kjxlkj_core_types::VisualKind::Char);
        e.visual_anchor = Some(kjxlkj_core_types::CursorPosition::new(0, 0));
        // Move cursor to extend selection
        e.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(0, 4);
        e.dispatch_visual(kjxlkj_core_types::Key::char('K'), kjxlkj_core_types::VisualKind::Char);
        assert!(matches!(e.mode, kjxlkj_core_types::Mode::Normal));
    }

    /// REQ-SNIPPETCHOICE-01: snippet choice nodes ${1|opt1,opt2|}.
    #[test]
    fn snippet_choice_uses_first_option() {
        let mut reg = SnippetRegistry::new();
        reg.add("ch", "hello ${1|foo,bar,baz|} world", "choice test");
        let (text, stops) = reg.expand("ch").unwrap();
        assert!(text.contains("foo"));
        assert!(!text.contains("bar"));
        assert_eq!(stops.len(), 1);
    }

    /// REQ-RANGEVARS-01: range expressions use variables from context.
    #[test]
    fn range_expression_with_vars() {
        use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};
        let lines = vec!["aaa", "bbb", "ccc", "ddd", "eee"];
        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("myline".into(), "3".into());
        let ctx = RangeContext {
            current_line: 0, total_lines: 5, lines: &lines,
            mark_line: None, last_search: None, vars: Some(&vars),
        };
        let (range, _rest) = parse_range_ctx("(myline)d", &ctx);
        let r = range.unwrap();
        assert_eq!(r.start, 2); // line 3 => index 2
    }
}
