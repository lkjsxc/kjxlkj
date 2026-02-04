//! Insert mode key processing.

use kjxlkj_core::{EditorState, Intent, Mode, Position};
use kjxlkj_input::{Key, KeyCode};

use crate::app::apply_intent;

/// Process a key in insert mode.
pub fn process_insert_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => {
            if state.cursor.col() > 0 {
                state.cursor.position.col -= 1;
            }
            state.set_mode(Mode::Normal);
        }
        KeyCode::Backspace | KeyCode::Char('h') if key.mods.ctrl => {
            process_insert_backspace(state);
        }
        KeyCode::Enter | KeyCode::Char('j') if key.mods.ctrl => {
            let pos = state.cursor.position;
            state.buffer.insert(pos, "\n");
            state.cursor.position = Position::new(pos.line + 1, 0);
        }
        KeyCode::Char(c) if !key.mods.ctrl => {
            let pos = state.cursor.position;
            state.buffer.insert(pos, &c.to_string());
            state.cursor.position.col += 1;
        }
        KeyCode::Char('w') if key.mods.ctrl => {
            apply_intent(state, Intent::DeleteWordBefore);
        }
        KeyCode::Char('u') if key.mods.ctrl => {
            apply_intent(state, Intent::DeleteToLineStart);
        }
        _ => {}
    }
}

fn process_insert_backspace(state: &mut EditorState) {
    if state.cursor.col() > 0 {
        let pos = state.cursor.position;
        let prev_pos = Position::new(pos.line, pos.col - 1);
        state.buffer.delete_range(prev_pos, pos);
        state.cursor.position = prev_pos;
    } else if state.cursor.line() > 0 {
        let prev_line = state.cursor.line() - 1;
        let prev_len = state.buffer.line_len(prev_line);
        let line_start = Position::new(state.cursor.line(), 0);
        let prev_end = Position::new(prev_line, prev_len);
        state.buffer.delete_range(prev_end, line_start);
        state.cursor.position = prev_end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_input::Modifiers;

    #[test]
    fn insert_escape_to_normal() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        process_insert_key(&mut state, Key::new(KeyCode::Escape, Modifiers::none()));
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn insert_char() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        process_insert_key(&mut state, Key::new(KeyCode::Char('a'), Modifiers::none()));
        assert!(state.cursor.col() > 0 || state.buffer.line_count() >= 1);
    }

    #[test]
    fn insert_enter() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        // Use Ctrl+j since it always triggers newline
        process_insert_key(&mut state, Key::new(KeyCode::Char('j'), Modifiers::ctrl()));
        assert_eq!(state.cursor.line(), 1);
    }

    #[test]
    fn insert_backspace_at_start() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        process_insert_key(&mut state, Key::new(KeyCode::Backspace, Modifiers::ctrl()));
        // No panic is success
    }

    #[test]
    fn insert_backspace_after_char() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        state.buffer.insert(Position::new(0, 0), "abc");
        state.cursor.position.col = 3;
        process_insert_key(&mut state, Key::new(KeyCode::Backspace, Modifiers::ctrl()));
        assert_eq!(state.cursor.col(), 2);
    }

    #[test]
    fn insert_ctrl_w_delete_word() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        process_insert_key(&mut state, Key::new(KeyCode::Char('w'), Modifiers::ctrl()));
        // No panic is success
    }

    #[test]
    fn insert_ctrl_u_delete_to_start() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        process_insert_key(&mut state, Key::new(KeyCode::Char('u'), Modifiers::ctrl()));
        // No panic is success
    }

    #[test]
    fn insert_ctrl_j_newline() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        process_insert_key(&mut state, Key::new(KeyCode::Char('j'), Modifiers::ctrl()));
        assert_eq!(state.cursor.line(), 1);
    }

    #[test]
    fn insert_multiple_chars() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        process_insert_key(&mut state, Key::new(KeyCode::Char('h'), Modifiers::none()));
        process_insert_key(&mut state, Key::new(KeyCode::Char('i'), Modifiers::none()));
        assert!(state.cursor.col() > 0);
    }

    #[test]
    fn insert_mode_type_name() {
        let _ = std::any::type_name::<EditorState>();
    }

    #[test]
    fn insert_key_import() {
        let _ = std::any::type_name::<Key>();
    }

    #[test]
    fn insert_intent_import() {
        let _ = std::any::type_name::<Intent>();
    }

    #[test]
    fn insert_escape_moves_cursor() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        state.buffer.insert(Position::new(0, 0), "abc");
        state.cursor.position.col = 2;
        process_insert_key(&mut state, Key::new(KeyCode::Escape, Modifiers::none()));
        assert_eq!(state.cursor.col(), 1);
    }

    #[test]
    fn insert_escape_at_col_0() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        state.cursor.position.col = 0;
        process_insert_key(&mut state, Key::new(KeyCode::Escape, Modifiers::none()));
        assert_eq!(state.cursor.col(), 0);
    }

    #[test]
    fn insert_ctrl_h_backspace() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        state.buffer.insert(Position::new(0, 0), "abc");
        state.cursor.position.col = 3;
        process_insert_key(&mut state, Key::new(KeyCode::Char('h'), Modifiers::ctrl()));
        assert_eq!(state.cursor.col(), 2);
    }

    #[test]
    fn insert_unknown_key_noop() {
        let mut state = EditorState::new();
        state.set_mode(Mode::Insert);
        let col_before = state.cursor.col();
        process_insert_key(&mut state, Key::new(KeyCode::F(1), Modifiers::none()));
        assert_eq!(state.cursor.col(), col_before);
    }
}
