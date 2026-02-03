//! Headless test runner.

use std::fs;

use anyhow::Result;
use serde::Deserialize;

use kjxlkj_core::{EditorState, KeyCode, KeyInput, Modifiers};

/// A scripted key input.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ScriptStep {
    Key(ScriptKey),
    Tagged(TaggedStep),
}

#[derive(Debug, Deserialize)]
pub struct ScriptKey {
    pub code: String,
    #[serde(default)]
    pub mods: ScriptMods,
}

#[derive(Debug, Default, Deserialize)]
pub struct ScriptMods {
    #[serde(default)]
    pub ctrl: bool,
    #[serde(default)]
    pub alt: bool,
    #[serde(default)]
    pub shift: bool,
}

#[derive(Debug, Deserialize)]
pub struct TaggedStep {
    pub kind: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Runs a headless script.
pub fn run_script(path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let steps: Vec<ScriptStep> = serde_json::from_str(&content)?;

    let mut state = EditorState::new();

    for step in steps {
        match step {
            ScriptStep::Key(key) => {
                let input = parse_key(&key);
                state.handle_key(input);
            }
            ScriptStep::Tagged(tagged) => {
                handle_tagged(&mut state, &tagged)?;
            }
        }

        if state.is_quit_requested() {
            break;
        }
    }

    let snapshot = state.snapshot();
    println!("Final buffer content:");
    for line in &snapshot.buffer.lines {
        println!("{}", line);
    }
    println!("Mode: {:?}", snapshot.status.mode);
    println!(
        "Cursor: {}:{}",
        snapshot.status.cursor_line, snapshot.status.cursor_col
    );

    Ok(())
}

fn parse_key(key: &ScriptKey) -> KeyInput {
    let code = match key.code.as_str() {
        "Escape" | "Esc" => KeyCode::Escape,
        "Enter" | "Return" => KeyCode::Enter,
        "Backspace" => KeyCode::Backspace,
        "Left" => KeyCode::Left,
        "Right" => KeyCode::Right,
        "Up" => KeyCode::Up,
        "Down" => KeyCode::Down,
        "Tab" => KeyCode::Tab,
        s if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap()),
        _ => KeyCode::Other,
    };

    KeyInput {
        code,
        modifiers: Modifiers {
            ctrl: key.mods.ctrl,
            alt: key.mods.alt,
            shift: key.mods.shift,
        },
    }
}

fn handle_tagged(state: &mut EditorState, step: &TaggedStep) -> Result<()> {
    match step.kind.as_str() {
        "assert_mode" => {
            let expected = step.data.get("mode").and_then(|v| v.as_str());
            if let Some(mode) = expected {
                let actual = format!("{:?}", state.mode());
                if actual.to_lowercase() != mode.to_lowercase() {
                    anyhow::bail!(
                        "Mode assertion failed: expected {}, got {}",
                        mode,
                        actual
                    );
                }
            }
        }
        "assert_content" => {
            let expected = step.data.get("content").and_then(|v| v.as_str());
            if let Some(content) = expected {
                let actual = state.buffer().content();
                if actual.trim() != content.trim() {
                    anyhow::bail!(
                        "Content assertion failed:\nexpected: {}\ngot: {}",
                        content,
                        actual
                    );
                }
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_char_key() {
        let key = ScriptKey {
            code: "a".to_string(),
            mods: ScriptMods::default(),
        };
        let input = parse_key(&key);
        assert_eq!(input.code, KeyCode::Char('a'));
    }

    #[test]
    fn parse_escape() {
        let key = ScriptKey {
            code: "Escape".to_string(),
            mods: ScriptMods::default(),
        };
        let input = parse_key(&key);
        assert_eq!(input.code, KeyCode::Escape);
    }
}
