//! Headless mode for E2E testing.

use anyhow::Result;
use kjxlkj_core::types::{Key, KeyCode, Modifiers};
use kjxlkj_core::EditorState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::handler::handle_key;

/// Run in headless mode with a script.
pub async fn run_headless(args: &[String]) -> Result<()> {
    let script_idx = args.iter().position(|a| a == "--script");
    let script_path = script_idx
        .and_then(|i| args.get(i + 1))
        .ok_or_else(|| anyhow::anyhow!("--script path required"))?;

    let file_idx = args
        .iter()
        .position(|a| !a.starts_with('-') && a != "kjxlkj");
    let file_path = file_idx.and_then(|i| {
        if i > 0 && args.get(i - 1).map(|s| s.as_str()) == Some("--script") {
            None
        } else {
            args.get(i).map(|s| s.as_str())
        }
    });

    let mut editor = EditorState::new();

    // Load file if specified.
    if let Some(path) = file_path {
        if Path::new(path).exists() {
            let content = fs::read_to_string(path)?;
            editor.buffer = kjxlkj_core::state::BufferState::from_file(
                kjxlkj_core::types::BufferId::new(0),
                path.to_string(),
                &content,
            );
        }
    }

    // Parse and execute script.
    let script_content = fs::read_to_string(script_path)?;
    let keys = parse_script(&script_content)?;

    for key in keys {
        handle_key(&mut editor, key).await;
        if editor.should_quit {
            break;
        }
    }

    // Output final state.
    let output = HeadlessOutput {
        buffer_content: editor.buffer.text.to_string(),
        cursor_line: editor.buffer.cursor.line,
        cursor_col: editor.buffer.cursor.col,
        mode: format!("{:?}", editor.mode.mode),
        message: editor.status_message.clone(),
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct HeadlessOutput {
    buffer_content: String,
    cursor_line: usize,
    cursor_col: usize,
    mode: String,
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ScriptKey {
    code: String,
    #[serde(default)]
    mods: ScriptMods,
}

#[derive(Debug, Default, Deserialize)]
struct ScriptMods {
    #[serde(default)]
    ctrl: bool,
    #[serde(default)]
    alt: bool,
    #[serde(default)]
    shift: bool,
}

fn parse_script(content: &str) -> Result<Vec<Key>> {
    // Try parsing as array of key objects.
    if let Ok(keys) = serde_json::from_str::<Vec<ScriptKey>>(content) {
        return Ok(keys.into_iter().map(script_key_to_key).collect());
    }

    // Try parsing as simple string of characters.
    if let Ok(chars) = serde_json::from_str::<String>(content) {
        return Ok(chars.chars().map(Key::char).collect());
    }

    Err(anyhow::anyhow!("Invalid script format"))
}

fn script_key_to_key(sk: ScriptKey) -> Key {
    let code = match sk.code.as_str() {
        "Esc" | "Escape" => KeyCode::Esc,
        "Enter" | "Return" => KeyCode::Enter,
        "Backspace" => KeyCode::Backspace,
        "Tab" => KeyCode::Tab,
        "Up" => KeyCode::Up,
        "Down" => KeyCode::Down,
        "Left" => KeyCode::Left,
        "Right" => KeyCode::Right,
        s if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap()),
        _ => KeyCode::Null,
    };

    let mods = Modifiers {
        ctrl: sk.mods.ctrl,
        alt: sk.mods.alt,
        shift: sk.mods.shift,
    };

    Key::with_mods(code, mods)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_script() {
        let script = r#""ihello""#;
        let keys = parse_script(script).unwrap();
        assert_eq!(keys.len(), 6);
    }

    #[test]
    fn test_parse_key_objects() {
        let script = r#"[{"code": "i"}, {"code": "a"}]"#;
        let keys = parse_script(script).unwrap();
        assert_eq!(keys.len(), 2);
    }
}
