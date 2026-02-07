//! Normal mode key interpretation — primary handler and motion dispatch.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    Direction, EditorAction, KeyCode, KeyEvent, Mode, Modifiers, Motion, Operator,
};

use crate::normal_mode_ext::handle_normal_g_key;
use crate::normal_mode_ops::{
    apply_motion, apply_operator_motion, handle_operator,
    handle_remaining, open_above, open_below,
};
use crate::normal_mode_scroll::handle_scroll_key;

/// Handle a key event in Normal mode.
pub fn handle_normal_key(state: &mut EditorState, key: KeyEvent) -> Option<EditorAction> {
    if state.macro_state.is_recording() && key != KeyEvent::char('q') {
        state.macro_state.record_key(key.clone());
    }
    if key.modifiers.contains(Modifiers::CTRL) {
        return handle_ctrl_key(state, &key);
    }
    dispatch_normal(state, &key)
}

fn dispatch_normal(state: &mut EditorState, key: &KeyEvent) -> Option<EditorAction> {
    match &key.code {
        KeyCode::Char(c @ '1'..='9') => {
            accumulate_count(state, (*c as u32 - '0' as u32) as usize);
            None
        }
        KeyCode::Char('0') if state.mode.take_count().is_some() => None,
        KeyCode::Char('h') | KeyCode::Left => motion_action(state, Motion::Left),
        KeyCode::Char('l') | KeyCode::Right => motion_action(state, Motion::Right),
        KeyCode::Char('j') | KeyCode::Down => motion_action(state, Motion::Down),
        KeyCode::Char('k') | KeyCode::Up => motion_action(state, Motion::Up),
        KeyCode::Char('w') => motion_action(state, Motion::WordForward),
        KeyCode::Char('b') => motion_action(state, Motion::WordBackward),
        KeyCode::Char('e') => motion_action(state, Motion::WordEnd),
        KeyCode::Char('W') => motion_action(state, Motion::BigWordForward),
        KeyCode::Char('B') => motion_action(state, Motion::BigWordBackward),
        KeyCode::Char('E') => motion_action(state, Motion::BigWordEnd),
        KeyCode::Char('0') => motion_action(state, Motion::LineStart),
        KeyCode::Char('^') | KeyCode::Char('_') => motion_action(state, Motion::FirstNonBlank),
        KeyCode::Char('$') => motion_action(state, Motion::LineEnd),
        KeyCode::Char('|') => {
            let col = take_count(state).unwrap_or(1).saturating_sub(1);
            motion_action(state, Motion::GoToColumn(col))
        }
        KeyCode::Char('G') => {
            match take_count(state) {
                Some(n) => motion_action(state, Motion::GoToLine(n.saturating_sub(1))),
                None => motion_action(state, Motion::FileEnd),
            }
        }
        KeyCode::Char('H') => motion_action(state, Motion::ScreenTop),
        KeyCode::Char('M') => motion_action(state, Motion::ScreenMiddle),
        KeyCode::Char('L') => motion_action(state, Motion::ScreenBottom),
        KeyCode::Char('%') => {
            if let Some(n) = take_count(state) {
                motion_action(state, Motion::GoToPercent(n))
            } else {
                motion_action(state, Motion::MatchingBracket)
            }
        }
        KeyCode::Char('{') => motion_action(state, Motion::ParagraphBackward),
        KeyCode::Char('}') => motion_action(state, Motion::ParagraphForward),
        KeyCode::Char('(') => motion_action(state, Motion::SentenceBackward),
        KeyCode::Char(')') => motion_action(state, Motion::SentenceForward),
        KeyCode::Char('i') => Some(EditorAction::ChangeMode(Mode::Insert)),
        KeyCode::Char('I') => {
            let line = state.active_window().cursor.line;
            state.active_window_mut().cursor.col = first_non_blank_col(state, line);
            Some(EditorAction::ChangeMode(Mode::Insert))
        }
        KeyCode::Char('a') => {
            state.active_window_mut().cursor.col += 1;
            Some(EditorAction::ChangeMode(Mode::Insert))
        }
        KeyCode::Char('A') => {
            let line = state.active_window().cursor.line;
            state.active_window_mut().cursor.col = state.active_buffer().line_len(line);
            Some(EditorAction::ChangeMode(Mode::Insert))
        }
        KeyCode::Char('o') => open_below(state),
        KeyCode::Char('O') => open_above(state),
        KeyCode::Char('v') => Some(EditorAction::ChangeMode(Mode::Visual)),
        KeyCode::Char('V') => Some(EditorAction::ChangeMode(Mode::VisualLine)),
        KeyCode::Char('R') => Some(EditorAction::ChangeMode(Mode::Replace)),
        KeyCode::Char(':') => {
            state.command_line.activate(":");
            Some(EditorAction::ChangeMode(Mode::Command))
        }
        KeyCode::Char('/') => {
            state.command_line.activate("/");
            Some(EditorAction::ChangeMode(Mode::Command))
        }
        KeyCode::Char('?') => {
            state.command_line.activate("?");
            Some(EditorAction::ChangeMode(Mode::Command))
        }
        KeyCode::Char('n') => Some(EditorAction::SearchNext),
        KeyCode::Char('N') => Some(EditorAction::SearchPrev),
        KeyCode::Char('*') | KeyCode::Char('#') => None,
        KeyCode::Char('d') => handle_operator(state, Operator::Delete),
        KeyCode::Char('y') => handle_operator(state, Operator::Yank),
        KeyCode::Char('c') => handle_operator(state, Operator::Change),
        KeyCode::Char('>') => handle_operator(state, Operator::Indent),
        KeyCode::Char('<') => handle_operator(state, Operator::Outdent),
        KeyCode::Char('g') => handle_normal_g_key(state),
        _ => handle_remaining(state, key),
    }
}

fn handle_ctrl_key(state: &mut EditorState, key: &KeyEvent) -> Option<EditorAction> {
    match &key.code {
        KeyCode::Char('r') => Some(EditorAction::Redo),
        KeyCode::Char('o') => Some(EditorAction::JumpList(Direction::Backward)),
        KeyCode::Char('i') => Some(EditorAction::JumpList(Direction::Forward)),
        KeyCode::Char('g') => {
            let buf = state.active_buffer();
            let info = format!(
                "\"{}\" {} lines",
                buf.path().unwrap_or(buf.name()),
                buf.line_count()
            );
            state.set_message(info);
            None
        }
        KeyCode::Char('a') | KeyCode::Char('x') | KeyCode::Char('^') => None,
        _ => handle_scroll_key(state, key),
    }
}

fn accumulate_count(state: &mut EditorState, digit: usize) {
    let prev = state.mode.take_count().unwrap_or(0);
    state.mode.set_count(prev * 10 + digit);
}

pub(crate) fn take_count(state: &mut EditorState) -> Option<usize> {
    state.mode.take_count()
}

pub(crate) fn motion_action(state: &mut EditorState, motion: Motion) -> Option<EditorAction> {
    let count = take_count(state).unwrap_or(1);
    if let Some(op) = state.mode.clear_pending_operator() {
        apply_operator_motion(state, op, motion, count);
        return None;
    }
    apply_motion(state, &motion, count);
    None
}

pub(crate) fn first_non_blank_col(state: &EditorState, line: usize) -> usize {
    let text = state.active_buffer().line(line).unwrap_or_default();
    text.chars().position(|c| !c.is_ascii_whitespace()).unwrap_or(0)
}

#[cfg(test)]
#[path = "normal_mode_tests.rs"]
mod tests;
