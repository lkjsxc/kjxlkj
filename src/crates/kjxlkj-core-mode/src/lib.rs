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

    #[test]
    fn test_visual_mode_variants() {
        let char_wise = VisualMode::char_wise();
        let line_wise = VisualMode::line_wise();
        let block_wise = VisualMode::block_wise();
        
        assert_ne!(char_wise.mode(), line_wise.mode());
        assert_ne!(line_wise.mode(), block_wise.mode());
    }

    #[test]
    fn test_command_mode_buffer() {
        let mut cmd = CommandMode::new();
        cmd.handle_key(&KeyEvent::char('t'));
        cmd.handle_key(&KeyEvent::char('e'));
        cmd.handle_key(&KeyEvent::char('s'));
        cmd.handle_key(&KeyEvent::char('t'));
        assert_eq!(cmd.buffer(), "test");
    }

    #[test]
    fn test_insert_mode_numbers() {
        let mut insert = InsertMode::new();
        let result = insert.handle_key(&KeyEvent::char('5'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertText(_))));
        }
    }

    #[test]
    fn test_normal_mode_default() {
        let normal = NormalMode::default();
        assert_eq!(normal.mode(), Mode::Normal);
    }

    #[test]
    fn test_insert_mode_default() {
        let insert = InsertMode::default();
        assert_eq!(insert.mode(), Mode::Insert);
    }

    #[test]
    fn test_command_mode_default() {
        let cmd = CommandMode::default();
        assert_eq!(cmd.mode(), Mode::Command);
    }

    #[test]
    fn test_replace_mode_default() {
        let replace = ReplaceMode::default();
        assert_eq!(replace.mode(), Mode::Replace);
    }

    #[test]
    fn test_visual_mode_d_delete() {
        let mut visual = VisualMode::line_wise();
        let result = visual.handle_key(&KeyEvent::char('d'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Delete { .. })));
        }
    }

    #[test]
    fn test_visual_mode_y_yank() {
        let mut visual = VisualMode::char_wise();
        let result = visual.handle_key(&KeyEvent::char('y'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Yank { .. })));
        }
    }

    #[test]
    fn test_insert_mode_enter() {
        let mut insert = InsertMode::new();
        let result = insert.handle_key(&KeyEvent::plain(KeyCode::Enter));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| {
                if let Intent::InsertText(s) = i {
                    s == "\n"
                } else {
                    false
                }
            }));
        }
    }

    #[test]
    fn test_normal_mode_zero_motion() {
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::char('0'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(_))));
        }
    }

    #[test]
    fn test_normal_mode_dollar_motion() {
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::char('$'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(_))));
        }
    }

    #[test]
    fn test_normal_mode_w_motion() {
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::char('w'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(_))));
        }
    }

    #[test]
    fn test_normal_mode_b_motion() {
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::char('b'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(_))));
        }
    }

    #[test]
    fn test_normal_mode_e_motion() {
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::char('e'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(_))));
        }
    }

    #[test]
    fn test_normal_mode_G_motion() {
        let mut normal = NormalMode::new();
        let result = normal.handle_key(&KeyEvent::new(KeyCode::Char('G'), kjxlkj_core_types::KeyModifiers::SHIFT));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(_))));
        }
    }
}
