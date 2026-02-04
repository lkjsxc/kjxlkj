//! Key sequence parsing.

use kjxlkj_core_edit::{Motion, MotionKind, Operator, OperatorKind};
use kjxlkj_core_types::Mode;

use crate::intent::{Intent, IntentKind};
use crate::key::{Key, KeyCode, KeyModifiers};

/// Parser state for multi-key sequences.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum ParserState {
    /// Ready for new sequence.
    Ready,
    /// Waiting for motion after operator.
    WaitingMotion(OperatorKind),
    /// Waiting for text object after operator + i/a.
    WaitingTextObject(OperatorKind, bool),
    /// Waiting for register name after ".
    WaitingRegister,
    /// Waiting for mark name.
    WaitingMark,
    /// Waiting for search character (f/F/t/T).
    WaitingSearchChar(bool, bool),
    /// Waiting for g-prefixed command.
    WaitingG,
}

/// Key parser for modal editing.
#[derive(Debug)]
pub struct KeyParser {
    state: ParserState,
    command_buffer: String,
}

impl KeyParser {
    /// Create a new parser.
    pub fn new() -> Self {
        Self {
            state: ParserState::Ready,
            command_buffer: String::new(),
        }
    }

    /// Reset parser state.
    pub fn reset(&mut self) {
        self.state = ParserState::Ready;
        self.command_buffer.clear();
    }

    /// Process a key in Normal mode.
    pub fn process_normal(
        &mut self,
        key: Key,
        count: &mut Option<usize>,
        register: &mut Option<char>,
        mode: &mut Mode,
    ) -> Intent {
        // Handle count prefix
        if let KeyCode::Char(c) = key.code {
            if key.modifiers == KeyModifiers::NONE {
                if c.is_ascii_digit() && (c != '0' || count.is_some()) {
                    let digit = c.to_digit(10).unwrap() as usize;
                    *count = Some(count.unwrap_or(0) * 10 + digit);
                    return Intent::pending();
                }
            }
        }

        let final_count = count.take().unwrap_or(1);
        let final_register = register.take();

        match &self.state {
            ParserState::Ready => {
                self.process_normal_ready(key, final_count, final_register, mode)
            }
            ParserState::WaitingMotion(op_kind) => {
                let op_kind = *op_kind;
                self.process_waiting_motion(key, op_kind, final_count)
            }
            ParserState::WaitingRegister => {
                if let KeyCode::Char(c) = key.code {
                    *register = Some(c);
                    self.state = ParserState::Ready;
                    return Intent::pending();
                }
                self.reset();
                Intent::noop()
            }
            ParserState::WaitingG => self.process_g_command(key, final_count, mode),
            ParserState::WaitingSearchChar(forward, inclusive) => {
                let _forward = *forward;
                let _inclusive = *inclusive;
                if let KeyCode::Char(_c) = key.code {
                    self.state = ParserState::Ready;
                    // TODO: implement f/F/t/T search
                    return Intent::noop();
                }
                self.reset();
                Intent::noop()
            }
            _ => {
                self.reset();
                Intent::noop()
            }
        }
    }

    /// Process a key when in ready state (Normal mode).
    fn process_normal_ready(
        &mut self,
        key: Key,
        count: usize,
        _register: Option<char>,
        mode: &mut Mode,
    ) -> Intent {
        if key.modifiers == KeyModifiers::NONE {
            if let KeyCode::Char(c) = key.code {
                return match c {
                    // Mode entry
                    'i' => {
                        *mode = Mode::Insert;
                        Intent::new(IntentKind::EnterInsert { after: false })
                    }
                    'a' => {
                        *mode = Mode::Insert;
                        Intent::new(IntentKind::EnterInsert { after: true })
                    }
                    'I' => {
                        *mode = Mode::Insert;
                        Intent::new(IntentKind::EnterInsert { after: false })
                    }
                    'A' => {
                        *mode = Mode::Insert;
                        Intent::new(IntentKind::EnterInsert { after: true })
                    }
                    'o' => {
                        *mode = Mode::Insert;
                        Intent::new(IntentKind::EnterInsertLine { below: true })
                    }
                    'O' => {
                        *mode = Mode::Insert;
                        Intent::new(IntentKind::EnterInsertLine { below: false })
                    }
                    'v' => {
                        *mode = Mode::Visual;
                        Intent::new(IntentKind::EnterVisual)
                    }
                    'V' => {
                        *mode = Mode::VisualLine;
                        Intent::new(IntentKind::EnterVisualLine)
                    }
                    'R' => {
                        *mode = Mode::Replace;
                        Intent::new(IntentKind::EnterReplace)
                    }
                    ':' => {
                        *mode = Mode::Command;
                        Intent::new(IntentKind::EnterCommand)
                    }
                    '/' => Intent::new(IntentKind::SearchForward),
                    '?' => Intent::new(IntentKind::SearchBackward),
                    'n' => Intent::new(IntentKind::SearchNext).with_count(count),
                    'N' => Intent::new(IntentKind::SearchPrev).with_count(count),

                    // Motions
                    'h' => Intent::new(IntentKind::Move(Motion::char_motion(false, count))),
                    'l' => Intent::new(IntentKind::Move(Motion::char_motion(true, count))),
                    'j' => Intent::new(IntentKind::Move(Motion::line_motion(true, count))),
                    'k' => Intent::new(IntentKind::Move(Motion::line_motion(false, count))),
                    'w' => Intent::new(IntentKind::Move(Motion::word_motion(true, count))),
                    'b' => Intent::new(IntentKind::Move(Motion::word_motion(false, count))),
                    '0' => Intent::new(IntentKind::Move(Motion {
                        kind: MotionKind::Line,
                        forward: false,
                        count: 1,
                        inclusive: false,
                    })),
                    '$' => Intent::new(IntentKind::Move(Motion {
                        kind: MotionKind::Line,
                        forward: true,
                        count: 1,
                        inclusive: false,
                    })),

                    // Operators
                    'd' => {
                        self.state = ParserState::WaitingMotion(OperatorKind::Delete);
                        Intent::pending()
                    }
                    'c' => {
                        self.state = ParserState::WaitingMotion(OperatorKind::Change);
                        Intent::pending()
                    }
                    'y' => {
                        self.state = ParserState::WaitingMotion(OperatorKind::Yank);
                        Intent::pending()
                    }

                    // Direct actions
                    'x' => Intent::new(IntentKind::DeleteChar).with_count(count),
                    'X' => Intent::new(IntentKind::DeleteCharBefore).with_count(count),
                    'u' => Intent::new(IntentKind::Undo).with_count(count),
                    '.' => Intent::new(IntentKind::RepeatLast).with_count(count),
                    'J' => Intent::new(IntentKind::JoinLines).with_count(count),
                    'p' => Intent::noop(), // TODO: paste
                    'P' => Intent::noop(), // TODO: paste before

                    // Register prefix
                    '"' => {
                        self.state = ParserState::WaitingRegister;
                        Intent::pending()
                    }

                    // g-prefix commands
                    'g' => {
                        self.state = ParserState::WaitingG;
                        Intent::pending()
                    }

                    // Search char
                    'f' => {
                        self.state = ParserState::WaitingSearchChar(true, true);
                        Intent::pending()
                    }
                    'F' => {
                        self.state = ParserState::WaitingSearchChar(false, true);
                        Intent::pending()
                    }
                    't' => {
                        self.state = ParserState::WaitingSearchChar(true, false);
                        Intent::pending()
                    }
                    'T' => {
                        self.state = ParserState::WaitingSearchChar(false, false);
                        Intent::pending()
                    }

                    _ => Intent::noop(),
                };
            }
        }

        // Ctrl-key bindings
        if key.modifiers == KeyModifiers::CTRL {
            if let KeyCode::Char(c) = key.code {
                return match c {
                    'r' => Intent::new(IntentKind::Redo).with_count(count),
                    'u' => Intent::new(IntentKind::ScrollHalfUp),
                    'd' => Intent::new(IntentKind::ScrollHalfDown),
                    'b' => Intent::new(IntentKind::ScrollUp(count)),
                    'f' => Intent::new(IntentKind::ScrollDown(count)),
                    'v' => {
                        *mode = Mode::VisualBlock;
                        Intent::new(IntentKind::EnterVisualBlock)
                    }
                    _ => Intent::noop(),
                };
            }
        }

        Intent::noop()
    }

    /// Process motion after an operator.
    fn process_waiting_motion(&mut self, key: Key, op_kind: OperatorKind, count: usize) -> Intent {
        self.state = ParserState::Ready;

        if key.modifiers == KeyModifiers::NONE {
            if let KeyCode::Char(c) = key.code {
                let operator = Operator::new(op_kind).with_count(count);

                // Check for line-wise operator (dd, cc, yy)
                let double_key = match op_kind {
                    OperatorKind::Delete => 'd',
                    OperatorKind::Change => 'c',
                    OperatorKind::Yank => 'y',
                    _ => '\0',
                };
                if c == double_key {
                    return Intent::new(IntentKind::OperatorLine { operator });
                }

                // Motion keys
                let motion = match c {
                    'h' => Some(Motion::char_motion(false, count)),
                    'l' => Some(Motion::char_motion(true, count)),
                    'j' => Some(Motion::line_motion(true, count)),
                    'k' => Some(Motion::line_motion(false, count)),
                    'w' => Some(Motion::word_motion(true, count)),
                    'b' => Some(Motion::word_motion(false, count)),
                    '$' => Some(Motion {
                        kind: MotionKind::Line,
                        forward: true,
                        count: 1,
                        inclusive: true,
                    }),
                    '0' => Some(Motion {
                        kind: MotionKind::Line,
                        forward: false,
                        count: 1,
                        inclusive: false,
                    }),
                    _ => None,
                };

                if let Some(motion) = motion {
                    return Intent::new(IntentKind::OperatorMotion { operator, motion });
                }
            }
        }

        Intent::noop()
    }

    /// Process g-prefixed command.
    fn process_g_command(&mut self, key: Key, count: usize, _mode: &mut Mode) -> Intent {
        self.state = ParserState::Ready;

        if key.modifiers == KeyModifiers::NONE {
            if let KeyCode::Char(c) = key.code {
                return match c {
                    'g' => {
                        // gg - go to first line
                        Intent::new(IntentKind::Move(Motion {
                            kind: MotionKind::Line,
                            forward: false,
                            count,
                            inclusive: false,
                        }))
                    }
                    _ => Intent::noop(),
                };
            }
        }

        Intent::noop()
    }

    /// Process key in Visual mode.
    pub fn process_visual(&mut self, key: Key, count: &mut Option<usize>, mode: &mut Mode) -> Intent {
        let final_count = count.take().unwrap_or(1);

        if key.modifiers == KeyModifiers::NONE {
            if let KeyCode::Char(c) = key.code {
                return match c {
                    // Motions extend selection
                    'h' => Intent::new(IntentKind::Move(Motion::char_motion(false, final_count))),
                    'l' => Intent::new(IntentKind::Move(Motion::char_motion(true, final_count))),
                    'j' => Intent::new(IntentKind::Move(Motion::line_motion(true, final_count))),
                    'k' => Intent::new(IntentKind::Move(Motion::line_motion(false, final_count))),
                    'w' => Intent::new(IntentKind::Move(Motion::word_motion(true, final_count))),
                    'b' => Intent::new(IntentKind::Move(Motion::word_motion(false, final_count))),

                    // Operators on selection
                    'd' | 'x' => {
                        *mode = Mode::Normal;
                        Intent::new(IntentKind::OperatorLine {
                            operator: Operator::new(OperatorKind::Delete),
                        })
                    }
                    'y' => {
                        *mode = Mode::Normal;
                        Intent::new(IntentKind::OperatorLine {
                            operator: Operator::new(OperatorKind::Yank),
                        })
                    }
                    'c' => {
                        *mode = Mode::Insert;
                        Intent::new(IntentKind::OperatorLine {
                            operator: Operator::new(OperatorKind::Change),
                        })
                    }

                    _ => Intent::noop(),
                };
            }
        }

        Intent::noop()
    }

    /// Process key in Command mode.
    pub fn process_command(&mut self, key: Key, mode: &mut Mode) -> Intent {
        match key.code {
            KeyCode::Enter => {
                let cmd = self.command_buffer.clone();
                self.command_buffer.clear();
                *mode = Mode::Normal;
                Intent::new(IntentKind::ExecuteCommand(cmd))
            }
            KeyCode::Backspace => {
                if self.command_buffer.pop().is_none() {
                    *mode = Mode::Normal;
                    return Intent::new(IntentKind::ExitToNormal);
                }
                Intent::pending()
            }
            KeyCode::Char(c) if key.modifiers == KeyModifiers::NONE => {
                self.command_buffer.push(c);
                Intent::pending()
            }
            _ => Intent::pending(),
        }
    }
}

impl Default for KeyParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_count_prefix() {
        let mut parser = KeyParser::new();
        let mut count = None;
        let mut register = None;
        let mut mode = Mode::Normal;

        parser.process_normal(Key::char('3'), &mut count, &mut register, &mut mode);
        assert_eq!(count, Some(3));

        parser.process_normal(Key::char('5'), &mut count, &mut register, &mut mode);
        assert_eq!(count, Some(35));
    }

    #[test]
    fn parse_motion() {
        let mut parser = KeyParser::new();
        let mut count = None;
        let mut register = None;
        let mut mode = Mode::Normal;

        let intent = parser.process_normal(Key::char('j'), &mut count, &mut register, &mut mode);
        assert!(matches!(intent.kind, IntentKind::Move(_)));
    }

    #[test]
    fn parse_operator_motion() {
        let mut parser = KeyParser::new();
        let mut count = None;
        let mut register = None;
        let mut mode = Mode::Normal;

        let intent = parser.process_normal(Key::char('d'), &mut count, &mut register, &mut mode);
        assert!(intent.is_pending());

        let intent = parser.process_normal(Key::char('w'), &mut count, &mut register, &mut mode);
        assert!(matches!(intent.kind, IntentKind::OperatorMotion { .. }));
    }

    #[test]
    fn parse_dd() {
        let mut parser = KeyParser::new();
        let mut count = None;
        let mut register = None;
        let mut mode = Mode::Normal;

        parser.process_normal(Key::char('d'), &mut count, &mut register, &mut mode);
        let intent = parser.process_normal(Key::char('d'), &mut count, &mut register, &mut mode);
        assert!(matches!(intent.kind, IntentKind::OperatorLine { .. }));
    }
}
