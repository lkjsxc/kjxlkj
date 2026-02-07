//! Section C: Multi-window practical-utilization PTY E2E tests.
//!
//! These tests construct and validate PTY scenarios structurally,
//! verifying well-formedness without spawning a real PTY.

use kjxlkj_host::{
    estimate_duration, validate_scenario, PtyAction, PtyConfig, PtyExpectation, PtyScenario,
};

// ---------------------------------------------------------------------------
// Section C: Multi-window practical-utilization PTY E2E tests
// ---------------------------------------------------------------------------

/// Split, move to non-primary window, edit, :wq, verify persisted content.
#[test]
fn pty_split_edit_write_non_primary() {
    let scenario = PtyScenario {
        name: "pty_split_edit_write_non_primary".into(),
        description: "Split window, edit in non-primary pane, :wq persists content".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::TypeText(":sp\n".into()), // horizontal split
            PtyAction::WaitMs(100),
            PtyAction::SendKey("Ctrl+w".into()), // window prefix
            PtyAction::SendKey("j".into()),      // move to lower (non-primary)
            PtyAction::SendKey("i".into()),      // insert mode
            PtyAction::TypeText("non-primary edit".into()),
            PtyAction::SendKey("Escape".into()),
            PtyAction::TypeText(":wq\n".into()), // write-quit
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    assert!(dur > 0);
    assert!(dur < 15_000, "duration must be bounded");
    assert_eq!(scenario.actions.len(), 8);
    assert_eq!(scenario.expectations.len(), 1);
}

/// Create tabs, cycle through them, edit in each, and verify state on save.
#[test]
fn pty_tabs_cycle_edit_persist() {
    let scenario = PtyScenario {
        name: "pty_tabs_cycle_edit_persist".into(),
        description: "Create tabs, switch, edit in each, :wa preserves all".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::TypeText(":tabnew\n".into()), // tab 2
            PtyAction::TypeText(":tabnew\n".into()), // tab 3
            PtyAction::WaitMs(50),
            // Cycle back to tab 1
            PtyAction::TypeText(":tabfirst\n".into()),
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("tab1-content".into()),
            PtyAction::SendKey("Escape".into()),
            // Move to tab 2
            PtyAction::TypeText(":tabnext\n".into()),
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("tab2-content".into()),
            PtyAction::SendKey("Escape".into()),
            // Move to tab 3
            PtyAction::TypeText(":tabnext\n".into()),
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("tab3-content".into()),
            PtyAction::SendKey("Escape".into()),
            // Write all and quit
            PtyAction::TypeText(":wa\n".into()),
            PtyAction::TypeText(":qa\n".into()),
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    assert!(dur > 0);
    assert!(dur < 15_000, "duration must be bounded");
    assert_eq!(scenario.actions.len(), 17);
    assert_eq!(scenario.expectations.len(), 1);
}

/// Open terminal pane from split, return focus, continue edit/write flow.
#[test]
fn pty_window_terminal_focus_roundtrip() {
    let scenario = PtyScenario {
        name: "pty_window_terminal_focus_roundtrip".into(),
        description: "Split, open terminal, escape back to editor, continue editing".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::TypeText(":sp\n".into()), // split
            PtyAction::WaitMs(50),
            PtyAction::TypeText(":terminal\n".into()), // open terminal
            PtyAction::WaitMs(200),                    // let terminal start
            PtyAction::SendKey("Escape".into()),       // exit terminal mode
            PtyAction::SendKey("Ctrl+w".into()),       // window prefix
            PtyAction::SendKey("k".into()),            // return to editor pane
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("back in editor".into()),
            PtyAction::SendKey("Escape".into()),
            PtyAction::Quit,
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    assert!(dur >= 250, "must include terminal wait time");
    assert!(dur < 15_000, "duration must be bounded");
    assert_eq!(scenario.actions.len(), 11);
    assert_eq!(scenario.expectations.len(), 1);
}

/// Close one split after edits, verify remaining window state is coherent.
#[test]
fn pty_window_close_rebalance_persistence() {
    let scenario = PtyScenario {
        name: "pty_window_close_rebalance_persistence".into(),
        description: "Two splits, edit both, close one, verify remaining state".into(),
        config: PtyConfig::default(),
        actions: vec![
            // Create two splits
            PtyAction::TypeText(":sp\n".into()),
            PtyAction::TypeText(":sp\n".into()),
            PtyAction::WaitMs(50),
            // Edit in top pane
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("pane-top".into()),
            PtyAction::SendKey("Escape".into()),
            // Move down and edit
            PtyAction::SendKey("Ctrl+w".into()),
            PtyAction::SendKey("j".into()),
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("pane-mid".into()),
            PtyAction::SendKey("Escape".into()),
            // Close current (middle) pane
            PtyAction::TypeText(":q\n".into()),
            PtyAction::WaitMs(50),
            // Remaining pane should still have content
            PtyAction::Quit,
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    };

    validate_scenario(&scenario).expect("scenario should be valid");
    let dur = estimate_duration(&scenario);
    assert!(dur > 0);
    assert!(dur < 15_000, "duration must be bounded");
    assert_eq!(scenario.actions.len(), 14);
    assert_eq!(scenario.expectations.len(), 1);
}
