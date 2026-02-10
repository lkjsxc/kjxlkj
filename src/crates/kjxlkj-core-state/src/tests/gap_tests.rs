//! Tests for gap-closure features.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, KeyCode, KeyModifiers, Mode, Operator, VisualKind};
    use std::path::PathBuf;

    #[test]
    fn replace_char_overwrites_and_advances() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        for c in "abc".chars() {
            state.do_insert_char(c);
        }
        state.change_mode(Mode::Normal);
        state.do_motion(kjxlkj_core_types::MotionAction::LineStart);
        state.change_mode(Mode::Replace);
        state.do_replace_char('X');
        state.change_mode(Mode::Normal);
        state.do_motion(kjxlkj_core_types::MotionAction::LineStart);
        let bid = state.active_buffer_id().unwrap();
        let content = state.buffers.get(&bid).unwrap().to_string_content();
        assert!(
            content.starts_with('X'),
            "first char should be X, got: {content}"
        );
    }
    #[test]
    fn replace_backspace_restores_original() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        for c in "abc".chars() {
            state.do_insert_char(c);
        }
        state.change_mode(Mode::Normal);
        state.do_motion(kjxlkj_core_types::MotionAction::LineStart);
        state.change_mode(Mode::Replace);
        state.do_replace_char('X');
        state.do_replace_backspace();
        state.change_mode(Mode::Normal);
        state.do_motion(kjxlkj_core_types::MotionAction::LineStart);
        let bid = state.active_buffer_id().unwrap();
        let content = state.buffers.get(&bid).unwrap().to_string_content();
        assert!(
            content.starts_with('a'),
            "should restore 'a', got: {content}"
        );
    }
    #[test]
    fn visual_delete_removes_selection() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        for c in "hello world".chars() {
            state.do_insert_char(c);
        }
        state.change_mode(Mode::Normal);
        state.do_motion(kjxlkj_core_types::MotionAction::LineStart);
        state.change_mode(Mode::Visual(VisualKind::Char));
        for _ in 0..4 {
            state.do_motion(kjxlkj_core_types::MotionAction::Right);
        }
        state.do_visual_operator(Operator::Delete);
        let bid = state.active_buffer_id().unwrap();
        let content = state.buffers.get(&bid).unwrap().to_string_content();
        assert!(
            content.starts_with(" world"),
            "should delete 'hello', got: {content}"
        );
        assert_eq!(state.mode, Mode::Normal);
    }
    #[test]
    fn visual_yank_preserves_text() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        for c in "abc".chars() {
            state.do_insert_char(c);
        }
        state.change_mode(Mode::Normal);
        state.do_motion(kjxlkj_core_types::MotionAction::LineStart);
        state.change_mode(Mode::Visual(VisualKind::Char));
        state.do_motion(kjxlkj_core_types::MotionAction::Right);
        state.do_visual_operator(Operator::Yank);
        let bid = state.active_buffer_id().unwrap();
        let content = state.buffers.get(&bid).unwrap().to_string_content();
        assert!(content.contains("abc"), "yank must not modify buffer");
        assert_eq!(state.registers.read('"'), "ab");
    }
    #[test]
    fn visual_dispatch_d_produces_visual_operator() {
        use kjxlkj_core_mode::ModeDispatcher;
        let mut disp = ModeDispatcher::new();
        let key = Key::new(KeyCode::Char('d'), KeyModifiers::NONE);
        let result = disp.dispatch(&key, &Mode::Visual(VisualKind::Char));
        match result {
            kjxlkj_core_mode::DispatchResult::Action(
                kjxlkj_core_types::KeyAction::VisualOperator(Operator::Delete),
            ) => {}
            other => panic!("expected VisualOperator(Delete), got: {other:?}"),
        }
    }
    #[test]
    fn visual_dispatch_esc_exits_visual() {
        use kjxlkj_core_mode::ModeDispatcher;
        let mut disp = ModeDispatcher::new();
        let key = Key::new(KeyCode::Esc, KeyModifiers::NONE);
        let result = disp.dispatch(&key, &Mode::Visual(VisualKind::Char));
        match result {
            kjxlkj_core_mode::DispatchResult::ModeChange(Mode::Normal) => {}
            other => panic!("expected ModeChange(Normal), got: {other:?}"),
        }
    }
    #[test]
    fn explorer_expand_collapse() {
        let dir = std::env::temp_dir().join("kjxlkj_test_explorer");
        let _ = std::fs::create_dir_all(dir.join("subdir"));
        let _ = std::fs::write(dir.join("file.txt"), "test");
        let _ = std::fs::write(dir.join("subdir/child.txt"), "child");

        let mut state = EditorState::new(80, 24);
        state.explorer.root = dir.clone();
        state.explorer.refresh();
        assert!(!state.explorer.entries.is_empty(), "should list entries");
        // Find the subdir entry
        let subdir_idx = state
            .explorer
            .entries
            .iter()
            .position(|e| e.name == "subdir")
            .expect("subdir should exist");
        state.explorer.selected = subdir_idx;
        // Expand
        state.explorer.expand_or_open();
        let child_exists = state.explorer.entries.iter().any(|e| e.name == "child.txt");
        assert!(child_exists, "child.txt should be visible after expand");
        // Collapse
        state.explorer.collapse_or_parent();
        let child_gone = !state.explorer.entries.iter().any(|e| e.name == "child.txt");
        assert!(child_gone, "child.txt should be hidden after collapse");

        let _ = std::fs::remove_dir_all(&dir);
    }
    #[test]
    fn explorer_key_dispatch_jk() {
        let mut state = EditorState::new(80, 24);
        state.explorer.add_entry(PathBuf::from("/a"), false);
        state.explorer.add_entry(PathBuf::from("/b"), false);
        assert_eq!(state.explorer.selected, 0);
        let j = Key::new(KeyCode::Char('j'), KeyModifiers::NONE);
        state.dispatch_explorer_key(&j);
        assert_eq!(state.explorer.selected, 1);
        let k = Key::new(KeyCode::Char('k'), KeyModifiers::NONE);
        state.dispatch_explorer_key(&k);
        assert_eq!(state.explorer.selected, 0);
    }
    #[test]
    fn terminal_resize_updates_size() {
        use kjxlkj_service_terminal::TerminalService;
        let mut svc = TerminalService::new();
        let id = svc.spawn(80, 24);
        // Resize
        for inst in svc.terminals.values_mut() {
            inst.resize(120, 50);
        }
        let inst = svc.terminals.get(&id).unwrap();
        assert_eq!(inst.screen.cols, 120);
        assert_eq!(inst.screen.rows, 50);
    }
    #[test]
    fn session_save_load_roundtrip() {
        let state = EditorState::new(80, 24);
        let session = state.session_save();
        let json = session.to_json().expect("serialize");
        let loaded = crate::session::SessionData::from_json(&json).expect("deser");
        assert_eq!(loaded.version, 1);
        assert!(!loaded.tabs.is_empty());
    }
    #[test]
    fn write_with_path_creates_file() {
        let dir = std::env::temp_dir().join("kjxlkj_test_write_path");
        let _ = std::fs::create_dir_all(&dir);
        let target = dir.join("output.txt");
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        for c in "hello".chars() {
            state.do_insert_char(c);
        }
        state.change_mode(Mode::Normal);
        state.execute_ex(&format!("w {}", target.display()));
        assert!(target.exists(), "file should be created");
        let content = std::fs::read_to_string(&target).unwrap();
        assert!(content.contains("hello"), "file should contain 'hello'");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
