//! Key sequence parser: transforms key events into editor intents.

use kjxlkj_core_types::{
    FindCharKind, Intent, KeyCode, KeyEvent, MotionKind,
    OperatorKind, RegisterName,
};
pub(crate) use crate::pending_state::PendingState;
use crate::pending_state::char_to_motion;

/// Parses key events into intents based on current mode.
pub struct KeyParser {
    pub(crate) pending: PendingState,
    pub(crate) count_buf: Option<usize>,
    pub(crate) last_find: Option<(char, FindCharKind)>,
}

impl KeyParser {
    pub fn new() -> Self {
        Self { pending: PendingState::None, count_buf: None, last_find: None }
    }
    pub fn reset(&mut self) {
        self.pending = PendingState::None;
        self.count_buf = None;
    }
    fn take_count(&mut self) -> usize { self.count_buf.take().unwrap_or(1) }

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
                    if let Some(reg) = RegisterName::from_char(c) {
                        return Intent::SelectRegister(reg);
                    }
                }
                return Intent::Noop;
            }
            PendingState::Mark => { self.pending = PendingState::None; if let KeyCode::Char(c) = key.code { return Intent::SetMark(c); } return Intent::Noop; }
            PendingState::JumpMark => { self.pending = PendingState::None; if let KeyCode::Char(c) = key.code { return Intent::JumpToMark(c); } return Intent::Noop; }
            PendingState::JumpMarkLine => { self.pending = PendingState::None; if let KeyCode::Char(c) = key.code { return Intent::JumpToMarkLine(c); } return Intent::Noop; }
            PendingState::MacroRecord => { self.pending = PendingState::None; if let KeyCode::Char(c) = key.code { return Intent::MacroToggleRecord(c); } return Intent::Noop; }
            PendingState::MacroPlay => {
                self.pending = PendingState::None;
                if let KeyCode::Char(c) = key.code {
                    return if c == '@' { Intent::MacroRepeatLast } else { Intent::MacroPlay(c) };
                }
                return Intent::Noop;
            }
            PendingState::ReplaceChar => { self.pending = PendingState::None; if let KeyCode::Char(c) = key.code { return Intent::ReplaceChar(c); } return Intent::Noop; }
            PendingState::Leader => { self.pending = PendingState::None; return crate::parser_sequences::parse_leader_chord(key); }
            PendingState::CtrlW => { self.pending = PendingState::None; return crate::parser_sequences::parse_ctrl_w_chord(key); }
            PendingState::G => {
                let count = self.take_count();
                return crate::parser_sequences::parse_g_sequence(&mut self.pending, count, key);
            }
            PendingState::Z => { return crate::parser_sequences::parse_z_sequence(&mut self.pending, key); }
            PendingState::GOperator(op, count) => {
                let (op, count) = (*op, *count);
                self.pending = PendingState::None;
                return crate::parser_sequences::parse_g_operator(key, op, count, &char_to_motion);
            }
            PendingState::Operator(op, count) => {
                let (op, count) = (*op, *count);
                return crate::parser_sequences::parse_operator_motion(&mut self.pending, key, op, count, &char_to_motion);
            }
            PendingState::OperatorTextObject(op, inner, _count) => {
                let (op, inner) = (*op, *inner);
                self.pending = PendingState::None;
                return crate::parser_sequences::parse_text_object_key(key, op, inner);
            }
            _ => {}
        }
        // Count accumulation
        if let KeyCode::Char(c) = key.code {
            if c.is_ascii_digit() && !key.ctrl && !key.alt {
                if c == '0' && self.count_buf.is_none() {
                    return Intent::Motion(MotionKind::LineStart, 1);
                }
                let digit = c as usize - '0' as usize;
                let curr = self.count_buf.unwrap_or(0);
                self.count_buf = Some(curr * 10 + digit);
                return Intent::Noop;
            }
        }
        let count = self.take_count();
        crate::parser_normal::parse_normal_key(&mut self.pending, &mut self.count_buf, key, count)
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

impl Default for KeyParser {
    fn default() -> Self { Self::new() }
}
