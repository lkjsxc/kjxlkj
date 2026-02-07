//! Normal mode extensions: g-prefix commands, leader keys, and misc.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    Direction, EditorAction, KeyCode, KeyEvent, Mode, Motion,
};

use crate::normal_mode::{motion_action, take_count};

/// Handle `g`-prefixed commands. Called after `g` is pressed.
/// In a real implementation this would wait for the next key;
/// here we read it from an internal mini-state for simplicity.
pub fn handle_normal_g_key(_state: &mut EditorState) -> Option<EditorAction> {
    // In the facade, `g` alone doesn't produce an action.
    // The full key sequence handling is done via pending state.
    // For the crate API, we expose the sub-dispatch directly:
    // Callers can invoke `dispatch_g_sequence(state, second_key)`.
    None
}

/// Dispatch the second key after `g`.
pub fn dispatch_g_sequence(
    state: &mut EditorState,
    key: &KeyEvent,
) -> Option<EditorAction> {
    match &key.code {
        KeyCode::Char('g') => {
            // gg — go to first line (or {count}gg)
            let line = take_count(state).map(|n| n.saturating_sub(1)).unwrap_or(0);
            motion_action(state, Motion::GoToLine(line))
        }
        KeyCode::Char('e') => motion_action(state, Motion::PrevWordEnd),
        KeyCode::Char('E') => motion_action(state, Motion::PrevBigWordEnd),
        KeyCode::Char('_') => motion_action(state, Motion::LastNonBlank),
        KeyCode::Char('m') => motion_action(state, Motion::LineMiddle),
        KeyCode::Char('J') => Some(EditorAction::JoinLine(true)), // gJ no spaces
        KeyCode::Char('U') => Some(EditorAction::UpperCase),
        KeyCode::Char('u') => Some(EditorAction::LowerCase),
        KeyCode::Char('~') => Some(EditorAction::ToggleCase),
        KeyCode::Char('v') => {
            // gv: reselect last visual selection
            Some(EditorAction::ChangeMode(Mode::Visual))
        }
        KeyCode::Char('p') => Some(EditorAction::Paste(Direction::Forward)),
        KeyCode::Char('P') => Some(EditorAction::Paste(Direction::Backward)),
        KeyCode::Char(';') => None, // g; change list backward (stub)
        KeyCode::Char(',') => None, // g, change list forward (stub)
        _ => None,
    }
}

/// Handle Space-prefixed leader key dispatch.
pub fn handle_leader_key(
    state: &mut EditorState,
    key: &KeyEvent,
) -> Option<EditorAction> {
    match &key.code {
        KeyCode::Char('e') => {
            state.set_message("[explorer toggle]");
            None
        }
        KeyCode::Char('t') => {
            state.set_message("[terminal toggle]");
            None
        }
        KeyCode::Char('f') => {
            state.set_message("[find files]");
            None
        }
        KeyCode::Char('g') => {
            state.set_message("[live grep]");
            None
        }
        KeyCode::Char('b') => {
            state.set_message("[buffer list]");
            None
        }
        KeyCode::Char('u') => {
            state.set_message("[undo tree]");
            None
        }
        _ => None,
    }
}

/// Handle ZZ and ZQ sequences.
pub fn dispatch_z_sequence(
    _state: &mut EditorState,
    key: &KeyEvent,
) -> Option<EditorAction> {
    match &key.code {
        KeyCode::Char('Z') => Some(EditorAction::WriteQuit(None)),
        KeyCode::Char('Q') => Some(EditorAction::ForceQuit),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Position;

    #[test]
    fn g_key_returns_none() {
        let mut state = EditorState::new();
        let r = handle_normal_g_key(&mut state);
        assert_eq!(r, None);
    }

    #[test]
    fn gg_goes_to_start() {
        let mut state = EditorState::new();
        state.active_buffer_mut().insert_text(Position::ZERO, "a\nb\nc\n");
        state.active_window_mut().cursor.line = 2;
        dispatch_g_sequence(&mut state, &KeyEvent::char('g'));
        assert_eq!(state.active_window().cursor.line, 0);
    }

    #[test]
    fn gj_joins_no_spaces() {
        let mut state = EditorState::new();
        let action = dispatch_g_sequence(&mut state, &KeyEvent::char('J'));
        assert_eq!(action, Some(EditorAction::JoinLine(true)));
    }

    #[test]
    fn gu_lowercase() {
        let mut state = EditorState::new();
        let action = dispatch_g_sequence(&mut state, &KeyEvent::char('u'));
        assert_eq!(action, Some(EditorAction::LowerCase));
    }

    #[test]
    fn leader_e_explorer() {
        let mut state = EditorState::new();
        handle_leader_key(&mut state, &KeyEvent::char('e'));
        assert_eq!(state.message, Some("[explorer toggle]".to_string()));
    }

    #[test]
    fn leader_f_find() {
        let mut state = EditorState::new();
        handle_leader_key(&mut state, &KeyEvent::char('f'));
        assert_eq!(state.message, Some("[find files]".to_string()));
    }

    #[test]
    fn zz_write_quit() {
        let mut state = EditorState::new();
        let action = dispatch_z_sequence(&mut state, &KeyEvent::char('Z'));
        assert_eq!(action, Some(EditorAction::WriteQuit(None)));
    }

    #[test]
    fn zq_force_quit() {
        let mut state = EditorState::new();
        let action = dispatch_z_sequence(&mut state, &KeyEvent::char('Q'));
        assert_eq!(action, Some(EditorAction::ForceQuit));
    }

    #[test]
    fn gv_reselect() {
        let mut state = EditorState::new();
        let action = dispatch_g_sequence(&mut state, &KeyEvent::char('v'));
        assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Visual)));
    }

    #[test]
    fn leader_unknown_noop() {
        let mut state = EditorState::new();
        let r = handle_leader_key(&mut state, &KeyEvent::char('z'));
        assert_eq!(r, None);
    }
}
