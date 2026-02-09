//! Wave 15 tests: regex search count, session marks, block paste,
//! wildmenu scroll, expr cmdline, formatprg, regex branches, snippets.
#[cfg(test)]
mod wave15_tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode};

    fn make_editor() -> EditorState {
        let mut ed = EditorState::new(80, 24);
        ed.open_file("test.txt", "line one\nline two\nline three\nline four\n");
        ed
    }

    /// REQ-REGEXCOUNT-01: Regex search count for \v patterns.
    #[test]
    fn regex_search_count() {
        let mut ed = make_editor();
        ed.search.pattern = Some("\\vli.e".to_string());
        ed.search.active = true;
        ed.search_next();
        let (_, total) = ed.search.match_count.unwrap();
        assert!(total >= 4, "Expected >= 4 regex matches for \\vli.e");
    }

    /// REQ-SESSMARKS-01: Session save includes global marks.
    #[test]
    fn session_saves_global_marks() {
        let mut ed = make_editor();
        ed.windows.focused_mut().cursor.line = 2;
        ed.set_mark_at_cursor('A');
        // Trigger session save to a temp path.
        let tmp = "/tmp/kjxlkj_test_session.vim";
        ed.handle_mksession(Some(tmp));
        let content = std::fs::read_to_string(tmp).unwrap_or_default();
        assert!(
            content.contains("mark A"),
            "Session should include global mark A"
        );
        let _ = std::fs::remove_file(tmp);
    }

    /// REQ-BLOCKPASTE-01: Visual paste dispatch.
    #[test]
    fn visual_paste_dispatch() {
        let mut ed = make_editor();
        // Yank first line.
        ed.yank_lines(1);
        // Enter visual char mode and paste.
        ed.mode = Mode::Visual(kjxlkj_core_types::VisualKind::Char);
        ed.visual_anchor = Some(kjxlkj_core_types::CursorPosition::new(1, 0));
        ed.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(1, 3);
        ed.dispatch_visual(Key::char('p'), kjxlkj_core_types::VisualKind::Char);
        assert!(matches!(ed.mode, Mode::Normal));
    }

    /// REQ-WILDSCROLL-01: Wildmenu completions in snapshot.
    #[test]
    fn wildmenu_scroll_snapshot() {
        let mut ed = make_editor();
        // Set up many completions for testing.
        ed.cmdline.completion.candidates = (0..50).map(|i| format!("cmd{i}")).collect();
        ed.cmdline.completion.index = Some(45);
        ed.cmdline.open(':');
        let snap = ed.snapshot();
        assert_eq!(snap.cmdline.completions.len(), 50);
        assert_eq!(snap.cmdline.completion_index, Some(45));
    }

    /// REQ-EXPRCMD-01: Ctrl-R = opens expression cmdline prompt.
    #[test]
    fn expr_cmdline_prompt() {
        let mut ed = make_editor();
        ed.mode = Mode::Insert;
        ed.handle_insert_register('=');
        assert!(ed.cmdline.active);
        assert_eq!(ed.cmdline.prefix, Some('='));
    }

    /// REQ-FORMATPRG-01: formatprg option detected.
    #[test]
    fn formatprg_option() {
        let mut ed = make_editor();
        ed.options
            .set("formatprg", crate::options::OptionValue::Str("fmt".into()));
        ed.format_lines(0, 1);
        let notif = ed.notifications.last();
        assert!(notif.is_some());
        assert!(notif.unwrap().message.contains("formatprg"));
    }

    /// REQ-REGEXBRANCH-01: Magic mode \| alternation in search.
    #[test]
    fn magic_branch_search() {
        let mut ed = make_editor();
        ed.search.pattern = Some("one\\|four".to_string());
        ed.search.active = true;
        ed.search_next();
        // Cursor should land on "one" in line 0.
        assert_eq!(ed.windows.focused().cursor.line, 0);
    }

    /// REQ-SNIPPETS-01: Snippet registry and expand.
    #[test]
    fn snippet_registry() {
        let mut reg = crate::snippets::SnippetRegistry::new();
        reg.add("fn", "fn ${1}() {\n    $0\n}", "Function template");
        assert!(reg.get("fn").is_some());
        let (text, stops) = reg.expand("fn").unwrap();
        assert!(text.contains("fn"), "Expansion should contain 'fn'");
        assert!(!text.contains("$1"), "Tab stops should be stripped");
        assert!(!stops.is_empty(), "Should have tab-stop offsets");
    }
}
