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

    #[test]
    fn input_event_focus_gained() {
        let event = InputEvent::FocusGained;
        assert_eq!(event, InputEvent::FocusGained);
    }

    #[test]
    fn input_event_focus_lost() {
        let event = InputEvent::FocusLost;
        assert_eq!(event, InputEvent::FocusLost);
    }

    #[test]
    fn input_event_equality() {
        let e1 = InputEvent::key(Key::char('a'));
        let e2 = InputEvent::key(Key::char('a'));
        let e3 = InputEvent::key(Key::char('b'));
        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
    }

    #[test]
    fn input_event_ordering_preserved() {
        // Input events should process in sequence order
        let events = vec![
            InputEvent::key(Key::char('h')),
            InputEvent::key(Key::char('e')),
            InputEvent::key(Key::char('l')),
            InputEvent::key(Key::char('l')),
            InputEvent::key(Key::char('o')),
        ];
        // Verify sequential access maintains order
        for (i, e) in events.iter().enumerate() {
            if let InputEvent::Key(k) = e {
                let expected = ['h', 'e', 'l', 'l', 'o'][i];
                assert_eq!(k.code, KeyCode::Char(expected));
            }
        }
    }

    #[test]
    fn input_event_resize_zero() {
        let event = InputEvent::resize(0, 0);
        if let InputEvent::Resize(w, h) = event {
            assert_eq!(w, 0);
            assert_eq!(h, 0);
        }
    }

    #[test]
    fn input_event_resize_large() {
        let event = InputEvent::resize(4096, 2160);
        if let InputEvent::Resize(w, h) = event {
            assert_eq!(w, 4096);
            assert_eq!(h, 2160);
        }
    }

    #[test]
    fn input_event_clone() {
        let event = InputEvent::key(Key::char('x'));
        let cloned = event.clone();
        assert_eq!(event, cloned);
    }

    #[test]
    fn input_event_debug() {
        let event = InputEvent::FocusGained;
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("FocusGained"));
    }

    #[test]
    fn input_event_resize_not_key() {
        let event = InputEvent::resize(80, 24);
        assert!(!matches!(event, InputEvent::Key(_)));
    }

    #[test]
    fn input_event_key_not_resize() {
        let event = InputEvent::key(Key::char('a'));
        assert!(!matches!(event, InputEvent::Resize(_, _)));
    }

    #[test]
    fn input_event_focus_variants() {
        let gained = InputEvent::FocusGained;
        let lost = InputEvent::FocusLost;
        assert_ne!(gained, lost);
    }

    #[test]
    fn input_event_focus_gained_eq() {
        let e1 = InputEvent::FocusGained;
        let e2 = InputEvent::FocusGained;
        assert_eq!(e1, e2);
    }

    #[test]
    fn input_event_focus_lost_eq() {
        let e1 = InputEvent::FocusLost;
        let e2 = InputEvent::FocusLost;
        assert_eq!(e1, e2);
    }

    #[test]
    fn input_event_resize_eq() {
        let e1 = InputEvent::resize(80, 24);
        let e2 = InputEvent::resize(80, 24);
        assert_eq!(e1, e2);
    }

    #[test]
    fn input_event_resize_diff_width() {
        let e1 = InputEvent::resize(80, 24);
        let e2 = InputEvent::resize(100, 24);
        assert_ne!(e1, e2);
    }

    #[test]
    fn input_event_resize_diff_height() {
        let e1 = InputEvent::resize(80, 24);
        let e2 = InputEvent::resize(80, 50);
        assert_ne!(e1, e2);
    }

    #[test]
    fn input_event_focus_gained_not_lost() {
        let e1 = InputEvent::FocusGained;
        let e2 = InputEvent::FocusLost;
        assert_ne!(e1, e2);
    }

    #[test]
    fn input_event_type_name() {
        let name = std::any::type_name::<InputEvent>();
        assert!(name.contains("InputEvent"));
    }
}
