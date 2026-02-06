//! Headless mode: run scripts without a terminal.

use anyhow::Result;
use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{Intent, Mode, Size};

/// Run the editor in headless mode (for testing, scripting).
pub async fn run_headless(
    file: Option<String>,
    script: Option<String>,
) -> Result<()> {
    let size = Size::new(80, 24);
    let mut state = EditorState::new(size);

    // Create initial buffer
    let text = if let Some(ref path) = file {
        match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    let bid = if text.is_empty() {
        state.create_buffer()
    } else {
        state.create_buffer_from_text(&text)
    };

    if let Some(ref path) = file {
        if let Some(buf) = state.buffers.get_mut(&bid) {
            buf.text.set_file_path(path.as_str());
        }
    }

    let wid = state.create_window(bid);
    if let Some(win) = state.windows.get_mut(&wid) {
        win.height = size.height as usize - 2;
    }

    // Execute script if provided
    if let Some(ref script_path) = script {
        let script_content = std::fs::read_to_string(script_path)?;
        execute_script(&mut state, &script_content)?;
    }

    // Print final buffer content
    if let Some(buf) = state.active_buffer() {
        print!("{}", buf.text.text());
    }

    Ok(())
}

/// Execute a headless script (simple command format).
fn execute_script(state: &mut EditorState, script: &str) -> Result<()> {
    for line in script.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        execute_command(state, line)?;
    }
    Ok(())
}

/// Execute a single headless command.
fn execute_command(state: &mut EditorState, cmd: &str) -> Result<()> {
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    match parts[0] {
        "keys" => {
            if let Some(keys) = parts.get(1) {
                for c in keys.chars() {
                    let key = kjxlkj_core_types::KeyEvent::char(c);
                    let intent = match state.current_mode() {
                        Mode::Normal | Mode::OperatorPending => {
                            state.parser.parse_normal(&key)
                        }
                        Mode::Insert => state.parser.parse_insert(&key),
                        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                            state.parser.parse_visual(&key)
                        }
                        Mode::Command => state.parser.parse_command(&key),
                        Mode::Replace => state.parser.parse_replace(&key),
                    };
                    dispatch_intent(state, intent);
                }
            }
        }
        "escape" => {
            let key = kjxlkj_core_types::KeyEvent::special(
                kjxlkj_core_types::KeyCode::Escape,
            );
            let intent = match state.current_mode() {
                Mode::Insert => state.parser.parse_insert(&key),
                _ => state.parser.parse_normal(&key),
            };
            dispatch_intent(state, intent);
        }
        "enter" => {
            let key = kjxlkj_core_types::KeyEvent::special(
                kjxlkj_core_types::KeyCode::Enter,
            );
            let intent = match state.current_mode() {
                Mode::Insert => state.parser.parse_insert(&key),
                _ => state.parser.parse_normal(&key),
            };
            dispatch_intent(state, intent);
        }
        "assert_mode" => {
            if let Some(expected) = parts.get(1) {
                let mode = state.current_mode();
                let actual = format!("{}", mode).to_lowercase();
                if actual != expected.to_lowercase() {
                    anyhow::bail!(
                        "mode assert failed: expected '{}', got '{}'",
                        expected,
                        actual
                    );
                }
            }
        }
        "assert_line" => {
            if let Some(args) = parts.get(1) {
                let mut parts = args.splitn(2, ' ');
                if let (Some(line_s), Some(expected)) = (parts.next(), parts.next()) {
                    let line_num: usize = line_s.parse()?;
                    if let Some(buf) = state.active_buffer() {
                        let actual = buf.text.line_to_string(line_num);
                        if actual != expected {
                            anyhow::bail!(
                                "line {} assert: expected '{}', got '{}'",
                                line_num,
                                expected,
                                actual
                            );
                        }
                    }
                }
            }
        }
        "assert_cursor" => {
            if let Some(args) = parts.get(1) {
                let coords: Vec<&str> = args.split(',').collect();
                if coords.len() == 2 {
                    let line: usize = coords[0].trim().parse()?;
                    let col: usize = coords[1].trim().parse()?;
                    let cursor = state.cursor();
                    if cursor.line != line || cursor.col != col {
                        anyhow::bail!(
                            "cursor assert: expected ({},{}), got ({},{})",
                            line, col, cursor.line, cursor.col
                        );
                    }
                }
            }
        }
        "ex" => {
            if let Some(cmd_str) = parts.get(1) {
                dispatch_intent(
                    state,
                    Intent::ExCommand(cmd_str.to_string()),
                );
            }
        }
        _ => {
            tracing::warn!(cmd = cmd, "unknown headless command");
        }
    }
    Ok(())
}
