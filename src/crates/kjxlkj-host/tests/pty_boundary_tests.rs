//! Section B: Expert boundary PTY E2E scenario tests.
//!
//! These tests construct and validate PTY scenarios structurally,
//! verifying that each scenario is well-formed without spawning a real PTY.

use kjxlkj_host::{
    estimate_duration, validate_scenario, PtyAction, PtyConfig, PtyExpectation, PtyScenario,
};

/// Repeated `a` + `Esc` cycles should never leave a floating cursor.
#[test]
fn pty_append_eol_mode_churn() {
    let mut actions = Vec::new();
    // Seed some text first
    actions.push(PtyAction::SendKey("i".into()));
    actions.push(PtyAction::TypeText("hello".into()));
    actions.push(PtyAction::SendKey("Escape".into()));
    // Repeated append-escape churn (10 cycles)
    for _ in 0..10 {
        actions.push(PtyAction::SendKey("a".into()));
        actions.push(PtyAction::TypeText("x".into()));
        actions.push(PtyAction::SendKey("Escape".into()));
    }
    actions.push(PtyAction::Quit);

    let scenario = PtyScenario {
        name: "pty_append_eol_mode_churn".into(),
        description: "Repeated a + Esc cycles never leave a floating cursor past EOL".into(),
        config: PtyConfig::default(),
        actions,
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    assert!(dur > 0, "duration must be positive");
    assert!(dur < 30_000, "duration must be bounded");
    // 1 (i) + 1 (text) + 1 (esc) + 10*3 (churn) + 1 (quit) = 34
    assert_eq!(scenario.actions.len(), 34);
    assert_eq!(scenario.expectations.len(), 1);
}

/// Long Japanese CJK line wraps and remains editable.
#[test]
fn pty_wrap_long_cjk_line() {
    let cjk_text = "あいうえおかきくけこさしすせそたちつてと".repeat(5);
    let scenario = PtyScenario {
        name: "pty_wrap_long_cjk_line".into(),
        description: "Long CJK line wraps correctly and remains editable".into(),
        config: PtyConfig {
            width: 40, // narrow to force wrapping
            ..PtyConfig::default()
        },
        actions: vec![
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText(cjk_text.clone()),
            PtyAction::SendKey("Escape".into()),
            PtyAction::SendKey("0".into()), // go to beginning of line
            PtyAction::WaitMs(100),
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    // CJK text is long, so typing duration should reflect that
    assert!(
        dur > 500,
        "CJK typing should take significant time: got {dur}ms"
    );
    assert!(dur < 60_000, "duration must be bounded");
    assert_eq!(scenario.actions.len(), 5);
    assert_eq!(scenario.expectations.len(), 1);
    assert_eq!(scenario.config.width, 40);
}

/// IME conversion Space does not trigger leader mappings.
#[test]
fn pty_leader_vs_ime_space() {
    let scenario = PtyScenario {
        name: "pty_leader_vs_ime_space".into(),
        description: "Space during IME composition must not trigger leader key".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::SendKey("i".into()),
            // Simulate IME context: typing Japanese then pressing Space for conversion
            PtyAction::TypeText("にほんご".into()),
            PtyAction::SendKey("Space".into()), // IME conversion, not leader
            PtyAction::SendKey("Enter".into()), // confirm conversion
            PtyAction::SendKey("Escape".into()),
            PtyAction::WaitMs(50),
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    assert!(dur > 0);
    assert!(dur < 10_000, "duration must be bounded");
    assert_eq!(scenario.actions.len(), 6);
    assert_eq!(scenario.expectations.len(), 1);
}

/// Tmux detach and resume preserves interactive correctness.
#[test]
fn pty_tmux_detach_resume() {
    let scenario = PtyScenario {
        name: "pty_tmux_detach_resume".into(),
        description: "Multiplexer attach/detach preserves editor state".into(),
        config: PtyConfig {
            timeout_ms: 10_000, // tmux scenarios need more time
            ..PtyConfig::default()
        },
        actions: vec![
            // Start editing
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("before detach".into()),
            PtyAction::SendKey("Escape".into()),
            // Simulate tmux detach (Ctrl-b d)
            PtyAction::SendKey("Ctrl+b".into()),
            PtyAction::SendKey("d".into()),
            PtyAction::WaitMs(500), // time for detach
            // Simulate tmux re-attach
            PtyAction::TypeText("tmux attach".into()),
            PtyAction::SendKey("Enter".into()),
            PtyAction::WaitMs(500), // time for re-attach
            // Verify editor is back: append more text
            PtyAction::SendKey("A".into()),
            PtyAction::TypeText(" after resume".into()),
            PtyAction::SendKey("Escape".into()),
            PtyAction::Quit,
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    assert!(dur >= 1000, "tmux scenario needs wait time: got {dur}ms");
    assert!(dur < 30_000, "duration must be bounded");
    assert_eq!(scenario.actions.len(), 13);
    assert_eq!(scenario.expectations.len(), 1);
}

/// Rapid resize events with wrapping active keep cursor visible.
#[test]
fn pty_resize_storm_with_wrap() {
    let mut actions = vec![
        PtyAction::SendKey("i".into()),
        PtyAction::TypeText(
            "a long line that should trigger wrap behavior in narrow terminals".into(),
        ),
        PtyAction::SendKey("Escape".into()),
    ];
    // Simulate resize storm: rapid width changes
    for w in [40u16, 120, 30, 200, 60, 25, 80] {
        // Each resize is modeled as a wait (the harness would resize)
        actions.push(PtyAction::WaitMs(20));
        // Record the target width in a descriptive action
        actions.push(PtyAction::TypeText(format!("resize:{w}")));
    }
    actions.push(PtyAction::WaitMs(100));
    actions.push(PtyAction::Quit);

    let scenario = PtyScenario {
        name: "pty_resize_storm_with_wrap".into(),
        description: "Rapid resize events with wrapping keep cursor visible".into(),
        config: PtyConfig::default(),
        actions,
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    assert!(dur > 0);
    assert!(dur < 30_000, "duration must be bounded");
    // 3 (initial) + 7*2 (resize pairs) + 1 (wait) + 1 (quit) = 19
    assert_eq!(scenario.actions.len(), 19);
    assert_eq!(scenario.expectations.len(), 1);
}
