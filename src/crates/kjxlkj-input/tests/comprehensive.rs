//! Comprehensive tests for the input decoder.

use kjxlkj_input::*;
use kjxlkj_core_types::Size;

// ──────────── KeyCode variants ────────────

#[test]
fn key_code_char() {
    assert_eq!(KeyCode::Char('a'), KeyCode::Char('a'));
    assert_ne!(KeyCode::Char('a'), KeyCode::Char('b'));
}

#[test]
fn key_code_special_variants() {
    let codes = vec![
        KeyCode::Esc,
        KeyCode::Enter,
        KeyCode::Backspace,
        KeyCode::Tab,
        KeyCode::BackTab,
        KeyCode::Delete,
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
        KeyCode::Home,
        KeyCode::End,
        KeyCode::PageUp,
        KeyCode::PageDown,
    ];
    // All distinct
    for (i, a) in codes.iter().enumerate() {
        for (j, b) in codes.iter().enumerate() {
            if i == j {
                assert_eq!(a, b);
            } else {
                assert_ne!(a, b);
            }
        }
    }
}

#[test]
fn key_code_function_keys() {
    for n in 1..=12 {
        assert_eq!(KeyCode::F(n), KeyCode::F(n));
    }
    assert_ne!(KeyCode::F(1), KeyCode::F(2));
}

// ──────────── Modifiers ────────────

#[test]
fn modifiers_default() {
    let m = Modifiers::default();
    assert!(!m.ctrl);
    assert!(!m.alt);
    assert!(!m.shift);
}

#[test]
fn modifiers_ctrl() {
    let m = Modifiers {
        ctrl: true,
        ..Default::default()
    };
    assert!(m.ctrl);
    assert!(!m.alt);
    assert!(!m.shift);
}

#[test]
fn modifiers_alt() {
    let m = Modifiers {
        alt: true,
        ..Default::default()
    };
    assert!(m.alt);
}

#[test]
fn modifiers_shift() {
    let m = Modifiers {
        shift: true,
        ..Default::default()
    };
    assert!(m.shift);
}

#[test]
fn modifiers_all() {
    let m = Modifiers {
        ctrl: true,
        alt: true,
        shift: true,
    };
    assert!(m.ctrl && m.alt && m.shift);
}

#[test]
fn modifiers_eq() {
    let a = Modifiers { ctrl: true, alt: false, shift: false };
    let b = Modifiers { ctrl: true, alt: false, shift: false };
    assert_eq!(a, b);
}

#[test]
fn modifiers_ne() {
    let a = Modifiers { ctrl: true, alt: false, shift: false };
    let b = Modifiers { ctrl: false, alt: true, shift: false };
    assert_ne!(a, b);
}

// ──────────── KeyEvent ────────────

#[test]
fn key_event_basic() {
    let k = KeyEvent {
        code: KeyCode::Char('x'),
        modifiers: Modifiers::default(),
    };
    assert_eq!(k.code, KeyCode::Char('x'));
    assert!(!k.modifiers.ctrl);
}

#[test]
fn key_event_ctrl_c() {
    let k = KeyEvent {
        code: KeyCode::Char('c'),
        modifiers: Modifiers {
            ctrl: true,
            ..Default::default()
        },
    };
    assert!(k.modifiers.ctrl);
    assert_eq!(k.code, KeyCode::Char('c'));
}

#[test]
fn key_event_eq() {
    let a = KeyEvent {
        code: KeyCode::Enter,
        modifiers: Modifiers::default(),
    };
    let b = KeyEvent {
        code: KeyCode::Enter,
        modifiers: Modifiers::default(),
    };
    assert_eq!(a, b);
}

#[test]
fn key_event_clone() {
    let k = KeyEvent {
        code: KeyCode::Tab,
        modifiers: Modifiers { ctrl: true, alt: false, shift: false },
    };
    let k2 = k.clone();
    assert_eq!(k, k2);
}

// ──────────── EditorEvent ────────────

#[test]
fn editor_event_key() {
    let e = EditorEvent::Key(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: Modifiers::default(),
    });
    matches!(e, EditorEvent::Key(_));
}

#[test]
fn editor_event_resize() {
    let e = EditorEvent::Resize(Size::new(120, 40));
    assert_eq!(e, EditorEvent::Resize(Size::new(120, 40)));
}

#[test]
fn editor_event_paste() {
    let e = EditorEvent::Paste("hello".into());
    assert_eq!(e, EditorEvent::Paste("hello".into()));
}

#[test]
fn editor_event_focus() {
    assert_eq!(EditorEvent::FocusGained, EditorEvent::FocusGained);
    assert_eq!(EditorEvent::FocusLost, EditorEvent::FocusLost);
    assert_ne!(EditorEvent::FocusGained, EditorEvent::FocusLost);
}

#[test]
fn editor_event_mouse() {
    assert_eq!(EditorEvent::Mouse, EditorEvent::Mouse);
}

// ──────────── InputDecoder ────────────

#[test]
fn decoder_new() {
    let _d = InputDecoder::new();
}

#[test]
fn decoder_default() {
    let _d = InputDecoder::default();
}

#[test]
fn decoder_decode_char_key() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    let event = Event::Key(CKE {
        code: CK::Char('a'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    let result = d.decode(event);
    assert!(result.is_some());
    if let Some(EditorEvent::Key(k)) = result {
        assert_eq!(k.code, KeyCode::Char('a'));
        assert!(!k.modifiers.ctrl);
    }
}

#[test]
fn decoder_decode_ctrl_key() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    let event = Event::Key(CKE {
        code: CK::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    let result = d.decode(event);
    if let Some(EditorEvent::Key(k)) = result {
        assert!(k.modifiers.ctrl);
        assert_eq!(k.code, KeyCode::Char('c'));
    }
}

#[test]
fn decoder_decode_esc() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    let event = Event::Key(CKE {
        code: CK::Esc,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    let result = d.decode(event);
    if let Some(EditorEvent::Key(k)) = result {
        assert_eq!(k.code, KeyCode::Esc);
    }
}

#[test]
fn decoder_decode_enter() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    let event = Event::Key(CKE {
        code: CK::Enter,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    let result = d.decode(event);
    if let Some(EditorEvent::Key(k)) = result {
        assert_eq!(k.code, KeyCode::Enter);
    }
}

#[test]
fn decoder_decode_arrow_keys() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    for (ct_code, expected) in [
        (CK::Up, KeyCode::Up),
        (CK::Down, KeyCode::Down),
        (CK::Left, KeyCode::Left),
        (CK::Right, KeyCode::Right),
    ] {
        let event = Event::Key(CKE {
            code: ct_code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });
        if let Some(EditorEvent::Key(k)) = d.decode(event) {
            assert_eq!(k.code, expected);
        }
    }
}

#[test]
fn decoder_decode_function_key() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    let event = Event::Key(CKE {
        code: CK::F(5),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    if let Some(EditorEvent::Key(k)) = d.decode(event) {
        assert_eq!(k.code, KeyCode::F(5));
    }
}

#[test]
fn decoder_decode_resize() {
    use crossterm::event::Event;
    let d = InputDecoder::new();
    let event = Event::Resize(120, 40);
    let result = d.decode(event);
    assert_eq!(result, Some(EditorEvent::Resize(Size::new(120, 40))));
}

#[test]
fn decoder_decode_paste() {
    use crossterm::event::Event;
    let d = InputDecoder::new();
    let event = Event::Paste("pasted text".into());
    let result = d.decode(event);
    assert_eq!(result, Some(EditorEvent::Paste("pasted text".into())));
}

#[test]
fn decoder_decode_focus() {
    use crossterm::event::Event;
    let d = InputDecoder::new();
    assert_eq!(d.decode(Event::FocusGained), Some(EditorEvent::FocusGained));
    assert_eq!(d.decode(Event::FocusLost), Some(EditorEvent::FocusLost));
}

#[test]
fn decoder_shift_modifier() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    let event = Event::Key(CKE {
        code: CK::Char('A'),
        modifiers: KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    if let Some(EditorEvent::Key(k)) = d.decode(event) {
        assert!(k.modifiers.shift);
    }
}

#[test]
fn decoder_alt_modifier() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    let event = Event::Key(CKE {
        code: CK::Char('x'),
        modifiers: KeyModifiers::ALT,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    });
    if let Some(EditorEvent::Key(k)) = d.decode(event) {
        assert!(k.modifiers.alt);
    }
}

#[test]
fn decoder_home_end_page() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    for (ct_code, expected) in [
        (CK::Home, KeyCode::Home),
        (CK::End, KeyCode::End),
        (CK::PageUp, KeyCode::PageUp),
        (CK::PageDown, KeyCode::PageDown),
    ] {
        let event = Event::Key(CKE {
            code: ct_code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });
        if let Some(EditorEvent::Key(k)) = d.decode(event) {
            assert_eq!(k.code, expected);
        }
    }
}

#[test]
fn decoder_backspace_delete_tab() {
    use crossterm::event::{Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers, KeyEventKind, KeyEventState};
    let d = InputDecoder::new();
    for (ct_code, expected) in [
        (CK::Backspace, KeyCode::Backspace),
        (CK::Delete, KeyCode::Delete),
        (CK::Tab, KeyCode::Tab),
        (CK::BackTab, KeyCode::BackTab),
    ] {
        let event = Event::Key(CKE {
            code: ct_code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });
        if let Some(EditorEvent::Key(k)) = d.decode(event) {
            assert_eq!(k.code, expected);
        }
    }
}
