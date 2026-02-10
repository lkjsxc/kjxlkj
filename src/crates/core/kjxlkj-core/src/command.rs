//! Command execution.

use kjxlkj_core_state::EditorState;
use tracing::{info, warn};

/// Execute a command.
pub fn execute_command(state: &mut EditorState, cmd: &str) {
    let cmd = cmd.trim();
    let (name, args) = parse_command(cmd);
    
    match name {
        "q" | "quit" => {
            state.quit();
        }
        "q!" | "quit!" => {
            state.quit_force();
        }
        "w" | "write" => {
            if let Some(path) = args {
                info!(?path, "Write to path");
                // Write to path.
            } else {
                info!("Write current buffer");
            }
        }
        "wq" => {
            info!("Write and quit");
            state.quit();
        }
        "x" | "xit" => {
            info!("Save if modified and quit");
            state.quit();
        }
        "e" | "edit" => {
            if let Some(path) = args {
                info!(?path, "Edit file");
                // Open file.
            } else {
                info!("Reload current buffer");
            }
        }
        "set" => {
            if let Some(option) = args {
                info!(?option, "Set option");
                // Parse and set option.
            } else {
                info!("Show all options");
            }
        }
        "sp" | "split" => {
            info!("Split horizontal");
        }
        "vs" | "vsplit" => {
            info!("Split vertical");
        }
        "terminal" | "term" => {
            info!("Open terminal");
        }
        "Explorer" => {
            info!("Open explorer");
        }
        "bn" | "bnext" => {
            info!("Next buffer");
        }
        "bp" | "bprev" => {
            info!("Previous buffer");
        }
        "bd" | "bdelete" => {
            info!("Delete buffer");
        }
        _ => {
            warn!(?cmd, "Unknown command");
        }
    }
}

/// Parse a command into name and optional args.
fn parse_command(cmd: &str) -> (&str, Option<&str>) {
    if let Some(idx) = cmd.find(|c: char| c.is_whitespace()) {
        let (name, rest) = cmd.split_at(idx);
        (name, Some(rest.trim()))
    } else {
        (cmd, None)
    }
}

/// Perform undo.
pub fn undo(_state: &mut EditorState) {
    info!("Undo (not fully implemented)");
}

/// Perform redo.
pub fn redo(_state: &mut EditorState) {
    info!("Redo (not fully implemented)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_simple() {
        assert_eq!(parse_command("quit"), ("quit", None));
        assert_eq!(parse_command("q"), ("q", None));
    }

    #[test]
    fn test_parse_command_with_args() {
        assert_eq!(parse_command("e file.txt"), ("e", Some("file.txt")));
        assert_eq!(parse_command("set number"), ("set", Some("number")));
    }

    #[test]
    fn test_parse_command_with_path() {
        assert_eq!(parse_command("w /path/to/file"), ("w", Some("/path/to/file")));
    }
}
