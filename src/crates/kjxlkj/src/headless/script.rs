//! Script parsing and execution.

use anyhow::Result;
use kjxlkj_core::{EditorState, Mode};
use kjxlkj_input::{Key, KeyCode, Modifiers};
use serde::{Deserialize, Serialize};

use super::processor;

/// A scripted key event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptKey {
    pub code: String,
    #[serde(default)]
    pub ctrl: bool,
    #[serde(default)]
    pub alt: bool,
    #[serde(default)]
    pub shift: bool,
}

/// A script step.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ScriptStep {
    #[serde(rename = "key")]
    Key(ScriptKey),
    #[serde(rename = "keys")]
    Keys { keys: String },
    #[serde(rename = "assert_mode")]
    AssertMode { mode: String },
    #[serde(rename = "assert_cursor")]
    AssertCursor { line: usize, col: usize },
    #[serde(rename = "assert_line")]
    AssertLine { line: usize, content: String },
}

/// Run a script on the editor state.
pub fn run_script(state: &mut EditorState, script: &str) -> Result<()> {
    // Try to parse as array of steps
    if let Ok(steps) = serde_json::from_str::<Vec<ScriptStep>>(script) {
        for step in steps {
            execute_step(state, step)?;
        }
        return Ok(());
    }

    // Try to parse as simple key array
    if let Ok(keys) = serde_json::from_str::<Vec<ScriptKey>>(script) {
        for key in keys {
            let k = parse_key(&key)?;
            processor::process_key(state, k);
        }
        return Ok(());
    }

    anyhow::bail!("Invalid script format");
}

fn execute_step(state: &mut EditorState, step: ScriptStep) -> Result<()> {
    match step {
        ScriptStep::Key(key) => {
            let k = parse_key(&key)?;
            processor::process_key(state, k);
        }
        ScriptStep::Keys { keys } => {
            for c in keys.chars() {
                let k = Key::char(c);
                processor::process_key(state, k);
            }
        }
        ScriptStep::AssertMode { mode } => assert_mode(state, &mode)?,
        ScriptStep::AssertCursor { line, col } => assert_cursor(state, line, col)?,
        ScriptStep::AssertLine { line, content } => assert_line(state, line, &content)?,
    }
    Ok(())
}

fn assert_mode(state: &EditorState, mode: &str) -> Result<()> {
    let expected = match mode {
        "normal" | "Normal" | "NORMAL" => Mode::Normal,
        "insert" | "Insert" | "INSERT" => Mode::Insert,
        "visual" | "Visual" | "VISUAL" => Mode::Visual,
        "visual_line" | "Visual_Line" | "VISUAL_LINE" | "V-LINE" => Mode::VisualLine,
        "visual_block" | "Visual_Block" | "VISUAL_BLOCK" | "V-BLOCK" => Mode::VisualBlock,
        "command" | "Command" | "COMMAND" => Mode::Command,
        "replace" | "Replace" | "REPLACE" => Mode::Replace,
        _ => anyhow::bail!("Unknown mode: {}", mode),
    };
    if state.mode() != expected {
        anyhow::bail!(
            "Mode assertion failed: expected {:?}, got {:?}",
            expected,
            state.mode()
        );
    }
    Ok(())
}

fn assert_cursor(state: &EditorState, line: usize, col: usize) -> Result<()> {
    let actual_line = state.cursor.line();
    let actual_col = state.cursor.col();
    if actual_line != line || actual_col != col {
        anyhow::bail!(
            "Cursor assertion failed: expected ({}, {}), got ({}, {})",
            line,
            col,
            actual_line,
            actual_col
        );
    }
    Ok(())
}

fn assert_line(state: &EditorState, line: usize, content: &str) -> Result<()> {
    let actual = state.buffer.line(line).unwrap_or_default();
    if actual != content {
        anyhow::bail!(
            "Line {} assertion failed: expected {:?}, got {:?}",
            line,
            content,
            actual
        );
    }
    Ok(())
}

/// Parse a script key into an input key.
pub fn parse_key(key: &ScriptKey) -> Result<Key> {
    let code = match key.code.as_str() {
        "Escape" | "Esc" => KeyCode::Escape,
        "Enter" | "Return" => KeyCode::Enter,
        "Backspace" => KeyCode::Backspace,
        "Tab" => KeyCode::Tab,
        "Left" => KeyCode::Left,
        "Right" => KeyCode::Right,
        "Up" => KeyCode::Up,
        "Down" => KeyCode::Down,
        s if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap()),
        _ => anyhow::bail!("Unknown key code: {}", key.code),
    };

    Ok(Key::new(
        code,
        Modifiers {
            ctrl: key.ctrl,
            alt: key.alt,
            shift: key.shift,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_key() {
        let key = ScriptKey {
            code: "a".to_string(),
            ctrl: false,
            alt: false,
            shift: false,
        };
        let k = parse_key(&key).unwrap();
        assert_eq!(k.code, KeyCode::Char('a'));
    }

    #[test]
    fn parse_escape_key() {
        let key = ScriptKey {
            code: "Escape".to_string(),
            ctrl: false,
            alt: false,
            shift: false,
        };
        let k = parse_key(&key).unwrap();
        assert!(k.is_escape());
    }
}
