//! Visual mode state: selection tracking and key dispatch.

use kjxlkj_core_types::{Action, Key, KeyCode, KeyModifiers, Motion, VisualKind};

/// Visual mode state tracking the selection anchor and
/// kind.
#[derive(Debug)]
pub struct VisualModeState {
    /// Visual sub-kind (char / line / block).
    pub kind: VisualKind,
    /// Anchor position: (line, grapheme_offset).
    pub anchor: (usize, usize),
    /// Count prefix accumulator.
    pub(crate) count: Option<u32>,
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

    /// Toggle between visual sub-kinds.
    pub fn toggle_kind(&mut self, new: VisualKind) {
        self.kind = new;
    }

    /// Process a key event in Visual mode.
    pub fn process_key(&mut self, key: &Key) -> Option<Action> {
        if key.code == KeyCode::Esc {
            self.reset_count();
            return Some(Action::ReturnToNormal);
        }

        if let Some(d) = key.digit_value() {
            if d > 0 || self.count.is_some() {
                let cur = self.count.unwrap_or(0);
                self.count = Some(cur * 10 + d);
                return None;
            }
        }

        let count = self.effective_count();

        let action = match (&key.code, key.modifiers) {
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
            (KeyCode::Char('$'), KeyModifiers::NONE) => Action::MoveCursor(Motion::LineEnd, 1),
            (KeyCode::Char('G'), KeyModifiers::NONE) => {
                if self.count.is_some() {
                    Action::MoveCursor(Motion::GotoLine(count as usize - 1), 1)
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

            _ => {
                return self.dispatch_visual_command(key);
            }
        };

        self.reset_count();
        Some(action)
    }
}
