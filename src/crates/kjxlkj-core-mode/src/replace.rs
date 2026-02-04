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
}
