#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode};

    fn state() -> EditorState {
        EditorState::new(80, 24)
    }

    #[test]
    fn jumplist_ctrl_o_and_ctrl_i() {
        let mut s = state();
        s.open_file("test.txt", "line1\nline2\nline3\nline4\nline5\n");
        // Move down and push jumplist via search
        s.search.pattern = Some("line3".into());
        s.search.active = true;
        s.search_next(); // jumps to line3
        assert!(s.windows.focused().cursor.line >= 2);
        // Ctrl-O goes back
        s.handle_action(kjxlkj_core_types::Action::JumpOlder);
        assert_eq!(s.windows.focused().cursor.line, 0);
        // Ctrl-I goes forward
        s.handle_action(kjxlkj_core_types::Action::JumpNewer);
        assert!(s.windows.focused().cursor.line >= 2);
    }

    #[test]
    fn jumps_command_lists_entries() {
        let mut s = state();
        s.open_file("t.txt", "a\nb\nc\n");
        s.push_jumplist();
        s.move_cursor_down(1);
        s.push_jumplist();
        s.execute_ex_command("jumps");
        let msg = s.notifications.last().unwrap().message.clone();
        assert!(msg.contains("Jump List"));
    }

    #[test]
    fn alternate_file_register() {
        let mut s = state();
        s.open_file("first.txt", "aaa\n");
        s.open_file("second.txt", "bbb\n");
        // alternate_buffer was set when opening second file
        assert!(s.alternate_buffer.is_some());
        // Read # register via put: set pending_register and put
        s.pending_register = Some('#');
        // Use put_before to exercise read_register
        s.handle_key(Key::char('P'));
        let buf = s.buffers.get(s.current_buffer_id()).unwrap();
        let text = buf.content.to_string();
        assert!(text.contains("first.txt"));
    }

    #[test]
    fn count_prefixed_macro_playback() {
        let mut s = state();
        s.open_file("t.txt", "aaa\n");
        // Record macro that inserts 'X' at start
        s.start_recording('a');
        s.handle_key(Key::char('I'));
        s.handle_key(Key::char('X'));
        s.handle_key(Key::esc());
        s.stop_recording();
        // Play 3 times
        s.play_macro('a', 3);
        let buf = s.buffers.get(s.current_buffer_id()).unwrap();
        let text = buf.content.to_string();
        assert!(text.starts_with("XXX"));
    }

    #[test]
    fn noh_clears_search_highlight() {
        let mut s = state();
        s.open_file("t.txt", "hello world\n");
        s.search.pattern = Some("hello".into());
        s.search.active = true;
        assert!(s.search.active);
        s.execute_ex_command("noh");
        assert!(!s.search.active);
    }

    #[test]
    fn nohlsearch_alias() {
        let mut s = state();
        s.search.active = true;
        s.execute_ex_command("nohlsearch");
        assert!(!s.search.active);
    }

    #[test]
    fn nomagic_search_literal_dots() {
        let mut s = state();
        s.open_file("t.txt", "a.b\naXb\n");
        // With \M prefix, . should be literal
        s.search.pattern = Some("\\Ma.b".into());
        s.search.active = true;
        s.search_next();
        // Should find "a.b" on line 0
        assert_eq!(s.windows.focused().cursor.line, 0);
    }

    #[test]
    fn visual_block_in_render_selection() {
        let mut s = state();
        s.open_file("t.txt", "abcde\nfghij\nklmno\n");
        s.mode = Mode::Visual(kjxlkj_core_types::VisualKind::Block);
        s.visual_anchor = Some(kjxlkj_core_types::CursorPosition::new(0, 1));
        s.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(2, 3);
        let snap = s.snapshot();
        let tab = &snap.tabs[0];
        let ws = tab.windows.values().next().unwrap();
        let vs = ws.visual_selection.as_ref().unwrap();
        assert_eq!(vs.kind, kjxlkj_core_types::VisualKind::Block);
    }

    #[test]
    fn modeline_parsing_sets_option() {
        let mut s = state();
        // Open a file with a modeline
        s.open_file("t.txt", "\" vim: set tabstop=4:\nhello\n");
        assert_eq!(s.options.get_int("tabstop"), 4);
    }

    #[test]
    fn session_saves_layout() {
        let mut s = state();
        s.open_file("t.txt", "data\n");
        s.split_horizontal();
        // Save session
        let tmp = "/tmp/kjxlkj_test_wave12_session.vim";
        s.handle_mksession(Some(tmp));
        let content = std::fs::read_to_string(tmp).unwrap();
        assert!(content.contains("split"));
        let _ = std::fs::remove_file(tmp);
    }
}
