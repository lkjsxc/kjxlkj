//! Insert mode handler.

use crate::handler::{ModeHandler, ModeResult};
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode, MotionIntent};

/// State for insert mode (for Ctrl-r register sequence).
#[derive(Debug, Clone, Default)]
enum InsertState {
    #[default]
    Normal,
    /// Waiting for register name after Ctrl-r.
    WaitingForRegister,
}

/// Insert mode handler.
pub struct InsertMode {
    state: InsertState,
}

impl InsertMode {
    /// Create a new insert mode handler.
    pub fn new() -> Self {
        Self {
            state: InsertState::Normal,
        }
    }
}

impl Default for InsertMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeHandler for InsertMode {
    fn handle_key(&mut self, key: &KeyEvent) -> ModeResult {
        // Handle waiting for register
        if matches!(self.state, InsertState::WaitingForRegister) {
            self.state = InsertState::Normal;
            return match &key.code {
                KeyCode::Char(c) => ModeResult::intent(Intent::InsertFromRegister(*c)),
                KeyCode::Escape => ModeResult::Consumed(vec![]),
                _ => ModeResult::Ignored,
            };
        }

        // Handle Ctrl combinations
        if key.modifiers.ctrl {
            return match &key.code {
                KeyCode::Char('h') | KeyCode::Char('H') => {
                    // Backspace
                    ModeResult::intent(Intent::InsertText("\x08".to_string()))
                }
                KeyCode::Char('w') | KeyCode::Char('W') => {
                    // Delete word before cursor
                    ModeResult::intent(Intent::Delete { linewise: false, count: 1, motion: None })
                }
                KeyCode::Char('u') | KeyCode::Char('U') => {
                    // Delete to line start
                    ModeResult::intent(Intent::Delete { linewise: false, count: 1, motion: None })
                }
                KeyCode::Char('t') | KeyCode::Char('T') => {
                    // Indent
                    ModeResult::intent(Intent::Indent)
                }
                KeyCode::Char('d') | KeyCode::Char('D') => {
                    // Outdent
                    ModeResult::intent(Intent::Outdent)
                }
                KeyCode::Char('j') | KeyCode::Char('m') => {
                    // Newline
                    ModeResult::intent(Intent::InsertText("\n".to_string()))
                }
                KeyCode::Char('o') | KeyCode::Char('O') => {
                    // Execute one normal command
                    ModeResult::intent(Intent::SwitchMode(Mode::Normal))
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    // Insert from register - wait for register name
                    self.state = InsertState::WaitingForRegister;
                    ModeResult::Consumed(vec![])
                }
                _ => ModeResult::Ignored,
            };
        }

        match &key.code {
            KeyCode::Escape => ModeResult::intent(Intent::SwitchMode(Mode::Normal)),
            KeyCode::Enter => ModeResult::intent(Intent::InsertText("\n".to_string())),
            KeyCode::Backspace => ModeResult::intent(Intent::InsertText("\x08".to_string())),
            KeyCode::Tab => ModeResult::intent(Intent::InsertText("\t".to_string())),
            KeyCode::Char(c) => ModeResult::intent(Intent::InsertText(c.to_string())),
            KeyCode::Left => ModeResult::intent(Intent::Motion(MotionIntent::Left)),
            KeyCode::Right => ModeResult::intent(Intent::Motion(MotionIntent::Right)),
            KeyCode::Up => ModeResult::intent(Intent::Motion(MotionIntent::Up)),
            KeyCode::Down => ModeResult::intent(Intent::Motion(MotionIntent::Down)),
            KeyCode::Home => ModeResult::intent(Intent::Motion(MotionIntent::LineStart)),
            KeyCode::End => ModeResult::intent(Intent::Motion(MotionIntent::LineEnd)),
            KeyCode::Delete => ModeResult::intent(Intent::Delete { linewise: false, count: 1, motion: None }),
            _ => ModeResult::Ignored,
        }
    }

    fn mode(&self) -> Mode {
        Mode::Insert
    }

    fn reset(&mut self) {
        self.state = InsertState::Normal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_mode_char() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::char('a'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertText(_))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_escape() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Escape));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Normal))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_backspace() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Backspace));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertText(_))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_enter() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Enter));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertText(_))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_tab() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Tab));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertText(_))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_arrow_keys() {
        let mut mode = InsertMode::new();
        
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Left));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(MotionIntent::Left))));
        }
        
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Right));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(MotionIntent::Right))));
        }
        
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Up));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(MotionIntent::Up))));
        }
        
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Down));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(MotionIntent::Down))));
        }
    }

    #[test]
    fn test_insert_mode_home_end() {
        let mut mode = InsertMode::new();
        
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Home));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(MotionIntent::LineStart))));
        }
        
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::End));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Motion(MotionIntent::LineEnd))));
        }
    }

    #[test]
    fn test_insert_mode_default() {
        let mode = InsertMode::default();
        assert_eq!(mode.mode(), Mode::Insert);
    }

    #[test]
    fn test_insert_mode_mode() {
        let mode = InsertMode::new();
        assert_eq!(mode.mode(), Mode::Insert);
    }

    #[test]
    fn test_insert_mode_delete() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Delete));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Delete { .. })));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_ctrl_w() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::ctrl(KeyCode::Char('w')));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Delete { .. })));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_ctrl_u() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::ctrl(KeyCode::Char('u')));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Delete { .. })));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_ctrl_t() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::ctrl(KeyCode::Char('t')));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Indent)));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_ctrl_d() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::ctrl(KeyCode::Char('d')));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::Outdent)));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_ctrl_o() {
        let mut mode = InsertMode::new();
        let result = mode.handle_key(&KeyEvent::ctrl(KeyCode::Char('o')));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Normal))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_insert_mode_ctrl_r() {
        let mut mode = InsertMode::new();
        // Ctrl-r should enter waiting state
        let result = mode.handle_key(&KeyEvent::ctrl(KeyCode::Char('r')));
        assert!(matches!(result, ModeResult::Consumed(_)));
        
        // Next key should be the register name
        let result = mode.handle_key(&KeyEvent::char('a'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertFromRegister('a'))));
        } else {
            panic!("Expected consumed with InsertFromRegister");
        }
    }

    #[test]
    fn test_insert_mode_ctrl_r_escape_cancels() {
        let mut mode = InsertMode::new();
        mode.handle_key(&KeyEvent::ctrl(KeyCode::Char('r')));
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Escape));
        // Should cancel and return empty intents
        assert!(matches!(result, ModeResult::Consumed(_)));
    }

    #[test]
    fn test_insert_mode_reset() {
        let mut mode = InsertMode::new();
        mode.handle_key(&KeyEvent::ctrl(KeyCode::Char('r')));
        mode.reset();
        // After reset, should be back to normal state
        let result = mode.handle_key(&KeyEvent::char('a'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::InsertText(_))));
        } else {
            panic!("Expected consumed with InsertText");
        }
    }
}

