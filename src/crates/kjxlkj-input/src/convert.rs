//! Convert crossterm events to our key type.

use crate::{Key, KeyCode, Modifiers};
use crossterm::event::{Event, KeyCode as CtKeyCode, KeyEvent, KeyModifiers};

/// Convert a crossterm event to our Key type.
pub fn convert_event(event: Event) -> Option<Key> {
    match event {
        Event::Key(KeyEvent {
            code, modifiers, ..
        }) => {
            let mods = Modifiers {
                ctrl: modifiers.contains(KeyModifiers::CONTROL),
                alt: modifiers.contains(KeyModifiers::ALT),
                shift: modifiers.contains(KeyModifiers::SHIFT),
            };
            let code = match code {
                CtKeyCode::Char(c) => KeyCode::Char(c),
                CtKeyCode::Enter => KeyCode::Enter,
                CtKeyCode::Esc => KeyCode::Esc,
                CtKeyCode::Backspace => KeyCode::Backspace,
                CtKeyCode::Tab => KeyCode::Tab,
                CtKeyCode::Left => KeyCode::Left,
                CtKeyCode::Right => KeyCode::Right,
                CtKeyCode::Up => KeyCode::Up,
                CtKeyCode::Down => KeyCode::Down,
                CtKeyCode::Home => KeyCode::Home,
                CtKeyCode::End => KeyCode::End,
                CtKeyCode::PageUp => KeyCode::PageUp,
                CtKeyCode::PageDown => KeyCode::PageDown,
                CtKeyCode::Delete => KeyCode::Delete,
                CtKeyCode::Insert => KeyCode::Insert,
                CtKeyCode::F(n) => KeyCode::F(n),
                _ => return None,
            };
            Some(Key::new(code, mods))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventKind, KeyEventState};

    #[test]
    fn test_convert_char() {
        let event = Event::Key(KeyEvent {
            code: CtKeyCode::Char('a'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        let key = convert_event(event).unwrap();
        assert_eq!(key.code, KeyCode::Char('a'));
    }

    #[test]
    fn test_convert_ctrl() {
        let event = Event::Key(KeyEvent {
            code: CtKeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
        let key = convert_event(event).unwrap();
        assert!(key.mods.ctrl);
    }
}
