//! Sections D+: Multiplexer multi-window and stability PTY E2E tests.

use kjxlkj_host::{
    all_regression_scenarios, estimate_duration, validate_scenario, PtyAction, PtyConfig,
    PtyExpectation, PtyScenario,
};

/// Tmux-aware scenario with splits and tmux detach/resume.
#[test]
fn pty_multiplexer_multi_window_resume() {
    let scenario = PtyScenario {
        name: "pty_multiplexer_multi_window_resume".into(),
        description: "Splits + tabs with tmux detach/attach preserve layout and focus".into(),
        config: PtyConfig {
            timeout_ms: 12_000,
            ..PtyConfig::default()
        },
        actions: vec![
            // Set up splits and tabs
            PtyAction::TypeText(":sp\n".into()),
            PtyAction::TypeText(":tabnew\n".into()),
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("tab2-split".into()),
            PtyAction::SendKey("Escape".into()),
            // Tmux detach
            PtyAction::SendKey("Ctrl+b".into()),
            PtyAction::SendKey("d".into()),
            PtyAction::WaitMs(500),
            // Tmux re-attach
            PtyAction::TypeText("tmux attach".into()),
            PtyAction::SendKey("Enter".into()),
            PtyAction::WaitMs(500),
            // Verify editor is responsive
            PtyAction::SendKey("A".into()),
            PtyAction::TypeText(" resumed".into()),
            PtyAction::SendKey("Escape".into()),
            PtyAction::Quit,
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    assert!(dur >= 1000, "tmux scenario needs wait time: got {dur}ms");
    assert!(dur < 30_000, "duration must be bounded");
    assert_eq!(scenario.actions.len(), 15);
    assert_eq!(scenario.expectations.len(), 1);
}

// ---------------------------------------------------------------------------
// Section D: Stability and reproducibility
// ---------------------------------------------------------------------------

/// All scenarios must use deterministic, bounded timeouts.
#[test]
fn pty_scenarios_use_deterministic_timeouts() {
    let scenarios = all_regression_scenarios();
    for s in &scenarios {
        assert!(
            s.config.timeout_ms > 0,
            "scenario '{}' has zero timeout",
            s.name
        );
        assert!(
            s.config.timeout_ms <= 30_000,
            "scenario '{}' timeout {}ms exceeds 30s bound",
            s.name,
            s.config.timeout_ms
        );
        // Also verify estimated durations are bounded
        let dur = estimate_duration(s);
        assert!(
            dur < s.config.timeout_ms * 2,
            "scenario '{}' estimated duration {dur}ms exceeds 2× timeout",
            s.name
        );
    }
}

/// All scenarios must report non-empty, descriptive names for reproduction.
#[test]
fn pty_scenarios_report_reproduction_context() {
    let scenarios = all_regression_scenarios();
    for s in &scenarios {
        assert!(!s.name.is_empty(), "scenario name must not be empty");
        assert!(
            s.name.len() >= 3,
            "scenario '{}' name too short for reproduction context",
            s.name
        );
        assert!(
            !s.description.is_empty(),
            "scenario '{}' description must not be empty",
            s.name
        );
    }
}

/// All 7 regression scenarios from all_regression_scenarios() must validate.
#[test]
fn pty_all_regressions_validate() {
    let scenarios = all_regression_scenarios();
    assert_eq!(
        scenarios.len(),
        7,
        "expected exactly 7 regression scenarios"
    );
    for s in &scenarios {
        validate_scenario(s).unwrap_or_else(|e| {
            panic!("regression scenario '{}' failed validation: {e}", s.name);
        });
    }
}
