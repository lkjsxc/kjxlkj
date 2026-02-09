//! Tests for the options system.

use crate::options::{parse_set_command, OptionStore, OptionValue};

#[test]
fn default_options_exist() {
    let store = OptionStore::new();
    assert_eq!(store.get_int("tabstop"), 8);
    assert!(store.get_bool("wrap"));
    assert!(!store.get_bool("number"));
}

#[test]
fn set_boolean_option() {
    let mut store = OptionStore::new();
    let r = parse_set_command(&mut store, "number");
    assert!(r.is_ok());
    assert!(store.get_bool("number"));
}

#[test]
fn unset_boolean_option() {
    let mut store = OptionStore::new();
    store.set("number", OptionValue::Bool(true));
    let r = parse_set_command(&mut store, "nonumber");
    assert!(r.is_ok());
    assert!(!store.get_bool("number"));
}

#[test]
fn set_integer_option() {
    let mut store = OptionStore::new();
    let r = parse_set_command(&mut store, "tabstop=4");
    assert!(r.is_ok());
    assert_eq!(store.get_int("tabstop"), 4);
}

#[test]
fn query_option() {
    let store = OptionStore::new();
    let r = parse_set_command(&mut store.clone(), "tabstop?");
    assert!(r.is_ok());
    let msg = r.unwrap().unwrap();
    assert!(msg.contains("tabstop"));
    assert!(msg.contains("8"));
}

#[test]
fn unknown_option_error() {
    let mut store = OptionStore::new();
    let r = parse_set_command(&mut store, "bogus");
    assert!(r.is_err());
    let err = r.unwrap_err();
    assert!(err.contains("E518"));
}

#[test]
fn list_all_options() {
    let mut store = OptionStore::new();
    let r = parse_set_command(&mut store, "");
    assert!(r.is_ok());
    let msg = r.unwrap().unwrap();
    assert!(msg.contains("tabstop"));
    assert!(msg.contains("wrap"));
}

#[test]
fn set_command_in_editor() {
    let mut e = crate::editor::EditorState::new(80, 24);
    e.handle_key(kjxlkj_core_types::Key::char(':'));
    for c in "set tabstop=2".chars() {
        e.handle_key(kjxlkj_core_types::Key::char(c));
    }
    e.handle_key(kjxlkj_core_types::Key::new(
        kjxlkj_core_types::KeyCode::Enter,
        kjxlkj_core_types::Modifier::NONE,
    ));
    assert_eq!(e.options.get_int("tabstop"), 2);
}
