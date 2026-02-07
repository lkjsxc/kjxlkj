//! Command dispatch: execute ExCommands against EditorState.

use kjxlkj_core_types::{BufferId, EditorError, Position};

use crate::command_dispatch_ext::dispatch_extended;
use crate::commands::ExCommand;
use crate::editor_state::EditorState;
use crate::options::{apply_set_action, parse_set_arg};
use crate::viewport;

/// Execute an Ex command, modifying the editor state.
pub fn dispatch_command(state: &mut EditorState, cmd: ExCommand) -> Result<(), EditorError> {
    match cmd {
        ExCommand::Quit => {
            if state.active_buffer().is_modified() {
                return Err(EditorError::InvalidCommand(
                    "unsaved changes (use :q! to force)".into(),
                ));
            }
            state.should_quit = true;
            Ok(())
        }
        ExCommand::ForceQuit => {
            state.should_quit = true;
            Ok(())
        }
        ExCommand::QuitAll => {
            let any_modified = state.buffers.values().any(|b| b.is_modified());
            if any_modified {
                return Err(EditorError::InvalidCommand(
                    "unsaved changes (use :qa! to force)".into(),
                ));
            }
            state.should_quit = true;
            Ok(())
        }
        ExCommand::ForceQuitAll => {
            state.should_quit = true;
            Ok(())
        }
        ExCommand::Write(path) => dispatch_write(state, path),
        ExCommand::WriteAll => dispatch_write_all(state),
        ExCommand::WriteQuit(path) => {
            dispatch_write(state, path)?;
            state.should_quit = true;
            Ok(())
        }
        ExCommand::Edit(path, force) => dispatch_edit(state, &path, force),
        ExCommand::BufferList => {
            let list = state.buffer_list();
            state.set_message(list.join("\n"));
            Ok(())
        }
        ExCommand::BufferNext => {
            let ids = sorted_buffer_ids(state);
            let cur = state.active_buffer_id();
            let idx = ids.iter().position(|id| *id == cur).unwrap_or(0);
            let next = ids[(idx + 1) % ids.len()];
            state.switch_buffer(next);
            Ok(())
        }
        ExCommand::BufferPrev => {
            let ids = sorted_buffer_ids(state);
            let cur = state.active_buffer_id();
            let idx = ids.iter().position(|id| *id == cur).unwrap_or(0);
            let prev = ids[(idx + ids.len() - 1) % ids.len()];
            state.switch_buffer(prev);
            Ok(())
        }
        ExCommand::BufferDelete(force) => {
            let id = state.active_buffer_id();
            state.delete_buffer(id, force)
        }
        ExCommand::Split => {
            state.split_horizontal();
            Ok(())
        }
        ExCommand::VSplit => {
            state.split_vertical();
            Ok(())
        }
        ExCommand::New => {
            let id = state.create_buffer("[No Name]", "");
            state.split_horizontal();
            let last = state.windows.len() - 1;
            state.windows[last].buffer_id = id;
            Ok(())
        }
        ExCommand::VNew => {
            let id = state.create_buffer("[No Name]", "");
            state.split_vertical();
            let last = state.windows.len() - 1;
            state.windows[last].buffer_id = id;
            Ok(())
        }
        ExCommand::Only => {
            state.close_other_windows();
            Ok(())
        }
        ExCommand::Set(args) => {
            let action = parse_set_arg(&args);
            match apply_set_action(&mut state.options, action) {
                Ok(msg) => {
                    if !msg.is_empty() {
                        state.set_message(msg);
                    }
                    Ok(())
                }
                Err(e) => Err(EditorError::InvalidCommand(e)),
            }
        }
        ExCommand::GoToLine(n) => {
            let line = n.saturating_sub(1);
            state.active_window_mut().cursor = Position::new(line, 0);
            let cursor_line = state.active_window().cursor.line;
            viewport::follow_cursor_v(&mut state.viewport, cursor_line);
            Ok(())
        }
        ExCommand::Enew => {
            let id = state.create_buffer("[No Name]", "");
            state.switch_buffer(id);
            Ok(())
        }
        ExCommand::SwitchBuffer(n) => {
            let id = BufferId(n);
            if state.buffers.contains_key(&id) {
                state.switch_buffer(id);
                Ok(())
            } else {
                Err(EditorError::BufferNotFound(id))
            }
        }
        ExCommand::ScratchBuffer => {
            let id = state.create_buffer("[Scratch]", "");
            state.switch_buffer(id);
            Ok(())
        }
        ExCommand::NoHlSearch => {
            state.search.hl_search = false;
            Ok(())
        }
        ExCommand::SaveAs(path) => {
            state.active_buffer_mut().set_path(path.clone());
            dispatch_write(state, Some(path))
        }
        _ => dispatch_extended(state, cmd),
    }
}

fn dispatch_write(state: &mut EditorState, path: Option<String>) -> Result<(), EditorError> {
    if let Some(p) = path {
        state.active_buffer_mut().set_path(p);
    }
    let msg = state.write_buffer()?;
    state.set_message(msg);
    Ok(())
}

fn dispatch_write_all(state: &mut EditorState) -> Result<(), EditorError> {
    let ids: Vec<BufferId> = state.buffers.keys().copied().collect();
    for id in ids {
        if state.buffers[&id].is_modified() && state.buffers[&id].path().is_some() {
            let path = state.buffers[&id].path().unwrap().to_string();
            let text = state.buffers[&id].text();
            std::fs::write(&path, &text).map_err(EditorError::from)?;
            state.buffers.get_mut(&id).unwrap().mark_saved();
        }
    }
    state.set_message("all buffers written");
    Ok(())
}

fn dispatch_edit(state: &mut EditorState, path: &str, force: bool) -> Result<(), EditorError> {
    if !force && state.active_buffer().is_modified() {
        return Err(EditorError::InvalidCommand(
            "unsaved changes (use :e!)".into(),
        ));
    }
    if path.is_empty() {
        return Err(EditorError::InvalidCommand("no file name".into()));
    }
    let id = state.open_file(path)?;
    state.switch_buffer(id);
    Ok(())
}

fn sorted_buffer_ids(state: &EditorState) -> Vec<BufferId> {
    let mut ids: Vec<BufferId> = state.buffers.keys().copied().collect();
    ids.sort_by_key(|id| id.0);
    ids
}
