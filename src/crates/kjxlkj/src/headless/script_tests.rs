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

#[test]
fn parse_tab_key() {
    let key = ScriptKey {
        code: "Tab".to_string(),
        ctrl: false,
        alt: false,
        shift: false,
    };
    let k = parse_key(&key).unwrap();
    assert_eq!(k.code, KeyCode::Tab);
}

#[test]
fn parse_backspace_key() {
    let key = ScriptKey {
        code: "Backspace".to_string(),
        ctrl: false,
        alt: false,
        shift: false,
    };
    let k = parse_key(&key).unwrap();
    assert_eq!(k.code, KeyCode::Backspace);
}

#[test]
fn parse_alt_modifier() {
    let key = ScriptKey {
        code: "x".to_string(),
        ctrl: false,
        alt: true,
        shift: false,
    };
    let k = parse_key(&key).unwrap();
    assert!(k.mods.alt);
}

#[test]
fn assert_mode_insert_case_insensitive() {
    let mut state = EditorState::new();
    use kjxlkj_core::Mode;
    state.set_mode(Mode::Insert);
    assert!(assert_mode(&state, "insert").is_ok());
    assert!(assert_mode(&state, "Insert").is_ok());
    assert!(assert_mode(&state, "INSERT").is_ok());
}
