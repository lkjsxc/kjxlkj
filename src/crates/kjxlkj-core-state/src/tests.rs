//! Tests for state module.

use super::*;
use kjxlkj_core_types::{Key, KeyCode, Mode, RegisterContent, RegisterName};

mod editor_tests {
    use super::*;

    #[test]
    fn test_editor_state_new() {
        let state = EditorState::new();
        assert_eq!(state.mode(), Mode::Normal);
        assert!(!state.should_quit());
    }

    #[test]
    fn test_handle_movement() {
        let mut state = EditorState::new();

        // Insert some text first
        state.handle_key(Key::char('i')).unwrap();
        state.handle_key(Key::char('h')).unwrap();
        state.handle_key(Key::char('e')).unwrap();
        state.handle_key(Key::char('l')).unwrap();
        state.handle_key(Key::char('l')).unwrap();
        state.handle_key(Key::char('o')).unwrap();
        state.handle_key(Key::new(KeyCode::Esc)).unwrap();

        // Move left
        state.handle_key(Key::char('h')).unwrap();

        let snap = state.snapshot();
        // Cursor should have moved
        assert!(snap.active_window.cursor.col() < 5);
    }

    #[test]
    fn test_mode_switching() {
        let mut state = EditorState::new();

        // Enter insert mode
        state.handle_key(Key::char('i')).unwrap();
        assert_eq!(state.mode(), Mode::Insert);

        // Exit to normal
        state.handle_key(Key::new(KeyCode::Esc)).unwrap();
        assert_eq!(state.mode(), Mode::Normal);

        // Enter command mode
        state.handle_key(Key::char(':')).unwrap();
        assert_eq!(state.mode(), Mode::Command);
    }

    #[test]
    fn test_quit_command() {
        let mut state = EditorState::new();

        state.handle_key(Key::char(':')).unwrap();
        state.handle_key(Key::char('q')).unwrap();
        state.handle_key(Key::new(KeyCode::Enter)).unwrap();

        assert!(state.should_quit());
    }

    #[test]
    fn test_insert_text() {
        let mut state = EditorState::new();

        state.handle_key(Key::char('i')).unwrap();
        state.handle_key(Key::char('a')).unwrap();
        state.handle_key(Key::char('b')).unwrap();
        state.handle_key(Key::char('c')).unwrap();
        state.handle_key(Key::new(KeyCode::Esc)).unwrap();

        let snap = state.snapshot();
        assert!(snap.active_window.lines[0].text.contains("abc"));
    }
}

mod register_tests {
    use super::*;

    #[test]
    fn test_register_store() {
        let mut store = RegisterStore::new();

        store.set(
            RegisterName::Unnamed,
            RegisterContent::char("hello"),
        );

        let content = store.get(RegisterName::Unnamed).unwrap();
        assert_eq!(content.text, "hello");
    }

    #[test]
    fn test_black_hole_register() {
        let mut store = RegisterStore::new();

        store.set(
            RegisterName::BlackHole,
            RegisterContent::char("gone"),
        );

        assert!(store.get(RegisterName::BlackHole).is_none());
    }

    #[test]
    fn test_uppercase_append() {
        let mut store = RegisterStore::new();

        store.set(
            RegisterName::Named('a'),
            RegisterContent::char("hello"),
        );
        store.set(
            RegisterName::Named('A'),
            RegisterContent::char(" world"),
        );

        let content = store.get(RegisterName::Named('a')).unwrap();
        assert_eq!(content.text, "hello world");
    }
}
