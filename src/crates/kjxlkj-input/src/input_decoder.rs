//! Convert crossterm events to EditorEvent.

use crossterm::event as ct;
use kjxlkj_core_types::{EditorEvent, KeyCode, KeyEvent, Modifiers};

/// Convert a crossterm `Event` to an `EditorEvent`.
pub fn decode_event(event: ct::Event) -> Option<EditorEvent> {
    match event {
        ct::Event::Key(key) => Some(EditorEvent::KeyInput(decode_key(key))),
        ct::Event::Resize(w, h) => Some(EditorEvent::Resize(w, h)),
        ct::Event::FocusGained => Some(EditorEvent::FocusGained),
        ct::Event::FocusLost => Some(EditorEvent::FocusLost),
        _ => None,
    }
}

/// Convert a crossterm `KeyEvent` to our `KeyEvent` type.
pub fn decode_key(key: ct::KeyEvent) -> KeyEvent {
    KeyEvent::new(map_key_code(key.code), map_modifiers(key.modifiers))
}

/// Map a crossterm `KeyCode` to our `KeyCode`.
pub fn map_key_code(code: ct::KeyCode) -> KeyCode {
    match code {
        ct::KeyCode::Char(c) => KeyCode::Char(c),
        ct::KeyCode::Esc => KeyCode::Escape,
        ct::KeyCode::Enter => KeyCode::Enter,
        ct::KeyCode::Backspace => KeyCode::Backspace,
        ct::KeyCode::Tab => KeyCode::Tab,
        ct::KeyCode::Delete => KeyCode::Delete,
        ct::KeyCode::Left => KeyCode::Left,
        ct::KeyCode::Right => KeyCode::Right,
        ct::KeyCode::Up => KeyCode::Up,
        ct::KeyCode::Down => KeyCode::Down,
        ct::KeyCode::Home => KeyCode::Home,
        ct::KeyCode::End => KeyCode::End,
        ct::KeyCode::PageUp => KeyCode::PageUp,
        ct::KeyCode::PageDown => KeyCode::PageDown,
        ct::KeyCode::F(n) => KeyCode::F(n),
        ct::KeyCode::Insert => KeyCode::Insert,
        ct::KeyCode::Null => KeyCode::Null,
        _ => KeyCode::Null,
    }
}

/// Map crossterm `KeyModifiers` to our `Modifiers`.
pub fn map_modifiers(mods: ct::KeyModifiers) -> Modifiers {
    let mut result = Modifiers::NONE;
    if mods.contains(ct::KeyModifiers::CONTROL) {
        result = result.union(Modifiers::CTRL);
    }
    if mods.contains(ct::KeyModifiers::ALT) {
        result = result.union(Modifiers::ALT);
    }
    if mods.contains(ct::KeyModifiers::SHIFT) {
        result = result.union(Modifiers::SHIFT);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_char_key() {
        let ct_key = ct::KeyEvent::new(ct::KeyCode::Char('a'), ct::KeyModifiers::NONE);
        let key = decode_key(ct_key);
        assert_eq!(key.code, KeyCode::Char('a'));
        assert!(key.modifiers.is_empty());
    }

    #[test]
    fn decode_ctrl_c() {
        let ct_key = ct::KeyEvent::new(ct::KeyCode::Char('c'), ct::KeyModifiers::CONTROL);
        let key = decode_key(ct_key);
        assert_eq!(key.code, KeyCode::Char('c'));
        assert!(key.modifiers.contains(Modifiers::CTRL));
    }

    #[test]
    fn decode_resize_event() {
        let ev = ct::Event::Resize(80, 24);
        assert_eq!(decode_event(ev), Some(EditorEvent::Resize(80, 24)));
    }

    #[test]
    fn decode_focus_events() {
        assert_eq!(decode_event(ct::Event::FocusGained), Some(EditorEvent::FocusGained));
        assert_eq!(decode_event(ct::Event::FocusLost), Some(EditorEvent::FocusLost));
    }

    #[test]
    fn map_special_keys() {
        assert_eq!(map_key_code(ct::KeyCode::Esc), KeyCode::Escape);
        assert_eq!(map_key_code(ct::KeyCode::Enter), KeyCode::Enter);
        assert_eq!(map_key_code(ct::KeyCode::F(5)), KeyCode::F(5));
    }

    #[test]
    fn map_combined_modifiers() {
        let m = ct::KeyModifiers::CONTROL | ct::KeyModifiers::ALT;
        let result = map_modifiers(m);
        assert!(result.contains(Modifiers::CTRL));
        assert!(result.contains(Modifiers::ALT));
        assert!(!result.contains(Modifiers::SHIFT));
    }
}
