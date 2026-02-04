//! Normal mode input handling.

use kjxlkj_core_edit::{Motion, MotionKind, Operator, OperatorKind};
use kjxlkj_core_types::{Key, KeyCode, Mode};

use crate::input::{InputResult, ParsedInput};
use crate::state::ModeState;

/// Normal mode state machine.
#[derive(Debug, Default)]
pub struct NormalState;

impl NormalState {
    /// Process a key in normal mode.
    pub fn process_key(state: &mut ModeState, key: Key) -> InputResult {
        // Handle pending operator state.
        if let Some(op_char) = state.pending_operator {
            return Self::process_operator_pending(state, op_char, key);
        }

        // Handle pending multi-key commands (g, z, [, ], etc.).
        if !state.pending_keys.is_empty() {
            return Self::process_pending_sequence(state, key);
        }

        // Handle count prefix.
        if let KeyCode::Char(c) = key.code {
            if c.is_ascii_digit() && (c != '0' || state.count.is_some()) {
                state.accumulate_count(c);
                return InputResult::Pending;
            }
        }

        // Dispatch on key.
        match key.code {
            KeyCode::Char(c) if key.mods.ctrl => Self::process_ctrl_key(state, c),
            KeyCode::Char(c) => Self::process_char(state, c),
            KeyCode::Esc => {
                state.clear_pending();
                InputResult::Handled
            }
            KeyCode::Enter => InputResult::Parsed(ParsedInput::Motion(Motion::new(
                MotionKind::NextLineFirstNonBlank,
            ))),
            KeyCode::Backspace => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::Left,
                state.effective_count(),
            ))),
            KeyCode::Left => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::Left,
                state.effective_count(),
            ))),
            KeyCode::Right => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::Right,
                state.effective_count(),
            ))),
            KeyCode::Up => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::Up,
                state.effective_count(),
            ))),
            KeyCode::Down => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::Down,
                state.effective_count(),
            ))),
            _ => InputResult::Unhandled,
        }
    }

    fn process_ctrl_key(state: &mut ModeState, c: char) -> InputResult {
        let count = state.effective_count();
        match c {
            'r' => InputResult::Parsed(ParsedInput::Redo(count)),
            'd' => InputResult::Parsed(ParsedInput::ScrollHalfDown(count)),
            'u' => InputResult::Parsed(ParsedInput::ScrollHalfUp(count)),
            'f' => InputResult::Parsed(ParsedInput::ScrollPageDown(count)),
            'b' => InputResult::Parsed(ParsedInput::ScrollPageUp(count)),
            'e' => InputResult::Parsed(ParsedInput::ScrollLineDown(count)),
            'y' => InputResult::Parsed(ParsedInput::ScrollLineUp(count)),
            'o' => InputResult::Parsed(ParsedInput::JumpBack(count)),
            'i' => InputResult::Parsed(ParsedInput::JumpForward(count)),
            'a' => InputResult::Parsed(ParsedInput::IncrementNumber(count)),
            'x' => InputResult::Parsed(ParsedInput::DecrementNumber(count)),
            'v' => {
                state.set_mode(Mode::VisualBlock);
                InputResult::ModeChange(Mode::VisualBlock)
            }
            _ => InputResult::Unhandled,
        }
    }

    fn process_char(state: &mut ModeState, c: char) -> InputResult {
        let count = state.effective_count();
        match c {
            // Motions.
            'h' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::Left,
                count,
            ))),
            'l' | ' ' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::Right,
                count,
            ))),
            'j' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::Down,
                count,
            ))),
            'k' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::Up,
                count,
            ))),
            '0' => InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::LineStart))),
            '^' | '_' => {
                InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::FirstNonBlank)))
            }
            '$' => InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::LineEnd))),
            'w' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::WordStart,
                count,
            ))),
            'W' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::WordStartBig,
                count,
            ))),
            'b' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::WordBack,
                count,
            ))),
            'B' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::WordBackBig,
                count,
            ))),
            'e' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::WordEnd,
                count,
            ))),
            'E' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::WordEndBig,
                count,
            ))),
            'G' => {
                if state.count.is_some() {
                    InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::GotoLine(
                        count,
                    ))))
                } else {
                    InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::FileEnd)))
                }
            }
            '{' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::ParagraphBack,
                count,
            ))),
            '}' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::ParagraphForward,
                count,
            ))),
            '(' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::SentenceBack,
                count,
            ))),
            ')' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::SentenceForward,
                count,
            ))),
            '%' => {
                if state.count.is_some() {
                    InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::GotoPercent(
                        count,
                    ))))
                } else {
                    InputResult::Parsed(ParsedInput::Motion(Motion::new(
                        MotionKind::MatchingBracket,
                    )))
                }
            }
            'H' => InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::ScreenTop))),
            'M' => InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::ScreenMiddle))),
            'L' => InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::ScreenBottom))),
            '|' => InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::GotoColumn(
                count,
            )))),
            '+' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::NextLineFirstNonBlank,
                count,
            ))),
            '-' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::PrevLineFirstNonBlank,
                count,
            ))),

            // Mode changes.
            'i' => {
                state.set_mode(Mode::Insert);
                InputResult::ModeChange(Mode::Insert)
            }
            'I' => InputResult::Parsed(ParsedInput::InsertAtFirstNonBlank),
            'a' => InputResult::Parsed(ParsedInput::InsertAfter),
            'A' => InputResult::Parsed(ParsedInput::InsertAtEnd),
            'o' => InputResult::Parsed(ParsedInput::OpenBelow),
            'O' => InputResult::Parsed(ParsedInput::OpenAbove),
            'v' => {
                state.set_mode(Mode::Visual);
                InputResult::ModeChange(Mode::Visual)
            }
            'V' => {
                state.set_mode(Mode::VisualLine);
                InputResult::ModeChange(Mode::VisualLine)
            }
            'R' => {
                state.set_mode(Mode::Replace);
                InputResult::ModeChange(Mode::Replace)
            }
            ':' => {
                state.set_mode(Mode::Command);
                InputResult::ModeChange(Mode::Command)
            }

            // Operators (start operator-pending).
            'd' | 'c' | 'y' | '>' | '<' => {
                state.pending_operator = Some(c);
                InputResult::Pending
            }

            // Single-character edits.
            'x' => InputResult::Parsed(ParsedInput::DeleteChar(count)),
            'X' => InputResult::Parsed(ParsedInput::DeleteCharBefore(count)),
            'r' => InputResult::Parsed(ParsedInput::AwaitReplaceChar),
            's' => InputResult::Parsed(ParsedInput::SubstituteChar(count)),
            'S' => InputResult::Parsed(ParsedInput::SubstituteLine),
            'D' => InputResult::Parsed(ParsedInput::DeleteToEnd),
            'C' => InputResult::Parsed(ParsedInput::ChangeToEnd),
            'Y' => InputResult::Parsed(ParsedInput::YankLine(count)),
            'J' => InputResult::Parsed(ParsedInput::JoinLines(count, true)),
            '~' => InputResult::Parsed(ParsedInput::ToggleCaseChar),

            // Paste.
            'p' => InputResult::Parsed(ParsedInput::PasteAfter(count)),
            'P' => InputResult::Parsed(ParsedInput::PasteBefore(count)),

            // Undo/redo.
            'u' => InputResult::Parsed(ParsedInput::Undo(count)),

            // Repeat.
            '.' => InputResult::Parsed(ParsedInput::Repeat(count)),

            // Search.
            '/' => InputResult::Parsed(ParsedInput::SearchForward),
            '?' => InputResult::Parsed(ParsedInput::SearchBackward),
            'n' => InputResult::Parsed(ParsedInput::SearchNext(count)),
            'N' => InputResult::Parsed(ParsedInput::SearchPrev(count)),
            '*' => InputResult::Parsed(ParsedInput::SearchWordForward),
            '#' => InputResult::Parsed(ParsedInput::SearchWordBackward),

            // Find char.
            'f' | 'F' | 't' | 'T' => InputResult::Parsed(ParsedInput::AwaitFindChar(c)),
            ';' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::RepeatFind,
                count,
            ))),
            ',' => InputResult::Parsed(ParsedInput::Motion(Motion::with_count(
                MotionKind::RepeatFindReverse,
                count,
            ))),

            // Marks.
            'm' => InputResult::Parsed(ParsedInput::AwaitSetMark),
            '\'' | '`' => InputResult::Parsed(ParsedInput::AwaitGotoMark(c == '`')),

            // Registers.
            '"' => InputResult::Parsed(ParsedInput::AwaitRegister),

            // Macros.
            'q' => InputResult::Parsed(ParsedInput::ToggleMacroRecord),
            '@' => InputResult::Parsed(ParsedInput::AwaitPlayMacro),

            // Multi-key prefixes.
            'g' | 'z' | '[' | ']' | 'Z' => {
                state.pending_keys.push(c);
                InputResult::Pending
            }

            _ => InputResult::Unhandled,
        }
    }

    fn process_pending_sequence(state: &mut ModeState, key: Key) -> InputResult {
        let prefix = state.pending_keys.first().copied();
        state.pending_keys.clear();

        match (prefix, key.code) {
            // g-prefix commands.
            (Some('g'), KeyCode::Char('g')) => {
                let count = state.effective_count();
                if state.count.is_some() {
                    InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::GotoLine(
                        count,
                    ))))
                } else {
                    InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::FileStart)))
                }
            }
            (Some('g'), KeyCode::Char('e')) => InputResult::Parsed(ParsedInput::Motion(
                Motion::with_count(MotionKind::WordEndBack, state.effective_count()),
            )),
            (Some('g'), KeyCode::Char('E')) => InputResult::Parsed(ParsedInput::Motion(
                Motion::with_count(MotionKind::WordEndBackBig, state.effective_count()),
            )),
            (Some('g'), KeyCode::Char('_')) => {
                InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::LastNonBlank)))
            }
            (Some('g'), KeyCode::Char('m')) => {
                InputResult::Parsed(ParsedInput::Motion(Motion::new(MotionKind::LineMiddle)))
            }
            (Some('g'), KeyCode::Char('p')) => {
                InputResult::Parsed(ParsedInput::PasteAfterCursorAtEnd(state.effective_count()))
            }
            (Some('g'), KeyCode::Char('P')) => {
                InputResult::Parsed(ParsedInput::PasteBeforeCursorAtEnd(state.effective_count()))
            }
            (Some('g'), KeyCode::Char('J')) => {
                InputResult::Parsed(ParsedInput::JoinLines(state.effective_count(), false))
            }
            (Some('g'), KeyCode::Char('~')) => {
                state.pending_operator = Some('~');
                InputResult::Pending
            }
            (Some('g'), KeyCode::Char('u')) => {
                state.pending_operator = Some('u');
                InputResult::Pending
            }
            (Some('g'), KeyCode::Char('U')) => {
                state.pending_operator = Some('U');
                InputResult::Pending
            }
            (Some('g'), KeyCode::Char('*')) => {
                InputResult::Parsed(ParsedInput::SearchWordForwardPartial)
            }
            (Some('g'), KeyCode::Char('#')) => {
                InputResult::Parsed(ParsedInput::SearchWordBackwardPartial)
            }
            (Some('g'), KeyCode::Char(';')) => InputResult::Parsed(ParsedInput::ChangeListOlder),
            (Some('g'), KeyCode::Char(',')) => InputResult::Parsed(ParsedInput::ChangeListNewer),

            // z-prefix commands.
            (Some('z'), KeyCode::Char('z')) => InputResult::Parsed(ParsedInput::ScrollCursorCenter),
            (Some('z'), KeyCode::Char('t')) => InputResult::Parsed(ParsedInput::ScrollCursorTop),
            (Some('z'), KeyCode::Char('b')) => InputResult::Parsed(ParsedInput::ScrollCursorBottom),
            (Some('z'), KeyCode::Enter) => {
                InputResult::Parsed(ParsedInput::ScrollCursorTopFirstNonBlank)
            }
            (Some('z'), KeyCode::Char('.')) => {
                InputResult::Parsed(ParsedInput::ScrollCursorCenterFirstNonBlank)
            }
            (Some('z'), KeyCode::Char('-')) => {
                InputResult::Parsed(ParsedInput::ScrollCursorBottomFirstNonBlank)
            }

            // Z-prefix commands.
            (Some('Z'), KeyCode::Char('Z')) => InputResult::Parsed(ParsedInput::WriteQuit),
            (Some('Z'), KeyCode::Char('Q')) => InputResult::Parsed(ParsedInput::QuitNoSave),

            // [-prefix commands.
            (Some('['), KeyCode::Char('(')) => {
                InputResult::Parsed(ParsedInput::GotoPrevUnmatched('('))
            }
            (Some('['), KeyCode::Char('{')) => {
                InputResult::Parsed(ParsedInput::GotoPrevUnmatched('{'))
            }

            // ]-prefix commands.
            (Some(']'), KeyCode::Char(')')) => {
                InputResult::Parsed(ParsedInput::GotoNextUnmatched(')'))
            }
            (Some(']'), KeyCode::Char('}')) => {
                InputResult::Parsed(ParsedInput::GotoNextUnmatched('}'))
            }

            _ => InputResult::Unhandled,
        }
    }

    fn process_operator_pending(state: &mut ModeState, op_char: char, key: Key) -> InputResult {
        state.pending_operator = None;
        let count = state.effective_count();

        // Check for double-operator (linewise).
        if let KeyCode::Char(c) = key.code {
            if c == op_char {
                let op = Self::char_to_operator(op_char);
                return InputResult::Parsed(ParsedInput::OperatorLine(op, count));
            }

            // Check for text objects (i/a prefix).
            if c == 'i' || c == 'a' {
                state.pending_keys.push(op_char);
                state.pending_keys.push(c);
                return InputResult::Pending;
            }

            // Motion target.
            if let Some(motion) = Self::char_to_motion(c, count) {
                let op = Self::char_to_operator(op_char);
                return InputResult::Parsed(ParsedInput::OperatorMotion(op, motion));
            }
        }

        InputResult::Unhandled
    }

    fn char_to_operator(c: char) -> Operator {
        let kind = match c {
            'd' => OperatorKind::Delete,
            'c' => OperatorKind::Change,
            'y' => OperatorKind::Yank,
            '>' => OperatorKind::Indent,
            '<' => OperatorKind::Outdent,
            '~' => OperatorKind::ToggleCase,
            'u' => OperatorKind::Lowercase,
            'U' => OperatorKind::Uppercase,
            _ => OperatorKind::Delete,
        };
        Operator::new(kind)
    }

    fn char_to_motion(c: char, count: usize) -> Option<Motion> {
        let kind = match c {
            'h' => MotionKind::Left,
            'l' => MotionKind::Right,
            'j' => MotionKind::Down,
            'k' => MotionKind::Up,
            'w' => MotionKind::WordStart,
            'W' => MotionKind::WordStartBig,
            'b' => MotionKind::WordBack,
            'B' => MotionKind::WordBackBig,
            'e' => MotionKind::WordEnd,
            'E' => MotionKind::WordEndBig,
            '0' => MotionKind::LineStart,
            '^' => MotionKind::FirstNonBlank,
            '$' => MotionKind::LineEnd,
            'G' => MotionKind::FileEnd,
            '{' => MotionKind::ParagraphBack,
            '}' => MotionKind::ParagraphForward,
            '(' => MotionKind::SentenceBack,
            ')' => MotionKind::SentenceForward,
            '%' => MotionKind::MatchingBracket,
            _ => return None,
        };
        Some(Motion::with_count(kind, count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_key() {
        let mut state = ModeState::new();
        let result = NormalState::process_key(&mut state, Key::char('j'));
        assert!(matches!(
            result,
            InputResult::Parsed(ParsedInput::Motion(_))
        ));
    }

    #[test]
    fn test_count_accumulation() {
        let mut state = ModeState::new();
        NormalState::process_key(&mut state, Key::char('5'));
        assert_eq!(state.count, Some(5));
        NormalState::process_key(&mut state, Key::char('3'));
        assert_eq!(state.count, Some(53));
    }
}
