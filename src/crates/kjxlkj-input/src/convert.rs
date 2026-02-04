//! Crossterm to internal key conversion.

use crossterm::event::{KeyCode as CtKeyCode, KeyEvent, KeyModifiers as CtMods};
use kjxlkj_core_mode::{Key, KeyCode, KeyModifiers};

/// Convert a crossterm KeyEvent to our internal Key type.
pub fn crossterm_to_key(event: KeyEvent) -> Key {
    let code = match event.code {
        CtKeyCode::Char(c) => KeyCode::Char(c),
        CtKeyCode::Esc => KeyCode::Escape,
        CtKeyCode::Enter => KeyCode::Enter,
        CtKeyCode::Tab => KeyCode::Tab,
        CtKeyCode::Backspace => KeyCode::Backspace,
        CtKeyCode::Delete => KeyCode::Delete,
        CtKeyCode::Up => KeyCode::Up,
        CtKeyCode::Down => KeyCode::Down,
        CtKeyCode::Left => KeyCode::Left,
        CtKeyCode::Right => KeyCode::Right,
        CtKeyCode::Home => KeyCode::Home,
        CtKeyCode::End => KeyCode::End,
        CtKeyCode::PageUp => KeyCode::PageUp,
        CtKeyCode::PageDown => KeyCode::PageDown,
        CtKeyCode::F(n) => KeyCode::F(n),
        _ => return Key::new(KeyCode::Escape), // Fallback
    };

    let modifiers = KeyModifiers {
        ctrl: event.modifiers.contains(CtMods::CONTROL),
        alt: event.modifiers.contains(CtMods::ALT),
        shift: event.modifiers.contains(CtMods::SHIFT),
    };

    Key { code, modifiers }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_key() {
        let event = KeyEvent::new(CtKeyCode::Esc, CtMods::NONE);
        let key = crossterm_to_key(event);
        assert_eq!(key.code, KeyCode::Escape);
    }

    #[test]
    fn arrow_keys() {
        let event = KeyEvent::new(CtKeyCode::Up, CtMods::NONE);
        let key = crossterm_to_key(event);
        assert_eq!(key.code, KeyCode::Up);
    }

    #[test]
    fn function_keys() {
        let event = KeyEvent::new(CtKeyCode::F(5), CtMods::NONE);
        let key = crossterm_to_key(event);
        assert_eq!(key.code, KeyCode::F(5));
    }

    #[test]
    fn modifier_combinations() {
        let event = KeyEvent::new(CtKeyCode::Char('s'), CtMods::CONTROL | CtMods::SHIFT);
        let key = crossterm_to_key(event);
        assert!(key.modifiers.ctrl);
        assert!(key.modifiers.shift);
        assert!(!key.modifiers.alt);
    }
}
