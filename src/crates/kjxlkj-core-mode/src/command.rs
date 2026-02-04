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
}
