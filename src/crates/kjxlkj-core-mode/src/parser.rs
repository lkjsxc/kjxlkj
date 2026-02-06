//! Key sequence parser: transforms key events into editor intents.

use kjxlkj_core_types::{
    CaseOp, FindCharKind, Intent, KeyCode, KeyEvent, MotionKind,
    OperatorKind, RegisterName,
};

/// Pending parse state for multi-key sequences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PendingState {
    None,
    Count(usize),
    Operator(OperatorKind, usize),
    OperatorTextObject(OperatorKind, bool, usize),
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
    InsertRegister,
    Leader,
}

/// Parses key events into intents based on current mode.
pub struct KeyParser {
    pub(crate) pending: PendingState,
    pub(crate) count_buf: Option<usize>,
    pub(crate) last_find: Option<(char, FindCharKind)>,
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
                    if let Some(reg) =
                        RegisterName::from_char(c)
                    {
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
                return crate::parser_sequences::parse_leader_chord(key);
            }
            PendingState::G => {
                let count = self.take_count();
                return crate::parser_sequences::parse_g_sequence(
                    &mut self.pending,
                    count,
                    key,
                );
            }
            PendingState::Z => {
                return crate::parser_sequences::parse_z_sequence(
                    &mut self.pending,
                    key,
                );
            }
            PendingState::GOperator(op, count) => {
                let op = *op;
                let count = *count;
                self.pending = PendingState::None;
                return crate::parser_sequences::parse_g_operator(
                    key,
                    op,
                    count,
                    &|c| self_char_to_motion(c),
                );
            }
            PendingState::Operator(op, count) => {
                let op = *op;
                let count = *count;
                return crate::parser_sequences::parse_operator_motion(
                    &mut self.pending,
                    key,
                    op,
                    count,
                    &|c| self_char_to_motion(c),
                );
            }
            PendingState::OperatorTextObject(op, inner, _count) => {
                let op = *op;
                let inner = *inner;
                self.pending = PendingState::None;
                return crate::parser_sequences::parse_text_object_key(
                    key, op, inner,
                );
            }
            _ => {}
        }

        // Count accumulation
        if let KeyCode::Char(c) = key.code {
            if c.is_ascii_digit() && !key.ctrl && !key.alt {
                if c == '0' && self.count_buf.is_none() {
                    return Intent::Motion(
                        MotionKind::LineStart,
                        1,
                    );
                }
                let digit = c as usize - '0' as usize;
                let curr = self.count_buf.unwrap_or(0);
                self.count_buf = Some(curr * 10 + digit);
                return Intent::Noop;
            }
        }

        let count = self.take_count();
        crate::parser_normal::parse_normal_key(
            &mut self.pending,
            &mut self.count_buf,
            key,
            count,
        )
    }

    /// Parse a key in Insert mode.
    pub fn parse_insert(&mut self, key: &KeyEvent) -> Intent {
        // Handle pending Ctrl-r {register}
        if self.pending == PendingState::InsertRegister {
            self.pending = PendingState::None;
            if let KeyCode::Char(c) = key.code {
                return Intent::InsertFromRegister(c);
            }
            return Intent::Noop;
        }
        // Ctrl-r starts pending register insert
        if key.ctrl && key.code == KeyCode::Char('r') {
            self.pending = PendingState::InsertRegister;
            return Intent::Noop;
        }
        crate::parser_modes::parse_insert(key)
    }

    /// Parse a key in Visual mode.
    pub fn parse_visual(&mut self, key: &KeyEvent) -> Intent {
        crate::parser_modes::parse_visual(key)
    }

    /// Parse a key in Command mode.
    pub fn parse_command(&mut self, key: &KeyEvent) -> Intent {
        crate::parser_modes::parse_command(key)
    }

    /// Parse a key in Replace mode.
    pub fn parse_replace(&mut self, key: &KeyEvent) -> Intent {
        crate::parser_modes::parse_replace(key)
    }
}

/// Shared char-to-motion mapping (stateless).
fn self_char_to_motion(c: char) -> Option<MotionKind> {
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

impl Default for KeyParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{
        InsertPosition, ScrollKind,
    };

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
            Intent::EnterMode(kjxlkj_core_types::Mode::Normal)
        );
    }

    #[test]
    fn gg_goes_to_file_start() {
        let mut p = KeyParser::new();
        assert_eq!(
            p.parse_normal(&KeyEvent::char('g')),
            Intent::Noop,
        );
        assert_eq!(
            p.parse_normal(&KeyEvent::char('g')),
            Intent::Motion(MotionKind::FileStart, 1)
        );
    }

    #[test]
    fn dd_deletes_line() {
        let mut p = KeyParser::new();
        assert_eq!(
            p.parse_normal(&KeyEvent::char('d')),
            Intent::Noop,
        );
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
            Intent::Operator(
                OperatorKind::Delete,
                MotionKind::WordForward,
                1,
            )
        );
    }

    #[test]
    fn undo_redo() {
        let mut p = KeyParser::new();
        assert_eq!(
            p.parse_normal(&KeyEvent::char('u')),
            Intent::Undo,
        );
        assert_eq!(
            p.parse_normal(&KeyEvent::ctrl('r')),
            Intent::Redo,
        );
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
            Intent::FindChar(
                'x',
                FindCharKind::Forward,
            )
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
            Intent::Operator(
                OperatorKind::Delete,
                MotionKind::Right,
                1,
            )
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
