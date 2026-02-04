//! Convert crossterm events to our key types.

use crossterm::event::{Event, KeyCode as CtKeyCode, KeyEvent, KeyModifiers};
use kjxlkj_core_types::{Key, KeyCode, Modifiers};

/// Convert a crossterm event to our Key type.
pub fn convert_event(event: Event) -> Option<Key> {
    match event {
        Event::Key(key_event) => Some(convert_key_event(key_event)),
        _ => None,
    }
}

fn convert_key_event(event: KeyEvent) -> Key {
    let code = convert_key_code(event.code);
    let mods = convert_modifiers(event.modifiers);
    Key::with_mods(code, mods)
}

fn convert_key_code(code: CtKeyCode) -> KeyCode {
    match code {
        CtKeyCode::Char(c) => KeyCode::Char(c),
        CtKeyCode::Backspace => KeyCode::Backspace,
        CtKeyCode::Enter => KeyCode::Enter,
        CtKeyCode::Tab => KeyCode::Tab,
        CtKeyCode::Esc => KeyCode::Esc,
        CtKeyCode::Up => KeyCode::Up,
        CtKeyCode::Down => KeyCode::Down,
        CtKeyCode::Left => KeyCode::Left,
        CtKeyCode::Right => KeyCode::Right,
        CtKeyCode::Home => KeyCode::Home,
        CtKeyCode::End => KeyCode::End,
        CtKeyCode::PageUp => KeyCode::PageUp,
        CtKeyCode::PageDown => KeyCode::PageDown,
        CtKeyCode::Delete => KeyCode::Delete,
        CtKeyCode::Insert => KeyCode::Insert,
        CtKeyCode::F(n) => KeyCode::F(n),
        CtKeyCode::Null => KeyCode::Null,
        _ => KeyCode::Null,
    }
}

fn convert_modifiers(mods: KeyModifiers) -> Modifiers {
    Modifiers {
        ctrl: mods.contains(KeyModifiers::CONTROL),
        alt: mods.contains(KeyModifiers::ALT),
        shift: mods.contains(KeyModifiers::SHIFT),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_char() {
        let event = KeyEvent::new(CtKeyCode::Char('a'), KeyModifiers::NONE);
        let key = convert_key_event(event);
        assert_eq!(key.code, KeyCode::Char('a'));
        assert!(key.mods.is_empty());
    }

    #[test]
    fn test_convert_ctrl() {
        let event = KeyEvent::new(CtKeyCode::Char('c'), KeyModifiers::CONTROL);
        let key = convert_key_event(event);
        assert_eq!(key.code, KeyCode::Char('c'));
        assert!(key.mods.ctrl);
    }
}
