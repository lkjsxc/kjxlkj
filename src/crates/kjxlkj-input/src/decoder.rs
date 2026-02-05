//! Input event decoding.

use crossterm::event::{Event, KeyCode, KeyModifiers};
use kjxlkj_core_types::{EditorEvent, KeyEvent, Modifier};

/// Decode a crossterm event into an editor event.
pub fn decode_event(event: Event) -> Option<EditorEvent> {
    match event {
        Event::Key(key_event) => {
            let modifiers = Modifier {
                ctrl: key_event.modifiers.contains(KeyModifiers::CONTROL),
                alt: key_event.modifiers.contains(KeyModifiers::ALT),
                shift: key_event.modifiers.contains(KeyModifiers::SHIFT),
            };

            let key = match key_event.code {
                KeyCode::Char(c) => KeyEvent::Char(c, modifiers),
                KeyCode::Esc => KeyEvent::Escape,
                KeyCode::Enter => KeyEvent::Enter,
                KeyCode::Backspace => KeyEvent::Backspace,
                KeyCode::Tab => KeyEvent::Tab,
                KeyCode::Left => KeyEvent::Left,
                KeyCode::Right => KeyEvent::Right,
                KeyCode::Up => KeyEvent::Up,
                KeyCode::Down => KeyEvent::Down,
                KeyCode::Home => KeyEvent::Home,
                KeyCode::End => KeyEvent::End,
                KeyCode::PageUp => KeyEvent::PageUp,
                KeyCode::PageDown => KeyEvent::PageDown,
                KeyCode::Delete => KeyEvent::Delete,
                _ => return None,
            };

            Some(EditorEvent::Key(key))
        }
        Event::Resize(w, h) => Some(EditorEvent::Resize(w, h)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyEvent as CtKeyEvent;

    #[test]
    fn test_decode_char() {
        let ct_event = Event::Key(CtKeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        let result = decode_event(ct_event);
        assert!(matches!(
            result,
            Some(EditorEvent::Key(KeyEvent::Char('a', _)))
        ));
    }

    #[test]
    fn test_decode_escape() {
        let ct_event = Event::Key(CtKeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        let result = decode_event(ct_event);
        assert!(matches!(result, Some(EditorEvent::Key(KeyEvent::Escape))));
    }
}
