//! Command mode handler.

use crate::completion::{
    CommandCompletionProvider, CompletionProvider, CompletionState, FileCompletionProvider,
};
use crate::handler::{ModeHandler, ModeResult};
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode};

/// Command mode handler (Ex commands).
pub struct CommandMode {
    /// Current command line content.
    buffer: String,
    /// Cursor position in the command line.
    cursor: usize,
    /// Completion state (if active).
    completion: Option<CompletionState>,
    /// Command completion provider.
    command_provider: CommandCompletionProvider,
    /// File completion provider.
    file_provider: FileCompletionProvider,
}

impl CommandMode {
    /// Create a new command mode handler.
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            cursor: 0,
            completion: None,
            command_provider: CommandCompletionProvider::new(),
            file_provider: FileCompletionProvider::new(),
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

    /// Get the current completion state.
    pub fn completion(&self) -> Option<&CompletionState> {
        self.completion.as_ref()
    }

    /// Start completion based on current buffer content.
    fn start_completion(&mut self) {
        let prefix = &self.buffer[..self.cursor];

        // Determine what kind of completion to use
        let items = if prefix.contains(' ') {
            // After a space, complete files
            let parts: Vec<&str> = prefix.split_whitespace().collect();
            let file_prefix = parts.last().copied().unwrap_or("");
            self.file_provider.complete(file_prefix)
        } else {
            // At the start, complete commands
            self.command_provider.complete(prefix)
        };

        let mut state = CompletionState::new(self.buffer.clone(), 0);
        state.set_items(items);
        self.completion = Some(state);
    }

    /// Apply the current completion to the buffer.
    fn apply_completion(&mut self) {
        if let Some(ref state) = self.completion {
            if let Some(item) = state.current() {
                let prefix = &self.buffer[..self.cursor];

                // Find where to insert the completion
                let (prefix_start, _) = if prefix.contains(' ') {
                    let last_space = prefix.rfind(' ').unwrap_or(0);
                    (last_space + 1, &prefix[last_space + 1..])
                } else {
                    (0, prefix)
                };

                // Replace the prefix with the completion
                let after = self.buffer[self.cursor..].to_string();
                self.buffer = format!(
                    "{}{}{}",
                    &self.buffer[..prefix_start],
                    item.text,
                    after
                );
                self.cursor = prefix_start + item.text.len();
            }
        }
    }
}

impl Default for CommandMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeHandler for CommandMode {
    fn handle_key(&mut self, key: &KeyEvent) -> ModeResult {
        match (&key.code, &key.modifiers) {
            (KeyCode::Escape, _) => {
                // Cancel completion if active, otherwise exit mode
                if self.completion.is_some() {
                    if let Some(ref state) = self.completion {
                        self.buffer = state.original().to_string();
                    }
                    self.completion = None;
                    ModeResult::nop()
                } else {
                    self.buffer.clear();
                    self.cursor = 0;
                    ModeResult::intent(Intent::SwitchMode(Mode::Normal))
                }
            }
            (KeyCode::Enter, _) => {
                self.completion = None;
                let cmd = std::mem::take(&mut self.buffer);
                self.cursor = 0;
                ModeResult::intents(vec![
                    Intent::ExecuteCommand(cmd),
                    Intent::SwitchMode(Mode::Normal),
                ])
            }
            (KeyCode::Tab, modifiers) if modifiers.shift => {
                // Shift+Tab: previous completion
                if self.completion.is_none() {
                    self.start_completion();
                }
                if let Some(ref mut state) = self.completion {
                    state.move_prev();
                    self.apply_completion();
                }
                ModeResult::nop()
            }
            (KeyCode::Tab, _) => {
                // Tab: next completion
                if self.completion.is_none() {
                    self.start_completion();
                }
                if let Some(ref mut state) = self.completion {
                    state.move_next();
                    self.apply_completion();
                }
                ModeResult::nop()
            }
            (KeyCode::Char('n'), modifiers) if modifiers.ctrl => {
                // Ctrl+N: next completion
                if self.completion.is_none() {
                    self.start_completion();
                }
                if let Some(ref mut state) = self.completion {
                    state.move_next();
                    self.apply_completion();
                }
                ModeResult::nop()
            }
            (KeyCode::Char('p'), modifiers) if modifiers.ctrl => {
                // Ctrl+P: previous completion
                if self.completion.is_none() {
                    self.start_completion();
                }
                if let Some(ref mut state) = self.completion {
                    state.move_prev();
                    self.apply_completion();
                }
                ModeResult::nop()
            }
            (KeyCode::Char('y'), modifiers) if modifiers.ctrl => {
                // Ctrl+Y: accept completion
                if self.completion.is_some() {
                    self.apply_completion();
                    self.completion = None;
                }
                ModeResult::nop()
            }
            (KeyCode::Char('e'), modifiers) if modifiers.ctrl => {
                // Ctrl+E: cancel completion
                if let Some(ref state) = self.completion {
                    self.buffer = state.original().to_string();
                    self.cursor = self.buffer.len();
                }
                self.completion = None;
                ModeResult::nop()
            }
            (KeyCode::Backspace, _) => {
                self.completion = None;
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.buffer.remove(self.cursor);
                }
                ModeResult::nop()
            }
            (KeyCode::Char(c), _) => {
                self.completion = None;
                self.buffer.insert(self.cursor, *c);
                self.cursor += 1;
                ModeResult::nop()
            }
            (KeyCode::Left, _) => {
                self.completion = None;
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                ModeResult::nop()
            }
            (KeyCode::Right, _) => {
                self.completion = None;
                if self.cursor < self.buffer.len() {
                    self.cursor += 1;
                }
                ModeResult::nop()
            }
            (KeyCode::Home, _) => {
                self.completion = None;
                self.cursor = 0;
                ModeResult::nop()
            }
            (KeyCode::End, _) => {
                self.completion = None;
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
        self.completion = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::KeyModifiers;

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

    #[test]
    fn test_command_mode_multiple_chars() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        mode.handle_key(&KeyEvent::char('r'));
        mode.handle_key(&KeyEvent::char('i'));
        mode.handle_key(&KeyEvent::char('t'));
        mode.handle_key(&KeyEvent::char('e'));
        assert_eq!(mode.buffer(), "write");
    }

    #[test]
    fn test_command_mode_cursor_after_input() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('a'));
        mode.handle_key(&KeyEvent::char('b'));
        assert_eq!(mode.cursor(), 2);
    }

    #[test]
    fn test_command_mode_delete_middle() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('a'));
        mode.handle_key(&KeyEvent::char('b'));
        mode.handle_key(&KeyEvent::char('c'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Left));
        mode.handle_key(&KeyEvent::plain(KeyCode::Backspace));
        assert_eq!(mode.buffer(), "ac");
    }

    #[test]
    fn test_command_mode_navigation() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('h'));
        mode.handle_key(&KeyEvent::char('e'));
        mode.handle_key(&KeyEvent::char('l'));
        mode.handle_key(&KeyEvent::char('l'));
        mode.handle_key(&KeyEvent::char('o'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Home));
        mode.handle_key(&KeyEvent::plain(KeyCode::Right));
        mode.handle_key(&KeyEvent::plain(KeyCode::Right));
        assert_eq!(mode.cursor(), 2);
    }

    #[test]
    fn test_command_mode_space() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        mode.handle_key(&KeyEvent::char(' '));
        mode.handle_key(&KeyEvent::char('f'));
        assert_eq!(mode.buffer(), "w f");
    }

    #[test]
    fn test_command_mode_tab_completion() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Tab));

        // Should have completion active
        assert!(mode.completion().is_some());
        // Buffer should be updated to first match
        assert!(mode.buffer().starts_with('w'));
    }

    #[test]
    fn test_command_mode_tab_cycles() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));

        // First tab
        mode.handle_key(&KeyEvent::plain(KeyCode::Tab));
        let first = mode.buffer().to_string();

        // Second tab
        mode.handle_key(&KeyEvent::plain(KeyCode::Tab));
        let second = mode.buffer().to_string();

        // Should cycle to next completion (if multiple matches)
        assert!(first.starts_with('w'));
        assert!(second.starts_with('w'));
    }

    #[test]
    fn test_command_mode_ctrl_n_completion() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('q'));
        mode.handle_key(&KeyEvent::new(KeyCode::Char('n'), KeyModifiers::CTRL));

        assert!(mode.completion().is_some());
    }

    #[test]
    fn test_command_mode_ctrl_p_completion() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        mode.handle_key(&KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CTRL));

        assert!(mode.completion().is_some());
    }

    #[test]
    fn test_command_mode_escape_cancels_completion() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Tab));
        assert!(mode.completion().is_some());

        mode.handle_key(&KeyEvent::plain(KeyCode::Escape));
        assert!(mode.completion().is_none());
    }

    #[test]
    fn test_command_mode_typing_cancels_completion() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Tab));
        assert!(mode.completion().is_some());

        mode.handle_key(&KeyEvent::char('x'));
        assert!(mode.completion().is_none());
    }

    #[test]
    fn test_command_mode_ctrl_e_cancels() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        let original = mode.buffer().to_string();

        mode.handle_key(&KeyEvent::plain(KeyCode::Tab));
        mode.handle_key(&KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CTRL));

        assert!(mode.completion().is_none());
        assert_eq!(mode.buffer(), original);
    }

    #[test]
    fn test_command_mode_ctrl_y_accepts() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Tab));

        let completed = mode.buffer().to_string();
        mode.handle_key(&KeyEvent::new(KeyCode::Char('y'), KeyModifiers::CTRL));

        assert!(mode.completion().is_none());
        assert_eq!(mode.buffer(), completed);
    }

    #[test]
    fn test_command_mode_reset_clears_completion() {
        let mut mode = CommandMode::new();
        mode.handle_key(&KeyEvent::char('w'));
        mode.handle_key(&KeyEvent::plain(KeyCode::Tab));
        assert!(mode.completion().is_some());

        mode.reset();
        assert!(mode.completion().is_none());
        assert!(mode.buffer().is_empty());
    }
}
