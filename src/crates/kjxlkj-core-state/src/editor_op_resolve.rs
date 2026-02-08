//! Operator-pending key resolution for EditorState.
//!
//! When an operator key (d, c, y, etc.) is pressed,
//! the editor enters OperatorPending mode. The next
//! key is resolved here to produce an action.

use kjxlkj_core_types::{
    Action, Key, KeyCode, Motion, Mode, Operator,
    TextObject, TextObjectKind, TextObjectScope,
};

use crate::EditorState;

impl EditorState {
    /// Handle keys in operator-pending mode.
    pub(crate) fn dispatch_op_pending_key(
        &mut self,
        key: &Key,
        op: Operator,
    ) -> Option<Action> {
        match &key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                Some(Action::Nop)
            }
            // Text object prefix: wait for next key
            KeyCode::Char('i') | KeyCode::Char('a') => {
                let scope = if key.code == KeyCode::Char('i') {
                    TextObjectScope::Inner
                } else {
                    TextObjectScope::Around
                };
                self.op_text_obj_pending = Some((op, scope));
                None // Stay in OperatorPending mode
            }
            _ => {
                // Check if we have a pending text object scope
                if let Some((pending_op, scope)) = self.op_text_obj_pending.take() {
                    let action = self.resolve_text_obj_key(key, pending_op, scope);
                    self.mode = Mode::Normal;
                    return action;
                }
                let action = self.resolve_op_key(key, op);
                self.mode = Mode::Normal;
                action
            }
        }
    }

    /// Resolve a text object key after i/a prefix.
    fn resolve_text_obj_key(
        &mut self,
        key: &Key,
        op: Operator,
        scope: TextObjectScope,
    ) -> Option<Action> {
        let count = self.normal_state.effective_count();
        let kind = match &key.code {
            KeyCode::Char('w') => Some(TextObjectKind::Word),
            KeyCode::Char('W') => Some(TextObjectKind::BigWord),
            KeyCode::Char('s') => Some(TextObjectKind::Sentence),
            KeyCode::Char('p') => Some(TextObjectKind::Paragraph),
            KeyCode::Char('(') | KeyCode::Char(')') | KeyCode::Char('b') => Some(TextObjectKind::Parens),
            KeyCode::Char('[') | KeyCode::Char(']') => Some(TextObjectKind::Brackets),
            KeyCode::Char('{') | KeyCode::Char('}') | KeyCode::Char('B') => Some(TextObjectKind::Braces),
            KeyCode::Char('<') | KeyCode::Char('>') => Some(TextObjectKind::AngleBrackets),
            KeyCode::Char('"') => Some(TextObjectKind::DoubleQuote),
            KeyCode::Char('\'') => Some(TextObjectKind::SingleQuote),
            KeyCode::Char('`') => Some(TextObjectKind::Backtick),
            KeyCode::Char('t') => Some(TextObjectKind::Tag),
            KeyCode::Char('a') => Some(TextObjectKind::Argument),
            _ => None,
        };
        self.normal_state.reset();
        kind.map(|k| {
            Action::OperatorTextObject(
                op, TextObject::new(scope, k), count,
            )
        })
    }

    /// Resolve operator + key to an action.
    fn resolve_op_key(
        &mut self,
        key: &Key,
        op: Operator,
    ) -> Option<Action> {
        let count = self.normal_state.effective_count();
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
                return Some(Action::DoubleOperator(op, count));
            }
        }
        let motion = self.key_to_motion(key, count);
        self.normal_state.reset();
        motion.map(|(m, c)| match op {
            Operator::Delete => Action::Delete(m, c),
            Operator::Change => Action::Change(m, c),
            Operator::Yank => Action::Yank(m, c),
            _ => Action::DoubleOperator(op, c),
        })
    }

    /// Convert a key to a motion for operator-pending.
    ///
    /// Returns the Motion and the count. Only handles
    /// basic motion keys; text object motions (iw, aw)
    /// should be handled separately.
    pub(crate) fn key_to_motion(
        &self,
        key: &Key,
        count: u32,
    ) -> Option<(Motion, u32)> {
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
            KeyCode::Char('^') => {
                Motion::FirstNonBlank
            }
            KeyCode::Char('$') => Motion::LineEnd,
            KeyCode::Char('G') => {
                Motion::GotoLastLine
            }
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
    fn double_operator_dd() {
        let mut ed = EditorState::new(80, 24);
        ed.mode = Mode::OperatorPending(
            Operator::Delete,
        );
        let action = ed.dispatch_op_pending_key(
            &Key::char('d'),
            Operator::Delete,
        );
        assert!(matches!(
            action,
            Some(Action::DoubleOperator(
                Operator::Delete,
                1
            ))
        ));
    }

    #[test]
    fn operator_with_motion() {
        let mut ed = EditorState::new(80, 24);
        ed.mode = Mode::OperatorPending(
            Operator::Delete,
        );
        let action = ed.dispatch_op_pending_key(
            &Key::char('w'),
            Operator::Delete,
        );
        assert!(matches!(
            action,
            Some(Action::Delete(
                Motion::WordForward,
                1
            ))
        ));
    }

    #[test]
    fn escape_cancels_op_pending() {
        let mut ed = EditorState::new(80, 24);
        ed.mode = Mode::OperatorPending(
            Operator::Yank,
        );
        let action = ed.dispatch_op_pending_key(
            &Key::esc(),
            Operator::Yank,
        );
        assert!(matches!(action, Some(Action::Nop)));
        assert_eq!(ed.mode, Mode::Normal);
    }
}
