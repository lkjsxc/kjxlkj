//! Command mode handler.

use super::Action;
use kjxlkj_core::EditorState;
use kjxlkj_input::{Key, KeyCode};
use kjxlkj_services::Services;
use std::path::PathBuf;

/// Handle command mode keys.
pub fn handle_command(state: &mut EditorState, key: Key, services: &Services) -> Action {
    match key.code {
        KeyCode::Esc => {
            state.cmdline.clear();
            state.mode.to_normal();
        }
        KeyCode::Enter => {
            let cmd = std::mem::take(&mut state.cmdline);
            state.mode.to_normal();
            return execute_command(state, &cmd, services);
        }
        KeyCode::Backspace => {
            if state.cmdline.pop().is_none() {
                state.mode.to_normal();
            }
        }
        KeyCode::Char(c) => {
            state.cmdline.push(c);
        }
        _ => {}
    }
    Action::Continue
}

fn execute_command(state: &mut EditorState, cmd: &str, services: &Services) -> Action {
    let cmd = cmd.trim();

    // Handle quit commands
    if cmd == "q" || cmd == "quit" {
        if state.modified {
            state.status = "No write since last change (use :q! to override)".to_string();
            return Action::Continue;
        }
        return Action::Quit;
    }
    if cmd == "q!" || cmd == "quit!" || cmd == "qa" || cmd == "qa!" {
        return Action::Quit;
    }

    // Handle write commands
    if cmd == "w" || cmd == "write" {
        if let Err(e) = state.save() {
            state.status = format!("Error: {}", e);
        }
        return Action::Continue;
    }
    if let Some(path) = cmd
        .strip_prefix("w ")
        .or_else(|| cmd.strip_prefix("write "))
    {
        let path = PathBuf::from(path.trim());
        if let Err(e) = state.save_as(&path) {
            state.status = format!("Error: {}", e);
        }
        return Action::Continue;
    }

    // Handle write and quit
    if cmd == "wq" || cmd == "x" {
        let _ = state.save();
        return Action::Quit;
    }
    if let Some(path) = cmd.strip_prefix("wq ") {
        let path = PathBuf::from(path.trim());
        let _ = state.save_as(&path);
        return Action::Quit;
    }

    // Handle edit command
    if let Some(path) = cmd.strip_prefix("e ").or_else(|| cmd.strip_prefix("e! ")) {
        let path = PathBuf::from(path.trim());
        if let Err(e) = state.open_file(&path) {
            state.status = format!("Error: {}", e);
        }
        return Action::Continue;
    }

    // Handle shell command
    if let Some(shell_cmd) = cmd.strip_prefix("! ") {
        match services.terminal.run_command_first_line(shell_cmd) {
            Ok(output) => state.status = output,
            Err(e) => state.status = format!("Error: {}", e),
        }
        return Action::Continue;
    }

    // Handle substitute
    if cmd.starts_with("s/") {
        handle_substitute(state, cmd);
        return Action::Continue;
    }

    // Handle global command
    if cmd.starts_with("g/") {
        handle_global(state, cmd);
        return Action::Continue;
    }

    // Handle inverted global
    if cmd.starts_with("v/") {
        handle_inverted_global(state, cmd);
        return Action::Continue;
    }

    state.status = format!("Unknown command: {}", cmd);
    Action::Continue
}

fn handle_substitute(state: &mut EditorState, cmd: &str) {
    // Parse s/pattern/replacement/flags
    let parts: Vec<&str> = cmd[2..].split('/').collect();
    if parts.len() < 2 {
        state.status = "Invalid substitute command".to_string();
        return;
    }

    let pattern = parts[0];
    let replacement = parts.get(1).unwrap_or(&"");
    let flags = parts.get(2).unwrap_or(&"");
    let global = flags.contains('g');

    let line = state.cursor.pos.line;
    if let Some(content) = state.buffer.line(line) {
        let new_content = if global {
            content.replace(pattern, replacement)
        } else {
            content.replacen(pattern, replacement, 1)
        };
        if new_content != content {
            let start = kjxlkj_core::Position::new(line, 0);
            let end = kjxlkj_core::Position::new(line, content.len());
            state.buffer.delete(start, end);
            state.buffer.insert(start, &new_content);
            state.modified = true;
        }
    }
}

fn handle_global(state: &mut EditorState, cmd: &str) {
    // Parse g/pattern/command
    let parts: Vec<&str> = cmd[2..].split('/').collect();
    if parts.len() < 2 {
        state.status = "Invalid global command".to_string();
        return;
    }

    let pattern = parts[0];
    let subcmd = parts.get(1).unwrap_or(&"").trim();

    if subcmd == "d" {
        // Delete matching lines
        let mut line = 0;
        while line < state.buffer.line_count() {
            if let Some(content) = state.buffer.line(line) {
                if content.contains(pattern) {
                    let start = kjxlkj_core::Position::new(line, 0);
                    let end = if line + 1 < state.buffer.line_count() {
                        kjxlkj_core::Position::new(line + 1, 0)
                    } else {
                        let len = content.len();
                        kjxlkj_core::Position::new(line, len)
                    };
                    state.buffer.delete(start, end);
                    state.modified = true;
                    continue;
                }
            }
            line += 1;
        }
    }
}

fn handle_inverted_global(state: &mut EditorState, cmd: &str) {
    // Parse v/pattern/command
    let parts: Vec<&str> = cmd[2..].split('/').collect();
    if parts.len() < 2 {
        state.status = "Invalid global command".to_string();
        return;
    }

    let pattern = parts[0];
    let subcmd = parts.get(1).unwrap_or(&"").trim();

    if subcmd == "d" {
        // Delete non-matching lines
        let mut line = 0;
        while line < state.buffer.line_count() {
            if let Some(content) = state.buffer.line(line) {
                if !content.contains(pattern) {
                    let start = kjxlkj_core::Position::new(line, 0);
                    let end = if line + 1 < state.buffer.line_count() {
                        kjxlkj_core::Position::new(line + 1, 0)
                    } else {
                        let len = content.len();
                        kjxlkj_core::Position::new(line, len)
                    };
                    state.buffer.delete(start, end);
                    state.modified = true;
                    continue;
                }
            }
            line += 1;
        }
    }
}
