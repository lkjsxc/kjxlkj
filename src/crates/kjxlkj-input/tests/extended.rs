//! Extended input decoder tests.

use kjxlkj_input::*;
use kjxlkj_core_types::Size;
use crossterm::event::{
    Event, KeyCode as CK, KeyEvent as CKE, KeyModifiers as CKM,
    KeyEventKind, KeyEventState,
};

fn press(code: CK) -> Event {
    Event::Key(CKE {
        code,
        modifiers: CKM::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    })
}

fn press_mod(code: CK, mods: CKM) -> Event {
    Event::Key(CKE {
        code,
        modifiers: mods,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    })
}

// ──────────── InputDecoder construction ────────────

#[test]
fn decoder_new() {
    let _ = InputDecoder::new();
}

#[test]
fn decoder_default() {
    let _ = InputDecoder::default();
}

// ──────────── Char decoding ────────────

#[test]
fn decode_char_a() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::Char('a'))).unwrap();
    assert_eq!(
        ev,
        EditorEvent::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: Modifiers::default(),
        })
    );
}

#[test]
fn decode_char_z() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::Char('z'))).unwrap();
    match ev {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Char('z')),
        _ => panic!("expected Key"),
    }
}

#[test]
fn decode_digit() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::Char('5'))).unwrap();
    match ev {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Char('5')),
        _ => panic!("expected Key"),
    }
}

#[test]
fn decode_space() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::Char(' '))).unwrap();
    match ev {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Char(' ')),
        _ => panic!("expected Key"),
    }
}

// ──────────── Special keys ────────────

#[test]
fn decode_escape() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::Esc)).unwrap();
    match ev {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Esc),
        _ => panic!("expected Key"),
    }
}

#[test]
fn decode_enter() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::Enter)).unwrap();
    match ev {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Enter),
        _ => panic!("expected Key"),
    }
}

#[test]
fn decode_backspace() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::Backspace)).unwrap();
    match ev {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Backspace),
        _ => panic!("expected Key"),
    }
}

#[test]
fn decode_tab() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::Tab)).unwrap();
    match ev {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Tab),
        _ => panic!("expected Key"),
    }
}

#[test]
fn decode_backtab() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::BackTab)).unwrap();
    match ev {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::BackTab),
        _ => panic!("expected Key"),
    }
}

#[test]
fn decode_delete() {
    let d = InputDecoder::new();
    let ev = d.decode(press(CK::Delete)).unwrap();
    match ev {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Delete),
        _ => panic!("expected Key"),
    }
}

// ──────────── Arrow keys ────────────

#[test]
fn decode_up() {
    let d = InputDecoder::new();
    match d.decode(press(CK::Up)).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Up),
        _ => panic!(),
    }
}

#[test]
fn decode_down() {
    let d = InputDecoder::new();
    match d.decode(press(CK::Down)).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Down),
        _ => panic!(),
    }
}

#[test]
fn decode_left() {
    let d = InputDecoder::new();
    match d.decode(press(CK::Left)).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Left),
        _ => panic!(),
    }
}

#[test]
fn decode_right() {
    let d = InputDecoder::new();
    match d.decode(press(CK::Right)).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Right),
        _ => panic!(),
    }
}

// ──────────── Navigation keys ────────────

#[test]
fn decode_home() {
    let d = InputDecoder::new();
    match d.decode(press(CK::Home)).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::Home),
        _ => panic!(),
    }
}

#[test]
fn decode_end() {
    let d = InputDecoder::new();
    match d.decode(press(CK::End)).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::End),
        _ => panic!(),
    }
}

#[test]
fn decode_page_up() {
    let d = InputDecoder::new();
    match d.decode(press(CK::PageUp)).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::PageUp),
        _ => panic!(),
    }
}

#[test]
fn decode_page_down() {
    let d = InputDecoder::new();
    match d.decode(press(CK::PageDown)).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::PageDown),
        _ => panic!(),
    }
}

// ──────────── Function keys ────────────

#[test]
fn decode_f1() {
    let d = InputDecoder::new();
    match d.decode(press(CK::F(1))).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::F(1)),
        _ => panic!(),
    }
}

#[test]
fn decode_f12() {
    let d = InputDecoder::new();
    match d.decode(press(CK::F(12))).unwrap() {
        EditorEvent::Key(k) => assert_eq!(k.code, KeyCode::F(12)),
        _ => panic!(),
    }
}

// ──────────── Modifiers ────────────

#[test]
fn decode_ctrl_c() {
    let d = InputDecoder::new();
    let ev = d.decode(press_mod(CK::Char('c'), CKM::CONTROL)).unwrap();
    match ev {
        EditorEvent::Key(k) => {
            assert_eq!(k.code, KeyCode::Char('c'));
            assert!(k.modifiers.ctrl);
            assert!(!k.modifiers.alt);
        }
        _ => panic!(),
    }
}

#[test]
fn decode_alt_x() {
    let d = InputDecoder::new();
    let ev = d.decode(press_mod(CK::Char('x'), CKM::ALT)).unwrap();
    match ev {
        EditorEvent::Key(k) => {
            assert!(k.modifiers.alt);
            assert!(!k.modifiers.ctrl);
        }
        _ => panic!(),
    }
}

#[test]
fn decode_shift() {
    let d = InputDecoder::new();
    let ev = d.decode(press_mod(CK::Char('A'), CKM::SHIFT)).unwrap();
    match ev {
        EditorEvent::Key(k) => assert!(k.modifiers.shift),
        _ => panic!(),
    }
}

#[test]
fn decode_ctrl_alt() {
    let d = InputDecoder::new();
    let ev = d.decode(press_mod(CK::Char('d'), CKM::CONTROL | CKM::ALT)).unwrap();
    match ev {
        EditorEvent::Key(k) => {
            assert!(k.modifiers.ctrl);
            assert!(k.modifiers.alt);
        }
        _ => panic!(),
    }
}

// ──────────── Non-key events ────────────

#[test]
fn decode_resize() {
    let d = InputDecoder::new();
    let ev = d.decode(Event::Resize(100, 50)).unwrap();
    assert_eq!(ev, EditorEvent::Resize(Size::new(100, 50)));
}

#[test]
fn decode_paste() {
    let d = InputDecoder::new();
    let ev = d.decode(Event::Paste("pasted text".to_string())).unwrap();
    assert_eq!(ev, EditorEvent::Paste("pasted text".to_string()));
}

#[test]
fn decode_focus_gained() {
    let d = InputDecoder::new();
    let ev = d.decode(Event::FocusGained).unwrap();
    assert_eq!(ev, EditorEvent::FocusGained);
}

#[test]
fn decode_focus_lost() {
    let d = InputDecoder::new();
    let ev = d.decode(Event::FocusLost).unwrap();
    assert_eq!(ev, EditorEvent::FocusLost);
}

// ──────────── Edge cases ────────────

#[test]
fn modifiers_default() {
    let m = Modifiers::default();
    assert!(!m.ctrl);
    assert!(!m.alt);
    assert!(!m.shift);
}

#[test]
fn key_event_eq() {
    let a = KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: Modifiers::default(),
    };
    let b = KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: Modifiers::default(),
    };
    assert_eq!(a, b);
}

#[test]
fn key_event_ne() {
    let a = KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: Modifiers::default(),
    };
    let b = KeyEvent {
        code: KeyCode::Char('b'),
        modifiers: Modifiers::default(),
    };
    assert_ne!(a, b);
}

#[test]
fn editor_event_clone() {
    let ev = EditorEvent::Key(KeyEvent {
        code: KeyCode::Char('x'),
        modifiers: Modifiers { ctrl: true, alt: false, shift: false },
    });
    let ev2 = ev.clone();
    assert_eq!(ev, ev2);
}
