//! Normal mode key handlers.

use crate::Intent;
use kjxlkj_core_edit::{Motion, MotionKind, Operator, OperatorKind};
use kjxlkj_core_types::SelectionKind;

use super::state::{AwaitingChar, NormalModeState};

impl NormalModeState {
    /// Handle regular keys in normal mode.
    pub fn handle_normal_key(&mut self, key: char, count: usize) -> Intent {
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
            'G' => self.handle_g_key(count),
            '%' => self.handle_percent_key(count),
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
            'i' => Intent::EnterInsert { at_line_end: false, after_cursor: false },
            'I' => Intent::EnterInsert { at_line_end: false, after_cursor: false },
            'a' => Intent::EnterInsert { at_line_end: false, after_cursor: true },
            'A' => Intent::EnterInsert { at_line_end: true, after_cursor: true },
            'o' => Intent::OpenLineBelow,
            'O' => Intent::OpenLineAbove,
            'v' => Intent::StartVisual(SelectionKind::Char),
            'V' => Intent::StartVisual(SelectionKind::Line),
            'R' => Intent::EnterReplace,
            ':' => Intent::EnterCommand,
            // Operators
            'd' => self.set_operator(OperatorKind::Delete, count),
            'y' => self.set_operator(OperatorKind::Yank, count),
            'c' => self.set_operator(OperatorKind::Change, count),
            '>' => self.set_operator(OperatorKind::Indent, count),
            '<' => self.set_operator(OperatorKind::Outdent, count),
            // Single key actions
            'x' => Intent::DeleteChar,
            'X' => Intent::DeleteCharBefore,
            'D' => Intent::Execute(Operator::with_motion(
                OperatorKind::Delete, Motion::new(MotionKind::LineEnd))),
            'C' => Intent::Execute(Operator::with_motion(
                OperatorKind::Change, Motion::new(MotionKind::LineEnd))),
            's' => Intent::Execute(Operator::with_motion(
                OperatorKind::Change, Motion::new(MotionKind::Right))),
            'S' => Intent::Execute(Operator::line(OperatorKind::Change, 1)),
            'Y' => Intent::Execute(Operator::line(OperatorKind::Yank, count)),
            'p' => Intent::Paste { before: false, cursor_at_end: false },
            'P' => Intent::Paste { before: true, cursor_at_end: false },
            'u' => Intent::Undo,
            '.' => Intent::RepeatChange,
            'J' => Intent::JoinLines { with_space: true },
            '~' => Intent::ToggleCaseChar,
            // Search
            '/' => Intent::SearchForward,
            '?' => Intent::SearchBackward,
            'n' => Intent::NextMatch,
            'N' => Intent::PrevMatch,
            '*' => Intent::SearchWordUnderCursor { forward: true, whole_word: true },
            '#' => Intent::SearchWordUnderCursor { forward: false, whole_word: true },
            // Awaiting char
            'r' => { self.awaiting_char = Some(AwaitingChar::Replace); Intent::None }
            'f' => { self.set_awaiting_find(AwaitingChar::FindForward, count); Intent::None }
            'F' => { self.set_awaiting_find(AwaitingChar::FindBackward, count); Intent::None }
            't' => { self.set_awaiting_find(AwaitingChar::TillForward, count); Intent::None }
            'T' => { self.set_awaiting_find(AwaitingChar::TillBackward, count); Intent::None }
            'm' => { self.awaiting_char = Some(AwaitingChar::Mark); Intent::None }
            '`' => { self.awaiting_char = Some(AwaitingChar::JumpMark); Intent::None }
            '\'' => { self.awaiting_char = Some(AwaitingChar::JumpMarkLine); Intent::None }
            '"' => { self.awaiting_char = Some(AwaitingChar::Register); Intent::None }
            'q' => { self.awaiting_char = Some(AwaitingChar::MacroRecord); Intent::None }
            '@' => { self.awaiting_char = Some(AwaitingChar::MacroPlay); Intent::None }
            ';' => Intent::Move(Motion::new(MotionKind::RepeatFind)),
            ',' => Intent::Move(Motion::new(MotionKind::RepeatFindReverse)),
            _ => Intent::None,
        }
    }

    fn handle_g_key(&mut self, count: usize) -> Intent {
        if count > 1 || self.count.is_some() {
            Intent::Move(Motion::new(MotionKind::GoToLine(count)))
        } else {
            Intent::Move(Motion::new(MotionKind::FileEnd))
        }
    }

    fn handle_percent_key(&mut self, count: usize) -> Intent {
        if self.count.is_some() {
            Intent::Move(Motion::new(MotionKind::GoToPercent(count)))
        } else {
            Intent::Move(Motion::new(MotionKind::MatchingBracket))
        }
    }

    fn set_operator(&mut self, op: OperatorKind, count: usize) -> Intent {
        self.pending_operator = Some(op);
        self.count = Some(count);
        Intent::None
    }

    fn set_awaiting_find(&mut self, kind: AwaitingChar, count: usize) {
        self.awaiting_char = Some(kind);
        self.count = Some(count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn process_key_h_moves_left() {
        let mut state = NormalModeState::new();
        let intent = state.process_key('h', false, false);
        assert!(matches!(intent, Intent::Move(_)));
    }

    #[test]
    fn process_key_j_moves_down() {
        let mut state = NormalModeState::new();
        let intent = state.process_key('j', false, false);
        assert!(matches!(intent, Intent::Move(_)));
    }

    #[test]
    fn process_key_i_enters_insert() {
        let mut state = NormalModeState::new();
        let intent = state.process_key('i', false, false);
        assert!(matches!(intent, Intent::EnterInsert { .. }));
    }

    #[test]
    fn process_key_x_deletes_char() {
        let mut state = NormalModeState::new();
        let intent = state.process_key('x', false, false);
        assert_eq!(intent, Intent::DeleteChar);
    }

    #[test]
    fn process_key_colon_enters_command() {
        let mut state = NormalModeState::new();
        let intent = state.process_key(':', false, false);
        assert_eq!(intent, Intent::EnterCommand);
    }

    #[test]
    fn process_key_u_undo() {
        let mut state = NormalModeState::new();
        let intent = state.process_key('u', false, false);
        assert_eq!(intent, Intent::Undo);
    }

    #[test]
    fn process_key_k_moves_up() {
        let mut state = NormalModeState::new();
        let intent = state.process_key('k', false, false);
        assert!(matches!(intent, Intent::Move(_)));
    }

    #[test]
    fn process_key_l_moves_right() {
        let mut state = NormalModeState::new();
        let intent = state.process_key('l', false, false);
        assert!(matches!(intent, Intent::Move(_)));
    }
}
