//! Command mode key handling.

use kjxlkj_core_state::command_line_state::{map_cmdline_key, CmdlineAction};
use kjxlkj_core_state::{dispatch_command, parse_command, EditorState};
use kjxlkj_core_types::{Direction, EditorAction, KeyEvent, Mode};

/// Handle a key event in Command mode.
pub fn handle_command_key(
    state: &mut EditorState,
    key: KeyEvent,
) -> Option<EditorAction> {
    if state.macro_state.is_recording() {
        state.macro_state.record_key(key.clone());
    }

    let action = map_cmdline_key(&key)?;

    match action {
        CmdlineAction::InsertChar(ch) => {
            let pos = state.command_line.cursor_pos;
            state.command_line.content.insert(pos, ch);
            state.command_line.cursor_pos += 1;
            None
        }
        CmdlineAction::DeleteBack => {
            if state.command_line.cursor_pos > 0 {
                state.command_line.cursor_pos -= 1;
                state.command_line.content.remove(state.command_line.cursor_pos);
            } else {
                // Empty command line + backspace → cancel
                state.command_line.deactivate();
                return Some(EditorAction::ChangeMode(Mode::Normal));
            }
            None
        }
        CmdlineAction::DeleteWord => {
            let pos = state.command_line.cursor_pos;
            let new_pos = word_back(&state.command_line.content, pos);
            state.command_line.content.drain(new_pos..pos);
            state.command_line.cursor_pos = new_pos;
            None
        }
        CmdlineAction::DeleteToStart => {
            let pos = state.command_line.cursor_pos;
            state.command_line.content.drain(..pos);
            state.command_line.cursor_pos = 0;
            None
        }
        CmdlineAction::MoveLeft => {
            if state.command_line.cursor_pos > 0 {
                state.command_line.cursor_pos -= 1;
            }
            None
        }
        CmdlineAction::MoveRight => {
            let max = state.command_line.content.len();
            if state.command_line.cursor_pos < max {
                state.command_line.cursor_pos += 1;
            }
            None
        }
        CmdlineAction::MoveStart => {
            state.command_line.cursor_pos = 0;
            None
        }
        CmdlineAction::MoveEnd => {
            state.command_line.cursor_pos = state.command_line.content.len();
            None
        }
        CmdlineAction::HistoryPrev => {
            if let Some(entry) = state.command_line.history.prev().cloned() {
                state.command_line.content = entry;
                state.command_line.cursor_pos = state.command_line.content.len();
            }
            None
        }
        CmdlineAction::HistoryNext => {
            if let Some(entry) = state.command_line.history.next().cloned() {
                state.command_line.content = entry.clone();
                state.command_line.cursor_pos = state.command_line.content.len();
            } else {
                state.command_line.content.clear();
                state.command_line.cursor_pos = 0;
            }
            None
        }
        CmdlineAction::Accept => {
            let prefix = state.command_line.prefix.clone();
            let content = state.command_line.content.clone();
            state.command_line.history.push(content.clone());
            state.command_line.deactivate();
            state.mode.transition(Mode::Normal);
            execute_command_line(state, &prefix, &content)
        }
        CmdlineAction::Cancel => {
            state.command_line.deactivate();
            Some(EditorAction::ChangeMode(Mode::Normal))
        }
        CmdlineAction::Complete => {
            // Tab completion stub
            None
        }
        CmdlineAction::PasteRegister => {
            // Ctrl-r: paste register stub
            None
        }
    }
}

/// Execute the completed command line.
fn execute_command_line(
    state: &mut EditorState,
    prefix: &str,
    content: &str,
) -> Option<EditorAction> {
    match prefix {
        ":" => {
            match parse_command(content) {
                Ok(cmd) => match dispatch_command(state, cmd) {
                    Ok(()) => None,
                    Err(e) => {
                        state.set_message(format!("E: {e}"));
                        None
                    }
                },
                Err(e) => {
                    state.set_message(format!("E: {e}"));
                    None
                }
            }
        }
        "/" => {
            state.search.pattern = Some(content.to_string());
            state.search.direction = Direction::Forward;
            Some(EditorAction::Search(content.to_string(), Direction::Forward))
        }
        "?" => {
            state.search.pattern = Some(content.to_string());
            state.search.direction = Direction::Backward;
            Some(EditorAction::Search(content.to_string(), Direction::Backward))
        }
        _ => None,
    }
}

/// Find position after deleting one word backward in a string.
fn word_back(s: &str, pos: usize) -> usize {
    let bytes: Vec<u8> = s.bytes().collect();
    let mut i = pos;
    while i > 0 && bytes.get(i - 1).map_or(false, |b| (*b as char).is_whitespace()) {
        i -= 1;
    }
    while i > 0 && bytes.get(i - 1).map_or(false, |b| !(*b as char).is_whitespace()) {
        i -= 1;
    }
    i
}

#[cfg(test)]
#[path = "command_mode_tests.rs"]
mod tests;
