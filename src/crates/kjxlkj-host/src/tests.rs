//! Tests for host crate.

use super::*;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use kjxlkj_core_ui::Dimensions;

mod host_event_tests {
    use super::*;

    #[test]
    fn test_host_event_is_quit_ctrl_c() {
        let key = KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let event = HostEvent::Key(key);
        assert!(event.is_quit());
    }

    #[test]
    fn test_host_event_is_quit_regular_key() {
        let key = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let event = HostEvent::Key(key);
        assert!(!event.is_quit());
    }

    #[test]
    fn test_host_event_resize() {
        let event = HostEvent::Resize(Dimensions::new(80, 24));
        assert!(!event.is_quit());
    }

    #[test]
    fn test_host_event_focus() {
        let gained = HostEvent::FocusGained;
        let lost = HostEvent::FocusLost;
        assert!(!gained.is_quit());
        assert!(!lost.is_quit());
    }
}

mod dimensions_tests {
    use super::*;

    #[test]
    fn test_dimensions_from_event() {
        let event = HostEvent::Resize(Dimensions::new(120, 40));
        if let HostEvent::Resize(dims) = event {
            assert_eq!(dims.width, 120);
            assert_eq!(dims.height, 40);
        } else {
            panic!("Expected Resize event");
        }
    }
}
