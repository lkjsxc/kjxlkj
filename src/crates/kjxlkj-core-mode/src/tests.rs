//! Tests for mode module.

use super::*;
use kjxlkj_core_types::{Intent, Key, KeyCode, Mode};

mod state_tests {
    use super::*;

    #[test]
    fn test_mode_state_default() {
        let state = ModeState::new();
        assert_eq!(state.mode, Mode::Normal);
        assert!(state.count.is_none());
    }

    #[test]
    fn test_count_accumulation() {
        let mut state = ModeState::new();
        state.accumulate_count(1);
        state.accumulate_count(2);
        state.accumulate_count(3);
        assert_eq!(state.effective_count(), 123);
    }

    #[test]
    fn test_mode_transitions() {
        let mut state = ModeState::new();

        state.enter_mode(Mode::Insert);
        assert_eq!(state.mode, Mode::Insert);

        state.exit_to_normal();
        assert_eq!(state.mode, Mode::Normal);
    }
}

mod normal_tests {
    use super::*;

    #[test]
    fn test_movement_keys() {
        let mut state = ModeState::new();

        let intents = handle_normal_key(&mut state, Key::char('h'));
        assert!(intents
            .iter()
            .any(|i| matches!(i, Intent::CursorMove(_))));

        let intents = handle_normal_key(&mut state, Key::char('j'));
        assert!(!intents.is_empty());
    }

    #[test]
    fn test_count_prefix() {
        let mut state = ModeState::new();

        handle_normal_key(&mut state, Key::char('3'));
        assert_eq!(state.count, Some(3));

        let intents = handle_normal_key(&mut state, Key::char('j'));
        assert_eq!(intents.len(), 3);
    }

    #[test]
    fn test_mode_entry() {
        let mut state = ModeState::new();

        handle_normal_key(&mut state, Key::char('i'));
        assert_eq!(state.mode, Mode::Insert);
    }

    #[test]
    fn test_dd_delete_line() {
        let mut state = ModeState::new();

        handle_normal_key(&mut state, Key::char('d'));
        let intents = handle_normal_key(&mut state, Key::char('d'));

        assert!(intents.iter().any(|i| matches!(i, Intent::DeleteLine)));
    }

    #[test]
    fn test_yy_yank_line() {
        let mut state = ModeState::new();

        handle_normal_key(&mut state, Key::char('y'));
        let intents = handle_normal_key(&mut state, Key::char('y'));

        assert!(intents.iter().any(|i| matches!(i, Intent::YankLine)));
    }
}

mod insert_tests {
    use super::*;

    #[test]
    fn test_escape_exits() {
        let mut state = ModeState::new();
        state.enter_mode(Mode::Insert);

        let intents = handle_insert_key(&mut state, Key::new(KeyCode::Esc));
        assert!(intents.iter().any(|i| matches!(i, Intent::ExitToNormal)));
        assert_eq!(state.mode, Mode::Normal);
    }

    #[test]
    fn test_char_input() {
        let mut state = ModeState::new();
        state.enter_mode(Mode::Insert);

        let intents = handle_insert_key(&mut state, Key::char('a'));
        assert!(intents
            .iter()
            .any(|i| matches!(i, Intent::InsertText(s) if s == "a")));
    }
}

mod command_tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        assert!(matches!(parse_command("q"), Intent::Quit { force: false }));
        assert!(matches!(parse_command("q!"), Intent::Quit { force: true }));
        assert!(matches!(
            parse_command("w"),
            Intent::WriteBuffer { path: None, .. }
        ));
    }

    #[test]
    fn test_command_with_args() {
        match parse_command("e test.txt") {
            Intent::OpenFile(path) => {
                assert_eq!(path.to_str().unwrap(), "test.txt");
            }
            _ => panic!("Expected OpenFile"),
        }
    }

    #[test]
    fn test_external_command() {
        match parse_command("!ls -la") {
            Intent::RunExternalCommand(cmd) => {
                assert!(cmd.contains("ls"));
            }
            _ => panic!("Expected RunExternalCommand"),
        }
    }
}

mod visual_tests {
    use super::*;

    #[test]
    fn test_escape_exits_visual() {
        let mut state = ModeState::new();
        state.enter_mode(Mode::Visual);

        let intents = handle_visual_key(&mut state, Key::new(KeyCode::Esc));
        assert!(intents
            .iter()
            .any(|i| matches!(i, Intent::ClearSelection)));
        assert_eq!(state.mode, Mode::Normal);
    }
}
