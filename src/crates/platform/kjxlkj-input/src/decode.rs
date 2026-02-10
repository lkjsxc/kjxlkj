//! Key event decoding and normalization.

use crossterm::event::{
    Event as CrosstermEvent, KeyCode, KeyEvent as CrosstermKeyEvent, KeyModifiers,
};
use kjxlkj_core_types::{Key, KeyEvent, Modifiers, SpecialKey};
use tracing::trace;

/// Decode a crossterm event into our action type.
pub fn decode_event(event: CrosstermEvent) -> Option<DecodedEvent> {
    match event {
        CrosstermEvent::Key(key) => Some(DecodedEvent::Key(decode_key(key))),
        CrosstermEvent::Resize(cols, rows) => Some(DecodedEvent::Resize(cols, rows)),
        CrosstermEvent::Paste(text) => Some(DecodedEvent::Paste(text)),
        CrosstermEvent::FocusGained => Some(DecodedEvent::FocusGained),
        CrosstermEvent::FocusLost => Some(DecodedEvent::FocusLost),
        CrosstermEvent::Mouse(_) => None, // Ignore mouse events.
    }
}

/// Decoded event type.
#[derive(Debug, Clone)]
pub enum DecodedEvent {
    /// Key event.
    Key(KeyEvent),
    /// Resize event.
    Resize(u16, u16),
    /// Paste event.
    Paste(String),
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
}

/// Decode a crossterm key event into our KeyEvent.
///
/// This performs printable shift normalization: Shift+a becomes 'A'.
fn decode_key(key: CrosstermKeyEvent) -> KeyEvent {
    let raw_modifiers = key.modifiers;
    let shift = raw_modifiers.contains(KeyModifiers::SHIFT);
    let ctrl = raw_modifiers.contains(KeyModifiers::CONTROL);
    let alt = raw_modifiers.contains(KeyModifiers::ALT);

    let (key, modifiers) = match key.code {
        // Printable characters with shift normalization.
        KeyCode::Char(c) => {
            // For printable characters, the shift is already applied to the character.
            // e.g., Shift+a comes as 'A' with SHIFT modifier.
            // We normalize by using the character as-is and dropping shift for letters.
            let normalized_c = if c.is_ascii_alphabetic() {
                // Character is already uppercase if shift was pressed.
                c
            } else {
                c
            };

            // Drop shift modifier for printable characters (it's encoded in the char).
            let mods = Modifiers {
                shift: false, // Shift is encoded in the character itself.
                ctrl,
                alt,
            };

            (Key::Char(normalized_c), mods)
        }

        // Special keys.
        KeyCode::Esc => (Key::Special(SpecialKey::Escape), modifiers(shift, ctrl, alt)),
        KeyCode::Enter => (Key::Special(SpecialKey::Enter), modifiers(shift, ctrl, alt)),
        KeyCode::Tab => (Key::Special(SpecialKey::Tab), modifiers(shift, ctrl, alt)),
        KeyCode::Backspace => (Key::Special(SpecialKey::Backspace), modifiers(shift, ctrl, alt)),
        KeyCode::Delete => (Key::Special(SpecialKey::Delete), modifiers(shift, ctrl, alt)),
        KeyCode::Insert => (Key::Special(SpecialKey::Insert), modifiers(shift, ctrl, alt)),
        KeyCode::Home => (Key::Special(SpecialKey::Home), modifiers(shift, ctrl, alt)),
        KeyCode::End => (Key::Special(SpecialKey::End), modifiers(shift, ctrl, alt)),
        KeyCode::PageUp => (Key::Special(SpecialKey::PageUp), modifiers(shift, ctrl, alt)),
        KeyCode::PageDown => (Key::Special(SpecialKey::PageDown), modifiers(shift, ctrl, alt)),
        KeyCode::Up => (Key::Special(SpecialKey::Up), modifiers(shift, ctrl, alt)),
        KeyCode::Down => (Key::Special(SpecialKey::Down), modifiers(shift, ctrl, alt)),
        KeyCode::Left => (Key::Special(SpecialKey::Left), modifiers(shift, ctrl, alt)),
        KeyCode::Right => (Key::Special(SpecialKey::Right), modifiers(shift, ctrl, alt)),
        KeyCode::F(n) => (Key::Special(SpecialKey::F(n)), modifiers(shift, ctrl, alt)),

        // Fallback for other keys.
        _ => (Key::Special(SpecialKey::Escape), Modifiers::NONE),
    };

    trace!(?key, ?modifiers, "Decoded key");
    KeyEvent { key, modifiers }
}

/// Create modifiers struct.
fn modifiers(shift: bool, ctrl: bool, alt: bool) -> Modifiers {
    Modifiers { shift, ctrl, alt }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent as CtKeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    fn make_key_event(code: KeyCode, modifiers: KeyModifiers) -> CtKeyEvent {
        CtKeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn test_shift_a_becomes_uppercase() {
        let ct_event = make_key_event(KeyCode::Char('A'), KeyModifiers::SHIFT);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.key, Key::Char('A'));
        assert!(!decoded.modifiers.shift);
    }

    #[test]
    fn test_plain_a() {
        let ct_event = make_key_event(KeyCode::Char('a'), KeyModifiers::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.key, Key::Char('a'));
    }

    #[test]
    fn test_ctrl_w() {
        let ct_event = make_key_event(KeyCode::Char('w'), KeyModifiers::CONTROL);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.key, Key::Char('w'));
        assert!(decoded.modifiers.ctrl);
        assert!(!decoded.modifiers.shift);
    }

    #[test]
    fn test_ctrl_shift_v() {
        let ct_event = make_key_event(KeyCode::Char('V'), KeyModifiers::CONTROL | KeyModifiers::SHIFT);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.key, Key::Char('V'));
        assert!(decoded.modifiers.ctrl);
        assert!(!decoded.modifiers.shift); // Shift absorbed into char.
    }

    #[test]
    fn test_escape() {
        let ct_event = make_key_event(KeyCode::Esc, KeyModifiers::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.key, Key::Special(SpecialKey::Escape));
    }

    #[test]
    fn test_enter() {
        let ct_event = make_key_event(KeyCode::Enter, KeyModifiers::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.key, Key::Special(SpecialKey::Enter));
    }

    #[test]
    fn test_arrow_keys() {
        let ct_event = make_key_event(KeyCode::Up, KeyModifiers::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.key, Key::Special(SpecialKey::Up));
    }

    #[test]
    fn test_f_keys() {
        let ct_event = make_key_event(KeyCode::F(5), KeyModifiers::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.key, Key::Special(SpecialKey::F(5)));
    }
}
