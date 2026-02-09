/// Tests for EditorState.
#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Action, CursorPosition, Mode};

    #[test]
    fn test_new_editor_state() {
        let state = EditorState::new(80, 24);
        assert_eq!(state.mode, Mode::Normal);
        assert!(!state.quit_requested);
        assert_eq!(state.terminal_size, (80, 24));
    }

    #[test]
    fn test_insert_char() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_char('H');
        state.insert_char('i');
        let buf = state.buffers.current();
        assert_eq!(buf.content.to_string(), "Hi");
    }

    #[test]
    fn test_cursor_movement() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_text("hello\nworld\n");
        state.mode = Mode::Normal;
        state.windows.focused_mut().cursor = CursorPosition::new(0, 0);
        state.move_cursor_down(1);
        assert_eq!(state.windows.focused().cursor.line, 1);
        state.move_cursor_up(1);
        assert_eq!(state.windows.focused().cursor.line, 0);
    }

    #[test]
    fn test_quit_with_unsaved() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_char('x');
        state.mode = Mode::Normal;
        state.handle_action(Action::Quit);
        assert!(!state.quit_requested);
    }

    #[test]
    fn test_force_quit() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_char('x');
        state.mode = Mode::Normal;
        state.handle_action(Action::ForceQuit);
        assert!(state.quit_requested);
    }

    #[test]
    fn test_snapshot_sequence_increments() {
        let mut state = EditorState::new(80, 24);
        let s1 = state.snapshot();
        let s2 = state.snapshot();
        assert!(s2.sequence > s1.sequence);
    }

    #[test]
    fn test_open_file() {
        let mut state = EditorState::new(80, 24);
        state.open_file("/tmp/test.txt", "hello world");
        let buf = state.buffers.current();
        assert_eq!(buf.content.to_string(), "hello world");
        assert!(buf.path.is_some());
    }

    #[test]
    fn test_insert_newline() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_text("abc");
        state.insert_newline();
        state.insert_text("def");
        let buf = state.buffers.current();
        assert_eq!(buf.content.to_string(), "abc\ndef");
        assert_eq!(state.windows.focused().cursor.line, 1);
    }

    #[test]
    fn test_delete_char_backward() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_text("abc");
        state.delete_char_backward();
        let buf = state.buffers.current();
        assert_eq!(buf.content.to_string(), "ab");
    }

    #[test]
    fn test_move_to_line_start_end() {
        let mut state = EditorState::new(80, 24);
        state.mode = Mode::Insert;
        state.insert_text("hello world");
        state.mode = Mode::Normal;
        state.move_to_line_start();
        assert_eq!(state.windows.focused().cursor.grapheme, 0);
        state.move_to_line_end();
        assert!(state.windows.focused().cursor.grapheme > 0);
    }
}
