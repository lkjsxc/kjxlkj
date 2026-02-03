//! Headless mode for scripted testing.

use std::path::Path;

use anyhow::Result;

use kjxlkj_core::EditorState;
use kjxlkj_input::{parse_key_array, parse_script, ScriptStep};

/// Run the editor in headless mode with a script.
pub fn run_headless(editor: &mut EditorState, script_path: &Path) -> Result<HeadlessResult> {
    let content = std::fs::read_to_string(script_path)?;

    // Try parsing as script steps first
    if let Ok(steps) = parse_script(&content) {
        return run_script_steps(editor, steps);
    }

    // Fall back to simple key array
    let keys = parse_key_array(&content)?;
    for key in keys {
        editor.handle_key(key)?;
    }

    Ok(HeadlessResult {
        quit: editor.should_quit(),
        final_snapshot: editor.snapshot(),
    })
}

fn run_script_steps(editor: &mut EditorState, steps: Vec<ScriptStep>) -> Result<HeadlessResult> {
    let mut _assertions_passed = true;

    for step in steps {
        match step {
            ScriptStep::Key(k) => {
                let key = k.to_key();
                editor.handle_key(key)?;
            }
            ScriptStep::Wait { ms } => {
                std::thread::sleep(std::time::Duration::from_millis(ms));
            }
            ScriptStep::Assert { line, contains } => {
                let snapshot = editor.snapshot();
                let line_idx = line as usize;
                if let Some(line_snap) = snapshot.active_window.lines.get(line_idx) {
                    if !line_snap.text.contains(&contains) {
                        _assertions_passed = false;
                    }
                } else {
                    _assertions_passed = false;
                }
            }
        }
    }

    Ok(HeadlessResult {
        quit: editor.should_quit(),
        final_snapshot: editor.snapshot(),
    })
}

/// Result of headless execution.
pub struct HeadlessResult {
    /// Whether the editor quit.
    pub quit: bool,
    /// Final editor snapshot.
    pub final_snapshot: kjxlkj_core::EditorSnapshot,
}
