//! Wave 14 tests: wildmenu, block change, search count,
//! macro edit, global marks, range swap, class text obj, expr prompt.
#[cfg(test)]
mod wave14_tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode};

    fn make_editor() -> EditorState {
        let mut ed = EditorState::new(80, 24);
        let text = "line one\nline two\nline three\nline four\nline five\n";
        ed.open_file("test.txt", text);
        ed
    }

    /// REQ-WILDMENU-01: Completions surface in snapshot.
    #[test]
    fn wildmenu_snapshot() {
        let mut ed = make_editor();
        let snap = ed.snapshot();
        // Fresh editor: no completions.
        assert!(snap.cmdline.completions.is_empty());
        assert_eq!(snap.cmdline.completion_index, None);
    }

    /// REQ-BLOCKCHG-01: Block change sets pending insert.
    #[test]
    fn block_change_pending() {
        let mut ed = make_editor();
        // Enter visual block, select 3 lines, change.
        ed.mode = Mode::Visual(kjxlkj_core_types::VisualKind::Block);
        ed.visual_anchor = Some(kjxlkj_core_types::CursorPosition::new(0, 0));
        ed.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(2, 3);
        ed.dispatch_visual(Key::char('c'), kjxlkj_core_types::VisualKind::Block);
        assert!(ed.block_insert_pending.is_some());
        assert!(matches!(ed.mode, Mode::Insert));
    }

    /// REQ-SRCHCOUNT-01: Search count is populated after search.
    #[test]
    fn search_count_populated() {
        let mut ed = make_editor();
        ed.search.pattern = Some("line".to_string());
        ed.search.active = true;
        ed.search_next();
        assert!(ed.search.match_count.is_some());
        let (_, total) = ed.search.match_count.unwrap();
        assert!(total >= 4, "Expected >= 4 matches for 'line'");
    }

    /// REQ-MACROEDIT-01: sync_macro_to_register / sync_register_to_macro.
    #[test]
    fn macro_register_sync() {
        let mut ed = make_editor();
        // Record a short macro.
        ed.start_recording('a');
        ed.record_key(&Key::char('j'));
        ed.record_key(&Key::char('j'));
        ed.stop_recording();
        // Sync to register.
        ed.sync_macro_to_register('a');
        let reg = ed.registers.get(kjxlkj_core_edit::RegisterName::Named('a'));
        assert!(reg.is_some());
        assert_eq!(reg.unwrap().content, "jj");
        // Modify register and sync back.
        ed.registers.set(
            kjxlkj_core_edit::RegisterName::Named('a'),
            kjxlkj_core_edit::Register::new("kkk".to_string(), false),
        );
        ed.sync_register_to_macro('a');
        assert_eq!(ed.macro_store.get(&'a').unwrap().len(), 3);
    }

    /// REQ-GLOBALMARKS-01: Global mark stores buffer id.
    #[test]
    fn global_mark_buffer() {
        let mut ed = make_editor();
        ed.set_mark_at_cursor('A');
        let buf_id = ed.current_buffer_id();
        let pos = ed.marks.get('A', buf_id.0 as usize).copied();
        assert!(pos.is_some());
        assert_eq!(pos.unwrap().buffer_id, buf_id.0 as usize);
    }

    /// REQ-RANGEPROMPT-01: Backwards range is swapped, not rejected.
    #[test]
    fn backwards_range_swaps() {
        let mut ed = make_editor();
        ed.execute_ex_command("3,1d");
        let has_err = ed.notifications.iter().any(|n| n.message.contains("E493"));
        assert!(!has_err, "Backwards range should swap, not error");
    }

    /// REQ-CLASSTOBJ-01: Class text object resolves braces.
    #[test]
    fn class_text_object_braces() {
        let code = "fn foo() {\n  body\n}\n";
        let rope = kjxlkj_core_text::Rope::from(code);
        let pos = kjxlkj_core_types::CursorPosition::new(1, 0);
        let range = crate::text_objects_class::resolve_class_or_func(
            crate::text_objects::TextObjectKind::Inner, 'c', pos, &rope,
        );
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.line, 1);
        assert_eq!(r.end.line, 1);
        assert!(r.linewise);
    }

    /// REQ-EXPRPROMPT-01: Expression register evaluates arithmetic.
    #[test]
    fn expr_register_eval() {
        let mut ed = make_editor();
        // Put cursor at start, enter insert mode.
        ed.mode = Mode::Insert;
        ed.last_ex_command = "2+3".to_string();
        ed.handle_insert_register('=');
        // The result "5" should be inserted as text.
        let buf_id = ed.current_buffer_id();
        let text = ed.buffers.get(buf_id).unwrap().content.to_string();
        assert!(text.contains('5'), "Expected '5' inserted from expr eval");
    }
}
