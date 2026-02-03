//! Input event decoder.

use crossterm::event::{Event, KeyCode as CtKeyCode, KeyEvent, KeyModifiers};

use kjxlkj_core_mode::{KeyCode, KeyInput, Modifiers};

/// Decodes crossterm events into editor key inputs.
pub struct InputDecoder;

impl InputDecoder {
    /// Decodes a crossterm event into a key input.
    pub fn decode(event: Event) -> Option<KeyInput> {
        match event {
            Event::Key(key_event) => Some(Self::decode_key(key_event)),
            _ => None,
        }
    }

    fn decode_key(key: KeyEvent) -> KeyInput {
        let modifiers = Modifiers {
            ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
            alt: key.modifiers.contains(KeyModifiers::ALT),
            shift: key.modifiers.contains(KeyModifiers::SHIFT),
        };

        let code = match key.code {
            CtKeyCode::Char(c) => KeyCode::Char(c),
            CtKeyCode::Esc => KeyCode::Escape,
            CtKeyCode::Enter => KeyCode::Enter,
            CtKeyCode::Backspace => KeyCode::Backspace,
            CtKeyCode::Left => KeyCode::Left,
            CtKeyCode::Right => KeyCode::Right,
            CtKeyCode::Up => KeyCode::Up,
            CtKeyCode::Down => KeyCode::Down,
            CtKeyCode::Tab => KeyCode::Tab,
            _ => KeyCode::Other,
        };

        KeyInput { code, modifiers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyEventKind;

    #[test]
    fn decode_char() {
        let event = Event::Key(KeyEvent::new(CtKeyCode::Char('a'), KeyModifiers::NONE));
        let input = InputDecoder::decode(event).unwrap();
        assert_eq!(input.code, KeyCode::Char('a'));
    }

    #[test]
    fn decode_ctrl() {
        let event = Event::Key(KeyEvent::new(
            CtKeyCode::Char('c'),
            KeyModifiers::CONTROL,
        ));
        let input = InputDecoder::decode(event).unwrap();
        assert!(input.modifiers.ctrl);
    }
}
