//! PTY regression scenario definitions.

use crate::pty_harness::{PtyAction, PtyConfig, PtyExpectation, PtyScenario};

/// Insert newline in insert mode, verify cursor and content.
pub fn insert_newline_scenario() -> PtyScenario {
    PtyScenario {
        name: "insert-newline".into(),
        description: "Enter insert mode, type text, press Enter, verify newline".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("hello".into()),
            PtyAction::SendKey("Enter".into()),
            PtyAction::TypeText("world".into()),
            PtyAction::SendKey("Escape".into()),
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    }
}

/// Leader-e opens the file explorer.
pub fn leader_explorer_scenario() -> PtyScenario {
    PtyScenario {
        name: "leader-explorer".into(),
        description: "Press <leader>e to open file explorer".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::SendKey("Space".into()),
            PtyAction::SendKey("e".into()),
            PtyAction::WaitMs(100),
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    }
}

/// Leader-t opens terminal pane.
pub fn leader_terminal_scenario() -> PtyScenario {
    PtyScenario {
        name: "leader-terminal".into(),
        description: "Press <leader>t to open a terminal pane".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::SendKey("Space".into()),
            PtyAction::SendKey("t".into()),
            PtyAction::WaitMs(100),
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    }
}

/// gg motion moves to the first line.
pub fn gg_motion_scenario() -> PtyScenario {
    PtyScenario {
        name: "gg-motion".into(),
        description: "Use gg to go to the beginning of the document".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::SendKey("G".into()),
            PtyAction::SendKey("g".into()),
            PtyAction::SendKey("g".into()),
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    }
}

/// Undo and redo preserve content.
pub fn undo_redo_scenario() -> PtyScenario {
    PtyScenario {
        name: "undo-redo".into(),
        description: "Insert text, undo, redo, verify content integrity".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("foo".into()),
            PtyAction::SendKey("Escape".into()),
            PtyAction::SendKey("u".into()),
            PtyAction::SendKey("Ctrl+r".into()),
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    }
}

/// Append at EOL inserts after last character.
pub fn append_eol_scenario() -> PtyScenario {
    PtyScenario {
        name: "append-eol".into(),
        description: "A appends at end of line".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText("abc".into()),
            PtyAction::SendKey("Escape".into()),
            PtyAction::SendKey("A".into()),
            PtyAction::TypeText("d".into()),
            PtyAction::SendKey("Escape".into()),
        ],
        expectations: vec![PtyExpectation::ExitCode(0)],
    }
}

/// Write-quit persists the buffer to disk.
pub fn wq_persistence_scenario() -> PtyScenario {
    PtyScenario {
        name: "wq-persistence".into(),
        description: "Edit a file and :wq, verify file written".into(),
        config: PtyConfig::default(),
        actions: vec![
            PtyAction::WriteFile("/tmp/kjxlkj_test_wq.txt".into(), "start".into()),
            PtyAction::SendKey("i".into()),
            PtyAction::TypeText(" appended".into()),
            PtyAction::SendKey("Escape".into()),
            PtyAction::TypeText(":wq\n".into()),
        ],
        expectations: vec![
            PtyExpectation::FileExists("/tmp/kjxlkj_test_wq.txt".into()),
            PtyExpectation::ExitCode(0),
        ],
    }
}

/// Collect all regression scenarios into a single vector.
pub fn all_regression_scenarios() -> Vec<PtyScenario> {
    vec![
        insert_newline_scenario(),
        leader_explorer_scenario(),
        leader_terminal_scenario(),
        gg_motion_scenario(),
        undo_redo_scenario(),
        append_eol_scenario(),
        wq_persistence_scenario(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pty_harness::validate_scenario;

    #[test]
    fn all_scenarios_valid() {
        for s in all_regression_scenarios() {
            validate_scenario(&s).unwrap_or_else(|e| {
                panic!("scenario '{}' invalid: {e}", s.name);
            });
        }
    }

    #[test]
    fn scenario_count() {
        assert_eq!(all_regression_scenarios().len(), 7);
    }

    #[test]
    fn names_unique() {
        let scenarios = all_regression_scenarios();
        let mut names: Vec<_> = scenarios.iter().map(|s| &s.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), scenarios.len());
    }
}
