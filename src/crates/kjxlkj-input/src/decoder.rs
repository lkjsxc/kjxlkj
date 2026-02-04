//! Input event decoding from crossterm to kjxlkj types.

use crossterm::event::{Event, KeyCode as CTKeyCode, KeyEvent as CTKeyEvent, KeyModifiers as CTMods};
use kjxlkj_core_types::{EditorEvent, KeyCode, KeyEvent, KeyModifiers};

/// Input decoder for crossterm events.
pub struct InputDecoder;

impl InputDecoder {
    /// Create a new input decoder.
    pub fn new() -> Self {
        Self
    }

    /// Decode a crossterm event.
    pub fn decode(&self, event: Event) -> EditorEvent {
        decode_event(event)
    }
}

impl Default for InputDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Decode a crossterm event into an editor event.
pub fn decode_event(event: Event) -> EditorEvent {
    match event {
        Event::Key(key_event) => EditorEvent::Key(decode_key(key_event)),
        Event::Resize(width, height) => EditorEvent::Resize { width, height },
        Event::FocusGained => EditorEvent::Focus(true),
        Event::FocusLost => EditorEvent::Focus(false),
        Event::Mouse(_) => EditorEvent::Ignored,
        Event::Paste(text) => EditorEvent::Paste(text),
    }
}

/// Decode a crossterm key event.
fn decode_key(event: CTKeyEvent) -> KeyEvent {
    let code = decode_key_code(event.code);
    let modifiers = decode_modifiers(event.modifiers);
    KeyEvent { code, modifiers }
}

/// Decode crossterm key code.
fn decode_key_code(code: CTKeyCode) -> KeyCode {
    match code {
        CTKeyCode::Char(c) => KeyCode::Char(c),
        CTKeyCode::Backspace => KeyCode::Backspace,
        CTKeyCode::Enter => KeyCode::Enter,
        CTKeyCode::Left => KeyCode::Left,
        CTKeyCode::Right => KeyCode::Right,
        CTKeyCode::Up => KeyCode::Up,
        CTKeyCode::Down => KeyCode::Down,
        CTKeyCode::Home => KeyCode::Home,
        CTKeyCode::End => KeyCode::End,
        CTKeyCode::PageUp => KeyCode::PageUp,
        CTKeyCode::PageDown => KeyCode::PageDown,
        CTKeyCode::Tab => KeyCode::Tab,
        CTKeyCode::BackTab => KeyCode::BackTab,
        CTKeyCode::Delete => KeyCode::Delete,
        CTKeyCode::Insert => KeyCode::Insert,
        CTKeyCode::Esc => KeyCode::Escape,
        CTKeyCode::F(n) => KeyCode::F(n),
        CTKeyCode::Null => KeyCode::Null,
        CTKeyCode::CapsLock => KeyCode::CapsLock,
        CTKeyCode::ScrollLock => KeyCode::ScrollLock,
        CTKeyCode::NumLock => KeyCode::NumLock,
        CTKeyCode::PrintScreen => KeyCode::PrintScreen,
        CTKeyCode::Pause => KeyCode::Pause,
        CTKeyCode::Menu => KeyCode::Menu,
        CTKeyCode::KeypadBegin => KeyCode::KeypadBegin,
        CTKeyCode::Media(media) => KeyCode::Media(format!("{:?}", media)),
        CTKeyCode::Modifier(modifier) => KeyCode::Modifier(format!("{:?}", modifier)),
    }
}

/// Decode crossterm key modifiers.
fn decode_modifiers(mods: CTMods) -> KeyModifiers {
    KeyModifiers {
        ctrl: mods.contains(CTMods::CONTROL),
        alt: mods.contains(CTMods::ALT),
        shift: mods.contains(CTMods::SHIFT),
        meta: mods.contains(CTMods::META) || mods.contains(CTMods::SUPER),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_char() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('a'), CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Char('a'));
        assert!(!decoded.modifiers.ctrl);
    }

    #[test]
    fn test_decode_ctrl() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Char('c'), CTMods::CONTROL);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Char('c'));
        assert!(decoded.modifiers.ctrl);
    }

    #[test]
    fn test_decode_escape() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Esc, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Escape);
    }

    #[test]
    fn test_decode_arrow() {
        let ct_event = CTKeyEvent::new(CTKeyCode::Up, CTMods::NONE);
        let decoded = decode_key(ct_event);
        assert_eq!(decoded.code, KeyCode::Up);
    }

    #[test]
    fn test_decode_resize() {
        let event = Event::Resize(120, 40);
        let decoded = decode_event(event);
        assert!(matches!(decoded, EditorEvent::Resize { width: 120, height: 40 }));
    }

    #[test]
    fn test_decode_focus() {
        let event = Event::FocusGained;
        let decoded = decode_event(event);
        assert!(matches!(decoded, EditorEvent::Focus(true)));
    }
}
