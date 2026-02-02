//! Key sequence parser.

use crate::key::{Key, KeyEvent, KeyModifiers};

/// Parses key sequences from strings.
#[derive(Debug, Default)]
pub struct KeyParser;

impl KeyParser {
    /// Parses a key sequence string like "<C-w>h" or "dd".
    pub fn parse(input: &str) -> Vec<KeyEvent> {
        let mut result = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '<' {
                // Parse special key notation
                let mut special = String::new();
                while let Some(&next) = chars.peek() {
                    if next == '>' {
                        chars.next();
                        break;
                    }
                    special.push(chars.next().unwrap());
                }
                if let Some(event) = Self::parse_special(&special) {
                    result.push(event);
                }
            } else {
                // Plain character
                result.push(KeyEvent::new(Key::Char(c), KeyModifiers::NONE));
            }
        }

        result
    }

    /// Parses a special key notation like "C-w" or "Enter".
    fn parse_special(s: &str) -> Option<KeyEvent> {
        let parts: Vec<&str> = s.split('-').collect();
        let mut modifiers = KeyModifiers::NONE;
        let key_part;

        if parts.len() == 1 {
            key_part = parts[0];
        } else {
            // Parse modifiers
            for part in parts.iter().take(parts.len() - 1) {
                match part.to_uppercase().as_str() {
                    "C" => modifiers.ctrl = true,
                    "A" | "M" => modifiers.alt = true,
                    "S" => modifiers.shift = true,
                    _ => {}
                }
            }
            key_part = parts.last()?;
        }

        let key = Self::parse_key_name(key_part)?;
        Some(KeyEvent::new(key, modifiers))
    }

    /// Parses a key name.
    fn parse_key_name(name: &str) -> Option<Key> {
        let upper = name.to_uppercase();
        Some(match upper.as_str() {
            "CR" | "ENTER" | "RETURN" => Key::Enter,
            "ESC" | "ESCAPE" => Key::Escape,
            "TAB" => Key::Tab,
            "BS" | "BACKSPACE" => Key::Backspace,
            "DEL" | "DELETE" => Key::Delete,
            "INS" | "INSERT" => Key::Insert,
            "HOME" => Key::Home,
            "END" => Key::End,
            "PAGEUP" | "PGUP" => Key::PageUp,
            "PAGEDOWN" | "PGDN" => Key::PageDown,
            "UP" => Key::Up,
            "DOWN" => Key::Down,
            "LEFT" => Key::Left,
            "RIGHT" => Key::Right,
            "SPACE" => Key::Char(' '),
            "LT" => Key::Char('<'),
            "GT" => Key::Char('>'),
            "BAR" => Key::Char('|'),
            s if s.len() == 1 => Key::Char(s.chars().next()?),
            s if s.starts_with('F') => {
                let n: u8 = s[1..].parse().ok()?;
                Key::F(n)
            }
            _ => return None,
        })
    }
}
