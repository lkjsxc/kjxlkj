//! Replace mode key handling.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{EditorAction, KeyCode, KeyEvent, Mode, Position};

/// Handle a key event in Replace mode.
pub fn handle_replace_key(state: &mut EditorState, key: KeyEvent) -> Option<EditorAction> {
    if state.macro_state.is_recording() {
        state.macro_state.record_key(key.clone());
    }

    match &key.code {
        KeyCode::Escape => {
            let col = state.active_window().cursor.col;
            if col > 0 {
                state.active_window_mut().cursor.col = col - 1;
            }
            Some(EditorAction::ChangeMode(Mode::Normal))
        }
        KeyCode::Char(ch) => {
            replace_char_at_cursor(state, *ch);
            None
        }
        KeyCode::Backspace => {
            let col = state.active_window().cursor.col;
            if col > 0 {
                state.active_window_mut().cursor.col = col - 1;
            }
            None
        }
        KeyCode::Enter => {
            let pos = state.active_window().cursor;
            state.active_buffer_mut().insert_char(pos, '\n');
            state.active_window_mut().cursor = Position::new(pos.line + 1, 0);
            None
        }
        KeyCode::Left => {
            let col = state.active_window().cursor.col;
            state.active_window_mut().cursor.col = col.saturating_sub(1);
            None
        }
        KeyCode::Right => {
            state.active_window_mut().cursor.col += 1;
            None
        }
        _ => None,
    }
}

/// Replace the character under the cursor. If at end-of-line, insert instead.
fn replace_char_at_cursor(state: &mut EditorState, ch: char) {
    let pos = state.active_window().cursor;
    let line_len = state.active_buffer().line_len(pos.line);

    if pos.col < line_len {
        // Delete current char, then insert new one
        let end = Position::new(pos.line, pos.col + 1);
        state.active_buffer_mut().delete_range(pos, end);
        state.active_buffer_mut().insert_char(pos, ch);
    } else {
        // At EOL: insert
        state.active_buffer_mut().insert_char(pos, ch);
    }
    state.active_window_mut().cursor.col += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_replace() -> EditorState {
        let mut state = EditorState::new();
        state
            .active_buffer_mut()
            .insert_text(Position::ZERO, "hello");
        state.mode.transition(Mode::Replace);
        state
    }

    #[test]
    fn escape_returns_to_normal() {
        let mut state = setup_replace();
        state.active_window_mut().cursor.col = 2;
        let action = handle_replace_key(&mut state, KeyEvent::plain(KeyCode::Escape));
        assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Normal)));
        assert_eq!(state.active_window().cursor.col, 1);
    }

    #[test]
    fn char_replaces() {
        let mut state = setup_replace();
        state.active_window_mut().cursor.col = 0;
        handle_replace_key(&mut state, KeyEvent::char('X'));
        assert_eq!(state.active_buffer().line(0), Some("Xello".to_string()));
        assert_eq!(state.active_window().cursor.col, 1);
    }

    #[test]
    fn char_at_eol_inserts() {
        let mut state = setup_replace();
        state.active_window_mut().cursor.col = 5; // past end
        handle_replace_key(&mut state, KeyEvent::char('!'));
        assert_eq!(state.active_buffer().line(0), Some("hello!".to_string()));
    }

    #[test]
    fn backspace_moves_left() {
        let mut state = setup_replace();
        state.active_window_mut().cursor.col = 3;
        handle_replace_key(&mut state, KeyEvent::plain(KeyCode::Backspace));
        assert_eq!(state.active_window().cursor.col, 2);
    }

    #[test]
    fn backspace_at_zero_stays() {
        let mut state = setup_replace();
        handle_replace_key(&mut state, KeyEvent::plain(KeyCode::Backspace));
        assert_eq!(state.active_window().cursor.col, 0);
    }

    #[test]
    fn enter_inserts_newline() {
        let mut state = setup_replace();
        state.active_window_mut().cursor.col = 2;
        handle_replace_key(&mut state, KeyEvent::plain(KeyCode::Enter));
        assert_eq!(state.active_window().cursor.line, 1);
        assert_eq!(state.active_window().cursor.col, 0);
    }

    #[test]
    fn replace_multiple_chars() {
        let mut state = setup_replace();
        state.active_window_mut().cursor.col = 0;
        handle_replace_key(&mut state, KeyEvent::char('A'));
        handle_replace_key(&mut state, KeyEvent::char('B'));
        assert_eq!(state.active_buffer().line(0), Some("ABllo".to_string()));
        assert_eq!(state.active_window().cursor.col, 2);
    }

    #[test]
    fn arrow_keys() {
        let mut state = setup_replace();
        state.active_window_mut().cursor.col = 2;
        handle_replace_key(&mut state, KeyEvent::plain(KeyCode::Left));
        assert_eq!(state.active_window().cursor.col, 1);
        handle_replace_key(&mut state, KeyEvent::plain(KeyCode::Right));
        assert_eq!(state.active_window().cursor.col, 2);
    }
}
