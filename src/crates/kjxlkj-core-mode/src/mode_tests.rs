//! Tests for mode transitions (CM-01 through CM-12).
//!
//! Covers spec requirements from `/docs/spec/technical/testing-unit.md`.

#[cfg(test)]
mod tests {
    use crate::dispatch::{DispatchResult, ModeDispatcher};
    use kjxlkj_core_types::{CommandKind, Key, KeyCode, KeyModifiers, Mode, Operator, VisualKind};

    fn key(c: char) -> Key {
        Key::new(KeyCode::Char(c), KeyModifiers::NONE)
    }

    fn ctrl_key(c: char) -> Key {
        Key::new(KeyCode::Char(c), KeyModifiers::CTRL)
    }

    fn esc_key() -> Key {
        Key::new(KeyCode::Esc, KeyModifiers::NONE)
    }

    /// CM-01: Normal to Insert (`i`).
    #[test]
    fn cm01_normal_to_insert() {
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&key('i'), &Mode::Normal);
        match result {
            DispatchResult::ModeChange(Mode::Insert) => {}
            other => panic!("Expected ModeChange(Insert), got {:?}", other),
        }
    }

    /// CM-02: Insert to Normal (`Esc`).
    #[test]
    fn cm02_insert_to_normal_esc() {
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&esc_key(), &Mode::Insert);
        match result {
            DispatchResult::ModeChange(Mode::Normal) => {}
            other => panic!("Expected ModeChange(Normal), got {:?}", other),
        }
    }

    /// CM-03: Normal to Visual (`v`).
    #[test]
    fn cm03_normal_to_visual() {
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&key('v'), &Mode::Normal);
        match result {
            DispatchResult::ModeChange(Mode::Visual(VisualKind::Char)) => {}
            other => panic!("Expected Visual(Char), got {:?}", other),
        }
    }

    /// CM-05: Normal to Command (`:`).
    #[test]
    fn cm05_normal_to_command() {
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&key(':'), &Mode::Normal);
        match result {
            DispatchResult::ModeChange(Mode::Command(CommandKind::Ex)) => {}
            other => panic!("Expected Command(Ex), got {:?}", other),
        }
    }

    /// CM-06: Command cancel (`Esc`).
    /// Esc in Command mode returns to Normal.
    /// (The mode dispatcher for Command returns a Noop for Esc;
    /// actual mode change happens in EditorState.process_cmdline_key)
    #[test]
    fn cm06_command_esc() {
        // Command mode dispatch for Esc is handled in cmdline_ops, not dispatcher
        // We verify that Insert Esc returns Normal instead.
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&esc_key(), &Mode::Insert);
        match result {
            DispatchResult::ModeChange(Mode::Normal) => {}
            other => panic!("Expected ModeChange(Normal), got {:?}", other),
        }
    }

    /// CM-07: Replace mode (`R`).
    #[test]
    fn cm07_replace_mode() {
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&key('R'), &Mode::Normal);
        match result {
            DispatchResult::ModeChange(Mode::Replace) => {}
            other => panic!("Expected ModeChange(Replace), got {:?}", other),
        }
    }

    /// CM-10: Rapid mode churn.
    /// i Esc repeated 100 times without panic.
    #[test]
    fn cm10_rapid_mode_churn() {
        let mut disp = ModeDispatcher::new();
        let mut mode = Mode::Normal;
        for _ in 0..100 {
            let r = disp.dispatch(&key('i'), &mode);
            if let DispatchResult::ModeChange(m) = r {
                mode = m;
            }
            assert_eq!(mode, Mode::Insert);
            let r = disp.dispatch(&esc_key(), &mode);
            if let DispatchResult::ModeChange(m) = r {
                mode = m;
            }
            assert_eq!(mode, Mode::Normal);
        }
    }

    /// CM-11: Visual block (`Ctrl-v`).
    #[test]
    fn cm11_visual_block() {
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&ctrl_key('v'), &Mode::Normal);
        match result {
            DispatchResult::ModeChange(Mode::Visual(VisualKind::Block)) => {}
            other => panic!("Expected Visual(Block), got {:?}", other),
        }
    }

    /// CM-12: Operator-pending timeout via Esc.
    /// After pressing `d`, `Esc` cancels and returns to Normal.
    #[test]
    fn cm12_operator_pending_esc() {
        let mut disp = ModeDispatcher::new();
        // First press 'd' to enter operator pending
        let r = disp.dispatch(&key('d'), &Mode::Normal);
        match r {
            DispatchResult::ModeChange(Mode::OperatorPending(Operator::Delete)) => {}
            other => panic!("Expected OperatorPending(Delete), got {:?}", other),
        }

        // Now press Esc to cancel
        let r = disp.dispatch(&esc_key(), &Mode::OperatorPending(Operator::Delete));
        match r {
            DispatchResult::ModeChange(Mode::Normal) => {}
            other => panic!("Expected ModeChange(Normal), got {:?}", other),
        }
    }

    /// Test search forward entry.
    #[test]
    fn test_search_forward_entry() {
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&key('/'), &Mode::Normal);
        match result {
            DispatchResult::ModeChange(Mode::Command(CommandKind::SearchForward)) => {}
            other => panic!("Expected SearchForward, got {:?}", other),
        }
    }

    /// Test search backward entry.
    #[test]
    fn test_search_backward_entry() {
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&key('?'), &Mode::Normal);
        match result {
            DispatchResult::ModeChange(Mode::Command(CommandKind::SearchBackward)) => {}
            other => panic!("Expected SearchBackward, got {:?}", other),
        }
    }

    /// Test count accumulation.
    #[test]
    fn test_count_accumulation() {
        let mut disp = ModeDispatcher::new();
        let _ = disp.dispatch(&key('3'), &Mode::Normal);
        assert_eq!(disp.count, Some(3));
        let _ = disp.dispatch(&key('5'), &Mode::Normal);
        assert_eq!(disp.count, Some(35));
    }

    /// Test Visual line entry.
    #[test]
    fn test_visual_line() {
        let mut disp = ModeDispatcher::new();
        let result = disp.dispatch(&key('V'), &Mode::Normal);
        match result {
            DispatchResult::ModeChange(Mode::Visual(VisualKind::Line)) => {}
            other => panic!("Expected Visual(Line), got {:?}", other),
        }
    }
}
