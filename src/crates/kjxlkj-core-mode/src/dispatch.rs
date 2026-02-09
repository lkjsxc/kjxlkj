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
///
/// Tracks count prefix, register prefix, and pending g/z/[/] prefixes.
#[derive(Debug, Default)]
pub struct NormalDispatch {
    count: Option<usize>,
    register: Option<char>,
    g_prefix: bool,
    z_prefix: bool,
}

impl NormalDispatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get effective count (default 1).
    pub fn effective_count(&self) -> usize {
        self.count.unwrap_or(1)
    }

    /// Dispatch a key in normal mode.
    pub fn dispatch(&mut self, key: &Key) -> KeyDispatchResult {
        // Count prefix: digits 1-9 start count, 0 after digits
        if let KeyCode::Char(c) = &key.code {
            if key.modifiers == Modifier::NONE
                && (('1'..='9').contains(c) || (*c == '0' && self.count.is_some()))
            {
                let digit = (*c as usize) - ('0' as usize);
                self.count = Some(self.count.unwrap_or(0) * 10 + digit);
                return KeyDispatchResult::Consumed;
            }
        }

        // Register prefix: "x
        if let KeyCode::Char('"') = &key.code {
            if key.modifiers == Modifier::NONE && self.register.is_none() {
                // Next key will be the register name; but we
                // simplify by not consuming it here; handled
                // by core state.
                return KeyDispatchResult::Consumed;
            }
        }

        let count = self.effective_count();
        self.reset();

        // Dispatch single-key commands
        if key.modifiers == Modifier::NONE {
            if let KeyCode::Char(c) = &key.code {
                return self.dispatch_normal_char(*c, count);
            }
        }

        // Ctrl-key commands
        if key.modifiers.contains(Modifier::CTRL) {
            if let KeyCode::Char(c) = &key.code {
                return match c {
                    'r' => KeyDispatchResult::Action(Action::Redo),
                    'f' => KeyDispatchResult::Action(Action::PageDown),
                    'b' => KeyDispatchResult::Action(Action::PageUp),
                    'd' => KeyDispatchResult::Action(Action::HalfPageDown),
                    'u' => KeyDispatchResult::Action(Action::HalfPageUp),
                    'w' => KeyDispatchResult::Consumed, // Window prefix
                    _ => KeyDispatchResult::Unhandled,
                };
            }
        }

        // Arrow keys / special keys
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

    fn dispatch_normal_char(&self, c: char, count: usize) -> KeyDispatchResult {
        match c {
            // Motions
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

            // Insert mode entry
            'i' => KeyDispatchResult::Action(Action::InsertBefore),
            'a' => KeyDispatchResult::Action(Action::InsertAfter),
            'I' => KeyDispatchResult::Action(Action::InsertAtLineStart),
            'A' => KeyDispatchResult::Action(Action::InsertAtLineEnd),
            'o' => KeyDispatchResult::Action(Action::OpenBelow),
            'O' => KeyDispatchResult::Action(Action::OpenAbove),

            // Single-key commands
            'x' => KeyDispatchResult::Action(Action::DeleteChar(count)),
            'X' => KeyDispatchResult::Action(Action::DeleteCharBack(count)),
            'J' => KeyDispatchResult::Action(Action::JoinLines),
            '~' => KeyDispatchResult::Action(Action::ToggleCase),
            '.' => KeyDispatchResult::Action(Action::DotRepeat),
            'u' => KeyDispatchResult::Action(Action::Undo),
            'p' => KeyDispatchResult::Action(Action::PutAfter),
            'P' => KeyDispatchResult::Action(Action::PutBefore),

            // Mode transitions (handled by mode transition)
            'v' | 'V' | ':' | '/' | '?' | 'R' => KeyDispatchResult::Unhandled,

            // Operators (handled by mode transition)
            'd' | 'c' | 'y' | '>' | '<' | '=' => KeyDispatchResult::Unhandled,

            _ => KeyDispatchResult::Unhandled,
        }
    }

    fn reset(&mut self) {
        self.count = None;
        self.register = None;
        self.g_prefix = false;
        self.z_prefix = false;
    }
}
