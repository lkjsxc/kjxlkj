//! Script input for headless mode.

use serde::{Deserialize, Serialize};
use kjxlkj_core_types::{Key, KeyCode, KeyModifiers};

/// Script step for headless execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ScriptStep {
    /// Send a key event.
    #[serde(rename = "key")]
    Key(ScriptKey),
    /// Wait for a duration (ms).
    #[serde(rename = "wait")]
    Wait { ms: u64 },
    /// Assert buffer content.
    #[serde(rename = "assert")]
    Assert { line: u32, contains: String },
}

/// Key representation for scripts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptKey {
    pub code: String,
    #[serde(default)]
    pub mods: Vec<String>,
}

impl ScriptKey {
    /// Convert to our Key type.
    pub fn to_key(&self) -> Key {
        let code = parse_key_code(&self.code);
        let mut mods = KeyModifiers::NONE;
        for m in &self.mods {
            match m.to_lowercase().as_str() {
                "ctrl" | "control" => mods = mods.union(KeyModifiers::CTRL),
                "alt" => mods = mods.union(KeyModifiers::ALT),
                "shift" => mods = mods.union(KeyModifiers::SHIFT),
                _ => {}
            }
        }
        Key { code, mods }
    }
}

fn parse_key_code(s: &str) -> KeyCode {
    match s.to_lowercase().as_str() {
        "esc" | "escape" => KeyCode::Esc,
        "enter" | "return" | "cr" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "backspace" | "bs" => KeyCode::Backspace,
        "delete" | "del" => KeyCode::Delete,
        "insert" | "ins" => KeyCode::Insert,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pageup" | "pgup" => KeyCode::PageUp,
        "pagedown" | "pgdn" => KeyCode::PageDown,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "f1" => KeyCode::F(1),
        "f2" => KeyCode::F(2),
        "f3" => KeyCode::F(3),
        "f4" => KeyCode::F(4),
        "f5" => KeyCode::F(5),
        "f6" => KeyCode::F(6),
        "f7" => KeyCode::F(7),
        "f8" => KeyCode::F(8),
        "f9" => KeyCode::F(9),
        "f10" => KeyCode::F(10),
        "f11" => KeyCode::F(11),
        "f12" => KeyCode::F(12),
        "space" => KeyCode::Char(' '),
        s if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap()),
        _ => KeyCode::Char('\0'),
    }
}

/// Parse a script file.
pub fn parse_script(content: &str) -> Result<Vec<ScriptStep>, serde_json::Error> {
    serde_json::from_str(content)
}

/// Parse simple key array format.
pub fn parse_key_array(content: &str) -> Result<Vec<Key>, serde_json::Error> {
    let keys: Vec<ScriptKey> = serde_json::from_str(content)?;
    Ok(keys.into_iter().map(|k| k.to_key()).collect())
}
