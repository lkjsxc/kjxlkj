/// Extended tests for EditorState.
#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Action, CommandKind, CursorPosition, Key, Mode};

    #[test]
    fn test_handle_key_normal_j() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_text("line1\nline2\nline3");
        state.mode = Mode::Normal;
        state.windows.focused_mut().cursor = CursorPosition::new(0, 0);
        state.handle_action(Action::MoveDown(1));
        assert_eq!(state.windows.focused().cursor.line, 1);
    }

    #[test]
    fn test_handle_action_resize() {
        let mut state = EditorState::new(80, 24);
        state.handle_action(Action::Resize(120, 40));
        assert_eq!(state.terminal_size, (120, 40));
    }

    #[test]
    fn test_write_quit() {
        let mut state = EditorState::new(80, 24);
        state.handle_action(Action::WriteQuit);
        assert!(state.quit_requested);
    }

    #[test]
    fn test_enter_command_mode() {
        let mut state = EditorState::new(80, 24);
        state.handle_action(Action::EnterCommandEx);
        assert!(matches!(state.mode, Mode::Command(CommandKind::Ex)));
        assert!(state.cmdline.active);
    }

    #[test]
    fn test_cmdline_execute_quit() {
        let mut state = EditorState::new(80, 24);
        state.handle_action(Action::EnterCommandEx);
        state.cmdline.insert_char('q');
        state.cmdline.insert_char('!');
        state.execute_cmdline();
        assert!(state.quit_requested);
    }

    #[test]
    fn test_mode_transition_to_insert() {
        let mut state = EditorState::new(80, 24);
        let key = Key::char('i');
        state.handle_key(key);
        assert_eq!(state.mode, Mode::Insert);
    }

    #[test]
    fn test_mode_transition_insert_to_normal() {
        let mut state = EditorState::new(80, 24);
        state.handle_key(Key::char('i'));
        assert_eq!(state.mode, Mode::Insert);
        state.handle_key(Key::esc());
        assert_eq!(state.mode, Mode::Normal);
    }

    #[test]
    fn test_join_lines() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_text("abc\ndef");
        state.mode = Mode::Normal;
        state.windows.focused_mut().cursor = CursorPosition::new(0, 0);
        state.join_lines(true);
        let buf = state.buffers.current();
        assert_eq!(buf.content.to_string(), "abc def");
    }

    #[test]
    fn test_delete_lines() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_text("line1\nline2\nline3\n");
        state.mode = Mode::Normal;
        state.windows.focused_mut().cursor = CursorPosition::new(1, 0);
        state.delete_lines(1);
        let buf = state.buffers.current();
        assert!(!buf.content.to_string().contains("line2"));
    }

    #[test]
    fn test_yank_put() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_text("line1\nline2\nline3\n");
        state.mode = Mode::Normal;
        state.windows.focused_mut().cursor = CursorPosition::new(0, 0);
        state.yank_lines(1);
        assert!(state.registers.get_unnamed().is_some());
    }
}
