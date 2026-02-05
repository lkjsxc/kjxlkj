use anyhow::{bail, Context, Result};
use kjxlkj_core_state::Editor;
use kjxlkj_core_types::{EditorEvent, KeyEvent, Mode, Modifier};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ScriptFile {
    Steps(Vec<ScriptStep>),
    Keys(Vec<ScriptKey>),
}

#[derive(Debug, Deserialize)]
struct ScriptKey {
    code: String,
    #[serde(default)]
    ctrl: bool,
    #[serde(default)]
    alt: bool,
    #[serde(default)]
    shift: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
enum ScriptStep {
    #[serde(rename = "key")]
    Key {
        code: String,
        #[serde(default)]
        ctrl: bool,
        #[serde(default)]
        alt: bool,
        #[serde(default)]
        shift: bool,
    },
    #[serde(rename = "keys")]
    Keys { keys: String },
    #[serde(rename = "assert_mode")]
    AssertMode { mode: String },
    #[serde(rename = "assert_cursor")]
    AssertCursor { line: usize, col: usize },
    #[serde(rename = "assert_line")]
    AssertLine { line: usize, content: String },
}

pub fn run_headless(file: Option<PathBuf>, script: Option<PathBuf>) -> Result<()> {
    let mut editor = Editor::new(80, 24);

    if let Some(path) = file.as_ref() {
        editor
            .open_file(path)
            .with_context(|| format!("open file: {}", path.display()))?;
    }

    let Some(script_path) = script.as_ref() else {
        return Ok(());
    };

    let script_text = std::fs::read_to_string(script_path)
        .with_context(|| format!("read script: {}", script_path.display()))?;

    let parsed: ScriptFile =
        serde_json::from_str(&script_text).context("parse headless script JSON")?;

    let steps: Vec<ScriptStep> = match parsed {
        ScriptFile::Steps(steps) => steps,
        ScriptFile::Keys(keys) => keys
            .into_iter()
            .map(|k| ScriptStep::Key {
                code: k.code,
                ctrl: k.ctrl,
                alt: k.alt,
                shift: k.shift,
            })
            .collect(),
    };

    for (idx, step) in steps.iter().enumerate() {
        execute_step(idx, step, &mut editor)?;
    }

    Ok(())
}

fn execute_step(step_index: usize, step: &ScriptStep, editor: &mut Editor) -> Result<()> {
    match step {
        ScriptStep::Key {
            code,
            ctrl,
            alt,
            shift,
        } => {
            let mods = Modifier {
                ctrl: *ctrl,
                alt: *alt,
                shift: *shift,
            };
            let key = key_event_from_code(code, mods)
                .with_context(|| format!("step {step_index}: parse key '{code}'"))?;
            editor.process_event(EditorEvent::Key(key));
        }
        ScriptStep::Keys { keys } => {
            for c in keys.chars() {
                editor.process_event(EditorEvent::Key(KeyEvent::Char(c, Modifier::NONE)));
            }
        }
        ScriptStep::AssertMode { mode } => {
            let expected = parse_mode(mode)
                .with_context(|| format!("step {step_index}: parse expected mode '{mode}'"))?;
            let actual = editor.mode();
            if actual != expected {
                bail!(
                    "step {}: mode mismatch (expected {}, got {})",
                    step_index,
                    expected,
                    actual
                );
            }
        }

        ScriptStep::AssertCursor { line, col } => {
            let cursor = editor.cursor();
            if cursor.line != *line || cursor.column != *col {
                bail!(
                    "step {}: cursor mismatch (expected {}:{}, got {}:{})",
                    step_index,
                    line,
                    col,
                    cursor.line,
                    cursor.column
                );
            }
        }

        ScriptStep::AssertLine { line, content } => {
            let actual = editor
                .active_line(*line)
                .ok_or_else(|| anyhow::anyhow!("line out of bounds: {line}"))?;
            if actual != *content {
                bail!(
                    "step {}: line content mismatch at line {} (expected '{:?}', got '{:?}')",
                    step_index,
                    line,
                    content,
                    actual
                );
            }
        }
    }

    Ok(())
}

fn key_event_from_code(code: &str, mods: Modifier) -> Result<KeyEvent> {
    let normalized = code.trim();

    let named = match normalized {
        "Escape" | "Esc" => Some(KeyEvent::Escape),
        "Enter" | "Return" => Some(KeyEvent::Enter),
        "Backspace" => Some(KeyEvent::Backspace),
        "Tab" => Some(KeyEvent::Tab),
        "Left" => Some(KeyEvent::Left),
        "Right" => Some(KeyEvent::Right),
        "Up" => Some(KeyEvent::Up),
        "Down" => Some(KeyEvent::Down),
        _ => None,
    };

    if let Some(k) = named {
        return Ok(k);
    }

    let mut chars = normalized.chars();
    let Some(c) = chars.next() else {
        bail!("empty key code");
    };
    if chars.next().is_some() {
        bail!("key code must be a single character or a supported named key");
    }

    Ok(KeyEvent::Char(c, mods))
}

fn parse_mode(mode: &str) -> Result<Mode> {
    let m = mode.trim().to_ascii_lowercase();

    match m.as_str() {
        "normal" => Ok(Mode::Normal),
        "insert" => Ok(Mode::Insert),
        "visual" => Ok(Mode::Visual),
        "visual_line" | "visual-line" | "visualline" => Ok(Mode::VisualLine),
        "visual_block" | "visual-block" | "visualblock" => Ok(Mode::VisualBlock),
        "command" => Ok(Mode::Command),
        "search" => Ok(Mode::Search),
        "replace" => Ok(Mode::Replace),
        _ => bail!("unknown mode string: {mode}"),
    }
}
