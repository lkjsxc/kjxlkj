//! Ex command execution for editor state.

use kjxlkj_cmd::{Command, CommandKind, CommandParser};
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::Mode;

use crate::window_ops;
use crate::ActionResult;

/// Executes an ex command.
pub fn execute_ex_command(state: &mut EditorState, input: &str) -> ActionResult {
    let mut parser = CommandParser::new();
    let command = match parser.parse(input) {
        Ok(cmd) => cmd,
        Err(e) => return ActionResult::Error(format!("Parse error: {:?}", e)),
    };

    execute_command(state, &command)
}

/// Executes a parsed command.
fn execute_command(state: &mut EditorState, command: &Command) -> ActionResult {
    match &command.kind {
        CommandKind::Quit => {
            if command.force || !has_unsaved_changes(state) {
                ActionResult::Quit
            } else {
                ActionResult::Error("Unsaved changes. Use :q! to force.".to_string())
            }
        }

        CommandKind::QuitAll => {
            if command.force || !has_unsaved_changes(state) {
                ActionResult::Quit
            } else {
                ActionResult::Error("Unsaved changes. Use :qa! to force.".to_string())
            }
        }

        CommandKind::Write(path) => {
            if let Some(p) = path {
                if let Some(b) = state.active_buffer_mut() {
                    b.path = Some(p.clone());
                }
            }
            ActionResult::Save
        }

        CommandKind::WriteAll => ActionResult::Save,

        CommandKind::WriteQuit => ActionResult::SaveQuit,

        CommandKind::Edit(path) => {
            let path_str = path.to_string_lossy().to_string();
            ActionResult::OpenFile(path_str)
        }

        CommandKind::Split(path) => {
            window_ops::split_horizontal(state);
            if let Some(ref p) = path {
                let path_str = p.to_string_lossy().to_string();
                return ActionResult::OpenFile(path_str);
            }
            ActionResult::Ok
        }

        CommandKind::VSplit(path) => {
            window_ops::split_vertical(state);
            if let Some(ref p) = path {
                let path_str = p.to_string_lossy().to_string();
                return ActionResult::OpenFile(path_str);
            }
            ActionResult::Ok
        }

        CommandKind::Close => {
            window_ops::close_window(state);
            ActionResult::Ok
        }

        CommandKind::Only => {
            window_ops::only_window(state);
            ActionResult::Ok
        }

        CommandKind::BufferNext => {
            next_buffer(state);
            ActionResult::Ok
        }

        CommandKind::BufferPrev => {
            prev_buffer(state);
            ActionResult::Ok
        }

        CommandKind::ListBuffers => {
            // TODO: Display buffer list
            ActionResult::Ok
        }

        CommandKind::Noop => {
            // Go to line if range provided
            state.mode.transition(Mode::Normal);
            ActionResult::Ok
        }

        _ => ActionResult::Ok,
    }
}

/// Checks for unsaved changes.
fn has_unsaved_changes(state: &EditorState) -> bool {
    state.buffers.values().any(|b| b.modified)
}

/// Switches to next buffer.
fn next_buffer(state: &mut EditorState) {
    let ids: Vec<_> = state.buffers.keys().copied().collect();
    if ids.len() <= 1 {
        return;
    }

    let Some(window) = state.windows.get_mut(&state.layout.active) else {
        return;
    };

    let current_idx = ids.iter().position(|&id| id == window.buffer_id);
    if let Some(idx) = current_idx {
        let next_idx = (idx + 1) % ids.len();
        window.buffer_id = ids[next_idx];
    }
}

/// Switches to previous buffer.
fn prev_buffer(state: &mut EditorState) {
    let ids: Vec<_> = state.buffers.keys().copied().collect();
    if ids.len() <= 1 {
        return;
    }

    let Some(window) = state.windows.get_mut(&state.layout.active) else {
        return;
    };

    let current_idx = ids.iter().position(|&id| id == window.buffer_id);
    if let Some(idx) = current_idx {
        let prev_idx = if idx == 0 { ids.len() - 1 } else { idx - 1 };
        window.buffer_id = ids[prev_idx];
    }
}
