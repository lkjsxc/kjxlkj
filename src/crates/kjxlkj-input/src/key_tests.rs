//! Tests for key representation.

use crate::{Key, KeyCode, Modifiers};

#[test]
fn key_char() {
    let k = Key::char('a');
    assert_eq!(k.code, KeyCode::Char('a'));
    assert!(!k.mods.ctrl);
}

#[test]
fn key_ctrl_char() {
    let k = Key::ctrl_char('r');
    assert_eq!(k.code, KeyCode::Char('r'));
    assert!(k.mods.ctrl);
}

#[test]
fn key_is_escape() {
    let k = Key::new(KeyCode::Escape, Modifiers::none());
    assert!(k.is_escape());
}

#[test]
fn key_is_enter() {
    let k = Key::new(KeyCode::Enter, Modifiers::none());
    assert!(k.is_enter());
}

#[test]
fn key_is_backspace() {
    let k = Key::new(KeyCode::Backspace, Modifiers::none());
    assert!(k.is_backspace());
}

#[test]
fn key_ctrl_h_is_backspace() {
    let k = Key::ctrl_char('h');
    assert!(k.is_backspace());
}

#[test]
fn modifiers_none() {
    let m = Modifiers::none();
    assert!(!m.ctrl);
    assert!(!m.alt);
    assert!(!m.shift);
}

#[test]
fn modifiers_ctrl() {
    let m = Modifiers::ctrl();
    assert!(m.ctrl);
    assert!(!m.alt);
    assert!(!m.shift);
}

#[test]
fn key_equality() {
    let k1 = Key::char('a');
    let k2 = Key::char('a');
    assert_eq!(k1, k2);
}

#[test]
fn key_inequality() {
    let k1 = Key::char('a');
    let k2 = Key::char('b');
    assert_ne!(k1, k2);
}

#[test]
fn key_not_escape() {
    let k = Key::char('a');
    assert!(!k.is_escape());
}

#[test]
fn key_not_enter() {
    let k = Key::char('a');
    assert!(!k.is_enter());
}

#[test]
fn key_not_backspace() {
    let k = Key::char('a');
    assert!(!k.is_backspace());
}

#[test]
fn key_code_arrow_keys() {
    let left = KeyCode::Left;
    let right = KeyCode::Right;
    let up = KeyCode::Up;
    let down = KeyCode::Down;
    assert_ne!(left, right);
    assert_ne!(up, down);
}

#[test]
fn modifiers_alt() {
    let m = Modifiers {
        ctrl: false,
        alt: true,
        shift: false,
    };
    assert!(m.alt);
}

#[test]
fn modifiers_shift() {
    let m = Modifiers {
        ctrl: false,
        alt: false,
        shift: true,
    };
    assert!(m.shift);
}

#[test]
fn key_tab() {
    let k = Key::new(KeyCode::Tab, Modifiers::none());
    assert_eq!(k.code, KeyCode::Tab);
}
