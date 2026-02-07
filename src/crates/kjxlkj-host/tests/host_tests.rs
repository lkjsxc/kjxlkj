use kjxlkj_host::{
    all_regression_scenarios, check_reachability, define_core_features, estimate_duration,
    has_command_entry, has_keybinding_entry, validate_scenario as validate_pty_scenario,
    EntryKind, FeatureSpec, PtyAction, PtyConfig, PtyExpectation, PtyScenario,
};
use kjxlkj_host::{
    build_edit_flow, build_wq_flow, detect_encoding, detect_line_ending, resolve_path,
    validate_write_target, FileOp,
};

// --- PTY scenario tests ---

#[test]
fn pty_valid_scenario_passes() {
    let s = PtyScenario {
        name: "basic".into(),
        description: "a test".into(),
        config: PtyConfig::default(),
        actions: vec![PtyAction::SendKey("i".into())],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };
    assert!(validate_pty_scenario(&s).is_ok());
}

#[test]
fn pty_empty_send_key_fails() {
    let s = PtyScenario {
        name: "bad".into(),
        description: "d".into(),
        config: PtyConfig::default(),
        actions: vec![PtyAction::SendKey(String::new())],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };
    assert!(validate_pty_scenario(&s).is_err());
}

#[test]
fn pty_duration_type_text() {
    let s = PtyScenario {
        name: "dur".into(),
        description: "d".into(),
        config: PtyConfig::default(),
        actions: vec![PtyAction::TypeText("hello".into())],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };
    assert_eq!(estimate_duration(&s), 50);
}

#[test]
fn pty_duration_mixed_actions() {
    let s = PtyScenario {
        name: "mix".into(),
        description: "d".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::WaitMs(100),
            PtyAction::SendKey("x".into()),
            PtyAction::Quit,
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };
    assert_eq!(estimate_duration(&s), 200);
}

// --- File flow tests ---

#[test]
fn resolve_path_tilde_expansion() {
    let resolved = resolve_path("~/nonexistent_test_path_xyz");
    assert!(!resolved.starts_with('~'));
}

#[test]
fn encoding_detection_utf8_bom() {
    assert_eq!(detect_encoding(&[0xEF, 0xBB, 0xBF, b'x']), "utf-8-bom");
}

#[test]
fn encoding_detection_utf16_le() {
    assert_eq!(detect_encoding(&[0xFF, 0xFE, 0, 0]), "utf-16-le");
}

#[test]
fn encoding_detection_plain_utf8() {
    assert_eq!(detect_encoding(b"plain text"), "utf-8");
}

#[test]
fn line_ending_mixed_favors_majority() {
    assert_eq!(detect_line_ending("a\r\nb\r\nc\n"), "\r\n");
}

#[test]
fn validate_write_empty_path_fails() {
    assert!(validate_write_target("").is_err());
}

#[test]
fn validate_write_tmp_ok() {
    assert!(validate_write_target("/tmp/test_file.txt").is_ok());
}

#[test]
fn build_edit_flow_returns_open_edit() {
    assert_eq!(build_edit_flow("a.rs"), vec![FileOp::Open, FileOp::Edit]);
}

#[test]
fn build_wq_flow_ends_with_write_quit() {
    let f = build_wq_flow("a.rs");
    assert_eq!(f.last(), Some(&FileOp::WriteQuit));
}

// --- Feature reachability tests ---

#[test]
fn core_features_at_least_15() {
    assert!(define_core_features().len() >= 15);
}

#[test]
fn all_core_features_reachable() {
    let report = check_reachability(define_core_features());
    assert_eq!(report.unreachable_count, 0);
}

#[test]
fn keybinding_check_cursor_movement() {
    let f = define_core_features();
    let cursor = f.iter().find(|x| x.name == "cursor-movement").unwrap();
    assert!(has_keybinding_entry(cursor));
}

#[test]
fn command_check_write() {
    let f = define_core_features();
    let write = f.iter().find(|x| x.name == "write").unwrap();
    assert!(has_command_entry(write));
    assert!(!has_keybinding_entry(write));
}

#[test]
fn unreachable_feature_detected() {
    let spec = FeatureSpec {
        name: "orphan".into(),
        entry_points: vec![],
        tested: false,
    };
    let report = check_reachability(vec![spec]);
    assert_eq!(report.unreachable_count, 1);
}

#[test]
fn leader_chord_feature_not_keybinding() {
    let f = define_core_features();
    let explorer = f.iter().find(|x| x.name == "file-explorer").unwrap();
    assert!(!has_keybinding_entry(explorer));
    assert!(explorer.entry_points[0].0 == EntryKind::LeaderChord);
}
