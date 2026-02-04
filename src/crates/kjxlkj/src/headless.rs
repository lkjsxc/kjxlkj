//! Headless execution mode.

use crate::handlers;
use anyhow::Result;
use kjxlkj_core::EditorState;
use kjxlkj_input::{Key, KeyCode, Modifiers};
use kjxlkj_services::Services;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Run the editor in headless mode.
pub fn run_headless(file_path: Option<PathBuf>, script_path: Option<PathBuf>) -> Result<()> {
    let mut state = EditorState::new();
    let services = Services::new();

    state.viewport.width = 80;
    state.viewport.height = 24;

    // Open file if provided
    if let Some(path) = file_path {
        if path.exists() {
            state.open_file(&path)?;
        } else {
            state.file_path = Some(path);
        }
    }

    // Run script if provided
    if let Some(script) = script_path {
        let content = fs::read_to_string(&script)?;
        let keys = parse_script(&content)?;

        for key in keys {
            let action = handlers::handle_key(&mut state, key, &services);
            state.ensure_cursor_valid();
            if let handlers::Action::Quit = action {
                break;
            }
        }
    }

    // Print final buffer content to stdout
    println!("{}", state.buffer.contents());

    Ok(())
}

#[derive(Deserialize)]
struct ScriptKey {
    code: ScriptKeyCode,
    #[serde(default)]
    mods: ScriptMods,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ScriptKeyCode {
    Char { char: char },
    Named { name: String },
}

#[derive(Deserialize, Default)]
struct ScriptMods {
    #[serde(default)]
    ctrl: bool,
    #[serde(default)]
    alt: bool,
    #[serde(default)]
    shift: bool,
}

fn parse_script(content: &str) -> Result<Vec<Key>> {
    let script: Vec<ScriptKey> = serde_json::from_str(content)?;
    let mut keys = Vec::new();

    for sk in script {
        let code = match sk.code {
            ScriptKeyCode::Char { char } => KeyCode::Char(char),
            ScriptKeyCode::Named { name } => match name.as_str() {
                "Enter" => KeyCode::Enter,
                "Esc" => KeyCode::Esc,
                "Backspace" => KeyCode::Backspace,
                "Tab" => KeyCode::Tab,
                "Left" => KeyCode::Left,
                "Right" => KeyCode::Right,
                "Up" => KeyCode::Up,
                "Down" => KeyCode::Down,
                _ => continue,
            },
        };
        let mods = Modifiers {
            ctrl: sk.mods.ctrl,
            alt: sk.mods.alt,
            shift: sk.mods.shift,
        };
        keys.push(Key::new(code, mods));
    }

    Ok(keys)
}
