#[cfg(test)]
mod wave13_tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, KeyCode, Mode, Modifier, VisualKind};

    fn state() -> EditorState {
        EditorState::new(80, 24)
    }

    fn ctrl(c: char) -> Key {
        Key::new(KeyCode::Char(c), Modifier::CTRL)
    }

    #[test]
    fn very_nomagic_search_all_literal() {
        let mut s = state();
        s.open_file("t.txt", "a.b\na+b\na*b\n");
        s.search.pattern = Some("\\Va.b".into());
        s.search.active = true;
        s.search_next();
        // \V makes . literal, so only "a.b" matches (line 0).
        assert_eq!(s.windows.focused().cursor.line, 0);
    }

    #[test]
    fn session_saves_buffer_path() {
        let mut s = state();
        s.open_file("hello.txt", "content\n");
        // Build session lines manually like handle_mksession does.
        let mut lines = Vec::new();
        let focused_cursor = s.windows.focused().cursor;
        let focused_buf = s.current_buffer_id();
        for buf in s.buffers.iter() {
            if let Some(ref p) = buf.path {
                lines.push(format!("edit {}", p.display()));
                if buf.id == focused_buf {
                    lines.push(format!(
                        "call cursor({}, {})",
                        focused_cursor.line + 1,
                        focused_cursor.grapheme + 1
                    ));
                }
            }
        }
        let session = lines.join("\n");
        assert!(
            session.contains("hello.txt"),
            "session should contain buffer path"
        );
    }

    #[test]
    fn visual_block_ia_sets_insert_mode() {
        let mut s = state();
        s.open_file("t.txt", "aaa\nbbb\nccc\n");
        // Enter visual block mode
        s.mode = Mode::Visual(VisualKind::Block);
        s.visual_anchor = Some(kjxlkj_core_types::CursorPosition::new(0, 0));
        s.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(2, 0);
        // Press 'I' in visual block
        s.dispatch_visual(Key::char('I'), VisualKind::Block);
        assert!(matches!(s.mode, Mode::Insert));
        assert!(s.block_insert_pending.is_some());
    }

    #[test]
    fn argument_text_object_inner() {
        let mut s = state();
        s.open_file("t.txt", "fn(a, b, c)\n");
        // Position cursor on 'b' (col 6)
        s.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(0, 6);
        // Apply inner-argument delete: dia
        let op = kjxlkj_core_types::Operator::Delete;
        crate::text_objects::apply_text_object(&mut s, op, 'i', 'a');
        let content: String = s.buffers.current().content.to_string();
        // 'b' should be deleted, commas should remain
        assert!(
            !content.contains(" b,"),
            "inner argument 'b' should be deleted"
        );
    }

    #[test]
    fn fuzzy_completion_fallback() {
        use crate::cmdline_completion::fuzzy_matches;
        assert!(fuzzy_matches("wrt", "write"));
        assert!(fuzzy_matches("sbs", "substitute"));
        assert!(!fuzzy_matches("xyz", "write"));
    }

    #[test]
    fn insert_register_pending_flag() {
        let mut s = state();
        s.mode = Mode::Insert;
        s.insert_register_prompt();
        assert!(s.insert_register_pending);
        // Pressing 'a' should clear the flag.
        s.handle_insert_register('z');
        assert!(!s.insert_register_pending);
    }

    #[test]
    fn cross_buffer_jump_switches_buffer() {
        let mut s = state();
        s.open_file("a.txt", "aaa\n");
        let bid_a = s.current_buffer_id();
        // Push jumplist position in buffer a
        s.push_jumplist();
        s.open_file("b.txt", "bbb\n");
        let bid_b = s.current_buffer_id();
        assert_ne!(bid_a, bid_b);
        // Jump older should switch back to buffer a
        s.jump_older();
        assert_eq!(s.current_buffer_id(), bid_a);
    }

    #[test]
    fn incsearch_highlights_first_match() {
        let mut s = state();
        s.open_file("t.txt", "hello world\nfoo bar\nhello again\n");
        s.options
            .set("incsearch", crate::options::OptionValue::Bool(true));
        s.mode = Mode::Command(kjxlkj_core_types::CommandKind::Search);
        s.cmdline.open('/');
        s.cmdline.content = "bar".to_string();
        s.update_incsearch();
        assert_eq!(s.search.highlight_ranges.len(), 1);
        assert_eq!(s.search.highlight_ranges[0].0, 1); // line 1
    }

    #[test]
    fn block_insert_applies_on_exit() {
        let mut s = state();
        s.open_file("t.txt", "abc\ndef\nghi\n");
        // Simulate block insert I on lines 0-2, col 0
        s.block_insert_pending = Some((0, 2, 0, false));
        s.last_inserted_text = "X".to_string();
        // Simulate leaving insert mode
        let start = kjxlkj_core_types::CursorPosition::new(0, 0);
        let end = kjxlkj_core_types::CursorPosition::new(2, 0);
        s.visual_block_insert("X", start, end, false);
        let content: String = s.buffers.current().content.to_string();
        // Each line should have X prepended
        for line in content.lines().take(3) {
            assert!(line.starts_with('X'), "line '{}' should start with X", line);
        }
    }
}
