//! Decode crossterm events into the internal key model.
//!
//! Stage 1 of the input pipeline.

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyModifiers as CtKeyMods,
};
use kjxlkj_core_types::{Key, KeyModifiers};

use crate::{normalize::normalize_key, InputEvent};

/// Decode a crossterm Event into an InputEvent.
pub fn decode_crossterm_event(event: Event) -> InputEvent {
    match event {
        Event::Key(key_event) => decode_key(key_event),
        Event::Resize(cols, rows) => {
            InputEvent::Resize(cols, rows)
        }
        Event::Paste(text) => InputEvent::Paste(text),
        Event::FocusGained => InputEvent::FocusGained,
        Event::FocusLost => InputEvent::FocusLost,
        Event::Mouse(_) => InputEvent::Ignored,
    }
}

fn decode_key(ke: KeyEvent) -> InputEvent {
    let mods = KeyModifiers {
        ctrl: ke.modifiers.contains(CtKeyMods::CONTROL),
        alt: ke.modifiers.contains(CtKeyMods::ALT),
        shift: ke.modifiers.contains(CtKeyMods::SHIFT),
    };

    let raw_key = match ke.code {
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Enter => Key::Enter,
        KeyCode::Tab => Key::Tab,
        KeyCode::BackTab => Key::BackTab,
        KeyCode::Esc => Key::Escape,
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Insert => Key::Insert,
        KeyCode::Delete => Key::Delete,
        KeyCode::F(n) => Key::F(n),
        KeyCode::Null => Key::Null,
        _ => return InputEvent::Ignored,
    };

    // Apply printable normalization (Stage 2).
    let (normalized_key, normalized_mods) =
        normalize_key(raw_key, mods);

    InputEvent::Key(normalized_key, normalized_mods)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_a_normalized() {
        let event = Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            CtKeyMods::SHIFT,
        ));
        match decode_crossterm_event(event) {
            InputEvent::Key(key, mods) => {
                // After normalization, Shift+a -> A with
                // shift absorbed.
                assert_eq!(key, Key::Char('A'));
                assert!(!mods.shift);
            }
            _ => panic!("expected key event"),
        }
    }

    #[test]
    fn plain_a() {
        let event = Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            CtKeyMods::NONE,
        ));
        match decode_crossterm_event(event) {
            InputEvent::Key(key, mods) => {
                assert_eq!(key, Key::Char('a'));
                assert!(!mods.shift);
            }
            _ => panic!("expected key event"),
        }
    }

    #[test]
    fn resize_event() {
        let event = Event::Resize(120, 40);
        match decode_crossterm_event(event) {
            InputEvent::Resize(c, r) => {
                assert_eq!(c, 120);
                assert_eq!(r, 40);
            }
            _ => panic!("expected resize"),
        }
    }
}
