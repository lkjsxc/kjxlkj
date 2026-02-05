//! Input event decoder.

use crossterm::event::{Event, KeyCode, KeyEvent as CrosstermKeyEvent, KeyModifiers};
use kjxlkj_core_types::{EditorEvent, KeyEvent, Modifier};

/// Decode a crossterm event into an editor event.
pub fn decode_event(event: Event) -> Option<EditorEvent> {
    match event {
        Event::Key(key) => Some(EditorEvent::Key(decode_key(key))),
        Event::Resize(w, h) => Some(EditorEvent::Resize {
            width: w,
            height: h,
        }),
        Event::FocusGained => Some(EditorEvent::Focus(true)),
        Event::FocusLost => Some(EditorEvent::Focus(false)),
        _ => None,
    }
}

fn decode_key(key: CrosstermKeyEvent) -> KeyEvent {
    let mods = Modifier {
        ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
        alt: key.modifiers.contains(KeyModifiers::ALT),
        shift: key.modifiers.contains(KeyModifiers::SHIFT),
    };

    match key.code {
        KeyCode::Char(c) => KeyEvent::Char(c, mods),
        KeyCode::Esc => KeyEvent::Escape,
        KeyCode::Enter => KeyEvent::Enter,
        KeyCode::Backspace => KeyEvent::Backspace,
        KeyCode::Delete => KeyEvent::Delete,
        KeyCode::Tab => KeyEvent::Tab,
        KeyCode::BackTab => KeyEvent::BackTab,
        KeyCode::Up => KeyEvent::Up,
        KeyCode::Down => KeyEvent::Down,
        KeyCode::Left => KeyEvent::Left,
        KeyCode::Right => KeyEvent::Right,
        KeyCode::Home => KeyEvent::Home,
        KeyCode::End => KeyEvent::End,
        KeyCode::PageUp => KeyEvent::PageUp,
        KeyCode::PageDown => KeyEvent::PageDown,
        KeyCode::F(n) => KeyEvent::F(n),
        _ => KeyEvent::Escape, // Fallback
    }
}

/// Input decoder with buffered state.
pub struct InputDecoder {
    pending: Vec<KeyEvent>,
}

impl InputDecoder {
    /// Create a new input decoder.
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }

    /// Decode and buffer a crossterm event.
    pub fn decode(&mut self, event: Event) -> Option<EditorEvent> {
        decode_event(event)
    }

    /// Check if there are pending events.
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Clear pending events.
    pub fn clear(&mut self) {
        self.pending.clear();
    }
}

impl Default for InputDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_char() {
        let key = CrosstermKeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        let decoded = decode_key(key);
        assert!(matches!(decoded, KeyEvent::Char('a', _)));
    }

    #[test]
    fn test_decode_ctrl_char() {
        let key = CrosstermKeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        let decoded = decode_key(key);
        if let KeyEvent::Char(c, m) = decoded {
            assert_eq!(c, 'c');
            assert!(m.ctrl);
        } else {
            panic!("Expected Char event");
        }
    }

    #[test]
    fn test_decode_escape() {
        let key = CrosstermKeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        let decoded = decode_key(key);
        assert!(matches!(decoded, KeyEvent::Escape));
    }

    #[test]
    fn test_decode_resize() {
        let event = Event::Resize(80, 24);
        let decoded = decode_event(event);
        assert!(matches!(
            decoded,
            Some(EditorEvent::Resize {
                width: 80,
                height: 24
            })
        ));
    }
}
