//! Replace mode handler.

use crate::handler::{ModeHandler, ModeResult};
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode, MotionIntent};

/// Replace mode handler.
pub struct ReplaceMode;

impl ReplaceMode {
    /// Create a new replace mode handler.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReplaceMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeHandler for ReplaceMode {
    fn handle_key(&mut self, key: &KeyEvent) -> ModeResult {
        match &key.code {
            KeyCode::Escape => ModeResult::intent(Intent::SwitchMode(Mode::Normal)),
            KeyCode::Backspace => ModeResult::intent(Intent::Motion(MotionIntent::Left)),
            KeyCode::Char(c) => {
                // Replace character at cursor and move right
                ModeResult::intents(vec![
                    Intent::ReplaceChar(*c),
                    Intent::Motion(MotionIntent::Right),
                ])
            }
            KeyCode::Left => ModeResult::intent(Intent::Motion(MotionIntent::Left)),
            KeyCode::Right => ModeResult::intent(Intent::Motion(MotionIntent::Right)),
            KeyCode::Up => ModeResult::intent(Intent::Motion(MotionIntent::Up)),
            KeyCode::Down => ModeResult::intent(Intent::Motion(MotionIntent::Down)),
            _ => ModeResult::Ignored,
        }
    }

    fn mode(&self) -> Mode {
        Mode::Replace
    }

    fn reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_mode_char() {
        let mut mode = ReplaceMode::new();
        let result = mode.handle_key(&KeyEvent::char('x'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::ReplaceChar('x'))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_replace_mode_escape() {
        let mut mode = ReplaceMode::new();
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Escape));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Normal))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_replace_mode_mode() {
        let mode = ReplaceMode::new();
        assert_eq!(mode.mode(), Mode::Replace);
    }

    #[test]
    fn test_replace_mode_default() {
        let mode = ReplaceMode::default();
        assert_eq!(mode.mode(), Mode::Replace);
    }

    #[test]
    fn test_replace_mode_multiple_chars() {
        let mut mode = ReplaceMode::new();
        mode.handle_key(&KeyEvent::char('a'));
        mode.handle_key(&KeyEvent::char('b'));
        // Each char should be replaced independently
        let result = mode.handle_key(&KeyEvent::char('c'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::ReplaceChar('c'))));
        }
    }

    #[test]
    fn test_replace_mode_backspace() {
        let mut mode = ReplaceMode::new();
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Backspace));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(MotionIntent::Left))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_replace_mode_arrows() {
        let mut mode = ReplaceMode::new();
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Left));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(MotionIntent::Left))));
        }
        
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Right));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(MotionIntent::Right))));
        }
    }

    #[test]
    fn test_replace_mode_reset() {
        let mut mode = ReplaceMode::new();
        mode.handle_key(&KeyEvent::char('a'));
        mode.reset();
        // After reset, should still be in replace mode
        assert_eq!(mode.mode(), Mode::Replace);
    }

    #[test]
    fn test_replace_mode_special_chars() {
        let mut mode = ReplaceMode::new();
        let result = mode.handle_key(&KeyEvent::char('!'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::ReplaceChar('!'))));
        }
    }

    #[test]
    fn test_replace_mode_space() {
        let mut mode = ReplaceMode::new();
        let result = mode.handle_key(&KeyEvent::char(' '));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::ReplaceChar(' '))));
        }
    }
}

