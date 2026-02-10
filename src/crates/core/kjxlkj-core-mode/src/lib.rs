//! Mode handling and dispatch.
//!
//! This crate provides mode state machines and key dispatch.

mod dispatch;
mod handler;
mod insert;
mod normal;
mod other_modes;
mod state;

pub use dispatch::*;
pub use handler::*;
pub use state::*;

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{Key, KeyEvent, Modifiers};

    fn key(c: char) -> KeyEvent {
        KeyEvent { key: Key::Char(c), modifiers: Modifiers::NONE }
    }

    fn key_ctrl(c: char) -> KeyEvent {
        KeyEvent { key: Key::Char(c), modifiers: Modifiers { ctrl: true, shift: false, alt: false } }
    }

    #[test]
    fn test_i_enters_insert() {
        let mut state = ModeState::new();
        let result = dispatch_key(&mut state, &key('i'));
        match result {
            HandleResult::Consumed(actions) => {
                assert!(actions.iter().any(|a| matches!(a, ModeAction::EnterInsert(_))));
            }
            _ => panic!("Expected consumed"),
        }
    }

    #[test]
    fn test_uppercase_a_enters_insert_eol() {
        let mut state = ModeState::new();
        let result = dispatch_key(&mut state, &key('A'));
        match result {
            HandleResult::Consumed(actions) => {
                assert!(actions.iter().any(|a| matches!(a, ModeAction::EnterInsert(InsertPosition::EndOfLine))));
            }
            _ => panic!("Expected consumed"),
        }
    }

    #[test]
    fn test_colon_enters_command() {
        let mut state = ModeState::new();
        let result = dispatch_key(&mut state, &key(':'));
        match result {
            HandleResult::Consumed(actions) => {
                assert!(actions.iter().any(|a| matches!(a, ModeAction::EnterCommand(_))));
            }
            _ => panic!("Expected consumed"),
        }
    }

    #[test]
    fn test_ctrl_w_sets_window_prefix() {
        let mut state = ModeState::new();
        let result = dispatch_key(&mut state, &key_ctrl('w'));
        assert!(matches!(result, HandleResult::Pending));
        assert_eq!(state.pending_prefix, PendingPrefix::Window);
    }

    #[test]
    fn test_g_sets_g_prefix() {
        let mut state = ModeState::new();
        let result = dispatch_key(&mut state, &key('g'));
        assert!(matches!(result, HandleResult::Pending));
        assert_eq!(state.pending_prefix, PendingPrefix::G);
    }

    #[test]
    fn test_gg_goes_to_line_1() {
        let mut state = ModeState::new();
        dispatch_key(&mut state, &key('g')); // Set G prefix.
        let result = dispatch_key(&mut state, &key('g')); // Complete gg.
        match result {
            HandleResult::Consumed(actions) => {
                assert!(actions.iter().any(|a| matches!(a, ModeAction::MoveCursor(_, _))));
            }
            _ => panic!("Expected consumed"),
        }
    }
}
