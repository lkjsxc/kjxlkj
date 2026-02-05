//! Terminal input decoding.

use crossterm::event::{self, Event, KeyCode as CtKeyCode, KeyModifiers as CtKeyModifiers};
use kjxlkj_core_types::{InputEvent, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

/// Decode crossterm events to editor events.
pub fn poll_event(timeout: Duration) -> Option<InputEvent> {
    if event::poll(timeout).ok()? {
        let event = event::read().ok()?;
        decode_event(event)
    } else {
        None
    }
}

/// Decode a crossterm event.
pub fn decode_event(event: Event) -> Option<InputEvent> {
    match event {
        Event::Key(key) => Some(InputEvent::Key(decode_key(key))),
        Event::Resize(cols, rows) => Some(InputEvent::Resize { cols, rows }),
        Event::FocusGained => Some(InputEvent::FocusGained),
        Event::FocusLost => Some(InputEvent::FocusLost),
        Event::Paste(text) => Some(InputEvent::Paste(text)),
        _ => None,
    }
}

/// Decode a crossterm key event.
pub fn decode_key(key: event::KeyEvent) -> KeyEvent {
    let code = match key.code {
        CtKeyCode::Char(c) => KeyCode::Char(c),
        CtKeyCode::F(n) => KeyCode::F(n),
        CtKeyCode::Backspace => KeyCode::Backspace,
        CtKeyCode::Enter => KeyCode::Enter,
        CtKeyCode::Left => KeyCode::Left,
        CtKeyCode::Right => KeyCode::Right,
        CtKeyCode::Up => KeyCode::Up,
        CtKeyCode::Down => KeyCode::Down,
        CtKeyCode::Home => KeyCode::Home,
        CtKeyCode::End => KeyCode::End,
        CtKeyCode::PageUp => KeyCode::PageUp,
        CtKeyCode::PageDown => KeyCode::PageDown,
        CtKeyCode::Tab => KeyCode::Tab,
        CtKeyCode::BackTab => KeyCode::BackTab,
        CtKeyCode::Delete => KeyCode::Delete,
        CtKeyCode::Insert => KeyCode::Insert,
        CtKeyCode::Esc => KeyCode::Esc,
        CtKeyCode::Null => KeyCode::Null,
        _ => KeyCode::Null,
    };

    let mut modifiers = KeyModifiers::NONE;
    if key.modifiers.contains(CtKeyModifiers::SHIFT) {
        modifiers = modifiers.union(KeyModifiers::SHIFT);
    }
    if key.modifiers.contains(CtKeyModifiers::CONTROL) {
        modifiers = modifiers.union(KeyModifiers::CONTROL);
    }
    if key.modifiers.contains(CtKeyModifiers::ALT) {
        modifiers = modifiers.union(KeyModifiers::ALT);
    }

    KeyEvent::new(code, modifiers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_char() {
        let ct_key = event::KeyEvent::new(CtKeyCode::Char('a'), CtKeyModifiers::NONE);
        let key = decode_key(ct_key);
        assert!(matches!(key.code, KeyCode::Char('a')));
    }

    #[test]
    fn test_decode_ctrl() {
        let ct_key = event::KeyEvent::new(CtKeyCode::Char('c'), CtKeyModifiers::CONTROL);
        let key = decode_key(ct_key);
        assert!(key.modifiers.ctrl());
    }
}
