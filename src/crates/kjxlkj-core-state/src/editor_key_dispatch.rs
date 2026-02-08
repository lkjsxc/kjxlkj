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
                // Terminal mode: forward all keys.
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

    /// Handle keys in operator-pending mode.
    fn dispatch_op_pending_key(
        &mut self,
        key: &Key,
        op: kjxlkj_core_types::Operator,
    ) -> Option<Action> {
        match &key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                Some(Action::Nop)
            }
            _ => {
                // Produce a motion or double-op action.
                let action =
                    self.resolve_op_key(key, op);
                self.mode = Mode::Normal;
                action
            }
        }
    }

    /// Resolve operator + key to an action.
    fn resolve_op_key(
        &mut self,
        key: &Key,
        op: kjxlkj_core_types::Operator,
    ) -> Option<Action> {
        use kjxlkj_core_types::{Motion, Operator};

        let count =
            self.normal_state.effective_count();

        // Same key as operator → double (dd, cc, yy).
        let op_char = match op {
            Operator::Delete => 'd',
            Operator::Change => 'c',
            Operator::Yank => 'y',
            Operator::Indent => '>',
            Operator::Dedent => '<',
            Operator::Format => 'q',
            Operator::ToggleCase => '~',
            Operator::Uppercase => 'U',
            Operator::Lowercase => 'u',
            Operator::Reindent => '=',
        };

        if let KeyCode::Char(c) = key.code {
            if c == op_char {
                self.normal_state.reset();
                return Some(
                    Action::DoubleOperator(op, count),
                );
            }
        }

        // Try to resolve motion from key.
        let motion = self.key_to_motion(key, count);
        self.normal_state.reset();

        motion.map(|(m, c)| match op {
            Operator::Delete => Action::Delete(m, c),
            Operator::Change => Action::Change(m, c),
            Operator::Yank => Action::Yank(m, c),
            _ => Action::DoubleOperator(op, c),
        })
    }

    /// Convert a key to a motion (for operator-pending).
    fn key_to_motion(
        &self,
        key: &Key,
        count: u32,
    ) -> Option<(kjxlkj_core_types::Motion, u32)> {
        use kjxlkj_core_types::Motion;

        let m = match &key.code {
            KeyCode::Char('h') | KeyCode::Left => {
                Motion::Left
            }
            KeyCode::Char('l') | KeyCode::Right => {
                Motion::Right
            }
            KeyCode::Char('j') | KeyCode::Down => {
                Motion::Down
            }
            KeyCode::Char('k') | KeyCode::Up => {
                Motion::Up
            }
            KeyCode::Char('w') => Motion::WordForward,
            KeyCode::Char('W') => {
                Motion::WordForwardBig
            }
            KeyCode::Char('b') => Motion::WordBackward,
            KeyCode::Char('B') => {
                Motion::WordBackwardBig
            }
            KeyCode::Char('e') => {
                Motion::WordEndForward
            }
            KeyCode::Char('E') => {
                Motion::WordEndForwardBig
            }
            KeyCode::Char('0') => Motion::LineStart,
            KeyCode::Char('^') => Motion::FirstNonBlank,
            KeyCode::Char('$') => Motion::LineEnd,
            KeyCode::Char('G') => Motion::GotoLastLine,
            KeyCode::Char('{') => {
                Motion::ParagraphBackward
            }
            KeyCode::Char('}') => {
                Motion::ParagraphForward
            }
            _ => return None,
        };
        Some((m, count))
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
