//! Visual mode handler.

use crate::handler::{ModeHandler, ModeResult};
use crate::parser::{ActionKind, OperatorKind, ParseResult, Parser};
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode, MotionIntent};

/// Visual mode kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualKind {
    Char,
    Line,
    Block,
}

/// Visual mode handler.
pub struct VisualMode {
    kind: VisualKind,
    parser: Parser,
}

impl VisualMode {
    /// Create a new visual mode handler.
    pub fn new(kind: VisualKind) -> Self {
        Self {
            kind,
            parser: Parser::new(),
        }
    }

    /// Create char-wise visual mode.
    pub fn char_wise() -> Self {
        Self::new(VisualKind::Char)
    }

    /// Create line-wise visual mode.
    pub fn line_wise() -> Self {
        Self::new(VisualKind::Line)
    }

    /// Create block-wise visual mode.
    pub fn block_wise() -> Self {
        Self::new(VisualKind::Block)
    }
}

impl ModeHandler for VisualMode {
    fn handle_key(&mut self, key: &KeyEvent) -> ModeResult {
        // Check for escape first
        if matches!(key.code, KeyCode::Escape) {
            return ModeResult::intent(Intent::SwitchMode(Mode::Normal));
        }

        // Check for operators that end visual mode
        match &key.code {
            KeyCode::Char('d') | KeyCode::Char('x') => {
                let linewise = self.kind == VisualKind::Line;
                return ModeResult::intents(vec![
                    Intent::Delete { linewise, count: 1, motion: None },
                    Intent::SwitchMode(Mode::Normal),
                ]);
            }
            KeyCode::Char('y') => {
                let linewise = self.kind == VisualKind::Line;
                return ModeResult::intents(vec![
                    Intent::Yank { linewise, count: 1, motion: None },
                    Intent::SwitchMode(Mode::Normal),
                ]);
            }
            KeyCode::Char('c') | KeyCode::Char('s') => {
                let linewise = self.kind == VisualKind::Line;
                return ModeResult::intents(vec![
                    Intent::Change { linewise, count: 1, motion: None },
                    Intent::SwitchMode(Mode::Insert),
                ]);
            }
            KeyCode::Char('o') => {
                // Swap ends of selection
                return ModeResult::nop(); // Handle in core
            }
            KeyCode::Char('>') => {
                return ModeResult::intents(vec![
                    Intent::Indent,
                    Intent::SwitchMode(Mode::Normal),
                ]);
            }
            KeyCode::Char('<') => {
                return ModeResult::intents(vec![
                    Intent::Outdent,
                    Intent::SwitchMode(Mode::Normal),
                ]);
            }
            KeyCode::Char('~') => {
                return ModeResult::intents(vec![
                    Intent::ToggleCase,
                    Intent::SwitchMode(Mode::Normal),
                ]);
            }
            KeyCode::Char('U') => {
                return ModeResult::intents(vec![
                    Intent::Uppercase,
                    Intent::SwitchMode(Mode::Normal),
                ]);
            }
            KeyCode::Char('u') => {
                return ModeResult::intents(vec![
                    Intent::Lowercase,
                    Intent::SwitchMode(Mode::Normal),
                ]);
            }
            _ => {}
        }

        // Handle motions (extend selection)
        match self.parser.parse(key) {
            ParseResult::Complete(cmd) => {
                if let ActionKind::Motion(motion) = cmd.action {
                    let mut intents = Vec::new();
                    for _ in 0..cmd.count {
                        intents.push(Intent::Motion(motion.clone()));
                    }
                    ModeResult::intents(intents)
                } else {
                    ModeResult::Ignored
                }
            }
            ParseResult::Pending => ModeResult::Pending,
            ParseResult::Invalid => ModeResult::Ignored,
        }
    }

    fn mode(&self) -> Mode {
        match self.kind {
            VisualKind::Char => Mode::Visual,
            VisualKind::Line => Mode::VisualLine,
            VisualKind::Block => Mode::VisualBlock,
        }
    }

    fn reset(&mut self) {
        self.parser.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_mode_escape() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Escape));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Normal))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_delete() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('d'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Delete { .. })));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_yank() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('y'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Yank { .. })));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_change() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('c'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Change { .. })));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_char_wise() {
        let mode = VisualMode::char_wise();
        assert_eq!(mode.mode(), Mode::Visual);
    }

    #[test]
    fn test_visual_mode_line_wise() {
        let mode = VisualMode::line_wise();
        assert_eq!(mode.mode(), Mode::VisualLine);
    }

    #[test]
    fn test_visual_mode_block_wise() {
        let mode = VisualMode::block_wise();
        assert_eq!(mode.mode(), Mode::VisualBlock);
    }

    #[test]
    fn test_visual_mode_reset() {
        let mut mode = VisualMode::char_wise();
        mode.handle_key(&KeyEvent::char('d'));
        mode.reset();
        // After reset, state should be clean
        assert_eq!(mode.mode(), Mode::Visual);
    }

    #[test]
    fn test_visual_mode_uppercase() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('U'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Uppercase)));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_lowercase() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('u'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Lowercase)));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_toggle_case() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('~'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::ToggleCase)));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_indent() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('>'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Indent)));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_outdent() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('<'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Outdent)));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_x_delete() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('x'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Delete { .. })));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_mode_s_change() {
        let mut mode = VisualMode::char_wise();
        let result = mode.handle_key(&KeyEvent::char('s'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Change { .. })));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_line_delete_linewise() {
        let mut mode = VisualMode::line_wise();
        let result = mode.handle_key(&KeyEvent::char('d'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Delete { linewise: true, .. })));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_visual_kind_equality() {
        assert_eq!(VisualKind::Char, VisualKind::Char);
        assert_ne!(VisualKind::Char, VisualKind::Line);
        assert_ne!(VisualKind::Line, VisualKind::Block);
    }

    #[test]
    fn test_visual_kind_debug() {
        let kind = VisualKind::Block;
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Block"));
    }

    #[test]
    fn test_visual_kind_clone() {
        let kind = VisualKind::Line;
        let cloned = kind.clone();
        assert_eq!(kind, cloned);
    }
}
