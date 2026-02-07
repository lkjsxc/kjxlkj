//! Keybinding notation parser (e.g., `<C-s>`, `<A-x>`, `<Leader>f`).

use serde::{Deserialize, Serialize};

/// Special key names recognized in angle-bracket notation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialKey {
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    Delete,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Insert,
    Nul,
    Leader,
}

/// A single key chord parsed from notation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyChord {
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

/// Resolve a special key name to its canonical string representation.
pub fn resolve_special(name: &str) -> Option<String> {
    let s = match name.to_lowercase().as_str() {
        "space" | "sp" => "Space",
        "enter" | "return" | "cr" => "Enter",
        "esc" | "escape" => "Escape",
        "bs" | "backspace" => "Backspace",
        "tab" => "Tab",
        "del" | "delete" => "Delete",
        "left" => "Left",
        "right" => "Right",
        "up" => "Up",
        "down" => "Down",
        "home" => "Home",
        "end" => "End",
        "pageup" => "PageUp",
        "pagedown" => "PageDown",
        "insert" => "Insert",
        "nul" => "Nul",
        "leader" => "Leader",
        s if s.starts_with('f') => {
            let n: u8 = s[1..].parse().ok()?;
            if (1..=12).contains(&n) {
                return Some(format!("F{n}"));
            }
            return None;
        }
        _ => return None,
    };
    Some(s.to_string())
}

/// Parse a single key notation like `<C-a>`, `<A-S-x>`, `<Space>`, or `j`.
pub fn parse_key_notation(notation: &str) -> Option<KeyChord> {
    if notation.starts_with('<') && notation.ends_with('>') {
        let inner = &notation[1..notation.len() - 1];
        let parts: Vec<&str> = inner.split('-').collect();
        let (mut ctrl, mut alt, mut shift) = (false, false, false);
        for &p in &parts[..parts.len() - 1] {
            match p.to_uppercase().as_str() {
                "C" => ctrl = true,
                "A" | "M" => alt = true,
                "S" => shift = true,
                _ => return None,
            }
        }
        let key_part = parts.last()?;
        let key = resolve_special(key_part).unwrap_or_else(|| key_part.to_string());
        Some(KeyChord {
            key,
            ctrl,
            alt,
            shift,
        })
    } else if notation.len() == 1 {
        Some(KeyChord {
            key: notation.to_string(),
            ctrl: false,
            alt: false,
            shift: false,
        })
    } else {
        None
    }
}

// Re-export sequence parsing from dedicated module.
pub use crate::keybinding_dsl_seq::{parse_key_sequence, validate_key_sequence};
