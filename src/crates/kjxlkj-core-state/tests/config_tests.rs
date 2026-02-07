//! Tests for ConfigStore, EditorOptions, SetAction, and option parsing.

use kjxlkj_core_state::config_store::{build_defaults, ConfigStore, OptionScope};
use kjxlkj_core_state::options::{apply_set_action, parse_set_arg, EditorOptions, SetAction};

// --- ConfigStore ---

#[test]
fn config_store_defaults() {
    let store = build_defaults();
    assert_eq!(store.get("number"), Some("true"));
    assert_eq!(store.get("tabstop"), Some("8"));
    assert_eq!(store.get("scrolloff"), Some("5"));
}

#[test]
fn config_store_set_and_get() {
    let mut store = build_defaults();
    assert!(store.set("tabstop", "4"));
    assert_eq!(store.get("tabstop"), Some("4"));
}

#[test]
fn config_store_reset_to_default() {
    let mut store = build_defaults();
    store.set("tabstop", "2");
    store.reset("tabstop");
    assert_eq!(store.get("tabstop"), Some("8"));
}

#[test]
fn config_store_nonexistent_returns_none() {
    let store = build_defaults();
    assert!(store.get("nonexistent").is_none());
}

#[test]
fn config_store_set_nonexistent_returns_false() {
    let mut store = build_defaults();
    assert!(!store.set("nonexistent", "value"));
}

#[test]
fn config_store_names_has_entries() {
    let store = build_defaults();
    assert!(store.names().len() >= 10);
}

#[test]
fn config_store_register_new() {
    let mut store = ConfigStore::new();
    store.register("custom", OptionScope::Global, "hello");
    assert_eq!(store.get("custom"), Some("hello"));
}

// --- parse_set_arg ---

#[test]
fn parse_set_bool_enable() {
    assert_eq!(
        parse_set_arg("number"),
        SetAction::SetBool("number".into(), true)
    );
}

#[test]
fn parse_set_bool_disable() {
    assert_eq!(
        parse_set_arg("nonumber"),
        SetAction::SetBool("number".into(), false)
    );
}

#[test]
fn parse_set_wrap() {
    assert_eq!(
        parse_set_arg("wrap"),
        SetAction::SetBool("wrap".into(), true)
    );
}

#[test]
fn parse_set_nowrap() {
    assert_eq!(
        parse_set_arg("nowrap"),
        SetAction::SetBool("wrap".into(), false)
    );
}

#[test]
fn parse_set_int_tabstop() {
    assert_eq!(
        parse_set_arg("tabstop=4"),
        SetAction::SetInt("tabstop".into(), 4)
    );
}

#[test]
fn parse_set_int_shiftwidth() {
    assert_eq!(
        parse_set_arg("shiftwidth=2"),
        SetAction::SetInt("shiftwidth".into(), 2)
    );
}

#[test]
fn parse_set_show_all() {
    assert_eq!(parse_set_arg(""), SetAction::ShowAll);
    assert_eq!(parse_set_arg("all"), SetAction::ShowAll);
}

#[test]
fn parse_set_query() {
    assert_eq!(parse_set_arg("number?"), SetAction::Query("number".into()));
}

// --- EditorOptions defaults ---

#[test]
fn editor_options_defaults() {
    let opts = EditorOptions::default();
    assert!(opts.number);
    assert!(opts.wrap);
    assert_eq!(opts.tabstop, 8);
    assert_eq!(opts.shiftwidth, 8);
    assert!(!opts.expandtab);
    assert!(opts.hlsearch);
}

// --- apply_set_action ---

#[test]
fn apply_set_bool_option() {
    let mut opts = EditorOptions::default();
    apply_set_action(&mut opts, SetAction::SetBool("wrap".into(), false)).unwrap();
    assert!(!opts.wrap);
}

#[test]
fn apply_set_int_option() {
    let mut opts = EditorOptions::default();
    apply_set_action(&mut opts, SetAction::SetInt("tabstop".into(), 4)).unwrap();
    assert_eq!(opts.tabstop, 4);
}

#[test]
fn apply_set_invalid_returns_error() {
    let mut opts = EditorOptions::default();
    assert!(apply_set_action(&mut opts, SetAction::Invalid("bad".into())).is_err());
}

// --- OptionScope ---

#[test]
fn option_scope_variants() {
    assert_eq!(OptionScope::Global, OptionScope::Global);
    assert_ne!(OptionScope::Buffer, OptionScope::Window);
}
