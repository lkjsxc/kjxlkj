//! Visual mode state: selection tracking and key dispatch.

use kjxlkj_core_types::{
    Action, Key, KeyCode, KeyModifiers, Motion, Operator, VisualKind,
};

/// Visual mode state tracking the selection anchor and kind.
#[derive(Debug)]
pub struct VisualModeState {
    /// Visual sub-kind (char / line / block).
    pub kind: VisualKind,
    /// Anchor position: (line, grapheme_offset).
    pub anchor: (usize, usize),
    /// Count prefix accumulator.
    count: Option<u32>,
}

impl VisualModeState {
    pub fn new(kind: VisualKind, anchor: (usize, usize)) -> Self {
        Self {
            kind,
            anchor,
            count: None,
        }
    }

    pub fn effective_count(&self) -> u32 {
        self.count.unwrap_or(1)
    }

    /// Reset count prefix.
    pub fn reset_count(&mut self) {
        self.count = None;
    }

    /// Toggle between visual-char and visual-line.
    pub fn toggle_kind(&mut self, new: VisualKind) {
        if self.kind == new {
            // Same kind → exit visual (return to Normal).
            self.kind = new; // caller handles transition
        } else {
            self.kind = new;
        }
    }

    /// Process a key event in Visual mode.
    pub fn process_key(&mut self, key: &Key) -> Option<Action> {
        // Escape → leave visual
        if key.code == KeyCode::Esc {
            self.reset_count();
            return Some(Action::ReturnToNormal);
        }

        // Count prefix digits
        if let Some(d) = key.digit_value() {
            if d > 0 || self.count.is_some() {
                let cur = self.count.unwrap_or(0);
                self.count = Some(cur * 10 + d);
                return None;
            }
        }

        let count = self.effective_count();

        let action = match (&key.code, key.modifiers) {
            // Motions
            (KeyCode::Char('h'), KeyModifiers::NONE) | (KeyCode::Left, _) => {
                Action::MoveCursor(Motion::Left, count)
            }
            (KeyCode::Char('l'), KeyModifiers::NONE) | (KeyCode::Right, _) => {
                Action::MoveCursor(Motion::Right, count)
            }
            (KeyCode::Char('j'), KeyModifiers::NONE) | (KeyCode::Down, _) => {
                Action::MoveCursor(Motion::Down, count)
            }
            (KeyCode::Char('k'), KeyModifiers::NONE) | (KeyCode::Up, _) => {
                Action::MoveCursor(Motion::Up, count)
            }
            (KeyCode::Char('w'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::WordForward, count)
            }
            (KeyCode::Char('b'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::WordBackward, count)
            }
            (KeyCode::Char('e'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::WordEndForward, count)
            }
            (KeyCode::Char('0'), KeyModifiers::NONE) if self.count.is_none() => {
                Action::MoveCursor(Motion::LineStart, 1)
            }
            (KeyCode::Char('^'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::FirstNonBlank, 1)
            }
            (KeyCode::Char('$'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::LineEnd, 1)
            }
            (KeyCode::Char('G'), KeyModifiers::NONE) => {
                if self.count.is_some() {
                    Action::MoveCursor(
                        Motion::GotoLine(count as usize - 1),
                        1,
                    )
                } else {
                    Action::MoveCursor(Motion::GotoLastLine, 1)
                }
            }
            (KeyCode::Char('{'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::ParagraphBackward, count)
            }
            (KeyCode::Char('}'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::ParagraphForward, count)
            }

            // Swap anchor/cursor
            (KeyCode::Char('o'), KeyModifiers::NONE) => Action::Nop,

            // Switch visual kind
            (KeyCode::Char('v'), KeyModifiers::NONE) => {
                self.toggle_kind(VisualKind::Char);
                self.reset_count();
                return Some(Action::EnterVisual(VisualKind::Char));
            }
            (KeyCode::Char('V'), KeyModifiers::NONE) => {
                self.toggle_kind(VisualKind::Line);
                self.reset_count();
                return Some(Action::EnterVisual(VisualKind::Line));
            }
            (KeyCode::Char('v'), m)
                if m.contains(KeyModifiers::CTRL) =>
            {
                self.toggle_kind(VisualKind::Block);
                self.reset_count();
                return Some(Action::EnterVisual(VisualKind::Block));
            }

            // Operators on selection
            (KeyCode::Char('d'), KeyModifiers::NONE)
            | (KeyCode::Char('x'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Delete(Motion::Right, 1));
            }
            (KeyCode::Char('c'), KeyModifiers::NONE)
            | (KeyCode::Char('s'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Change(Motion::Right, 1));
            }
            (KeyCode::Char('y'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Yank(Motion::Right, 1));
            }
            (KeyCode::Char('>'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::DoubleOperator(Operator::Indent, 1));
            }
            (KeyCode::Char('<'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::DoubleOperator(Operator::Dedent, 1));
            }
            (KeyCode::Char('~'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::ToggleCaseChar);
            }
            (KeyCode::Char('U'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::DoubleOperator(
                    Operator::Uppercase,
                    1,
                ));
            }
            (KeyCode::Char('u'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::DoubleOperator(
                    Operator::Lowercase,
                    1,
                ));
            }

            // Put
            (KeyCode::Char('p'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Put(false));
            }
            (KeyCode::Char('P'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::Put(true));
            }

            // Join
            (KeyCode::Char('J'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::JoinLines);
            }

            // Enter command mode with selection range
            (KeyCode::Char(':'), KeyModifiers::NONE) => {
                self.reset_count();
                return Some(Action::EnterCommand(
                    kjxlkj_core_types::ActionCommandKind::Ex,
                ));
            }

            _ => {
                self.reset_count();
                return Some(Action::Nop);
            }
        };

        self.reset_count();
        Some(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_exits_visual() {
        let mut s = VisualModeState::new(VisualKind::Char, (0, 0));
        let action = s.process_key(&Key::esc());
        assert!(matches!(action, Some(Action::ReturnToNormal)));
    }

    #[test]
    fn motion_in_visual() {
        let mut s = VisualModeState::new(VisualKind::Char, (0, 0));
        let action = s.process_key(&Key::char('j'));
        assert!(matches!(
            action,
            Some(Action::MoveCursor(Motion::Down, 1))
        ));
    }

    #[test]
    fn delete_selection() {
        let mut s = VisualModeState::new(VisualKind::Line, (0, 0));
        let action = s.process_key(&Key::char('d'));
        assert!(matches!(action, Some(Action::Delete(_, _))));
    }
}
