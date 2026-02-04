//! Ex command execution.

use kjxlkj_core::{EditorState, Intent};

use super::{apply_intent, load_file};

/// Execute an Ex command.
pub fn execute_command(state: &mut EditorState, cmd: &str) {
    let cmd = cmd.trim();

    // Handle search commands
    if cmd.starts_with('/') || cmd.starts_with('?') {
        execute_search(state, cmd);
        return;
    }

    // Parse Ex commands
    match cmd {
        "q" | "quit" => execute_quit(state, false),
        "q!" | "quit!" => execute_quit(state, true),
        "w" | "write" => apply_intent(state, Intent::Write { path: None }),
        "wq" | "x" => {
            apply_intent(state, Intent::Write { path: None });
            state.should_quit = true;
        }
        "qa" | "qa!" | "qall" | "qall!" => {
            state.should_quit = true;
        }
        _ if cmd.starts_with("w ") => {
            let path = cmd[2..].trim();
            apply_intent(state, Intent::Write { path: Some(path.to_string()) });
        }
        _ if cmd.starts_with("e ") || cmd.starts_with("e! ") => {
            execute_edit(state, cmd);
        }
        _ if cmd.starts_with("! ") => {
            execute_shell(state, cmd);
        }
        _ => {
            state.set_status(format!("Unknown command: {}", cmd));
        }
    }
}

fn execute_search(state: &mut EditorState, cmd: &str) {
    let pattern = &cmd[1..];
    if pattern.is_empty() {
        return;
    }

    state.mode_state.search_pattern = pattern.to_string();
    let text = state.buffer.text();
    let start_pos = state.buffer.pos_to_char_idx(state.cursor.position).unwrap_or(0);

    // Search from current position
    if let Some(idx) = text[start_pos + 1..].find(pattern) {
        let new_pos = state.buffer.char_idx_to_pos(start_pos + 1 + idx);
        state.cursor.position = new_pos;
    } else if let Some(idx) = text.find(pattern) {
        // Wrap around to beginning
        let new_pos = state.buffer.char_idx_to_pos(idx);
        state.cursor.position = new_pos;
        state.set_status("search wrapped");
    }
}

fn execute_quit(state: &mut EditorState, force: bool) {
    if !force && state.buffer.is_modified() {
        state.set_status("No write since last change");
    } else {
        state.should_quit = true;
    }
}

fn execute_edit(state: &mut EditorState, cmd: &str) {
    let path = if cmd.starts_with("e! ") {
        cmd[3..].trim()
    } else {
        cmd[2..].trim()
    };
    if let Err(e) = load_file(state, path) {
        state.set_status(format!("Error: {}", e));
    }
}

fn execute_shell(state: &mut EditorState, cmd: &str) {
    let shell_cmd = &cmd[2..];
    match kjxlkj_services::terminal::TerminalService::run_command_str(shell_cmd) {
        Ok(output) => {
            let first_line = output.lines().next().unwrap_or("");
            state.set_status(first_line.to_string());
        }
        Err(e) => state.set_status(format!("Error: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core::Position;

    #[test]
    fn quit_command_forces_modified() {
        let mut state = EditorState::new();
        // Insert to make modified
        state.buffer.insert(Position::new(0, 0), "x");
        execute_command(&mut state, "q!");
        assert!(state.should_quit);
    }

    #[test]
    fn quit_command_denies_modified() {
        let mut state = EditorState::new();
        // Insert to make modified
        state.buffer.insert(Position::new(0, 0), "x");
        execute_command(&mut state, "q");
        assert!(!state.should_quit);
    }

    #[test]
    fn quit_command_unmodified() {
        let mut state = EditorState::new();
        execute_command(&mut state, "q");
        assert!(state.should_quit);
    }

    #[test]
    fn unknown_command_sets_status() {
        let mut state = EditorState::new();
        execute_command(&mut state, "foobar");
    }

    #[test]
    fn qa_command_quits() {
        let mut state = EditorState::new();
        execute_command(&mut state, "qa");
        assert!(state.should_quit);
    }

    #[test]
    fn wq_command_quits() {
        let mut state = EditorState::new();
        execute_command(&mut state, "wq");
        assert!(state.should_quit);
    }

    #[test]
    fn x_command_quits() {
        let mut state = EditorState::new();
        execute_command(&mut state, "x");
        assert!(state.should_quit);
    }
}
