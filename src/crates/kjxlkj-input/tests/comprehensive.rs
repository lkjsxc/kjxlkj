//! Comprehensive tests for kjxlkj-input.

use crossterm::event::{Event, KeyCode, KeyEvent as CrosstermKeyEvent, KeyModifiers};
use kjxlkj_core_types::{EditorEvent, KeyEvent};
use kjxlkj_input::*;

mod decode_event_tests {
    use super::*;

    #[test]
    fn test_decode_key_event() {
        let key = CrosstermKeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        let event = Event::Key(key);
        let decoded = decode_event(event);
        assert!(decoded.is_some());
        assert!(matches!(decoded, Some(EditorEvent::Key(_))));
    }

    #[test]
    fn test_decode_resize_event() {
        let event = Event::Resize(120, 40);
        let decoded = decode_event(event);
        assert!(matches!(
            decoded,
            Some(EditorEvent::Resize {
                width: 120,
                height: 40
            })
        ));
    }

    #[test]
    fn test_decode_focus_gained() {
        let event = Event::FocusGained;
        let decoded = decode_event(event);
        assert!(matches!(decoded, Some(EditorEvent::Focus(true))));
    }

    #[test]
    fn test_decode_focus_lost() {
        let event = Event::FocusLost;
        let decoded = decode_event(event);
        assert!(matches!(decoded, Some(EditorEvent::Focus(false))));
    }

    #[test]
    fn test_decode_char_a() {
        let key = CrosstermKeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Char('a', _)));
        } else {
            panic!("Expected Key event");
        }
    }

    #[test]
    fn test_decode_char_ctrl() {
        let key = CrosstermKeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(KeyEvent::Char(c, m))) = decode_event(event) {
            assert_eq!(c, 'c');
            assert!(m.ctrl);
        } else {
            panic!("Expected Char event with ctrl");
        }
    }

    #[test]
    fn test_decode_char_alt() {
        let key = CrosstermKeyEvent::new(KeyCode::Char('x'), KeyModifiers::ALT);
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(KeyEvent::Char(c, m))) = decode_event(event) {
            assert_eq!(c, 'x');
            assert!(m.alt);
        } else {
            panic!("Expected Char event with alt");
        }
    }

    #[test]
    fn test_decode_char_shift() {
        let key = CrosstermKeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT);
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(KeyEvent::Char(c, m))) = decode_event(event) {
            assert_eq!(c, 'A');
            assert!(m.shift);
        } else {
            panic!("Expected Char event with shift");
        }
    }

    #[test]
    fn test_decode_escape() {
        let key = CrosstermKeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Escape));
        } else {
            panic!("Expected Escape event");
        }
    }

    #[test]
    fn test_decode_enter() {
        let key = CrosstermKeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Enter));
        } else {
            panic!("Expected Enter event");
        }
    }

    #[test]
    fn test_decode_backspace() {
        let key = CrosstermKeyEvent::new(KeyCode::Backspace, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Backspace));
        } else {
            panic!("Expected Backspace event");
        }
    }

    #[test]
    fn test_decode_delete() {
        let key = CrosstermKeyEvent::new(KeyCode::Delete, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Delete));
        } else {
            panic!("Expected Delete event");
        }
    }

    #[test]
    fn test_decode_tab() {
        let key = CrosstermKeyEvent::new(KeyCode::Tab, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Tab));
        } else {
            panic!("Expected Tab event");
        }
    }

    #[test]
    fn test_decode_backtab() {
        let key = CrosstermKeyEvent::new(KeyCode::BackTab, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::BackTab));
        } else {
            panic!("Expected BackTab event");
        }
    }

    #[test]
    fn test_decode_arrow_up() {
        let key = CrosstermKeyEvent::new(KeyCode::Up, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Up));
        } else {
            panic!("Expected Up event");
        }
    }

    #[test]
    fn test_decode_arrow_down() {
        let key = CrosstermKeyEvent::new(KeyCode::Down, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Down));
        } else {
            panic!("Expected Down event");
        }
    }

    #[test]
    fn test_decode_arrow_left() {
        let key = CrosstermKeyEvent::new(KeyCode::Left, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Left));
        } else {
            panic!("Expected Left event");
        }
    }

    #[test]
    fn test_decode_arrow_right() {
        let key = CrosstermKeyEvent::new(KeyCode::Right, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Right));
        } else {
            panic!("Expected Right event");
        }
    }

    #[test]
    fn test_decode_home() {
        let key = CrosstermKeyEvent::new(KeyCode::Home, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::Home));
        } else {
            panic!("Expected Home event");
        }
    }

    #[test]
    fn test_decode_end() {
        let key = CrosstermKeyEvent::new(KeyCode::End, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::End));
        } else {
            panic!("Expected End event");
        }
    }

    #[test]
    fn test_decode_page_up() {
        let key = CrosstermKeyEvent::new(KeyCode::PageUp, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::PageUp));
        } else {
            panic!("Expected PageUp event");
        }
    }

    #[test]
    fn test_decode_page_down() {
        let key = CrosstermKeyEvent::new(KeyCode::PageDown, KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::PageDown));
        } else {
            panic!("Expected PageDown event");
        }
    }

    #[test]
    fn test_decode_f1() {
        let key = CrosstermKeyEvent::new(KeyCode::F(1), KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::F(1)));
        } else {
            panic!("Expected F1 event");
        }
    }

    #[test]
    fn test_decode_f12() {
        let key = CrosstermKeyEvent::new(KeyCode::F(12), KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::F(12)));
        } else {
            panic!("Expected F12 event");
        }
    }
}

mod input_decoder_tests {
    use super::*;

    #[test]
    fn test_decoder_new() {
        let decoder = InputDecoder::new();
        assert!(!decoder.has_pending());
    }

    #[test]
    fn test_decoder_default() {
        let decoder = InputDecoder::default();
        assert!(!decoder.has_pending());
    }

    #[test]
    fn test_decoder_decode() {
        let mut decoder = InputDecoder::new();
        let key = CrosstermKeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        let event = Event::Key(key);
        let decoded = decoder.decode(event);
        assert!(decoded.is_some());
    }

    #[test]
    fn test_decoder_clear() {
        let mut decoder = InputDecoder::new();
        decoder.clear();
        assert!(!decoder.has_pending());
    }
}

mod modifier_tests {
    use super::*;

    #[test]
    fn test_modifier_combinations() {
        let key = CrosstermKeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::CONTROL | KeyModifiers::ALT,
        );
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(KeyEvent::Char(_, m))) = decode_event(event) {
            assert!(m.ctrl);
            assert!(m.alt);
            assert!(!m.shift);
        } else {
            panic!("Expected Char event");
        }
    }

    #[test]
    fn test_all_modifiers() {
        let key = CrosstermKeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT,
        );
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(KeyEvent::Char(_, m))) = decode_event(event) {
            assert!(m.ctrl);
            assert!(m.alt);
            assert!(m.shift);
        } else {
            panic!("Expected Char event");
        }
    }
}

mod extra_decode_tests {
    use super::*;

    #[test]
    fn test_decode_char_uppercase() {
        let key = CrosstermKeyEvent::new(KeyCode::Char('Z'), KeyModifiers::SHIFT);
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(KeyEvent::Char(c, _))) = decode_event(event) {
            assert_eq!(c, 'Z');
        } else {
            panic!("Expected char event");
        }
    }

    #[test]
    fn test_decode_char_special() {
        let key = CrosstermKeyEvent::new(KeyCode::Char(':'), KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(KeyEvent::Char(c, _))) = decode_event(event) {
            assert_eq!(c, ':');
        } else {
            panic!("Expected char event");
        }
    }

    #[test]
    fn test_decode_ctrl_shift_combo() {
        let key = CrosstermKeyEvent::new(
            KeyCode::Char('s'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        );
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(KeyEvent::Char(_, m))) = decode_event(event) {
            assert!(m.ctrl);
            assert!(m.shift);
        } else {
            panic!("Expected char event");
        }
    }

    #[test]
    fn test_decode_f5() {
        let key = CrosstermKeyEvent::new(KeyCode::F(5), KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::F(5)));
        } else {
            panic!("Expected F5");
        }
    }

    #[test]
    fn test_decode_f10() {
        let key = CrosstermKeyEvent::new(KeyCode::F(10), KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(k)) = decode_event(event) {
            assert!(matches!(k, KeyEvent::F(10)));
        } else {
            panic!("Expected F10");
        }
    }

    #[test]
    fn test_decode_insert_key() {
        let key = CrosstermKeyEvent::new(KeyCode::Insert, KeyModifiers::empty());
        let event = Event::Key(key);
        // Insert key might not be mapped, just check it doesn't panic
        let _ = decode_event(event);
    }

    #[test]
    fn test_decode_resize_small() {
        let event = Event::Resize(10, 5);
        let decoded = decode_event(event);
        if let Some(EditorEvent::Resize { width, height }) = decoded {
            assert_eq!(width, 10);
            assert_eq!(height, 5);
        } else {
            panic!("Expected resize");
        }
    }

    #[test]
    fn test_decode_resize_large() {
        let event = Event::Resize(1920, 1080);
        let decoded = decode_event(event);
        if let Some(EditorEvent::Resize { width, height }) = decoded {
            assert_eq!(width, 1920);
            assert_eq!(height, 1080);
        } else {
            panic!("Expected resize");
        }
    }

    #[test]
    fn test_decode_numeric_chars() {
        for digit in '0'..='9' {
            let key = CrosstermKeyEvent::new(KeyCode::Char(digit), KeyModifiers::empty());
            let event = Event::Key(key);
            if let Some(EditorEvent::Key(KeyEvent::Char(c, _))) = decode_event(event) {
                assert_eq!(c, digit);
            } else {
                panic!("Expected digit {}", digit);
            }
        }
    }
}

mod extra_input_tests {
    use super::*;

    #[test]
    fn test_decode_printable_chars() {
        for c in 'a'..='z' {
            let key = CrosstermKeyEvent::new(KeyCode::Char(c), KeyModifiers::empty());
            let event = Event::Key(key);
            if let Some(EditorEvent::Key(KeyEvent::Char(decoded, _))) = decode_event(event) {
                assert_eq!(decoded, c);
            }
        }
    }

    #[test]
    fn test_decode_uppercase_chars() {
        for c in 'A'..='Z' {
            let key = CrosstermKeyEvent::new(KeyCode::Char(c), KeyModifiers::empty());
            let event = Event::Key(key);
            if let Some(EditorEvent::Key(KeyEvent::Char(decoded, _))) = decode_event(event) {
                assert_eq!(decoded, c);
            }
        }
    }

    #[test]
    fn test_decode_punctuation() {
        let chars = [
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '=', '[', ']',
        ];
        for c in chars {
            let key = CrosstermKeyEvent::new(KeyCode::Char(c), KeyModifiers::empty());
            let event = Event::Key(key);
            if let Some(EditorEvent::Key(KeyEvent::Char(decoded, _))) = decode_event(event) {
                assert_eq!(decoded, c);
            }
        }
    }

    #[test]
    fn test_decode_space() {
        let key = CrosstermKeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty());
        let event = Event::Key(key);
        if let Some(EditorEvent::Key(KeyEvent::Char(c, _))) = decode_event(event) {
            assert_eq!(c, ' ');
        }
    }

    #[test]
    fn test_decode_various_resize_sizes() {
        let sizes = [(80, 24), (120, 40), (200, 60), (1, 1), (10000, 10000)];
        for (w, h) in sizes {
            let event = Event::Resize(w, h);
            if let Some(EditorEvent::Resize { width, height }) = decode_event(event) {
                assert_eq!(width, w);
                assert_eq!(height, h);
            }
        }
    }
}
