//! Modal editing state machines.
//!
//! This crate handles mode-specific key interpretation and intent generation.

mod command;
mod handler;
mod insert;
mod normal;
pub mod parser;
mod replace;
mod visual;

pub use command::CommandMode;
pub use handler::{ModeHandler, ModeResult};
pub use insert::InsertMode;
pub use normal::NormalMode;
pub use parser::{ParseResult, Parser};
pub use replace::ReplaceMode;
pub use visual::VisualMode;

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode};

    /// Tests for mode scoping - the same key does different things in different modes.
    #[test]
    fn test_mode_scoping_escape_key() {
        // In Insert mode, Escape goes to Normal
        let mut insert = InsertMode::new();
        let result = insert.handle_key(&KeyEvent::plain(KeyCode::Escape));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Normal))));
        }

        // In Visual mode, Escape also goes to Normal
        let mut visual = VisualMode::char_wise();
        let result = visual.handle_key(&KeyEvent::plain(KeyCode::Escape));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Normal))));
        }

        // In Command mode, Escape also goes to Normal
        let mut command = CommandMode::new();
        let result = command.handle_key(&KeyEvent::plain(KeyCode::Escape));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Normal))));
        }
    }

    #[test]
    fn test_mode_scoping_j_key() {
        // In Normal mode, 'j' moves down
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::char('j'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(_))));
        }

        // In Insert mode, 'j' inserts the letter j
        let mut insert = InsertMode::new();
        let result = insert.handle_key(&KeyEvent::char('j'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertText(_))));
        }

        // In Command mode, 'j' goes into the command buffer
        let mut command = CommandMode::new();
        command.handle_key(&KeyEvent::char('j'));
        assert_eq!(command.buffer(), "j");
    }

    #[test]
    fn test_mode_scoping_d_key() {
        // In Normal mode, 'd' starts a pending delete
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::char('d'));
        assert!(matches!(result, ModeResult::Pending));

        // In Insert mode, 'd' inserts the letter d
        let mut insert = InsertMode::new();
        let result = insert.handle_key(&KeyEvent::char('d'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertText(_))));
        }

        // In Visual mode, 'd' deletes selection
        let mut visual = VisualMode::char_wise();
        let result = visual.handle_key(&KeyEvent::char('d'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Delete { .. })));
        }
    }

    #[test]
    fn test_mode_scoping_colon_key() {
        // In Normal mode, ':' enters command mode
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::char(':'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Command))));
        }

        // In Insert mode, ':' inserts the colon
        let mut insert = InsertMode::new();
        let result = insert.handle_key(&KeyEvent::char(':'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertText(_))));
        }
    }

    /// Tests for mapping precedence - mode handlers correctly report their mode
    #[test]
    fn test_mode_precedence_identification() {
        let normal = NormalMode::new();
        assert_eq!(normal.mode(), Mode::Normal);

        let insert = InsertMode::new();
        assert_eq!(insert.mode(), Mode::Insert);

        let visual = VisualMode::char_wise();
        assert_eq!(visual.mode(), Mode::Visual);

        let visual_line = VisualMode::line_wise();
        assert_eq!(visual_line.mode(), Mode::VisualLine);

        let visual_block = VisualMode::block_wise();
        assert_eq!(visual_block.mode(), Mode::VisualBlock);

        let command = CommandMode::new();
        assert_eq!(command.mode(), Mode::Command);

        let replace = ReplaceMode::new();
        assert_eq!(replace.mode(), Mode::Replace);
    }

    /// Tests for recursion safety - mode transitions are well-defined
    #[test]
    fn test_mode_transition_safety() {
        // Normal -> Insert -> Normal cycle
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::char('i'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Insert))));
        }

        let mut insert = InsertMode::new();
        let result = insert.handle_key(&KeyEvent::plain(KeyCode::Escape));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Normal))));
        }
    }

    /// Tests for timing determinism - same input always produces same output
    #[test]
    fn test_timing_determinism() {
        for _ in 0..10 {
            let mut normal = NormalMode::new();
            let result1 = normal.handle_key(&KeyEvent::char('j'));
            
            let mut normal2 = NormalMode::new();
            let result2 = normal2.handle_key(&KeyEvent::char('j'));
            
            // Both should be Consumed
            assert!(matches!(result1, ModeResult::Consumed(_)));
            assert!(matches!(result2, ModeResult::Consumed(_)));
        }
    }

    #[test]
    fn test_mode_reset_preserves_mode() {
        let mut normal = NormalMode::new();
        normal.handle_key(&KeyEvent::char('d'));
        normal.reset();
        assert_eq!(normal.mode(), Mode::Normal);

        let mut insert = InsertMode::new();
        insert.reset();
        assert_eq!(insert.mode(), Mode::Insert);

        let mut visual = VisualMode::line_wise();
        visual.reset();
        assert_eq!(visual.mode(), Mode::VisualLine);
    }
}
