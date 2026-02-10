//! Decode crossterm events into internal types.

use crossterm::event::{Event, KeyCode as CtKeyCode, KeyEvent, KeyModifiers as CtMods};
use kjxlkj_core_types::{Action, Key, KeyCode, KeyModifiers};

/// Decoded event: either an Action or a Key.
pub enum DecodedEvent {
    Action(Action),
    Key(Key),
    Ignore,
}

/// Decode a crossterm event into our internal types.
pub fn decode_crossterm_event(event: Event) -> DecodedEvent {
    match event {
        Event::Key(key_event) => {
            let key = decode_key_event(key_event);
            DecodedEvent::Key(key)
        }
        Event::Resize(cols, rows) => DecodedEvent::Action(Action::Resize(cols, rows)),
        Event::Paste(text) => DecodedEvent::Action(Action::Paste(text)),
        Event::FocusGained => DecodedEvent::Action(Action::FocusGained),
        Event::FocusLost => DecodedEvent::Action(Action::FocusLost),
        Event::Mouse(_) => DecodedEvent::Ignore,
    }
}

fn decode_key_event(event: KeyEvent) -> Key {
    let modifiers = decode_modifiers(event.modifiers);
    let code = decode_key_code(event.code, event.modifiers);

    // Normalize: shifted printable keys.
    // If Shift is the only modifier and it's a Char, make it uppercase
    // and drop the Shift modifier. This ensures Shift+a dispatches as 'A'.
    if modifiers == KeyModifiers::SHIFT {
        if let KeyCode::Char(c) = &code {
            if c.is_ascii_alphabetic() {
                return Key::new(KeyCode::Char(c.to_ascii_uppercase()), KeyModifiers::NONE);
            }
        }
    }

    Key::new(code, modifiers)
}

fn decode_key_code(code: CtKeyCode, _mods: CtMods) -> KeyCode {
    match code {
        CtKeyCode::Char(c) => {
            // If shift is pressed and the char is already uppercase,
            // the terminal already handled the shift.
            KeyCode::Char(c)
        }
        CtKeyCode::Esc => KeyCode::Esc,
        CtKeyCode::Enter => KeyCode::Enter,
        CtKeyCode::Backspace => KeyCode::Backspace,
        CtKeyCode::Tab => KeyCode::Tab,
        CtKeyCode::BackTab => KeyCode::BackTab,
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
        CtKeyCode::Null => KeyCode::Null,
        _ => KeyCode::Null,
    }
}

fn decode_modifiers(mods: CtMods) -> KeyModifiers {
    let mut result = KeyModifiers::NONE;
    if mods.contains(CtMods::SHIFT) {
        result = result.union(KeyModifiers::SHIFT);
    }
    if mods.contains(CtMods::CONTROL) {
        result = result.union(KeyModifiers::CTRL);
    }
    if mods.contains(CtMods::ALT) {
        result = result.union(KeyModifiers::ALT);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventKind, KeyEventState};

    fn make_key_event(code: CtKeyCode, mods: CtMods) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: mods,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn test_shift_a_normalizes_to_uppercase_a() {
        let event = make_key_event(CtKeyCode::Char('a'), CtMods::SHIFT);
        let key = decode_key_event(event);
        assert_eq!(key.code, KeyCode::Char('A'));
        assert_eq!(key.modifiers, KeyModifiers::NONE);
    }

    #[test]
    fn test_plain_a_stays_lowercase() {
        let event = make_key_event(CtKeyCode::Char('a'), CtMods::NONE);
        let key = decode_key_event(event);
        assert_eq!(key.code, KeyCode::Char('a'));
        assert_eq!(key.modifiers, KeyModifiers::NONE);
    }

    #[test]
    fn test_ctrl_r_preserves_modifier() {
        let event = make_key_event(CtKeyCode::Char('r'), CtMods::CONTROL);
        let key = decode_key_event(event);
        assert_eq!(key.code, KeyCode::Char('r'));
        assert!(key.modifiers.contains(KeyModifiers::CTRL));
    }
}
