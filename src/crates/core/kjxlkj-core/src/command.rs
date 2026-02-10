//! Command execution.

use kjxlkj_core_state::EditorState;
use tracing::{info, warn};

/// Execute a command.
pub fn execute_command(state: &mut EditorState, cmd: &str) {
    let cmd = cmd.trim();
    match cmd {
        "q" | "quit" => {
            state.quit();
        }
        "w" | "write" => {
            info!("Write command (not implemented)");
        }
        "wq" => {
            info!("Write and quit (not implemented)");
            state.quit();
        }
        _ => {
            warn!(?cmd, "Unknown command");
        }
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
