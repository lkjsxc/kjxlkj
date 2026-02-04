//! Input event types.

use crate::Key;

/// An input event from the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    /// A key press.
    Key(Key),
    /// Terminal resized.
    Resize(u16, u16),
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
}

impl InputEvent {
    /// Create a key event.
    pub fn key(key: Key) -> Self {
        Self::Key(key)
    }

    /// Create a resize event.
    pub fn resize(width: u16, height: u16) -> Self {
        Self::Resize(width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KeyCode;

    #[test]
    fn input_event_key() {
        let event = InputEvent::key(Key::char('a'));
        if let InputEvent::Key(k) = event {
            assert_eq!(k.code, KeyCode::Char('a'));
        } else {
            panic!("Expected Key event");
        }
    }

    #[test]
    fn input_event_resize() {
        let event = InputEvent::resize(80, 24);
        if let InputEvent::Resize(w, h) = event {
            assert_eq!(w, 80);
            assert_eq!(h, 24);
        } else {
            panic!("Expected Resize event");
        }
    }
}
