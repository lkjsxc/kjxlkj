use kjxlkj_core_types::{Action, Key, KeyCode, Modifier};

/// Result of dispatching a key in normal mode.
#[derive(Debug)]
pub enum KeyDispatchResult {
    /// Key produced an action.
    Action(Action),
    /// Key was consumed as part of a prefix (count, register).
    Consumed,
    /// Key was not recognized.
    Unhandled,
}

/// Normal mode key dispatcher.
/// Tracks count prefix and pending multi-key prefix.
#[derive(Debug, Default)]
pub struct NormalDispatch {
    count: Option<usize>,
    pending: Option<char>,
}

impl NormalDispatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get effective count (default 1).
    pub fn effective_count(&self) -> usize {
        self.count.unwrap_or(1)
    }

    /// Extract accumulated count and clear it.
    pub fn take_count(&mut self) -> usize {
        let c = self.effective_count();
        self.count = None;
        c
    }

    /// Whether a multi-key prefix is pending.
    pub fn has_pending(&self) -> bool {
        self.pending.is_some()
    }

    /// Dispatch a key in normal mode.
    pub fn dispatch(&mut self, key: &Key) -> KeyDispatchResult {
        if let Some(prefix) = self.pending.take() {
            let count = self.effective_count();
            self.count = None;
            return self.dispatch_prefix(prefix, key, count);
        }
        if let KeyCode::Char(c) = &key.code {
            if key.modifiers == Modifier::NONE
                && (('1'..='9').contains(c) || (*c == '0' && self.count.is_some()))
            {
                let digit = (*c as usize) - ('0' as usize);
                self.count = Some(self.count.unwrap_or(0) * 10 + digit);
                return KeyDispatchResult::Consumed;
            }
        }
        if key.modifiers == Modifier::NONE {
            if let KeyCode::Char(c) = &key.code {
                if matches!(
                    c,
                    'g' | 'z' | 'm' | '\'' | '`' | '"' | '@' | 'q' | 'f' | 'F' | 't' | 'T'
                ) {
                    self.pending = Some(*c);
                    return KeyDispatchResult::Consumed;
                }
            }
        }
        let count = self.effective_count();
        self.count = None;
        if key.modifiers == Modifier::NONE {
            if let KeyCode::Char(c) = &key.code {
                return Self::dispatch_normal_char(*c, count);
            }
        }
        if key.modifiers.contains(Modifier::CTRL) {
            if let KeyCode::Char(c) = &key.code {
                return match c {
                    'r' => KeyDispatchResult::Action(Action::Redo),
                    'f' => KeyDispatchResult::Action(Action::PageDown),
                    'b' => KeyDispatchResult::Action(Action::PageUp),
                    'd' => KeyDispatchResult::Action(Action::HalfPageDown),
                    'u' => KeyDispatchResult::Action(Action::HalfPageUp),
                    'o' => KeyDispatchResult::Action(Action::JumpOlder),
                    'i' => KeyDispatchResult::Action(Action::JumpNewer),
                    'w' => KeyDispatchResult::Consumed,
                    _ => KeyDispatchResult::Unhandled,
                };
            }
        }
        Self::dispatch_special(key, count)
    }

    fn dispatch_prefix(&self, prefix: char, key: &Key, count: usize) -> KeyDispatchResult {
        if let KeyCode::Char(c) = &key.code {
            if key.modifiers == Modifier::NONE {
                return match prefix {
                    'g' => Self::dispatch_g(*c, count),
                    'z' => Self::dispatch_z(*c),
                    'm' => KeyDispatchResult::Action(Action::SetMark(*c)),
                    '\'' => KeyDispatchResult::Action(Action::JumpToMarkLine(*c)),
                    '`' => KeyDispatchResult::Action(Action::JumpToMark(*c)),
                    '"' => KeyDispatchResult::Action(Action::SelectRegister(*c)),
                    '@' => KeyDispatchResult::Action(Action::PlayMacro(*c, count)),
                    'q' => KeyDispatchResult::Action(Action::StartRecording(*c)),
                    'f' => KeyDispatchResult::Action(Action::FindCharForward(*c)),
                    'F' => KeyDispatchResult::Action(Action::FindCharBackward(*c)),
                    't' => KeyDispatchResult::Action(Action::TillCharForward(*c)),
                    'T' => KeyDispatchResult::Action(Action::TillCharBackward(*c)),
                    _ => KeyDispatchResult::Unhandled,
                };
            }
        }
        KeyDispatchResult::Unhandled
    }

    fn dispatch_g(c: char, count: usize) -> KeyDispatchResult {
        match c {
            'g' => KeyDispatchResult::Action(Action::MoveToTop),
            'j' => KeyDispatchResult::Action(Action::MoveDown(count)),
            'k' => KeyDispatchResult::Action(Action::MoveUp(count)),
            'v' => KeyDispatchResult::Action(Action::VisualReselect),
            'q' => KeyDispatchResult::Action(Action::EnterOperatorPending(
                kjxlkj_core_types::Operator::Format,
            )),
            'w' => KeyDispatchResult::Action(Action::EnterOperatorPending(
                kjxlkj_core_types::Operator::FormatKeepCursor,
            )),
            'u' => KeyDispatchResult::Action(Action::EnterOperatorPending(
                kjxlkj_core_types::Operator::Lowercase,
            )),
            'U' => KeyDispatchResult::Action(Action::EnterOperatorPending(
                kjxlkj_core_types::Operator::Uppercase,
            )),
            ';' => KeyDispatchResult::Action(Action::ChangelistOlder),
            ',' => KeyDispatchResult::Action(Action::ChangelistNewer),
            '\'' | '`' => KeyDispatchResult::Action(Action::JumpFromMarkStack),
            _ => KeyDispatchResult::Unhandled,
        }
    }

    fn dispatch_z(c: char) -> KeyDispatchResult {
        match c { 'z' => KeyDispatchResult::Action(Action::ScrollCenterCursor), 't' => KeyDispatchResult::Action(Action::ScrollCursorTop), 'b' => KeyDispatchResult::Action(Action::ScrollCursorBottom), _ => KeyDispatchResult::Unhandled }
    }

    fn dispatch_normal_char(c: char, count: usize) -> KeyDispatchResult {
        match c {
            'h' => KeyDispatchResult::Action(Action::MoveLeft(count)),
            'j' => KeyDispatchResult::Action(Action::MoveDown(count)),
            'k' => KeyDispatchResult::Action(Action::MoveUp(count)),
            'l' => KeyDispatchResult::Action(Action::MoveRight(count)),
            '0' => KeyDispatchResult::Action(Action::MoveToLineStart),
            '^' => KeyDispatchResult::Action(Action::MoveToFirstNonBlank),
            '$' => KeyDispatchResult::Action(Action::MoveToLineEnd),
            'w' => KeyDispatchResult::Action(Action::MoveWordForward(count)),
            'b' => KeyDispatchResult::Action(Action::MoveWordBackward(count)),
            'e' => KeyDispatchResult::Action(Action::MoveWordEndForward(count)),
            'G' => KeyDispatchResult::Action(Action::MoveToBottom),
            '%' => KeyDispatchResult::Action(Action::MoveToMatchingBracket),
            'n' => KeyDispatchResult::Action(Action::SearchNext),
            'N' => KeyDispatchResult::Action(Action::SearchPrev),
            'i' => KeyDispatchResult::Action(Action::InsertBefore),
            'a' => KeyDispatchResult::Action(Action::InsertAfter),
            'I' => KeyDispatchResult::Action(Action::InsertAtLineStart),
            'A' => KeyDispatchResult::Action(Action::InsertAtLineEnd),
            'o' => KeyDispatchResult::Action(Action::OpenBelow),
            'O' => KeyDispatchResult::Action(Action::OpenAbove),
            'x' => KeyDispatchResult::Action(Action::DeleteChar(count)),
            'X' => KeyDispatchResult::Action(Action::DeleteCharBack(count)),
            'J' => KeyDispatchResult::Action(Action::JoinLines),
            'K' => KeyDispatchResult::Action(Action::LookupKeyword(count)),
            '~' => KeyDispatchResult::Action(Action::ToggleCase),
            ';' => KeyDispatchResult::Action(Action::RepeatFindChar),
            ',' => KeyDispatchResult::Action(Action::RepeatFindCharReverse),
            '.' => KeyDispatchResult::Action(Action::DotRepeat),
            'u' => KeyDispatchResult::Action(Action::Undo),
            'p' => KeyDispatchResult::Action(Action::PutAfter),
            'P' => KeyDispatchResult::Action(Action::PutBefore),
            'v' | 'V' | ':' | '/' | '?' | 'R' | 's' | 'S' | 'C' => KeyDispatchResult::Unhandled,
            'd' | 'c' | 'y' | '>' | '<' | '=' => KeyDispatchResult::Unhandled,
            _ => KeyDispatchResult::Unhandled,
        }
    }

    fn dispatch_special(key: &Key, count: usize) -> KeyDispatchResult {
        match &key.code {
            KeyCode::Up => KeyDispatchResult::Action(Action::MoveUp(count)),
            KeyCode::Down => KeyDispatchResult::Action(Action::MoveDown(count)),
            KeyCode::Left => KeyDispatchResult::Action(Action::MoveLeft(count)),
            KeyCode::Right => KeyDispatchResult::Action(Action::MoveRight(count)),
            KeyCode::Home => KeyDispatchResult::Action(Action::MoveToLineStart),
            KeyCode::End => KeyDispatchResult::Action(Action::MoveToLineEnd),
            KeyCode::PageUp => KeyDispatchResult::Action(Action::PageUp),
            KeyCode::PageDown => KeyDispatchResult::Action(Action::PageDown),
            _ => KeyDispatchResult::Unhandled,
        }
    }
}
