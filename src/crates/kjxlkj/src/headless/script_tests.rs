//! Tests for script module.

use super::script::{assert_cursor, assert_mode, parse_key, ScriptKey};
use kjxlkj_core::EditorState;
use kjxlkj_input::KeyCode;

#[test]
fn parse_simple_key() {
    let key = ScriptKey {
        code: "a".to_string(),
        ctrl: false,
        alt: false,
        shift: false,
    };
    let k = parse_key(&key).unwrap();
    assert_eq!(k.code, KeyCode::Char('a'));
}

#[test]
fn parse_escape_key() {
    let key = ScriptKey {
        code: "Escape".to_string(),
        ctrl: false,
        alt: false,
        shift: false,
    };
    let k = parse_key(&key).unwrap();
    assert!(k.is_escape());
}

#[test]
fn parse_enter_key() {
    let key = ScriptKey {
        code: "Enter".to_string(),
        ctrl: false,
        alt: false,
        shift: false,
    };
    let k = parse_key(&key).unwrap();
    assert_eq!(k.code, KeyCode::Enter);
}

#[test]
fn parse_ctrl_modifier() {
    let key = ScriptKey {
        code: "c".to_string(),
        ctrl: true,
        alt: false,
        shift: false,
    };
    let k = parse_key(&key).unwrap();
    assert!(k.mods.ctrl);
}

#[test]
fn parse_arrow_key() {
    let key = ScriptKey {
        code: "Left".to_string(),
        ctrl: false,
        alt: false,
        shift: false,
    };
    let k = parse_key(&key).unwrap();
    assert_eq!(k.code, KeyCode::Left);
}

#[test]
fn assert_mode_valid() {
    let state = EditorState::new();
    let result = assert_mode(&state, "normal");
    assert!(result.is_ok());
}

#[test]
fn assert_cursor_valid() {
    let state = EditorState::new();
    let result = assert_cursor(&state, 0, 0);
    assert!(result.is_ok());
}
