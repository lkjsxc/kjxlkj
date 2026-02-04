//! Terminal input decoding.
//!
//! This crate provides:
//! - Crossterm event to Key conversion
//! - Input event stream handling

mod convert;

pub use convert::crossterm_to_key;

use crossterm::event::Event;
use kjxlkj_core_mode::Key;

/// Input event from terminal.
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Key press event.
    Key(Key),
    /// Terminal resize event.
    Resize(u16, u16),
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
}

/// Convert a crossterm event to an input event.
pub fn convert_event(event: Event) -> Option<InputEvent> {
    match event {
        Event::Key(key_event) => Some(InputEvent::Key(crossterm_to_key(key_event))),
        Event::Resize(w, h) => Some(InputEvent::Resize(w, h)),
        Event::FocusGained => Some(InputEvent::FocusGained),
        Event::FocusLost => Some(InputEvent::FocusLost),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode as CtKeyCode, KeyEvent, KeyModifiers as CtMods};
    use kjxlkj_core_mode::{KeyCode, KeyModifiers};

    #[test]
    fn convert_key_event() {
        let ct_event = KeyEvent::new(CtKeyCode::Char('a'), CtMods::NONE);
        let key = crossterm_to_key(ct_event);
        assert_eq!(key.code, KeyCode::Char('a'));
        assert_eq!(key.modifiers, KeyModifiers::NONE);
    }

    #[test]
    fn convert_ctrl_key() {
        let ct_event = KeyEvent::new(CtKeyCode::Char('c'), CtMods::CONTROL);
        let key = crossterm_to_key(ct_event);
        assert_eq!(key.code, KeyCode::Char('c'));
        assert!(key.modifiers.ctrl);
    }
}
