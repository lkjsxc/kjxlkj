//! Headless script support for testing and automation.

use kjxlkj_core_types::{KeyCode, KeyEvent, Mode, Modifiers};
use serde::{Deserialize, Serialize};

/// Steps in a headless test script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptStep {
    Key { code: String, ctrl: bool, alt: bool, shift: bool },
    Keys { keys: String },
    AssertMode { mode: String },
    AssertCursor { line: usize, col: usize },
    AssertLine { line: usize, content: String },
}

/// Parse a JSON script string into a list of steps.
pub fn parse_script(json: &str) -> Result<Vec<ScriptStep>, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

/// Parse a JSON value representing a key into a `KeyEvent`.
pub fn parse_script_key(value: &serde_json::Value) -> Option<KeyEvent> {
    let obj = value.as_object()?;
    let code_str = obj.get("code")?.as_str()?;
    let ctrl = obj.get("ctrl").and_then(|v| v.as_bool()).unwrap_or(false);
    let alt = obj.get("alt").and_then(|v| v.as_bool()).unwrap_or(false);
    let shift = obj.get("shift").and_then(|v| v.as_bool()).unwrap_or(false);
    let code = string_to_keycode(code_str)?;
    let mut mods = Modifiers::NONE;
    if ctrl { mods = mods.union(Modifiers::CTRL); }
    if alt { mods = mods.union(Modifiers::ALT); }
    if shift { mods = mods.union(Modifiers::SHIFT); }
    Some(KeyEvent::new(code, mods))
}

fn string_to_keycode(s: &str) -> Option<KeyCode> {
    match s {
        "Escape" | "Esc" => Some(KeyCode::Escape),
        "Enter" | "Return" => Some(KeyCode::Enter),
        "Backspace" => Some(KeyCode::Backspace),
        "Tab" => Some(KeyCode::Tab),
        "Delete" => Some(KeyCode::Delete),
        "Left" => Some(KeyCode::Left),
        "Right" => Some(KeyCode::Right),
        "Up" => Some(KeyCode::Up),
        "Down" => Some(KeyCode::Down),
        "Home" => Some(KeyCode::Home),
        "End" => Some(KeyCode::End),
        "PageUp" => Some(KeyCode::PageUp),
        "PageDown" => Some(KeyCode::PageDown),
        "Insert" => Some(KeyCode::Insert),
        "Space" => Some(KeyCode::Char(' ')),
        s if s.len() == 1 => Some(KeyCode::Char(s.chars().next()?)),
        s if s.starts_with('F') => {
            let n: u8 = s[1..].parse().ok()?;
            Some(KeyCode::F(n))
        }
        _ => None,
    }
}

/// Convert a `ScriptStep` to a list of key events.
pub fn script_step_to_keys(step: &ScriptStep) -> Vec<KeyEvent> {
    match step {
        ScriptStep::Key { code, ctrl, alt, shift } => {
            if let Some(kc) = string_to_keycode(code) {
                let mut mods = Modifiers::NONE;
                if *ctrl { mods = mods.union(Modifiers::CTRL); }
                if *alt { mods = mods.union(Modifiers::ALT); }
                if *shift { mods = mods.union(Modifiers::SHIFT); }
                vec![KeyEvent::new(kc, mods)]
            } else {
                vec![]
            }
        }
        ScriptStep::Keys { keys } => {
            keys.chars().map(|c| KeyEvent::plain(KeyCode::Char(c))).collect()
        }
        _ => vec![],
    }
}

/// Parse a mode string to a `Mode` enum value.
pub fn parse_mode_string(mode: &str) -> Option<Mode> {
    match mode.to_lowercase().as_str() {
        "normal" | "n" => Some(Mode::Normal),
        "insert" | "i" => Some(Mode::Insert),
        "visual" | "v" => Some(Mode::Visual),
        "visual_line" | "vl" => Some(Mode::VisualLine),
        "visual_block" | "vb" => Some(Mode::VisualBlock),
        "command" | "c" => Some(Mode::Command),
        "replace" | "r" => Some(Mode::Replace),
        "terminal" | "t" => Some(Mode::Terminal),
        "operator_pending" | "op" => Some(Mode::OperatorPending),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_script_json() {
        let json = r#"[
            {"Key":{"code":"i","ctrl":false,"alt":false,"shift":false}},
            {"Keys":{"keys":"hello"}},
            {"AssertMode":{"mode":"insert"}}
        ]"#;
        let steps = parse_script(json).unwrap();
        assert_eq!(steps.len(), 3);
    }

    #[test]
    fn parse_script_key_value() {
        let val: serde_json::Value = serde_json::json!({"code": "a", "ctrl": true});
        let key = parse_script_key(&val).unwrap();
        assert_eq!(key.code, KeyCode::Char('a'));
        assert!(key.modifiers.contains(Modifiers::CTRL));
    }

    #[test]
    fn step_to_keys_string() {
        let step = ScriptStep::Keys { keys: "abc".into() };
        assert_eq!(script_step_to_keys(&step).len(), 3);
    }

    #[test]
    fn mode_parsing() {
        assert_eq!(parse_mode_string("normal"), Some(Mode::Normal));
        assert_eq!(parse_mode_string("INSERT"), Some(Mode::Insert));
        assert_eq!(parse_mode_string("v"), Some(Mode::Visual));
        assert_eq!(parse_mode_string("bogus"), None);
    }

    #[test]
    fn key_step_to_keys() {
        let step = ScriptStep::Key {
            code: "Escape".into(), ctrl: false, alt: false, shift: false,
        };
        let keys = script_step_to_keys(&step);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].code, KeyCode::Escape);
    }

    #[test]
    fn assert_step_no_keys() {
        let step = ScriptStep::AssertMode { mode: "normal".into() };
        assert!(script_step_to_keys(&step).is_empty());
    }

    #[test]
    fn assert_cursor_step() {
        let step = ScriptStep::AssertCursor { line: 1, col: 0 };
        assert!(script_step_to_keys(&step).is_empty());
    }
}
