//! Headless key processor.

use kjxlkj_core::{EditorState, Intent, Mode, Position};
use kjxlkj_input::{Key, KeyCode};

use super::motion;

/// Process a key event in headless mode.
pub fn process_key(state: &mut EditorState, key: Key) {
    match state.mode() {
        Mode::Normal => process_normal_key(state, key),
        Mode::Insert => process_insert_key(state, key),
        Mode::Command => process_command_key(state, key),
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => process_visual_key(state, key),
        Mode::Replace => process_replace_key(state, key),
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

fn process_visual_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => {
            state.selection = None;
            state.set_mode(Mode::Normal);
        }
        KeyCode::Char(c) if !key.mods.ctrl => {
            // Basic motions
            match c {
                'h' => { if state.cursor.col() > 0 { state.cursor.position.col -= 1; } }
                'l' => {
                    let max = state.buffer.line_len(state.cursor.line()).saturating_sub(1);
                    if state.cursor.col() < max { state.cursor.position.col += 1; }
                }
                'j' => {
                    if state.cursor.line() + 1 < state.buffer.line_count() {
                        state.cursor.position.line += 1;
                    }
                }
                'k' => { if state.cursor.line() > 0 { state.cursor.position.line -= 1; } }
                _ => {}
            }
            // Update selection cursor position
            if let Some(ref mut sel) = state.selection {
                sel.cursor = state.cursor.position;
            }
        }
        _ => {}
    }
}

fn process_replace_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => state.set_mode(Mode::Normal),
        KeyCode::Char(c) if !key.mods.ctrl => {
            // Replace character under cursor
            let pos = state.cursor.position;
            let next = Position::new(pos.line, pos.col + 1);
            state.buffer.delete_range(pos, next);
            state.buffer.insert(pos, &c.to_string());
            state.cursor.position.col += 1;
        }
        _ => {}
    }
}

fn apply_intent(state: &mut EditorState, intent: Intent) {
    match intent {
        Intent::None => {}
        Intent::Move(m) => motion::apply(state, m),
        Intent::EnterInsert { at_line_end, after_cursor } => {
            enter_insert(state, at_line_end, after_cursor);
        }
        Intent::EnterCommand => state.set_mode(Mode::Command),
        Intent::EnterReplace => state.set_mode(Mode::Replace),
        Intent::StartVisual(kind) => {
            state.set_mode(Mode::Visual);
            let pos = state.cursor.position;
            state.selection = Some(kjxlkj_core::Selection::new(pos, pos, kind));
        }
        Intent::OpenLineBelow => {
            let line = state.cursor.line();
            let len = state.buffer.line_len(line);
            let pos = Position::new(line, len);
            state.buffer.insert(pos, "\n");
            state.cursor.position = Position::new(line + 1, 0);
            state.set_mode(Mode::Insert);
        }
        Intent::OpenLineAbove => {
            let line = state.cursor.line();
            let pos = Position::new(line, 0);
            state.buffer.insert(pos, "\n");
            state.cursor.position = Position::new(line, 0);
            state.set_mode(Mode::Insert);
        }
        Intent::Quit { .. } => state.should_quit = true,
        _ => {}
    }
}

fn enter_insert(state: &mut EditorState, at_line_end: bool, after_cursor: bool) {
    if at_line_end {
        let len = state.buffer.line_len(state.cursor.line());
        state.cursor.position.col = len;
    } else if after_cursor {
        let len = state.buffer.line_len(state.cursor.line());
        if state.cursor.col() < len { state.cursor.position.col += 1; }
    }
    state.set_mode(Mode::Insert);
}

fn execute_command(state: &mut EditorState, cmd: &str) {
    match cmd.trim() {
        "q" | "q!" | "quit" | "quit!" => state.should_quit = true,
        _ => {}
    }
}
