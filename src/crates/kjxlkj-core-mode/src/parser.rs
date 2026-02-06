//! Key sequence parser: transforms key events into editor intents.

use kjxlkj_core_types::{
    CaseOp, FindCharKind, InsertPosition, Intent, KeyCode, KeyEvent, Mode,
    MotionKind, OperatorKind, PastePosition, RegisterName, ScrollKind,
    TextObjectKind,
};

/// Pending parse state for multi-key sequences.
#[derive(Debug, Clone, PartialEq, Eq)]
enum PendingState {
    None,
    Count(usize),
    Operator(OperatorKind, usize),
    G,
    GOperator(CaseOp, usize),
    Z,
    FindChar(FindCharKind),
    Register,
    Mark,
    JumpMark,
    JumpMarkLine,
    MacroRecord,
    MacroPlay,
    ReplaceChar,
    Leader,
}

/// Parses key events into intents based on current mode.
pub struct KeyParser {
    pending: PendingState,
    count_buf: Option<usize>,
    last_find: Option<(char, FindCharKind)>,
}

impl KeyParser {
    pub fn new() -> Self {
        Self {
            pending: PendingState::None,
            count_buf: None,
            last_find: None,
        }
    }

    /// Reset pending state (e.g., on Escape).
    pub fn reset(&mut self) {
        self.pending = PendingState::None;
        self.count_buf = None;
    }

    fn take_count(&mut self) -> usize {
        self.count_buf.take().unwrap_or(1)
    }

    /// Parse a key event in Normal mode and return an intent.
    pub fn parse_normal(&mut self, key: &KeyEvent) -> Intent {
        // Handle pending states first
        match &self.pending {
            PendingState::FindChar(kind) => {
                let kind = *kind;
                self.pending = PendingState::None;
                if let KeyCode::Char(c) = key.code {
                    self.last_find = Some((c, kind));
                    return Intent::FindChar(c, kind);
                }
                return Intent::Noop;
            }
            PendingState::Register => {
                self.pending = PendingState::None;
                if let KeyCode::Char(c) = key.code {
                    if let Some(reg) = RegisterName::from_char(c) {
                        return Intent::SelectRegister(reg);
                    }
                }
                return Intent::Noop;
            }
            PendingState::Mark => {
                self.pending = PendingState::None;
                if let KeyCode::Char(c) = key.code {
                    return Intent::SetMark(c);
                }
                return Intent::Noop;
            }
            PendingState::JumpMark => {
                self.pending = PendingState::None;
                if let KeyCode::Char(c) = key.code {
                    return Intent::JumpToMark(c);
                }
                return Intent::Noop;
            }
            PendingState::JumpMarkLine => {
                self.pending = PendingState::None;
                if let KeyCode::Char(c) = key.code {
                    return Intent::JumpToMarkLine(c);
                }
                return Intent::Noop;
            }
            PendingState::MacroRecord => {
                self.pending = PendingState::None;
                if let KeyCode::Char(c) = key.code {
                    return Intent::MacroToggleRecord(c);
                }
                return Intent::Noop;
            }
            PendingState::MacroPlay => {
                self.pending = PendingState::None;
                if let KeyCode::Char(c) = key.code {
                    if c == '@' {
                        return Intent::MacroRepeatLast;
                    }
                    return Intent::MacroPlay(c);
                }
                return Intent::Noop;
            }
            PendingState::ReplaceChar => {
                self.pending = PendingState::None;
                if let KeyCode::Char(c) = key.code {
                    return Intent::ReplaceChar(c);
                }
                return Intent::Noop;
            }
            PendingState::Leader => {
                self.pending = PendingState::None;
                return self.parse_leader_chord(key);
            }
            PendingState::G => {
                return self.parse_g_sequence(key);
            }
            PendingState::Z => {
                return self.parse_z_sequence(key);
            }
            PendingState::GOperator(op, count) => {
                let op = *op;
                let count = *count;
                self.pending = PendingState::None;
                return self.parse_g_operator(key, op, count);
            }
            PendingState::Operator(op, count) => {
                let op = *op;
                let count = *count;
                return self.parse_operator_motion(key, op, count);
            }
            _ => {}
        }

        // Count accumulation
        if let KeyCode::Char(c) = key.code {
            if c.is_ascii_digit() && !key.ctrl && !key.alt {
                if c == '0' && self.count_buf.is_none() {
                    // '0' alone is line-start motion
                    return Intent::Motion(MotionKind::LineStart, 1);
                }
                let digit = c as usize - '0' as usize;
                let curr = self.count_buf.unwrap_or(0);
                self.count_buf = Some(curr * 10 + digit);
                return Intent::Noop;
            }
        }

        let count = self.take_count();
        self.parse_normal_key(key, count)
    }

    fn parse_normal_key(&mut self, key: &KeyEvent, count: usize) -> Intent {
        if key.ctrl {
            return self.parse_ctrl_key(key, count);
        }
        match &key.code {
            KeyCode::Escape => Intent::EnterMode(Mode::Normal),
            KeyCode::Enter => Intent::Motion(MotionKind::NextNonBlankLine, count),
            KeyCode::Backspace => Intent::Motion(MotionKind::Left, count),
            KeyCode::Left => Intent::Motion(MotionKind::Left, count),
            KeyCode::Right => Intent::Motion(MotionKind::Right, count),
            KeyCode::Up => Intent::Motion(MotionKind::Up, count),
            KeyCode::Down => Intent::Motion(MotionKind::Down, count),
            KeyCode::Char(c) => self.parse_normal_char(*c, count),
            _ => Intent::Noop,
        }
    }

    fn parse_normal_char(&mut self, c: char, count: usize) -> Intent {
        match c {
            'h' => Intent::Motion(MotionKind::Left, count),
            'j' => Intent::Motion(MotionKind::Down, count),
            'k' => Intent::Motion(MotionKind::Up, count),
            'l' => Intent::Motion(MotionKind::Right, count),
            'w' => Intent::Motion(MotionKind::WordForward, count),
            'W' => Intent::Motion(MotionKind::WORDForward, count),
            'b' => Intent::Motion(MotionKind::WordBackward, count),
            'B' => Intent::Motion(MotionKind::WORDBackward, count),
            'e' => Intent::Motion(MotionKind::WordForwardEnd, count),
            'E' => Intent::Motion(MotionKind::WORDForwardEnd, count),
            '$' => Intent::Motion(MotionKind::LineEnd, 1),
            '^' | '_' => Intent::Motion(MotionKind::FirstNonBlank, 1),
            'G' => {
                if count > 1 {
                    Intent::Motion(MotionKind::GotoLine(count), 1)
                } else {
                    Intent::Motion(MotionKind::FileEnd, 1)
                }
            }
            '%' => {
                if count > 1 && count <= 100 {
                    Intent::Motion(MotionKind::GotoPercent(count), 1)
                } else {
                    Intent::Motion(MotionKind::MatchingBracket, 1)
                }
            }
            '|' => Intent::Motion(MotionKind::GotoColumn(count), 1),
            '+' => Intent::Motion(MotionKind::NextNonBlankLine, count),
            '-' => Intent::Motion(MotionKind::PrevNonBlankLine, count),
            'H' => Intent::Motion(MotionKind::ScreenTop, 1),
            'M' => Intent::Motion(MotionKind::ScreenMiddle, 1),
            'L' => Intent::Motion(MotionKind::ScreenBottom, 1),
            '{' => Intent::Motion(MotionKind::PrevParagraph, count),
            '}' => Intent::Motion(MotionKind::NextParagraph, count),
            '(' => Intent::Motion(MotionKind::PrevSentence, count),
            ')' => Intent::Motion(MotionKind::NextSentence, count),
            'i' => Intent::EnterInsert(InsertPosition::BeforeCursor),
            'I' => Intent::EnterInsert(InsertPosition::FirstNonBlank),
            'a' => Intent::EnterInsert(InsertPosition::AfterCursor),
            'A' => Intent::EnterInsert(InsertPosition::EndOfLine),
            'o' => Intent::OpenLine(true),
            'O' => Intent::OpenLine(false),
            'v' => Intent::EnterMode(Mode::Visual),
            'V' => Intent::EnterMode(Mode::VisualLine),
            'R' => Intent::EnterMode(Mode::Replace),
            ':' => Intent::EnterMode(Mode::Command),
            '/' => Intent::EnterMode(Mode::Command),
            '?' => Intent::EnterMode(Mode::Command),
            'x' => Intent::DeleteCharAt,
            'X' => Intent::DeleteCharBefore,
            'D' => Intent::DeleteToEnd,
            'C' => Intent::ChangeToEnd,
            's' => Intent::SubstituteChar,
            'S' => Intent::SubstituteLine,
            'Y' => Intent::YankLine(count),
            'p' => Intent::Paste(RegisterName::Unnamed, PastePosition::After),
            'P' => Intent::Paste(RegisterName::Unnamed, PastePosition::Before),
            'u' => Intent::Undo,
            '.' => Intent::RepeatLastChange,
            '*' => Intent::SearchWordForward,
            '#' => Intent::SearchWordBackward,
            'n' => Intent::SearchNext,
            'N' => Intent::SearchPrev,
            '~' => Intent::ToggleCase,
            'J' => Intent::JoinLines(true, count),
            'd' => {
                self.pending = PendingState::Operator(OperatorKind::Delete, count);
                Intent::Noop
            }
            'y' => {
                self.pending = PendingState::Operator(OperatorKind::Yank, count);
                Intent::Noop
            }
            'c' => {
                self.pending = PendingState::Operator(OperatorKind::Change, count);
                Intent::Noop
            }
            '>' => {
                self.pending = PendingState::Operator(OperatorKind::Indent, count);
                Intent::Noop
            }
            '<' => {
                self.pending = PendingState::Operator(OperatorKind::Outdent, count);
                Intent::Noop
            }
            'g' => {
                self.pending = PendingState::G;
                self.count_buf = if count > 1 { Some(count) } else { None };
                Intent::Noop
            }
            'z' => {
                self.pending = PendingState::Z;
                Intent::Noop
            }
            'r' => {
                self.pending = PendingState::ReplaceChar;
                Intent::Noop
            }
            'f' => {
                self.pending = PendingState::FindChar(FindCharKind::Forward);
                Intent::Noop
            }
            'F' => {
                self.pending = PendingState::FindChar(FindCharKind::Backward);
                Intent::Noop
            }
            't' => {
                self.pending = PendingState::FindChar(FindCharKind::TillForward);
                Intent::Noop
            }
            'T' => {
                self.pending = PendingState::FindChar(FindCharKind::TillBackward);
                Intent::Noop
            }
            ';' => Intent::RepeatFindChar,
            ',' => Intent::RepeatFindCharReverse,
            '"' => {
                self.pending = PendingState::Register;
                Intent::Noop
            }
            'm' => {
                self.pending = PendingState::Mark;
                Intent::Noop
            }
            '`' => {
                self.pending = PendingState::JumpMark;
                Intent::Noop
            }
            '\'' => {
                self.pending = PendingState::JumpMarkLine;
                Intent::Noop
            }
            'q' => {
                self.pending = PendingState::MacroRecord;
                Intent::Noop
            }
            '@' => {
                self.pending = PendingState::MacroPlay;
                Intent::Noop
            }
            'Z' => {
                self.pending = PendingState::Z; // ZZ/ZQ handling in Z
                Intent::Noop
            }
            ' ' => {
                self.pending = PendingState::Leader;
                Intent::Noop
            }
            _ => Intent::Noop,
        }
    }

    fn parse_ctrl_key(&mut self, key: &KeyEvent, count: usize) -> Intent {
        match &key.code {
            KeyCode::Char('r') => Intent::Redo,
            KeyCode::Char('d') => Intent::Scroll(ScrollKind::HalfPageDown),
            KeyCode::Char('u') => Intent::Scroll(ScrollKind::HalfPageUp),
            KeyCode::Char('f') => Intent::Scroll(ScrollKind::FullPageDown),
            KeyCode::Char('b') => Intent::Scroll(ScrollKind::FullPageUp),
            KeyCode::Char('e') => Intent::Scroll(ScrollKind::LineDown),
            KeyCode::Char('y') => Intent::Scroll(ScrollKind::LineUp),
            KeyCode::Char('o') => Intent::JumpListBack,
            KeyCode::Char('i') => Intent::JumpListForward,
            KeyCode::Char('a') => Intent::IncrementNumber(count as i64),
            KeyCode::Char('x') => Intent::IncrementNumber(-(count as i64)),
            KeyCode::Char('v') => Intent::EnterMode(Mode::VisualBlock),
            _ => Intent::Noop,
        }
    }

    fn parse_g_sequence(&mut self, key: &KeyEvent) -> Intent {
        self.pending = PendingState::None;
        let count = self.take_count();
        match &key.code {
            KeyCode::Char('g') => {
                if count > 1 {
                    Intent::Motion(MotionKind::GotoLine(count), 1)
                } else {
                    Intent::Motion(MotionKind::FileStart, 1)
                }
            }
            KeyCode::Char('_') => Intent::Motion(MotionKind::LastNonBlank, 1),
            KeyCode::Char('m') => Intent::Motion(MotionKind::MiddleOfLine, 1),
            KeyCode::Char('e') => Intent::Motion(MotionKind::WordBackwardEnd, count),
            KeyCode::Char('E') => Intent::Motion(MotionKind::WORDBackwardEnd, count),
            KeyCode::Char('J') => Intent::JoinLines(false, count),
            KeyCode::Char('~') => {
                self.pending = PendingState::GOperator(CaseOp::Toggle, count);
                Intent::Noop
            }
            KeyCode::Char('u') => {
                self.pending = PendingState::GOperator(CaseOp::Lower, count);
                Intent::Noop
            }
            KeyCode::Char('U') => {
                self.pending = PendingState::GOperator(CaseOp::Upper, count);
                Intent::Noop
            }
            KeyCode::Char('p') => {
                Intent::Paste(RegisterName::Unnamed, PastePosition::AfterCursorEnd)
            }
            KeyCode::Char('P') => {
                Intent::Paste(RegisterName::Unnamed, PastePosition::BeforeCursorEnd)
            }
            KeyCode::Char('*') => Intent::SearchWordForward,
            KeyCode::Char('#') => Intent::SearchWordBackward,
            KeyCode::Char(';') => Intent::ChangeListOlder,
            KeyCode::Char(',') => Intent::ChangeListNewer,
            _ => Intent::Noop,
        }
    }

    fn parse_z_sequence(&mut self, key: &KeyEvent) -> Intent {
        self.pending = PendingState::None;
        match &key.code {
            KeyCode::Char('z') => Intent::Scroll(ScrollKind::CursorCenter),
            KeyCode::Char('t') => Intent::Scroll(ScrollKind::CursorTop),
            KeyCode::Char('b') => Intent::Scroll(ScrollKind::CursorBottom),
            KeyCode::Char('.') => {
                Intent::Scroll(ScrollKind::CursorCenterFirstNonBlank)
            }
            KeyCode::Char('-') => {
                Intent::Scroll(ScrollKind::CursorBottomFirstNonBlank)
            }
            KeyCode::Enter => {
                Intent::Scroll(ScrollKind::CursorTopFirstNonBlank)
            }
            KeyCode::Char('Z') => {
                // ZZ → write and quit
                Intent::ExCommand(":wq".into())
            }
            KeyCode::Char('Q') => {
                // ZQ → quit without saving
                Intent::ExCommand(":q!".into())
            }
            _ => Intent::Noop,
        }
    }

    fn parse_leader_chord(&mut self, key: &KeyEvent) -> Intent {
        match &key.code {
            KeyCode::Char('e') => Intent::ExCommand(":explorer".into()),
            KeyCode::Char('t') => Intent::ExCommand(":terminal".into()),
            KeyCode::Char('f') => Intent::ExCommand(":find".into()),
            KeyCode::Char('g') => Intent::ExCommand(":livegrep".into()),
            KeyCode::Char('b') => Intent::ExCommand(":ls".into()),
            KeyCode::Char('u') => Intent::ExCommand(":undotree".into()),
            _ => Intent::Noop,
        }
    }

    fn parse_operator_motion(
        &mut self,
        key: &KeyEvent,
        op: OperatorKind,
        count: usize,
    ) -> Intent {
        self.pending = PendingState::None;
        match &key.code {
            KeyCode::Escape => Intent::Noop,
            KeyCode::Char(c) => {
                // Check for double-operator (dd, yy, cc, >>, <<)
                let double = match (op, c) {
                    (OperatorKind::Delete, 'd') => true,
                    (OperatorKind::Yank, 'y') => true,
                    (OperatorKind::Change, 'c') => true,
                    (OperatorKind::Indent, '>') => true,
                    (OperatorKind::Outdent, '<') => true,
                    _ => false,
                };
                if double {
                    return Intent::LineOperator(op, count);
                }
                // Check for text objects
                if *c == 'i' || *c == 'a' {
                    self.pending = PendingState::None;
                    // Next key determines text object
                    let inner = *c == 'i';
                    // Store for next parse
                    self.pending = PendingState::None;
                    // We need to wait for the next key
                    // This is handled by returning a special state
                    // For simplicity, we just handle common ones
                    return Intent::Noop; // Will be handled by operator-text-object
                }
                // Parse as motion
                if let Some(motion) = self.char_to_motion(*c) {
                    return Intent::Operator(op, motion, count);
                }
                Intent::Noop
            }
            _ => Intent::Noop,
        }
    }

    fn parse_g_operator(
        &mut self,
        key: &KeyEvent,
        case_op: CaseOp,
        count: usize,
    ) -> Intent {
        match &key.code {
            KeyCode::Char(c) => {
                // Check for double (g~~, guu, gUU)
                let double = match (case_op, c) {
                    (CaseOp::Toggle, '~') => true,
                    (CaseOp::Lower, 'u') => true,
                    (CaseOp::Upper, 'U') => true,
                    _ => false,
                };
                if double {
                    return Intent::CaseOperatorLine(case_op);
                }
                if let Some(motion) = self.char_to_motion(*c) {
                    return Intent::CaseOperator(case_op, motion, count);
                }
                Intent::Noop
            }
            _ => Intent::Noop,
        }
    }

    fn char_to_motion(&self, c: char) -> Option<MotionKind> {
        match c {
            'h' => Some(MotionKind::Left),
            'l' => Some(MotionKind::Right),
            'j' => Some(MotionKind::Down),
            'k' => Some(MotionKind::Up),
            'w' => Some(MotionKind::WordForward),
            'W' => Some(MotionKind::WORDForward),
            'b' => Some(MotionKind::WordBackward),
            'B' => Some(MotionKind::WORDBackward),
            'e' => Some(MotionKind::WordForwardEnd),
            'E' => Some(MotionKind::WORDForwardEnd),
            '0' => Some(MotionKind::LineStart),
            '$' => Some(MotionKind::LineEnd),
            '^' | '_' => Some(MotionKind::FirstNonBlank),
            'G' => Some(MotionKind::FileEnd),
            '{' => Some(MotionKind::PrevParagraph),
            '}' => Some(MotionKind::NextParagraph),
            '(' => Some(MotionKind::PrevSentence),
            ')' => Some(MotionKind::NextSentence),
            '%' => Some(MotionKind::MatchingBracket),
            _ => None,
        }
    }

    /// Parse a key in Insert mode.
    pub fn parse_insert(&mut self, key: &KeyEvent) -> Intent {
        if key.code == KeyCode::Escape {
            return Intent::EnterMode(Mode::Normal);
        }
        if key.ctrl {
            return match &key.code {
                KeyCode::Char('h') => Intent::DeleteCharBefore,
                KeyCode::Char('w') => Intent::ExCommand(":delete-word-before".into()),
                KeyCode::Char('u') => Intent::ExCommand(":delete-to-bol".into()),
                KeyCode::Char('j') | KeyCode::Char('m') => Intent::InsertNewline,
                KeyCode::Char('t') => Intent::Indent(true, 1),
                KeyCode::Char('d') => Intent::Indent(false, 1),
                KeyCode::Char('o') => Intent::ExCommand(":insert-normal".into()),
                _ => Intent::Noop,
            };
        }
        match &key.code {
            KeyCode::Char(c) => Intent::InsertChar(*c),
            KeyCode::Enter => Intent::InsertNewline,
            KeyCode::Backspace => Intent::DeleteCharBefore,
            KeyCode::Delete => Intent::DeleteCharAt,
            KeyCode::Tab => Intent::InsertChar('\t'),
            KeyCode::Left => Intent::Motion(MotionKind::Left, 1),
            KeyCode::Right => Intent::Motion(MotionKind::Right, 1),
            KeyCode::Up => Intent::Motion(MotionKind::Up, 1),
            KeyCode::Down => Intent::Motion(MotionKind::Down, 1),
            KeyCode::Home => Intent::Motion(MotionKind::LineStart, 1),
            KeyCode::End => Intent::Motion(MotionKind::LineEnd, 1),
            _ => Intent::Noop,
        }
    }

    /// Parse a key in Visual mode.
    pub fn parse_visual(&mut self, key: &KeyEvent) -> Intent {
        if key.code == KeyCode::Escape {
            return Intent::EnterMode(Mode::Normal);
        }
        if key.ctrl {
            return self.parse_ctrl_key(key, 1);
        }
        match &key.code {
            KeyCode::Char('h') | KeyCode::Left => Intent::Motion(MotionKind::Left, 1),
            KeyCode::Char('j') | KeyCode::Down => Intent::Motion(MotionKind::Down, 1),
            KeyCode::Char('k') | KeyCode::Up => Intent::Motion(MotionKind::Up, 1),
            KeyCode::Char('l') | KeyCode::Right => Intent::Motion(MotionKind::Right, 1),
            KeyCode::Char('w') => Intent::Motion(MotionKind::WordForward, 1),
            KeyCode::Char('b') => Intent::Motion(MotionKind::WordBackward, 1),
            KeyCode::Char('e') => Intent::Motion(MotionKind::WordForwardEnd, 1),
            KeyCode::Char('0') => Intent::Motion(MotionKind::LineStart, 1),
            KeyCode::Char('^') => Intent::Motion(MotionKind::FirstNonBlank, 1),
            KeyCode::Char('$') => Intent::Motion(MotionKind::LineEnd, 1),
            KeyCode::Char('G') => Intent::Motion(MotionKind::FileEnd, 1),
            KeyCode::Char('d') | KeyCode::Char('x') => {
                Intent::Operator(OperatorKind::Delete, MotionKind::Right, 1)
            }
            KeyCode::Char('y') => {
                Intent::Operator(OperatorKind::Yank, MotionKind::Right, 1)
            }
            KeyCode::Char('c') | KeyCode::Char('s') => {
                Intent::Operator(OperatorKind::Change, MotionKind::Right, 1)
            }
            KeyCode::Char('>') => Intent::Indent(true, 1),
            KeyCode::Char('<') => Intent::Indent(false, 1),
            KeyCode::Char('o') => Intent::VisualSwapEnd,
            _ => Intent::Noop,
        }
    }

    /// Parse a key in Command mode.
    pub fn parse_command(&mut self, key: &KeyEvent) -> Intent {
        match &key.code {
            KeyCode::Escape => Intent::EnterMode(Mode::Normal),
            _ => Intent::Noop, // Command line handles its own editing
        }
    }

    /// Parse a key in Replace mode.
    pub fn parse_replace(&mut self, key: &KeyEvent) -> Intent {
        if key.code == KeyCode::Escape {
            return Intent::EnterMode(Mode::Normal);
        }
        match &key.code {
            KeyCode::Char(c) => Intent::ReplaceInsert(*c),
            KeyCode::Backspace => Intent::Motion(MotionKind::Left, 1),
            _ => Intent::Noop,
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
    fn basic_motions() {
        let mut p = KeyParser::new();
        assert_eq!(
            p.parse_normal(&KeyEvent::char('h')),
            Intent::Motion(MotionKind::Left, 1)
        );
        assert_eq!(
            p.parse_normal(&KeyEvent::char('j')),
            Intent::Motion(MotionKind::Down, 1)
        );
    }

    #[test]
    fn count_prefix() {
        let mut p = KeyParser::new();
        p.parse_normal(&KeyEvent::char('3'));
        assert_eq!(
            p.parse_normal(&KeyEvent::char('j')),
            Intent::Motion(MotionKind::Down, 3)
        );
    }

    #[test]
    fn insert_mode_entry() {
        let mut p = KeyParser::new();
        assert_eq!(
            p.parse_normal(&KeyEvent::char('i')),
            Intent::EnterInsert(InsertPosition::BeforeCursor)
        );
    }

    #[test]
    fn escape_in_insert() {
        let mut p = KeyParser::new();
        assert_eq!(
            p.parse_insert(&KeyEvent::special(KeyCode::Escape)),
            Intent::EnterMode(Mode::Normal)
        );
    }

    #[test]
    fn gg_goes_to_file_start() {
        let mut p = KeyParser::new();
        assert_eq!(p.parse_normal(&KeyEvent::char('g')), Intent::Noop);
        assert_eq!(
            p.parse_normal(&KeyEvent::char('g')),
            Intent::Motion(MotionKind::FileStart, 1)
        );
    }

    #[test]
    fn dd_deletes_line() {
        let mut p = KeyParser::new();
        assert_eq!(p.parse_normal(&KeyEvent::char('d')), Intent::Noop);
        assert_eq!(
            p.parse_normal(&KeyEvent::char('d')),
            Intent::LineOperator(OperatorKind::Delete, 1)
        );
    }

    #[test]
    fn dw_deletes_word() {
        let mut p = KeyParser::new();
        p.parse_normal(&KeyEvent::char('d'));
        assert_eq!(
            p.parse_normal(&KeyEvent::char('w')),
            Intent::Operator(OperatorKind::Delete, MotionKind::WordForward, 1)
        );
    }

    #[test]
    fn undo_redo() {
        let mut p = KeyParser::new();
        assert_eq!(p.parse_normal(&KeyEvent::char('u')), Intent::Undo);
        assert_eq!(p.parse_normal(&KeyEvent::ctrl('r')), Intent::Redo);
    }

    #[test]
    fn zz_scroll_center() {
        let mut p = KeyParser::new();
        p.parse_normal(&KeyEvent::char('z'));
        assert_eq!(
            p.parse_normal(&KeyEvent::char('z')),
            Intent::Scroll(ScrollKind::CursorCenter)
        );
    }

    #[test]
    fn find_char() {
        let mut p = KeyParser::new();
        p.parse_normal(&KeyEvent::char('f'));
        assert_eq!(
            p.parse_normal(&KeyEvent::char('x')),
            Intent::FindChar('x', FindCharKind::Forward)
        );
    }

    #[test]
    fn leader_key() {
        let mut p = KeyParser::new();
        p.parse_normal(&KeyEvent::char(' '));
        assert_eq!(
            p.parse_normal(&KeyEvent::char('e')),
            Intent::ExCommand(":explorer".into())
        );
    }

    #[test]
    fn visual_mode_operators() {
        let mut p = KeyParser::new();
        assert_eq!(
            p.parse_visual(&KeyEvent::char('d')),
            Intent::Operator(OperatorKind::Delete, MotionKind::Right, 1)
        );
    }

    #[test]
    fn insert_char_in_insert_mode() {
        let mut p = KeyParser::new();
        assert_eq!(
            p.parse_insert(&KeyEvent::char('a')),
            Intent::InsertChar('a')
        );
    }

    #[test]
    fn replace_mode() {
        let mut p = KeyParser::new();
        assert_eq!(
            p.parse_replace(&KeyEvent::char('x')),
            Intent::ReplaceInsert('x')
        );
    }
}
