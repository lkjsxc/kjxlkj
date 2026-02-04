//! Normal mode state machine.

use crate::Intent;
use kjxlkj_core_edit::{Motion, MotionKind, Operator, OperatorKind, TextObject, TextObjectKind};
use kjxlkj_core_types::{Mode, RegisterName, SelectionKind};

/// Normal mode parsing state.
#[derive(Debug, Clone, Default)]
pub struct NormalModeState {
    /// Accumulated count.
    count: Option<usize>,
    /// Pending operator.
    pending_operator: Option<OperatorKind>,
    /// Pending register.
    pending_register: Option<RegisterName>,
    /// Last find character motion.
    last_find: Option<(MotionKind, char)>,
    /// Is awaiting character input (for r, f, t, etc).
    awaiting_char: Option<AwaitingChar>,
}

#[derive(Debug, Clone, Copy)]
enum AwaitingChar {
    Replace,
    FindForward,
    FindBackward,
    TillForward,
    TillBackward,
    Mark,
    JumpMark,
    JumpMarkLine,
    Register,
    MacroRecord,
    MacroPlay,
}

impl NormalModeState {
    /// Create a new normal mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the state.
    pub fn reset(&mut self) {
        self.count = None;
        self.pending_operator = None;
        self.awaiting_char = None;
    }

    /// Get the current count (default 1).
    pub fn get_count(&self) -> usize {
        self.count.unwrap_or(1)
    }

    /// Process a key and return an intent.
    pub fn process_key(&mut self, key: char, ctrl: bool, shift: bool) -> Intent {
        // Handle awaiting character first
        if let Some(awaiting) = self.awaiting_char.take() {
            return self.handle_awaiting_char(awaiting, key);
        }

        // Handle Ctrl keys
        if ctrl {
            return self.handle_ctrl_key(key);
        }

        // Handle digits for count
        if key.is_ascii_digit() && !(key == '0' && self.count.is_none()) {
            let digit = key as usize - '0' as usize;
            self.count = Some(self.count.unwrap_or(0) * 10 + digit);
            return Intent::None;
        }

        let count = self.get_count();

        // Handle pending operator
        if let Some(op) = self.pending_operator.take() {
            return self.handle_operator_motion(op, key, count);
        }

        // Handle regular keys
        self.handle_normal_key(key, count)
    }

    fn handle_awaiting_char(&mut self, awaiting: AwaitingChar, c: char) -> Intent {
        let count = self.get_count();
        self.reset();
        match awaiting {
            AwaitingChar::Replace => Intent::ReplaceChar(c),
            AwaitingChar::FindForward => {
                self.last_find = Some((MotionKind::FindChar(c), c));
                Intent::Move(Motion::new(MotionKind::FindChar(c)).with_count(count))
            }
            AwaitingChar::FindBackward => {
                self.last_find = Some((MotionKind::FindCharBackward(c), c));
                Intent::Move(Motion::new(MotionKind::FindCharBackward(c)).with_count(count))
            }
            AwaitingChar::TillForward => {
                self.last_find = Some((MotionKind::TillChar(c), c));
                Intent::Move(Motion::new(MotionKind::TillChar(c)).with_count(count))
            }
            AwaitingChar::TillBackward => {
                self.last_find = Some((MotionKind::TillCharBackward(c), c));
                Intent::Move(Motion::new(MotionKind::TillCharBackward(c)).with_count(count))
            }
            AwaitingChar::Mark => Intent::SetMark(c),
            AwaitingChar::JumpMark => Intent::JumpToMark {
                mark: c,
                line_start: false,
            },
            AwaitingChar::JumpMarkLine => Intent::JumpToMark {
                mark: c,
                line_start: true,
            },
            AwaitingChar::Register => {
                if let Some(reg) = RegisterName::from_char(c) {
                    self.pending_register = Some(reg);
                    Intent::SetRegister(reg)
                } else {
                    Intent::None
                }
            }
            AwaitingChar::MacroRecord => Intent::StartMacro(c),
            AwaitingChar::MacroPlay => Intent::PlayMacro(c),
        }
    }

    fn handle_ctrl_key(&mut self, key: char) -> Intent {
        let count = self.get_count();
        self.reset();
        match key {
            'r' => Intent::Redo,
            'o' => Intent::JumpList { forward: false },
            'i' => Intent::JumpList { forward: true },
            'd' => Intent::Scroll(crate::intent::ScrollIntent::HalfPageDown),
            'u' => Intent::Scroll(crate::intent::ScrollIntent::HalfPageUp),
            'f' => Intent::Scroll(crate::intent::ScrollIntent::FullPageDown),
            'b' => Intent::Scroll(crate::intent::ScrollIntent::FullPageUp),
            'e' => Intent::Scroll(crate::intent::ScrollIntent::LineDown),
            'y' => Intent::Scroll(crate::intent::ScrollIntent::LineUp),
            'a' => Intent::IncrementNumber,
            'x' => Intent::DecrementNumber,
            'v' => Intent::StartVisual(SelectionKind::Block),
            _ => Intent::None,
        }
    }

    fn handle_operator_motion(&mut self, op: OperatorKind, key: char, count: usize) -> Intent {
        self.reset();
        // Check for line-wise operator (dd, yy, cc, etc.)
        let is_linewise = match (op, key) {
            (OperatorKind::Delete, 'd') => true,
            (OperatorKind::Yank, 'y') => true,
            (OperatorKind::Change, 'c') => true,
            (OperatorKind::Indent, '>') => true,
            (OperatorKind::Outdent, '<') => true,
            _ => false,
        };

        if is_linewise {
            return Intent::Execute(Operator::line(op, count));
        }

        // Check for text objects
        if key == 'i' || key == 'a' {
            self.pending_operator = Some(op);
            self.awaiting_char = Some(if key == 'i' {
                AwaitingChar::FindForward // Placeholder - handle text object
            } else {
                AwaitingChar::FindBackward
            });
            return Intent::None;
        }

        // Motion
        if let Some(motion_kind) = self.key_to_motion(key) {
            let motion = Motion::new(motion_kind).with_count(count);
            Intent::Execute(Operator::with_motion(op, motion))
        } else {
            Intent::None
        }
    }

    fn handle_normal_key(&mut self, key: char, count: usize) -> Intent {
        self.reset();
        match key {
            // Movement
            'h' => Intent::Move(Motion::new(MotionKind::Left).with_count(count)),
            'j' => Intent::Move(Motion::new(MotionKind::Down).with_count(count)),
            'k' => Intent::Move(Motion::new(MotionKind::Up).with_count(count)),
            'l' => Intent::Move(Motion::new(MotionKind::Right).with_count(count)),
            ' ' => Intent::Move(Motion::new(MotionKind::Right).with_count(count)),
            '0' => Intent::Move(Motion::new(MotionKind::LineStart)),
            '^' => Intent::Move(Motion::new(MotionKind::FirstNonBlank)),
            '_' => Intent::Move(Motion::new(MotionKind::FirstNonBlank).with_count(count)),
            '$' => Intent::Move(Motion::new(MotionKind::LineEnd)),
            'w' => Intent::Move(Motion::new(MotionKind::WordStart).with_count(count)),
            'W' => Intent::Move(Motion::new(MotionKind::WordStart).with_count(count)),
            'b' => Intent::Move(Motion::new(MotionKind::WordStartBackward).with_count(count)),
            'B' => Intent::Move(Motion::new(MotionKind::WordStartBackward).with_count(count)),
            'e' => Intent::Move(Motion::new(MotionKind::WordEnd).with_count(count)),
            'E' => Intent::Move(Motion::new(MotionKind::WordEnd).with_count(count)),
            'G' => {
                if count > 1 || self.count.is_some() {
                    Intent::Move(Motion::new(MotionKind::GoToLine(count)))
                } else {
                    Intent::Move(Motion::new(MotionKind::FileEnd))
                }
            }
            '%' => {
                if self.count.is_some() {
                    Intent::Move(Motion::new(MotionKind::GoToPercent(count)))
                } else {
                    Intent::Move(Motion::new(MotionKind::MatchingBracket))
                }
            }
            '{' => Intent::Move(Motion::new(MotionKind::ParagraphBackward).with_count(count)),
            '}' => Intent::Move(Motion::new(MotionKind::ParagraphForward).with_count(count)),
            '(' => Intent::Move(Motion::new(MotionKind::SentenceBackward).with_count(count)),
            ')' => Intent::Move(Motion::new(MotionKind::SentenceForward).with_count(count)),
            'H' => Intent::Move(Motion::new(MotionKind::ScreenTop)),
            'M' => Intent::Move(Motion::new(MotionKind::ScreenMiddle)),
            'L' => Intent::Move(Motion::new(MotionKind::ScreenBottom)),
            '|' => Intent::Move(Motion::new(MotionKind::GoToColumn(count))),
            '+' => Intent::Move(Motion::new(MotionKind::Down).with_count(count)),
            '-' => Intent::Move(Motion::new(MotionKind::Up).with_count(count)),

            // Mode changes
            'i' => Intent::EnterInsert {
                at_line_end: false,
                after_cursor: false,
            },
            'I' => Intent::EnterInsert {
                at_line_end: false,
                after_cursor: false,
            },
            'a' => Intent::EnterInsert {
                at_line_end: false,
                after_cursor: true,
            },
            'A' => Intent::EnterInsert {
                at_line_end: true,
                after_cursor: true,
            },
            'o' => Intent::OpenLineBelow,
            'O' => Intent::OpenLineAbove,
            'v' => Intent::StartVisual(SelectionKind::Char),
            'V' => Intent::StartVisual(SelectionKind::Line),
            'R' => Intent::EnterReplace,
            ':' => Intent::EnterCommand,

            // Operators
            'd' => {
                self.pending_operator = Some(OperatorKind::Delete);
                self.count = Some(count);
                Intent::None
            }
            'y' => {
                self.pending_operator = Some(OperatorKind::Yank);
                self.count = Some(count);
                Intent::None
            }
            'c' => {
                self.pending_operator = Some(OperatorKind::Change);
                self.count = Some(count);
                Intent::None
            }
            '>' => {
                self.pending_operator = Some(OperatorKind::Indent);
                self.count = Some(count);
                Intent::None
            }
            '<' => {
                self.pending_operator = Some(OperatorKind::Outdent);
                self.count = Some(count);
                Intent::None
            }

            // Single key actions
            'x' => Intent::DeleteChar,
            'X' => Intent::DeleteCharBefore,
            'D' => Intent::Execute(Operator::with_motion(
                OperatorKind::Delete,
                Motion::new(MotionKind::LineEnd),
            )),
            'C' => Intent::Execute(Operator::with_motion(
                OperatorKind::Change,
                Motion::new(MotionKind::LineEnd),
            )),
            's' => Intent::Execute(Operator::with_motion(
                OperatorKind::Change,
                Motion::new(MotionKind::Right),
            )),
            'S' => Intent::Execute(Operator::line(OperatorKind::Change, 1)),
            'Y' => Intent::Execute(Operator::line(OperatorKind::Yank, count)),
            'p' => Intent::Paste {
                before: false,
                cursor_at_end: false,
            },
            'P' => Intent::Paste {
                before: true,
                cursor_at_end: false,
            },
            'u' => Intent::Undo,
            '.' => Intent::RepeatChange,
            'J' => Intent::JoinLines { with_space: true },
            '~' => Intent::ToggleCaseChar,

            // Search
            '/' => Intent::SearchForward,
            '?' => Intent::SearchBackward,
            'n' => Intent::NextMatch,
            'N' => Intent::PrevMatch,
            '*' => Intent::SearchWordUnderCursor {
                forward: true,
                whole_word: true,
            },
            '#' => Intent::SearchWordUnderCursor {
                forward: false,
                whole_word: true,
            },

            // Awaiting char
            'r' => {
                self.awaiting_char = Some(AwaitingChar::Replace);
                Intent::None
            }
            'f' => {
                self.awaiting_char = Some(AwaitingChar::FindForward);
                self.count = Some(count);
                Intent::None
            }
            'F' => {
                self.awaiting_char = Some(AwaitingChar::FindBackward);
                self.count = Some(count);
                Intent::None
            }
            't' => {
                self.awaiting_char = Some(AwaitingChar::TillForward);
                self.count = Some(count);
                Intent::None
            }
            'T' => {
                self.awaiting_char = Some(AwaitingChar::TillBackward);
                self.count = Some(count);
                Intent::None
            }
            'm' => {
                self.awaiting_char = Some(AwaitingChar::Mark);
                Intent::None
            }
            '`' => {
                self.awaiting_char = Some(AwaitingChar::JumpMark);
                Intent::None
            }
            '\'' => {
                self.awaiting_char = Some(AwaitingChar::JumpMarkLine);
                Intent::None
            }
            '"' => {
                self.awaiting_char = Some(AwaitingChar::Register);
                Intent::None
            }
            'q' => {
                self.awaiting_char = Some(AwaitingChar::MacroRecord);
                Intent::None
            }
            '@' => {
                self.awaiting_char = Some(AwaitingChar::MacroPlay);
                Intent::None
            }

            ';' => Intent::Move(Motion::new(MotionKind::RepeatFind)),
            ',' => Intent::Move(Motion::new(MotionKind::RepeatFindReverse)),

            _ => Intent::None,
        }
    }

    fn key_to_motion(&self, key: char) -> Option<MotionKind> {
        match key {
            'h' => Some(MotionKind::Left),
            'l' => Some(MotionKind::Right),
            'j' => Some(MotionKind::Down),
            'k' => Some(MotionKind::Up),
            'w' => Some(MotionKind::WordStart),
            'W' => Some(MotionKind::WordStart),
            'b' => Some(MotionKind::WordStartBackward),
            'B' => Some(MotionKind::WordStartBackward),
            'e' => Some(MotionKind::WordEnd),
            'E' => Some(MotionKind::WordEnd),
            '0' => Some(MotionKind::LineStart),
            '^' => Some(MotionKind::FirstNonBlank),
            '$' => Some(MotionKind::LineEnd),
            'G' => Some(MotionKind::FileEnd),
            '{' => Some(MotionKind::ParagraphBackward),
            '}' => Some(MotionKind::ParagraphForward),
            '(' => Some(MotionKind::SentenceBackward),
            ')' => Some(MotionKind::SentenceForward),
            '%' => Some(MotionKind::MatchingBracket),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_mode_movement() {
        let mut state = NormalModeState::new();
        let intent = state.process_key('j', false, false);
        assert!(matches!(intent, Intent::Move(_)));
    }

    #[test]
    fn normal_mode_count() {
        let mut state = NormalModeState::new();
        state.process_key('5', false, false);
        let intent = state.process_key('j', false, false);
        if let Intent::Move(motion) = intent {
            assert_eq!(motion.count, 5);
        } else {
            panic!("Expected Move intent");
        }
    }

    #[test]
    fn normal_mode_operator() {
        let mut state = NormalModeState::new();
        let intent1 = state.process_key('d', false, false);
        assert_eq!(intent1, Intent::None);
        let intent2 = state.process_key('d', false, false);
        assert!(matches!(intent2, Intent::Execute(Operator::Line { .. })));
    }
}
