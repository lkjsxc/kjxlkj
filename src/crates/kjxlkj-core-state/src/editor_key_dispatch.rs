//! Mode-aware key dispatch for EditorState.
//!
//! Bridges raw Key events to mode-specific handlers,
//! producing Actions that are then dispatched.

use kjxlkj_core_types::{Action, Key, KeyCode, Mode};

use crate::EditorState;

impl EditorState {
    /// Process a key event according to the current mode.
    ///
    /// This is the central input routing point: keys from
    /// the terminal are sent here and converted to Actions
    /// via mode-specific state machines.
    pub fn dispatch_key(&mut self, key: Key) {
        // Record key for macro if recording.
        self.record_key_if_needed(&key);
        let action = match self.mode {
            Mode::Normal => {
                self.normal_state.process_key(&key)
            }
            Mode::Insert => {
                Some(self.insert_state.process_key(&key))
            }
            Mode::Visual(_) => {
                if let Some(ref mut vs) =
                    self.visual_state
                {
                    vs.process_key(&key)
                } else {
                    Some(Action::ReturnToNormal)
                }
            }
            Mode::Command(_) => {
                if let Some(ref mut cs) =
                    self.command_state
                {
                    cs.process_key(&key)
                } else {
                    Some(Action::ReturnToNormal)
                }
            }
            Mode::Replace => {
                self.dispatch_replace_key(&key)
            }
            Mode::OperatorPending(op) => {
                self.dispatch_op_pending_key(&key, op)
            }
            Mode::TerminalInsert => {
                // Check for Ctrl-\ (escape from terminal)
                if let KeyCode::Char('\\') = key.code {
                    if key.modifiers.contains(kjxlkj_core_types::KeyModifiers::CTRL) {
                        self.terminal_escape_pending = true;
                        return;
                    }
                }
                if self.terminal_escape_pending {
                    self.terminal_escape_pending = false;
                    // Ctrl-\ followed by Ctrl-n → exit terminal mode
                    if let KeyCode::Char('n') = key.code {
                        if key.modifiers.contains(kjxlkj_core_types::KeyModifiers::CTRL) {
                            self.mode = Mode::Normal;
                            return;
                        }
                    }
                }
                // Otherwise forward key to terminal (no-op in state)
                None
            }
            Mode::InsertNormal => {
                // Single Normal command from Insert.
                let action =
                    self.normal_state.process_key(&key);
                if action.is_some() {
                    self.mode = Mode::Insert;
                }
                action
            }
        };

        if let Some(action) = action {
            self.dispatch(action);
        }
    }

    /// Handle keys in Replace mode.
    fn dispatch_replace_key(
        &mut self,
        key: &Key,
    ) -> Option<Action> {
        match &key.code {
            KeyCode::Esc => {
                Some(Action::ReturnToNormal)
            }
            KeyCode::Char(ch) => {
                self.do_replace_char_at_cursor(*ch);
                None
            }
            KeyCode::Backspace => {
                Some(Action::DeleteCharBackward)
            }
            KeyCode::Enter => {
                Some(Action::InsertChar('\n'))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_key_dispatch() {
        let mut ed = EditorState::new(80, 24);
        // 'j' should move cursor down.
        ed.dispatch_key(Key::char('j'));
        // Cursor stays at 0 because buffer only has
        // one line, but dispatch should not panic.
        assert_eq!(ed.mode, Mode::Normal);
    }

    #[test]
    fn insert_mode_key() {
        let mut ed = EditorState::new(80, 24);
        ed.dispatch_key(Key::char('i'));
        assert_eq!(ed.mode, Mode::Insert);
        ed.dispatch_key(Key::char('a'));
        let (_, col) = ed.cursor_pos();
        assert_eq!(col, 1);
    }

    #[test]
    fn replace_mode_key() {
        let mut ed = EditorState::new(80, 24);
        // Put some content first.
        ed.dispatch(Action::InsertChar('h'));
        ed.dispatch(Action::InsertChar('i'));
        ed.dispatch(Action::ReturnToNormal);
        // Enter replace mode.
        ed.mode = Mode::Replace;
        ed.dispatch_key(Key::char('X'));
        // Char at cursor overwritten.
        assert_eq!(ed.mode, Mode::Replace);
    }

    #[test]
    fn command_mode_key() {
        let mut ed = EditorState::new(80, 24);
        ed.dispatch_key(Key::char(':'));
        assert!(matches!(ed.mode, Mode::Command(_)));
        ed.dispatch_key(Key::esc());
        assert_eq!(ed.mode, Mode::Normal);
    }
}
