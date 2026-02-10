//! Tests for input decoding (KI-01 through KI-11).
//!
//! Covers spec requirements from `/docs/spec/technical/testing-unit.md`.

#[cfg(test)]
mod tests {
    use crate::decode::*;
    use crossterm::event::{
        Event, KeyCode as CtKeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers as CtMods,
    };
    use kjxlkj_core_types::{KeyCode, KeyModifiers};

    fn make_key_event(code: CtKeyCode, mods: CtMods) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: mods,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    /// KI-01: ASCII key parse.
    /// Char('a') with no modifiers.
    #[test]
    fn ki01_ascii_key() {
        let event = Event::Key(make_key_event(CtKeyCode::Char('a'), CtMods::NONE));
        match decode_crossterm_event(event) {
            DecodedEvent::Key(key) => {
                assert_eq!(key.code, KeyCode::Char('a'));
                assert_eq!(key.modifiers, KeyModifiers::NONE);
            }
            _ => panic!("Expected Key event"),
        }
    }

    /// KI-02: Arrow key.
    /// Up arrow decodes to Key::Up.
    #[test]
    fn ki02_arrow_key() {
        let event = Event::Key(make_key_event(CtKeyCode::Up, CtMods::NONE));
        match decode_crossterm_event(event) {
            DecodedEvent::Key(key) => {
                assert_eq!(key.code, KeyCode::Up);
            }
            _ => panic!("Expected Key event"),
        }
    }

    /// KI-03: Ctrl modifier.
    /// Ctrl+a.
    #[test]
    fn ki03_ctrl_modifier() {
        let event = Event::Key(make_key_event(CtKeyCode::Char('a'), CtMods::CONTROL));
        match decode_crossterm_event(event) {
            DecodedEvent::Key(key) => {
                assert_eq!(key.code, KeyCode::Char('a'));
                assert!(key.modifiers.contains(KeyModifiers::CTRL));
            }
            _ => panic!("Expected Key event"),
        }
    }

    /// KI-04: UTF-8 multi-byte.
    /// Char('あ') decodes to correct char.
    #[test]
    fn ki04_utf8_multibyte() {
        let event = Event::Key(make_key_event(CtKeyCode::Char('あ'), CtMods::NONE));
        match decode_crossterm_event(event) {
            DecodedEvent::Key(key) => {
                assert_eq!(key.code, KeyCode::Char('あ'));
            }
            _ => panic!("Expected Key event"),
        }
    }

    /// KI-05: Esc key.
    #[test]
    fn ki05_esc_key() {
        let event = Event::Key(make_key_event(CtKeyCode::Esc, CtMods::NONE));
        match decode_crossterm_event(event) {
            DecodedEvent::Key(key) => {
                assert_eq!(key.code, KeyCode::Esc);
            }
            _ => panic!("Expected Key event"),
        }
    }

    /// KI-09: Paste bracket.
    /// Paste event decodes to Action::Paste.
    #[test]
    fn ki09_paste_bracket() {
        let event = Event::Paste("hello".to_string());
        match decode_crossterm_event(event) {
            DecodedEvent::Action(action) => match action {
                kjxlkj_core_types::Action::Paste(text) => assert_eq!(text, "hello"),
                _ => panic!("Expected Paste action"),
            },
            _ => panic!("Expected Action event"),
        }
    }

    /// KI-10: Resize event.
    #[test]
    fn ki10_resize_event() {
        let event = Event::Resize(120, 40);
        match decode_crossterm_event(event) {
            DecodedEvent::Action(action) => match action {
                kjxlkj_core_types::Action::Resize(c, r) => {
                    assert_eq!(c, 120);
                    assert_eq!(r, 40);
                }
                _ => panic!("Expected Resize action"),
            },
            _ => panic!("Expected Action event"),
        }
    }

    /// Test Shift normalization for all alpha chars.
    #[test]
    fn test_shift_normalization_all_alpha() {
        for c in 'a'..='z' {
            let event = make_key_event(CtKeyCode::Char(c), CtMods::SHIFT);
            let key = super::super::decode::decode_crossterm_event(Event::Key(event));
            match key {
                DecodedEvent::Key(k) => {
                    assert_eq!(
                        k.code,
                        KeyCode::Char(c.to_ascii_uppercase()),
                        "Shift+{} should normalize to {}",
                        c,
                        c.to_ascii_uppercase()
                    );
                    assert_eq!(k.modifiers, KeyModifiers::NONE);
                }
                _ => panic!("Expected Key"),
            }
        }
    }

    /// Test mouse event ignored.
    #[test]
    fn test_mouse_ignored() {
        use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
        let event = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: CtMods::NONE,
        });
        match decode_crossterm_event(event) {
            DecodedEvent::Ignore => {}
            _ => panic!("Expected Ignore for mouse event"),
        }
    }
}
