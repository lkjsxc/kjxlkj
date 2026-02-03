//! Tests for input module.

use super::*;
use kjxlkj_core_types::KeyCode;

#[test]
fn test_script_key_parsing() {
    let key = ScriptKey {
        code: "a".to_string(),
        mods: vec![],
    };
    let k = key.to_key();
    assert_eq!(k.code, KeyCode::Char('a'));
}

#[test]
fn test_script_key_with_mods() {
    let key = ScriptKey {
        code: "c".to_string(),
        mods: vec!["ctrl".to_string()],
    };
    let k = key.to_key();
    assert!(k.mods.ctrl());
}

#[test]
fn test_parse_script() {
    let script = r#"[
        {"kind": "key", "code": "i", "mods": []},
        {"kind": "key", "code": "a", "mods": []},
        {"kind": "key", "code": "esc", "mods": []}
    ]"#;

    let steps = parse_script(script).unwrap();
    assert_eq!(steps.len(), 3);
}

#[test]
fn test_parse_key_array() {
    let keys = r#"[
        {"code": "h"},
        {"code": "j"},
        {"code": "k"},
        {"code": "l"}
    ]"#;

    let result = parse_key_array(keys).unwrap();
    assert_eq!(result.len(), 4);
}
