//! Command parsing and execution.

use kjxlkj_core::EditorState;
use anyhow::Result;

/// Parse and execute a command string.
#[allow(dead_code)]
pub fn execute(_editor: &mut EditorState, _cmd: &str) -> Result<()> {
    // Command execution is handled in app.rs for now
    Ok(())
}
