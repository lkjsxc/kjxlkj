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

/// Parse a key sequence notation like `<C-w>j` or `<Leader>ff`.
pub fn parse_key_sequence(notation: &str) -> Vec<KeyChord> {
    let mut result = Vec::new();
    let mut chars = notation.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c == '<' {
            let mut bracket = String::from('<');
            chars.next();
            while let Some(&ch) = chars.peek() {
                bracket.push(ch);
                chars.next();
                if ch == '>' {
                    break;
                }
            }
            if let Some(chord) = parse_key_notation(&bracket) {
                result.push(chord);
            }
        } else {
            chars.next();
            result.push(KeyChord {
                key: c.to_string(),
                ctrl: false,
                alt: false,
                shift: false,
            });
        }
    }
    result
}

/// Validate a key sequence notation string.
pub fn validate_key_sequence(notation: &str) -> Result<(), String> {
    if notation.is_empty() {
        return Err("empty key sequence".into());
    }
    let chords = parse_key_sequence(notation);
    if chords.is_empty() {
        return Err(format!("could not parse: {notation}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plain_key() {
        let k = parse_key_notation("j").unwrap();
        assert_eq!(k.key, "j");
        assert!(!k.ctrl && !k.alt && !k.shift);
    }

    #[test]
    fn parse_ctrl_key() {
        let k = parse_key_notation("<C-s>").unwrap();
        assert_eq!(k.key, "s");
        assert!(k.ctrl);
    }

    #[test]
    fn parse_special() {
        let k = parse_key_notation("<Space>").unwrap();
        assert_eq!(k.key, "Space");
    }

    #[test]
    fn parse_sequence() {
        let seq = parse_key_sequence("<C-w>j");
        assert_eq!(seq.len(), 2);
        assert!(seq[0].ctrl);
        assert_eq!(seq[1].key, "j");
    }

    #[test]
    fn validate_empty() {
        assert!(validate_key_sequence("").is_err());
    }

    #[test]
    fn resolve_f_keys() {
        assert_eq!(resolve_special("F1"), Some("F1".into()));
        assert_eq!(resolve_special("f12"), Some("F12".into()));
        assert_eq!(resolve_special("f13"), None);
    }

    #[test]
    fn parse_multi_modifier() {
        let k = parse_key_notation("<C-A-x>").unwrap();
        assert!(k.ctrl && k.alt);
        assert_eq!(k.key, "x");
    }
}
