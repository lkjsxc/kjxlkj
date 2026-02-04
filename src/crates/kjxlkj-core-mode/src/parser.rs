//! Key sequence parser for command composition.

use kjxlkj_core_types::{KeyCode, KeyEvent, MotionIntent};

/// Result of parsing a key sequence.
#[derive(Debug, Clone)]
pub enum ParseResult {
    /// Complete command parsed.
    Complete(ParsedCommand),
    /// More keys needed.
    Pending,
    /// Invalid sequence.
    Invalid,
}

/// A parsed command with optional count and operator.
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    /// Count prefix (default 1).
    pub count: usize,
    /// Optional operator.
    pub operator: Option<OperatorKind>,
    /// Motion or action.
    pub action: ActionKind,
}

/// Operator types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorKind {
    Delete,
    Yank,
    Change,
    Indent,
    Outdent,
    ToggleCase,
    Uppercase,
    Lowercase,
}

/// Action types (motions or commands).
#[derive(Debug, Clone)]
pub enum ActionKind {
    Motion(MotionIntent),
    Line, // For dd, yy, cc
    InsertMode,
    AppendMode,
    InsertLineStart,
    AppendLineEnd,
    OpenBelow,
    OpenAbove,
    VisualMode,
    VisualLineMode,
    VisualBlockMode,
    CommandMode,
    ReplaceMode,
    ReplaceChar(char),
    Undo,
    Redo,
    Paste { before: bool },
    Repeat,
    JoinLines { add_space: bool },
    Search { forward: bool },
    NextMatch,
    PrevMatch,
    SearchWord { forward: bool },
    DeleteChar,
    DeleteCharBefore,
    Substitute,
    SubstituteLine,
    DeleteToEnd,
    ChangeToEnd,
    YankLine,
    SetMark(char),
    JumpMark { mark: char, first_non_blank: bool },
    MacroToggle(char),
    MacroPlay(char),
    RepeatMacro,
    SelectRegister(char),
    Scroll(ScrollAction),
    ZCommand(ZAction),
    GCommand(GAction),
    FindChar { c: char, forward: bool, inclusive: bool },
    RepeatFind,
    RepeatFindBack,
    Increment(i64),
    WriteQuit,
    QuitNoSave,
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollAction {
    HalfPageDown,
    HalfPageUp,
    PageDown,
    PageUp,
    LineDown,
    LineUp,
}

#[derive(Debug, Clone, Copy)]
pub enum ZAction {
    CenterCursor,
    CursorToTop,
    CursorToBottom,
    CenterFirstNonBlank,
    TopFirstNonBlank,
    BottomFirstNonBlank,
}

#[derive(Debug, Clone, Copy)]
pub enum GAction {
    FileStart,
    LastNonBlank,
    MiddleLine,
    ToggleCaseLine,
    UppercaseLine,
    LowercaseLine,
    JoinNoSpace,
    PrevWordEnd,
}

/// Parser state for key sequences.
#[derive(Debug, Clone, Default)]
pub struct Parser {
    /// Accumulated count.
    count: Option<usize>,
    /// Current operator.
    operator: Option<OperatorKind>,
    /// Pending keys for multi-key commands.
    pending: Vec<KeyEvent>,
    /// Waiting for a character (f, t, r, m, etc.).
    waiting_for_char: Option<WaitingFor>,
}

#[derive(Debug, Clone, Copy)]
enum WaitingFor {
    FindChar { forward: bool, inclusive: bool },
    ReplaceChar,
    Mark,
    JumpMark,
    JumpMarkLine,
    Register,
    Macro,
    MacroPlay,
}

impl Parser {
    /// Create a new parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a key event.
    pub fn parse(&mut self, key: &KeyEvent) -> ParseResult {
        // Handle character waiting
        if let Some(waiting) = self.waiting_for_char {
            return self.handle_waiting_char(key, waiting);
        }

        // Handle digit for count
        if let KeyCode::Char(c) = &key.code {
            if c.is_ascii_digit() {
                let digit = *c as usize - '0' as usize;
                if self.count.is_none() && digit == 0 {
                    // 0 is a motion (line start)
                    return self.complete_motion(MotionIntent::LineStart);
                }
                self.count = Some(self.count.unwrap_or(0) * 10 + digit);
                return ParseResult::Pending;
            }
        }

        // Handle operators
        if self.operator.is_none() {
            if let Some(op) = self.try_parse_operator(key) {
                self.operator = Some(op);
                return ParseResult::Pending;
            }
        }

        // Handle operator-operator (dd, yy, cc)
        if let Some(op) = &self.operator {
            if self.is_same_operator(key, *op) {
                return self.complete_line_action();
            }
        }

        // Handle motions and other keys
        self.parse_motion_or_action(key)
    }

    /// Reset parser state.
    pub fn reset(&mut self) {
        self.count = None;
        self.operator = None;
        self.pending.clear();
        self.waiting_for_char = None;
    }

    fn handle_waiting_char(&mut self, key: &KeyEvent, waiting: WaitingFor) -> ParseResult {
        self.waiting_for_char = None;

        if let KeyCode::Char(c) = &key.code {
            let result = match waiting {
                WaitingFor::FindChar { forward, inclusive } => {
                    self.complete_motion(MotionIntent::FindChar {
                        c: *c,
                        inclusive,
                    })
                }
                WaitingFor::ReplaceChar => self.complete_action(ActionKind::ReplaceChar(*c)),
                WaitingFor::Mark => self.complete_action(ActionKind::SetMark(*c)),
                WaitingFor::JumpMark => self.complete_action(ActionKind::JumpMark {
                    mark: *c,
                    first_non_blank: false,
                }),
                WaitingFor::JumpMarkLine => self.complete_action(ActionKind::JumpMark {
                    mark: *c,
                    first_non_blank: true,
                }),
                WaitingFor::Register => self.complete_action(ActionKind::SelectRegister(*c)),
                WaitingFor::Macro => self.complete_action(ActionKind::MacroToggle(*c)),
                WaitingFor::MacroPlay => self.complete_action(ActionKind::MacroPlay(*c)),
            };
            return result;
        }

        self.reset();
        ParseResult::Invalid
    }

    fn try_parse_operator(&self, key: &KeyEvent) -> Option<OperatorKind> {
        if key.modifiers.any() {
            return None;
        }

        match &key.code {
            KeyCode::Char('d') => Some(OperatorKind::Delete),
            KeyCode::Char('y') => Some(OperatorKind::Yank),
            KeyCode::Char('c') => Some(OperatorKind::Change),
            KeyCode::Char('>') => Some(OperatorKind::Indent),
            KeyCode::Char('<') => Some(OperatorKind::Outdent),
            _ => None,
        }
    }

    fn is_same_operator(&self, key: &KeyEvent, op: OperatorKind) -> bool {
        match (&key.code, op) {
            (KeyCode::Char('d'), OperatorKind::Delete) => true,
            (KeyCode::Char('y'), OperatorKind::Yank) => true,
            (KeyCode::Char('c'), OperatorKind::Change) => true,
            (KeyCode::Char('>'), OperatorKind::Indent) => true,
            (KeyCode::Char('<'), OperatorKind::Outdent) => true,
            _ => false,
        }
    }

    fn parse_motion_or_action(&mut self, key: &KeyEvent) -> ParseResult {
        // Handle Ctrl combinations
        if key.modifiers.ctrl {
            return self.parse_ctrl_key(key);
        }

        match &key.code {
            KeyCode::Char('h') | KeyCode::Left => self.complete_motion(MotionIntent::Left),
            KeyCode::Char('l') | KeyCode::Right => self.complete_motion(MotionIntent::Right),
            KeyCode::Char('j') | KeyCode::Down => self.complete_motion(MotionIntent::Down),
            KeyCode::Char('k') | KeyCode::Up => self.complete_motion(MotionIntent::Up),
            KeyCode::Char(' ') => self.complete_motion(MotionIntent::Right),
            KeyCode::Backspace => self.complete_motion(MotionIntent::Left),
            KeyCode::Char('0') => self.complete_motion(MotionIntent::LineStart),
            KeyCode::Char('^') => self.complete_motion(MotionIntent::FirstNonBlank),
            KeyCode::Char('$') => self.complete_motion(MotionIntent::LineEnd),
            KeyCode::Char('w') => self.complete_motion(MotionIntent::WordStart),
            KeyCode::Char('W') => self.complete_motion(MotionIntent::WORDStart),
            KeyCode::Char('b') => self.complete_motion(MotionIntent::WordStartBack),
            KeyCode::Char('B') => self.complete_motion(MotionIntent::WORDStartBack),
            KeyCode::Char('e') => self.complete_motion(MotionIntent::WordEnd),
            KeyCode::Char('E') => self.complete_motion(MotionIntent::WORDEnd),
            KeyCode::Char('G') => {
                if self.count.is_some() {
                    let line = self.count.unwrap_or(1);
                    self.complete_motion(MotionIntent::GotoLine(line))
                } else {
                    self.complete_motion(MotionIntent::FileEnd)
                }
            }
            KeyCode::Char('H') => self.complete_motion(MotionIntent::ScreenTop),
            KeyCode::Char('M') => self.complete_motion(MotionIntent::ScreenMiddle),
            KeyCode::Char('L') => self.complete_motion(MotionIntent::ScreenBottom),
            KeyCode::Char('{') => self.complete_motion(MotionIntent::PrevParagraph),
            KeyCode::Char('}') => self.complete_motion(MotionIntent::NextParagraph),
            KeyCode::Char('(') => self.complete_motion(MotionIntent::PrevSentence),
            KeyCode::Char(')') => self.complete_motion(MotionIntent::NextSentence),
            KeyCode::Char('%') => {
                if self.count.is_some() {
                    let pct = self.count.unwrap_or(50).min(100) as u8;
                    self.complete_motion(MotionIntent::GotoPercent(pct))
                } else {
                    self.complete_motion(MotionIntent::MatchingBracket)
                }
            }
            KeyCode::Char('|') => {
                let col = self.count.unwrap_or(1);
                self.complete_motion(MotionIntent::GotoColumn(col))
            }
            // Mode changes
            KeyCode::Char('i') => self.complete_action(ActionKind::InsertMode),
            KeyCode::Char('a') => self.complete_action(ActionKind::AppendMode),
            KeyCode::Char('I') => self.complete_action(ActionKind::InsertLineStart),
            KeyCode::Char('A') => self.complete_action(ActionKind::AppendLineEnd),
            KeyCode::Char('o') => self.complete_action(ActionKind::OpenBelow),
            KeyCode::Char('O') => self.complete_action(ActionKind::OpenAbove),
            KeyCode::Char('v') => self.complete_action(ActionKind::VisualMode),
            KeyCode::Char('V') => self.complete_action(ActionKind::VisualLineMode),
            KeyCode::Char('R') => self.complete_action(ActionKind::ReplaceMode),
            KeyCode::Char(':') => self.complete_action(ActionKind::CommandMode),
            // Edit commands
            KeyCode::Char('x') => self.complete_action(ActionKind::DeleteChar),
            KeyCode::Char('X') => self.complete_action(ActionKind::DeleteCharBefore),
            KeyCode::Char('s') => self.complete_action(ActionKind::Substitute),
            KeyCode::Char('S') => self.complete_action(ActionKind::SubstituteLine),
            KeyCode::Char('D') => self.complete_action(ActionKind::DeleteToEnd),
            KeyCode::Char('C') => self.complete_action(ActionKind::ChangeToEnd),
            KeyCode::Char('Y') => self.complete_action(ActionKind::YankLine),
            KeyCode::Char('p') => self.complete_action(ActionKind::Paste { before: false }),
            KeyCode::Char('P') => self.complete_action(ActionKind::Paste { before: true }),
            KeyCode::Char('u') => self.complete_action(ActionKind::Undo),
            KeyCode::Char('.') => self.complete_action(ActionKind::Repeat),
            KeyCode::Char('J') => self.complete_action(ActionKind::JoinLines { add_space: true }),
            // Search
            KeyCode::Char('/') => self.complete_action(ActionKind::Search { forward: true }),
            KeyCode::Char('?') => self.complete_action(ActionKind::Search { forward: false }),
            KeyCode::Char('n') => self.complete_action(ActionKind::NextMatch),
            KeyCode::Char('N') => self.complete_action(ActionKind::PrevMatch),
            KeyCode::Char('*') => self.complete_action(ActionKind::SearchWord { forward: true }),
            KeyCode::Char('#') => self.complete_action(ActionKind::SearchWord { forward: false }),
            // Character waiting
            KeyCode::Char('f') => {
                self.waiting_for_char = Some(WaitingFor::FindChar {
                    forward: true,
                    inclusive: true,
                });
                ParseResult::Pending
            }
            KeyCode::Char('F') => {
                self.waiting_for_char = Some(WaitingFor::FindChar {
                    forward: false,
                    inclusive: true,
                });
                ParseResult::Pending
            }
            KeyCode::Char('t') => {
                self.waiting_for_char = Some(WaitingFor::FindChar {
                    forward: true,
                    inclusive: false,
                });
                ParseResult::Pending
            }
            KeyCode::Char('T') => {
                self.waiting_for_char = Some(WaitingFor::FindChar {
                    forward: false,
                    inclusive: false,
                });
                ParseResult::Pending
            }
            KeyCode::Char(';') => self.complete_action(ActionKind::RepeatFind),
            KeyCode::Char(',') => self.complete_action(ActionKind::RepeatFindBack),
            KeyCode::Char('r') => {
                self.waiting_for_char = Some(WaitingFor::ReplaceChar);
                ParseResult::Pending
            }
            KeyCode::Char('m') => {
                self.waiting_for_char = Some(WaitingFor::Mark);
                ParseResult::Pending
            }
            KeyCode::Char('`') => {
                self.waiting_for_char = Some(WaitingFor::JumpMark);
                ParseResult::Pending
            }
            KeyCode::Char('\'') => {
                self.waiting_for_char = Some(WaitingFor::JumpMarkLine);
                ParseResult::Pending
            }
            KeyCode::Char('"') => {
                self.waiting_for_char = Some(WaitingFor::Register);
                ParseResult::Pending
            }
            KeyCode::Char('q') => {
                self.waiting_for_char = Some(WaitingFor::Macro);
                ParseResult::Pending
            }
            KeyCode::Char('@') => {
                self.waiting_for_char = Some(WaitingFor::MacroPlay);
                ParseResult::Pending
            }
            KeyCode::Char('~') => self.complete_action(ActionKind::Increment(0)), // Toggle case
            KeyCode::Char('g') => {
                self.pending.push(key.clone());
                ParseResult::Pending
            }
            KeyCode::Char('z') => {
                self.pending.push(key.clone());
                ParseResult::Pending
            }
            KeyCode::Char('Z') => {
                self.pending.push(key.clone());
                ParseResult::Pending
            }
            KeyCode::Char('+') | KeyCode::Enter => {
                self.complete_motion(MotionIntent::Down) // Move to first non-blank of next line
            }
            KeyCode::Char('-') => {
                self.complete_motion(MotionIntent::Up) // Move to first non-blank of prev line
            }
            KeyCode::Char('_') => self.complete_motion(MotionIntent::FirstNonBlank),
            _ => {
                self.reset();
                ParseResult::Invalid
            }
        }
    }

    fn parse_ctrl_key(&mut self, key: &KeyEvent) -> ParseResult {
        match &key.code {
            KeyCode::Char('r') => self.complete_action(ActionKind::Redo),
            KeyCode::Char('d') => {
                self.complete_action(ActionKind::Scroll(ScrollAction::HalfPageDown))
            }
            KeyCode::Char('u') => {
                self.complete_action(ActionKind::Scroll(ScrollAction::HalfPageUp))
            }
            KeyCode::Char('f') => self.complete_action(ActionKind::Scroll(ScrollAction::PageDown)),
            KeyCode::Char('b') => self.complete_action(ActionKind::Scroll(ScrollAction::PageUp)),
            KeyCode::Char('e') => self.complete_action(ActionKind::Scroll(ScrollAction::LineDown)),
            KeyCode::Char('y') => self.complete_action(ActionKind::Scroll(ScrollAction::LineUp)),
            KeyCode::Char('a') => self.complete_action(ActionKind::Increment(1)),
            KeyCode::Char('x') => self.complete_action(ActionKind::Increment(-1)),
            KeyCode::Char('v') => self.complete_action(ActionKind::VisualBlockMode),
            KeyCode::Char('o') => {
                // Jump back in jumplist
                self.complete_motion(MotionIntent::FileStart) // Placeholder
            }
            KeyCode::Char('i') => {
                // Jump forward in jumplist
                self.complete_motion(MotionIntent::FileEnd) // Placeholder
            }
            _ => {
                self.reset();
                ParseResult::Invalid
            }
        }
    }

    fn complete_motion(&mut self, motion: MotionIntent) -> ParseResult {
        let cmd = ParsedCommand {
            count: self.count.unwrap_or(1),
            operator: self.operator.take(),
            action: ActionKind::Motion(motion),
        };
        self.reset();
        ParseResult::Complete(cmd)
    }

    fn complete_action(&mut self, action: ActionKind) -> ParseResult {
        let cmd = ParsedCommand {
            count: self.count.unwrap_or(1),
            operator: self.operator.take(),
            action,
        };
        self.reset();
        ParseResult::Complete(cmd)
    }

    fn complete_line_action(&mut self) -> ParseResult {
        let cmd = ParsedCommand {
            count: self.count.unwrap_or(1),
            operator: self.operator.take(),
            action: ActionKind::Line,
        };
        self.reset();
        ParseResult::Complete(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::KeyModifiers;

    #[test]
    fn test_parse_motion() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('j'));
        assert!(matches!(result, ParseResult::Complete(_)));
    }

    #[test]
    fn test_parse_count() {
        let mut parser = Parser::new();
        parser.parse(&KeyEvent::char('3'));
        let result = parser.parse(&KeyEvent::char('j'));
        if let ParseResult::Complete(cmd) = result {
            assert_eq!(cmd.count, 3);
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_operator_motion() {
        let mut parser = Parser::new();
        parser.parse(&KeyEvent::char('d'));
        let result = parser.parse(&KeyEvent::char('w'));
        if let ParseResult::Complete(cmd) = result {
            assert!(cmd.operator.is_some());
            assert!(matches!(cmd.operator, Some(OperatorKind::Delete)));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_dd() {
        let mut parser = Parser::new();
        parser.parse(&KeyEvent::char('d'));
        let result = parser.parse(&KeyEvent::char('d'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Line));
            assert!(matches!(cmd.operator, Some(OperatorKind::Delete)));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_yy() {
        let mut parser = Parser::new();
        parser.parse(&KeyEvent::char('y'));
        let result = parser.parse(&KeyEvent::char('y'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Line));
            assert!(matches!(cmd.operator, Some(OperatorKind::Yank)));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_cc() {
        let mut parser = Parser::new();
        parser.parse(&KeyEvent::char('c'));
        let result = parser.parse(&KeyEvent::char('c'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Line));
            assert!(matches!(cmd.operator, Some(OperatorKind::Change)));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_undo() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('u'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Undo));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_paste() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('p'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Paste { before: false }));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_paste_before() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('P'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Paste { before: true }));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_visual_mode() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('v'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::VisualMode));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_visual_line_mode() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('V'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::VisualLineMode));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_command_mode() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char(':'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::CommandMode));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_insert_mode() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('i'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::InsertMode));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_append_mode() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('a'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::AppendMode));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_open_below() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('o'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::OpenBelow));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_open_above() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('O'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::OpenAbove));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_repeat() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('.'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Repeat));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_delete_char() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('x'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::DeleteChar));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parser_reset() {
        let mut parser = Parser::new();
        parser.parse(&KeyEvent::char('d'));
        parser.reset();
        // After reset, new command should work
        let result = parser.parse(&KeyEvent::char('j'));
        assert!(matches!(result, ParseResult::Complete(_)));
    }

    #[test]
    fn test_parse_large_count() {
        let mut parser = Parser::new();
        parser.parse(&KeyEvent::char('9'));
        parser.parse(&KeyEvent::char('9'));
        let result = parser.parse(&KeyEvent::char('j'));
        if let ParseResult::Complete(cmd) = result {
            assert_eq!(cmd.count, 99);
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_search_forward() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('/'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Search { forward: true }));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_search_backward() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('?'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Search { forward: false }));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_next_match() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('n'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::NextMatch));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_prev_match() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('N'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::PrevMatch));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_join_lines() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('J'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::JoinLines { add_space: true }));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_replace_mode() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('R'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::ReplaceMode));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_substitute() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('s'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::Substitute));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_substitute_line() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('S'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::SubstituteLine));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_delete_to_end() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('D'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::DeleteToEnd));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_change_to_end() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('C'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::ChangeToEnd));
        } else {
            panic!("Expected complete");
        }
    }

    #[test]
    fn test_parse_yank_line() {
        let mut parser = Parser::new();
        let result = parser.parse(&KeyEvent::char('Y'));
        if let ParseResult::Complete(cmd) = result {
            assert!(matches!(cmd.action, ActionKind::YankLine));
        } else {
            panic!("Expected complete");
        }
    }
}
