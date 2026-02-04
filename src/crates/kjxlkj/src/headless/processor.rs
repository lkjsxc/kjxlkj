//! Headless key processor.

use kjxlkj_core::{EditorState, Intent, Mode, MotionKind, Position};
use kjxlkj_input::{Key, KeyCode};

/// Process a key event in headless mode.
pub fn process_key(state: &mut EditorState, key: Key) {
    match state.mode() {
        Mode::Normal => process_normal_key(state, key),
        Mode::Insert => process_insert_key(state, key),
        Mode::Command => process_command_key(state, key),
        _ => {}
    }
    state.clamp_cursor();
}

fn process_normal_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => state.mode_state.normal.reset(),
        KeyCode::Char(c) => {
            let intent = state
                .mode_state
                .normal
                .process_key(c, key.mods.ctrl, key.mods.shift);
            apply_intent(state, intent);
        }
        _ => {}
    }
}

fn process_insert_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => state.set_mode(Mode::Normal),
        KeyCode::Char(c) if !key.mods.ctrl => {
            let pos = state.cursor.position;
            state.buffer.insert(pos, &c.to_string());
            state.cursor.position.col += 1;
        }
        KeyCode::Enter => {
            let pos = state.cursor.position;
            state.buffer.insert(pos, "\n");
            state.cursor.position = Position::new(pos.line + 1, 0);
        }
        KeyCode::Backspace => {
            if state.cursor.col() > 0 {
                let pos = state.cursor.position;
                let prev = Position::new(pos.line, pos.col - 1);
                state.buffer.delete_range(prev, pos);
                state.cursor.position = prev;
            }
        }
        _ => {}
    }
}

fn process_command_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => {
            state.mode_state.command_line.clear();
            state.set_mode(Mode::Normal);
        }
        KeyCode::Enter => {
            let cmd = state.mode_state.command_line.clone();
            state.mode_state.command_line.clear();
            state.set_mode(Mode::Normal);
            execute_command(state, &cmd);
        }
        KeyCode::Char(c) if !key.mods.ctrl => {
            state.mode_state.command_line.push(c);
        }
        _ => {}
    }
}

fn apply_intent(state: &mut EditorState, intent: Intent) {
    match intent {
        Intent::None => {}
        Intent::Move(motion) => apply_motion(state, motion),
        Intent::EnterInsert { at_line_end, after_cursor } => {
            enter_insert(state, at_line_end, after_cursor);
        }
        Intent::EnterCommand => state.set_mode(Mode::Command),
        Intent::Quit { .. } => state.should_quit = true,
        _ => {}
    }
}

fn apply_motion(state: &mut EditorState, motion: kjxlkj_core::Motion) {
    for _ in 0..motion.count {
        match motion.kind {
            MotionKind::Left => {
                if state.cursor.col() > 0 {
                    state.cursor.position.col -= 1;
                }
            }
            MotionKind::Right => {
                let max = state.buffer.line_len(state.cursor.line()).saturating_sub(1);
                if state.cursor.col() < max {
                    state.cursor.position.col += 1;
                }
            }
            MotionKind::Up => {
                if state.cursor.line() > 0 {
                    state.cursor.position.line -= 1;
                }
            }
            MotionKind::Down => {
                if state.cursor.line() + 1 < state.buffer.line_count() {
                    state.cursor.position.line += 1;
                }
            }
            _ => {}
        }
    }
}

fn enter_insert(state: &mut EditorState, at_line_end: bool, after_cursor: bool) {
    if at_line_end {
        let len = state.buffer.line_len(state.cursor.line());
        state.cursor.position.col = len;
    } else if after_cursor {
        let len = state.buffer.line_len(state.cursor.line());
        if state.cursor.col() < len {
            state.cursor.position.col += 1;
        }
    }
    state.set_mode(Mode::Insert);
}

fn execute_command(state: &mut EditorState, cmd: &str) {
    match cmd.trim() {
        "q" | "q!" | "quit" | "quit!" => {
            state.should_quit = true;
        }
        _ => {}
    }
}
