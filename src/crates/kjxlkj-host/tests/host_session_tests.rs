use kjxlkj_host::{all_regression_scenarios, validate_scenario as validate_pty_scenario};
use kjxlkj_host::{
    integration_undo_redo_scenario, multi_buffer_scenario, open_edit_save_scenario,
    validate_integration_scenario,
};
use kjxlkj_host::{
    parse_session_buffers, serialize_session, SessionBuffer, SessionData, SessionWindow,
};

// --- Integration scenario tests ---

#[test]
fn integration_open_edit_save_valid() {
    assert!(validate_integration_scenario(&open_edit_save_scenario()).is_ok());
}

#[test]
fn integration_undo_redo_valid() {
    assert!(validate_integration_scenario(&integration_undo_redo_scenario()).is_ok());
}

#[test]
fn integration_multi_buffer_valid() {
    assert!(validate_integration_scenario(&multi_buffer_scenario()).is_ok());
}

// --- Regression scenarios ---

#[test]
fn all_regression_scenarios_count() {
    assert_eq!(all_regression_scenarios().len(), 7);
}

#[test]
fn all_regression_scenarios_valid() {
    for s in all_regression_scenarios() {
        validate_pty_scenario(&s).unwrap_or_else(|e| panic!("{}: {e}", s.name));
    }
}

#[test]
fn regression_scenario_names_unique() {
    let scenarios = all_regression_scenarios();
    let mut names: Vec<_> = scenarios.iter().map(|s| s.name.clone()).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), 7);
}

// --- Session tests ---

#[test]
fn session_serialize_roundtrip() {
    let data = SessionData {
        buffers: vec![SessionBuffer {
            path: "a.rs".into(),
            cursor_line: 1,
            cursor_col: 0,
        }],
        windows: vec![SessionWindow {
            buffer_index: 0,
            width: 80,
            height: 24,
        }],
        globals: vec![],
        cwd: "/tmp".into(),
    };
    let json = serialize_session(&data);
    let paths = parse_session_buffers(&json);
    assert_eq!(paths, vec!["a.rs"]);
}

#[test]
fn session_parse_invalid_json() {
    assert!(parse_session_buffers("not json").is_empty());
}

#[test]
fn session_empty() {
    let data = SessionData::empty();
    assert!(data.buffers.is_empty());
    let json = serialize_session(&data);
    assert!(json.contains("buffers"));
}
