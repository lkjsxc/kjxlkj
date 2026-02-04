//! Command mode handler.

use crate::handler::{ModeHandler, ModeResult};
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode};

/// Command mode handler (Ex commands).
pub struct CommandMode {
    /// Current command line content.
    buffer: String,
    /// Cursor position in the command line.
    cursor: usize,
}

impl CommandMode {
    /// Create a new command mode handler.
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            cursor: 0,
        }
    }

    /// Get the current command line content.
    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    /// Get the cursor position.
    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

impl Default for CommandMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeHandler for CommandMode {
    fn handle_key(&mut self, key: &KeyEvent) -> ModeResult {
        match &key.code {
            KeyCode::Escape => {
                self.buffer.clear();
                self.cursor = 0;
                ModeResult::intent(Intent::SwitchMode(Mode::Normal))
            }
            KeyCode::Enter => {
                let cmd = std::mem::take(&mut self.buffer);
                self.cursor = 0;
                ModeResult::intents(vec![
                    Intent::ExecuteCommand(cmd),
                    Intent::SwitchMode(Mode::Normal),
                ])
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.buffer.remove(self.cursor);
                }
                ModeResult::nop()
            }
            KeyCode::Char(c) => {
                self.buffer.insert(self.cursor, *c);
                self.cursor += 1;
                ModeResult::nop()
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                ModeResult::nop()
            }
            KeyCode::Right => {
                if self.cursor < self.buffer.len() {
                    self.cursor += 1;
                }
                ModeResult::nop()
            }
            KeyCode::Home => {
                self.cursor = 0;
                ModeResult::nop()
            }
            KeyCode::End => {
                self.cursor = self.buffer.len();
                ModeResult::nop()
            }
            _ => ModeResult::Ignored,
        }
    }

    fn mode(&self) -> Mode {
        Mode::Command
    }

    fn reset(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_mode_input() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        mode.handle_key(&KeyEvent::char('q'));
        assert_eq!(mode.buffer(), "wq");
    }

    #[test]
    fn test_command_mode_enter() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('q'));
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Enter));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::ExecuteCommand(_))));
        } else {
            panic!("Expected consumed");
        }
    }

    #[test]
    fn test_command_mode_escape() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        let result = mode.handle_key(&KeyEvent::plain(KeyCode::Escape));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Normal))));
        } else {
            panic!("Expected consumed");
        }
        assert!(mode.buffer().is_empty());
    }

    #[test]
    fn test_command_mode_backspace() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('a'));
        mode.handle_key(&KeyEvent::char('b'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Backspace));
        assert_eq!(mode.buffer(), "a");
    }

    #[test]
    fn test_command_mode_left_right() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('a'));
        mode.handle_key(&KeyEvent::char('b'));
        assert_eq!(mode.cursor(), 2);
        mode.handle_key(&KeyEvent::plain(KeyCode::Left));
        assert_eq!(mode.cursor(), 1);
        mode.handle_key(&KeyEvent::plain(KeyCode::Right));
        assert_eq!(mode.cursor(), 2);
    }

    #[test]
    fn test_command_mode_home_end() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('a'));
        mode.handle_key(&KeyEvent::char('b'));
        mode.handle_key(&KeyEvent::char('c'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Home));
        assert_eq!(mode.cursor(), 0);
        mode.handle_key(&KeyEvent::plain(KeyCode::End));
        assert_eq!(mode.cursor(), 3);
    }

    #[test]
    fn test_command_mode_reset() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('a'));
        mode.reset();
        assert!(mode.buffer().is_empty());
        assert_eq!(mode.cursor(), 0);
    }

    #[test]
    fn test_command_mode_mode() {
        let mode = CommandMode::new();
        assert_eq!(mode.mode(), Mode::Command);
    }

    #[test]
    fn test_command_mode_default() {
        let mode = CommandMode::default();
        assert!(mode.buffer().is_empty());
    }

    #[test]
    fn test_command_mode_left_at_start() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::plain(KeyCode::Left));
        assert_eq!(mode.cursor(), 0);
    }

    #[test]
    fn test_command_mode_right_at_end() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('a'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Right));
        assert_eq!(mode.cursor(), 1);
    }

    #[test]
    fn test_command_mode_backspace_at_start() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::plain(KeyCode::Backspace));
        assert!(mode.buffer().is_empty());
    }
}
