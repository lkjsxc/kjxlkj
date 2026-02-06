/// PTY regression test scenarios covering high-priority E2E items.

use crate::pty_harness::{
    validate_scenario, PtyAction, PtyExpectation, PtyScenario,
};

pub(crate) fn insert_newline_scenario() -> PtyScenario {
    let mut s = PtyScenario::new("insert_newline");
    s.add_action(PtyAction::SendKey("i".into()));
    s.add_action(PtyAction::TypeText("line1".into()));
    s.add_action(PtyAction::SendKey("Enter".into()));
    s.add_action(PtyAction::TypeText("line2".into()));
    s.add_action(PtyAction::SendKey("Escape".into()));
    s.add_action(PtyAction::TypeText(":wq".into()));
    s.add_action(PtyAction::SendKey("Enter".into()));
    s.add_expectation(PtyExpectation::FileContains {
        path: "test.txt".into(),
        expected: "line1\nline2".into(),
    });
    s.add_expectation(PtyExpectation::ExitCode(0));
    s
}

pub(crate) fn leader_explorer_scenario() -> PtyScenario {
    let mut s = PtyScenario::new("leader_explorer");
    s.add_action(PtyAction::SendKey("Space".into()));
    s.add_action(PtyAction::SendKey("e".into()));
    s.add_action(PtyAction::WaitMs(200));
    s.add_action(PtyAction::SendKey("Space".into()));
    s.add_action(PtyAction::SendKey("e".into()));
    s.add_action(PtyAction::Quit);
    s
}

pub(crate) fn leader_terminal_scenario() -> PtyScenario {
    let mut s = PtyScenario::new("leader_terminal");
    s.add_action(PtyAction::SendKey("Space".into()));
    s.add_action(PtyAction::SendKey("t".into()));
    s.add_action(PtyAction::WaitMs(200));
    s.add_action(PtyAction::SendKey("Space".into()));
    s.add_action(PtyAction::SendKey("t".into()));
    s.add_action(PtyAction::Quit);
    s
}

pub(crate) fn gg_motion_scenario() -> PtyScenario {
    let mut s = PtyScenario::new("gg_motion");
    s.add_action(PtyAction::WriteFile("multi\nline\nfile\ncontent".into()));
    s.add_action(PtyAction::SendKey("G".into()));
    s.add_action(PtyAction::WaitMs(50));
    s.add_action(PtyAction::SendKey("g".into()));
    s.add_action(PtyAction::SendKey("g".into()));
    s.add_action(PtyAction::Quit);
    s
}

pub(crate) fn undo_redo_scenario() -> PtyScenario {
    let mut s = PtyScenario::new("undo_redo");
    s.add_action(PtyAction::SendKey("i".into()));
    s.add_action(PtyAction::TypeText("hello".into()));
    s.add_action(PtyAction::SendKey("Escape".into()));
    s.add_action(PtyAction::SendKey("u".into()));
    s.add_action(PtyAction::WaitMs(50));
    s.add_action(PtyAction::SendKey("Ctrl-r".into()));
    s.add_action(PtyAction::Quit);
    s
}

pub(crate) fn append_eol_scenario() -> PtyScenario {
    let mut s = PtyScenario::new("append_eol");
    s.add_action(PtyAction::WriteFile("hello world".into()));
    s.add_action(PtyAction::SendKey("$".into()));
    s.add_action(PtyAction::SendKey("a".into()));
    s.add_action(PtyAction::TypeText("!".into()));
    s.add_action(PtyAction::SendKey("Escape".into()));
    s.add_action(PtyAction::Quit);
    s
}

pub(crate) fn multi_key_sequence_scenario() -> PtyScenario {
    let mut s = PtyScenario::new("multi_key_sequence");
    s.add_action(PtyAction::WriteFile("first\nsecond\nthird\nfourth".into()));
    s.add_action(PtyAction::SendKey("G".into()));
    s.add_action(PtyAction::SendKey("g".into()));
    s.add_action(PtyAction::SendKey("g".into()));
    s.add_action(PtyAction::WaitMs(50));
    s.add_action(PtyAction::Quit);
    s
}

pub(crate) fn all_regression_scenarios() -> Vec<PtyScenario> {
    vec![
        insert_newline_scenario(),
        leader_explorer_scenario(),
        leader_terminal_scenario(),
        gg_motion_scenario(),
        undo_redo_scenario(),
        append_eol_scenario(),
        multi_key_sequence_scenario(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_newline_valid() {
        assert!(validate_scenario(&insert_newline_scenario()).is_ok());
    }

    #[test]
    fn leader_explorer_valid() {
        assert!(validate_scenario(&leader_explorer_scenario()).is_ok());
    }

    #[test]
    fn leader_terminal_valid() {
        assert!(validate_scenario(&leader_terminal_scenario()).is_ok());
    }

    #[test]
    fn gg_valid() {
        assert!(validate_scenario(&gg_motion_scenario()).is_ok());
    }

    #[test]
    fn undo_redo_valid() {
        assert!(validate_scenario(&undo_redo_scenario()).is_ok());
    }

    #[test]
    fn append_eol_valid() {
        assert!(validate_scenario(&append_eol_scenario()).is_ok());
    }

    #[test]
    fn all_scenarios_count() {
        assert_eq!(all_regression_scenarios().len(), 7);
    }

    #[test]
    fn all_scenarios_validate() {
        for s in all_regression_scenarios() {
            assert!(validate_scenario(&s).is_ok(), "Failed: {}", s.name);
        }
    }
}
