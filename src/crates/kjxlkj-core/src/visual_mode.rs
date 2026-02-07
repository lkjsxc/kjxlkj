//! Visual mode key handling.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    EditorAction, KeyCode, KeyEvent, Mode, Modifiers, Motion,
};

use crate::visual_mode_ops::{move_visual_cursor, visual_range};

/// Handle a key event in Visual, Visual-Line, or Visual-Block mode.
pub fn handle_visual_key(
    state: &mut EditorState,
    key: KeyEvent,
) -> Option<EditorAction> {
    if state.macro_state.is_recording() {
        state.macro_state.record_key(key.clone());
    }

    if key.modifiers.contains(Modifiers::CTRL) {
        return handle_visual_ctrl(state, &key);
    }

    match &key.code {
        // Cancel visual mode
        KeyCode::Escape => {
            state.visual = None;
            Some(EditorAction::ChangeMode(Mode::Normal))
        }

        // Motion keys extend selection
        KeyCode::Char('h') | KeyCode::Left => {
            move_visual_cursor(state, Motion::Left, 1);
            None
        }
        KeyCode::Char('l') | KeyCode::Right => {
            move_visual_cursor(state, Motion::Right, 1);
            None
        }
        KeyCode::Char('j') | KeyCode::Down => {
            move_visual_cursor(state, Motion::Down, 1);
            None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            move_visual_cursor(state, Motion::Up, 1);
            None
        }
        KeyCode::Char('w') => {
            move_visual_cursor(state, Motion::WordForward, 1);
            None
        }
        KeyCode::Char('b') => {
            move_visual_cursor(state, Motion::WordBackward, 1);
            None
        }
        KeyCode::Char('e') => {
            move_visual_cursor(state, Motion::WordEnd, 1);
            None
        }
        KeyCode::Char('W') => {
            move_visual_cursor(state, Motion::BigWordForward, 1);
            None
        }
        KeyCode::Char('B') => {
            move_visual_cursor(state, Motion::BigWordBackward, 1);
            None
        }
        KeyCode::Char('0') => {
            move_visual_cursor(state, Motion::LineStart, 1);
            None
        }
        KeyCode::Char('^') => {
            move_visual_cursor(state, Motion::FirstNonBlank, 1);
            None
        }
        KeyCode::Char('$') => {
            move_visual_cursor(state, Motion::LineEnd, 1);
            None
        }
        KeyCode::Char('G') => {
            move_visual_cursor(state, Motion::FileEnd, 1);
            None
        }

        // gg: go to top
        KeyCode::Char('g') => None, // awaits second 'g'

        // Operators on selection
        KeyCode::Char('d') | KeyCode::Char('x') => {
            let range = visual_range(state);
            state.visual = None;
            state.mode.transition(Mode::Normal);
            Some(EditorAction::DeleteRange(range))
        }
        KeyCode::Char('y') => {
            let range = visual_range(state);
            state.visual = None;
            state.mode.transition(Mode::Normal);
            Some(EditorAction::Yank(range))
        }
        KeyCode::Char('c') | KeyCode::Char('s') => {
            let _range = visual_range(state);
            state.visual = None;
            Some(EditorAction::ChangeMode(Mode::Insert))
        }
        KeyCode::Char('>') => {
            state.visual = None;
            state.mode.transition(Mode::Normal);
            Some(EditorAction::Indent)
        }
        KeyCode::Char('<') => {
            state.visual = None;
            state.mode.transition(Mode::Normal);
            Some(EditorAction::Outdent)
        }

        // o: swap anchor and cursor
        KeyCode::Char('o') => {
            if let Some(ref mut sel) = state.visual {
                std::mem::swap(&mut sel.anchor, &mut sel.cursor);
                let new_pos = sel.cursor;

                state.active_window_mut().cursor = new_pos;
            }
            None
        }

        // Switch visual sub-modes
        KeyCode::Char('v') => {
            if state.mode.current() == Mode::Visual {
                state.visual = None;
                Some(EditorAction::ChangeMode(Mode::Normal))
            } else {
                Some(EditorAction::ChangeMode(Mode::Visual))
            }
        }
        KeyCode::Char('V') => {
            if state.mode.current() == Mode::VisualLine {
                state.visual = None;
                Some(EditorAction::ChangeMode(Mode::Normal))
            } else {
                Some(EditorAction::ChangeMode(Mode::VisualLine))
            }
        }

        // J join selected
        KeyCode::Char('J') => Some(EditorAction::JoinLine(false)),
        // ~ toggle case
        KeyCode::Char('~') => Some(EditorAction::ToggleCase),
        KeyCode::Char('U') => Some(EditorAction::UpperCase),
        KeyCode::Char('u') => Some(EditorAction::LowerCase),

        // : enter command mode with visual range
        KeyCode::Char(':') => {
            state.command_line.activate(":'<,'>");
            Some(EditorAction::ChangeMode(Mode::Command))
        }

        _ => None,
    }
}

fn handle_visual_ctrl(state: &mut EditorState, key: &KeyEvent) -> Option<EditorAction> {
    match &key.code {
        KeyCode::Char('v') => {
            if state.mode.current() == Mode::VisualBlock {
                state.visual = None;
                Some(EditorAction::ChangeMode(Mode::Normal))
            } else {
                Some(EditorAction::ChangeMode(Mode::VisualBlock))
            }
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "visual_mode_tests.rs"]
mod tests;
