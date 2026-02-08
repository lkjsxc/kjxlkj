//! Normal mode state: count prefix, register prefix, and key dispatch.

use kjxlkj_core_types::{
    Action, InsertPosition, Key, KeyCode, KeyModifiers, Motion,
    Operator, RegisterName, VisualKind,
};

/// State maintained during Normal mode key processing.
#[derive(Debug, Default)]
pub struct NormalModeState {
    /// Accumulated count prefix (None = no count entered yet).
    count: Option<u32>,
    /// Pending register prefix.
    register: Option<RegisterName>,
    /// Whether we're waiting for a register name character.
    register_pending: bool,
    /// Pending g-prefix.
    g_pending: bool,
    /// Pending z-prefix.
    z_pending: bool,
    /// Pending bracket prefix ([, ]).
    bracket_pending: Option<char>,
    /// Pending mark command (m, ', `).
    mark_pending: Option<MarkCommand>,
    /// Pending r (replace char).
    replace_char_pending: bool,
}

#[derive(Debug, Clone, Copy)]
enum MarkCommand {
    Set,
    JumpExact,
    JumpLine,
}

impl NormalModeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset all pending state.
    pub fn reset(&mut self) {
        self.count = None;
        self.register = None;
        self.register_pending = false;
        self.g_pending = false;
        self.z_pending = false;
        self.bracket_pending = None;
        self.mark_pending = None;
        self.replace_char_pending = false;
    }

    /// Get the effective count (default 1).
    pub fn effective_count(&self) -> u32 {
        self.count.unwrap_or(1)
    }

    /// Get the target register.
    pub fn target_register(&self) -> RegisterName {
        self.register.unwrap_or(RegisterName::Unnamed)
    }

    /// Process a key event in Normal mode.
    /// Returns an action if the key completes a command, or None if
    /// we need more keys (prefix state).
    pub fn process_key(&mut self, key: &Key) -> Option<Action> {
        // Handle pending sub-states first
        if self.replace_char_pending {
            self.replace_char_pending = false;
            if let KeyCode::Char(c) = key.code {
                let action = Action::ReplaceChar(c);
                self.reset();
                return Some(action);
            }
            self.reset();
            return Some(Action::Nop);
        }

        if let Some(mark_cmd) = self.mark_pending {
            self.mark_pending = None;
            if let KeyCode::Char(c) = key.code {
                let action = match mark_cmd {
                    MarkCommand::Set => Action::SetMark(c),
                    MarkCommand::JumpExact => Action::JumpToMark(c),
                    MarkCommand::JumpLine => Action::JumpToMarkLine(c),
                };
                self.reset();
                return Some(action);
            }
            self.reset();
            return Some(Action::Nop);
        }

        if self.register_pending {
            self.register_pending = false;
            if let KeyCode::Char(c) = key.code {
                self.register = RegisterName::from_char(c);
            }
            return None;
        }

        if self.g_pending {
            return self.process_g_key(key);
        }

        if self.z_pending {
            return self.process_z_key(key);
        }

        // Count prefix
        if let Some(digit) = key.digit_value() {
            if digit > 0 || self.count.is_some() {
                let current = self.count.unwrap_or(0);
                self.count = Some(current * 10 + digit);
                return None;
            }
        }

        // Standard key dispatch
        self.dispatch_key(key)
    }

    fn dispatch_key(&mut self, key: &Key) -> Option<Action> {
        let count = self.effective_count();

        let action = match (&key.code, key.modifiers) {
            // Motions
            (KeyCode::Char('h'), KeyModifiers::NONE) | (KeyCode::Left, _) => {
                Action::MoveCursor(Motion::Left, count)
            }
            (KeyCode::Char('l'), KeyModifiers::NONE) | (KeyCode::Right, _) => {
                Action::MoveCursor(Motion::Right, count)
            }
            (KeyCode::Char('j'), KeyModifiers::NONE) | (KeyCode::Down, _) => {
                Action::MoveCursor(Motion::Down, count)
            }
            (KeyCode::Char('k'), KeyModifiers::NONE) | (KeyCode::Up, _) => {
                Action::MoveCursor(Motion::Up, count)
            }
            (KeyCode::Char('w'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::WordForward, count)
            }
            (KeyCode::Char('W'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::WordForwardBig, count)
            }
            (KeyCode::Char('b'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::WordBackward, count)
            }
            (KeyCode::Char('B'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::WordBackwardBig, count)
            }
            (KeyCode::Char('e'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::WordEndForward, count)
            }
            (KeyCode::Char('E'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::WordEndForwardBig, count)
            }
            (KeyCode::Char('0'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::LineStart, 1)
            }
            (KeyCode::Char('^'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::FirstNonBlank, 1)
            }
            (KeyCode::Char('$'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::LineEnd, 1)
            }
            (KeyCode::Char('G'), KeyModifiers::NONE) => {
                if self.count.is_some() {
                    Action::MoveCursor(
                        Motion::GotoLine(count as usize - 1),
                        1,
                    )
                } else {
                    Action::MoveCursor(Motion::GotoLastLine, 1)
                }
            }
            (KeyCode::Char('{'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::ParagraphBackward, count)
            }
            (KeyCode::Char('}'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::ParagraphForward, count)
            }
            (KeyCode::Char('+'), KeyModifiers::NONE) | (KeyCode::Enter, KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::NextLineFirstNonBlank, count)
            }
            (KeyCode::Char('-'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::PrevLineFirstNonBlank, count)
            }

            // Insert mode entry
            (KeyCode::Char('i'), KeyModifiers::NONE) => {
                Action::EnterInsert(InsertPosition::BeforeCursor)
            }
            (KeyCode::Char('a'), KeyModifiers::NONE) => {
                Action::EnterInsert(InsertPosition::AfterCursor)
            }
            (KeyCode::Char('I'), KeyModifiers::NONE) => {
                Action::EnterInsert(InsertPosition::FirstNonBlank)
            }
            (KeyCode::Char('A'), KeyModifiers::NONE) => {
                Action::EnterInsert(InsertPosition::EndOfLine)
            }
            (KeyCode::Char('o'), KeyModifiers::NONE) => {
                Action::EnterInsert(InsertPosition::NewLineBelow)
            }
            (KeyCode::Char('O'), KeyModifiers::NONE) => {
                Action::EnterInsert(InsertPosition::NewLineAbove)
            }

            // Visual mode
            (KeyCode::Char('v'), KeyModifiers::NONE) => {
                Action::EnterVisual(VisualKind::Char)
            }
            (KeyCode::Char('V'), KeyModifiers::NONE) => {
                Action::EnterVisual(VisualKind::Line)
            }
            (KeyCode::Char('v'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::EnterVisual(VisualKind::Block)
            }

            // Operators
            (KeyCode::Char('d'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(Operator::Delete)
            }
            (KeyCode::Char('c'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(Operator::Change)
            }
            (KeyCode::Char('y'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(Operator::Yank)
            }
            (KeyCode::Char('>'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(Operator::Indent)
            }
            (KeyCode::Char('<'), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(Operator::Dedent)
            }
            (KeyCode::Char('='), KeyModifiers::NONE) => {
                Action::EnterOperatorPending(Operator::Reindent)
            }

            // Single-key commands
            (KeyCode::Char('x'), KeyModifiers::NONE) => Action::DeleteCharForward,
            (KeyCode::Char('X'), KeyModifiers::NONE) => Action::DeleteCharBackward,
            (KeyCode::Char('s'), KeyModifiers::NONE) => Action::SubstituteChar,
            (KeyCode::Char('S'), KeyModifiers::NONE) => Action::SubstituteLine,
            (KeyCode::Char('C'), KeyModifiers::NONE) => Action::ChangeToEnd,
            (KeyCode::Char('J'), KeyModifiers::NONE) => Action::JoinLines,
            (KeyCode::Char('~'), KeyModifiers::NONE) => Action::ToggleCaseChar,
            (KeyCode::Char('.'), KeyModifiers::NONE) => Action::DotRepeat,
            (KeyCode::Char('u'), KeyModifiers::NONE) => Action::Undo,
            (KeyCode::Char('r'), m) if m.contains(KeyModifiers::CTRL) => Action::Redo,
            (KeyCode::Char('p'), KeyModifiers::NONE) => Action::Put(false),
            (KeyCode::Char('P'), KeyModifiers::NONE) => Action::Put(true),
            (KeyCode::Char('n'), KeyModifiers::NONE) => Action::NextMatch,
            (KeyCode::Char('N'), KeyModifiers::NONE) => Action::PrevMatch,
            (KeyCode::Char('*'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::StarForward, 1)
            }
            (KeyCode::Char('#'), KeyModifiers::NONE) => {
                Action::MoveCursor(Motion::StarBackward, 1)
            }

            // Replace char
            (KeyCode::Char('r'), KeyModifiers::NONE) => {
                self.replace_char_pending = true;
                return None;
            }

            // Replace mode
            (KeyCode::Char('R'), KeyModifiers::NONE) => Action::EnterReplace,

            // Command mode
            (KeyCode::Char(':'), KeyModifiers::NONE) => {
                Action::EnterCommand(kjxlkj_core_types::ActionCommandKind::Ex)
            }
            (KeyCode::Char('/'), KeyModifiers::NONE) => {
                Action::EnterCommand(kjxlkj_core_types::ActionCommandKind::SearchForward)
            }
            (KeyCode::Char('?'), KeyModifiers::NONE) => {
                Action::EnterCommand(kjxlkj_core_types::ActionCommandKind::SearchBackward)
            }

            // Register prefix
            (KeyCode::Char('"'), KeyModifiers::NONE) => {
                self.register_pending = true;
                return None;
            }

            // g-prefix
            (KeyCode::Char('g'), KeyModifiers::NONE) => {
                self.g_pending = true;
                return None;
            }

            // z-prefix
            (KeyCode::Char('z'), KeyModifiers::NONE) => {
                self.z_pending = true;
                return None;
            }

            // Marks
            (KeyCode::Char('m'), KeyModifiers::NONE) => {
                self.mark_pending = Some(MarkCommand::Set);
                return None;
            }
            (KeyCode::Char('`'), KeyModifiers::NONE) => {
                self.mark_pending = Some(MarkCommand::JumpExact);
                return None;
            }
            (KeyCode::Char('\''), KeyModifiers::NONE) => {
                self.mark_pending = Some(MarkCommand::JumpLine);
                return None;
            }

            // Macros
            (KeyCode::Char('q'), KeyModifiers::NONE) => Action::RecordMacro('\0'),
            (KeyCode::Char('@'), KeyModifiers::NONE) => Action::PlayMacro('\0', count),

            // Window commands (Ctrl-w prefix handled at dispatch level)
            (KeyCode::Char('w'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::CycleWindow
            }

            // Ctrl-^ alternate file
            (KeyCode::Char('^'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::AlternateFile
            }

            // ZZ / ZQ
            (KeyCode::Char('Z'), KeyModifiers::NONE) => {
                // Need second key — simplified: treat as Nop
                Action::Nop
            }

            // Increment/decrement
            (KeyCode::Char('a'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::Increment(count as i64)
            }
            (KeyCode::Char('x'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::Increment(-(count as i64))
            }

            _ => Action::Nop,
        };

        self.reset();
        Some(action)
    }

    fn process_g_key(&mut self, key: &Key) -> Option<Action> {
        self.g_pending = false;
        let count = self.effective_count();
        let action = match &key.code {
            KeyCode::Char('g') => {
                if self.count.is_some() {
                    Action::MoveCursor(
                        Motion::GotoLine(count as usize - 1),
                        1,
                    )
                } else {
                    Action::MoveCursor(Motion::GotoFirstLine, 1)
                }
            }
            KeyCode::Char('J') => Action::JoinLinesNoSpace,
            KeyCode::Char('~') => {
                Action::EnterOperatorPending(Operator::ToggleCase)
            }
            KeyCode::Char('u') => {
                Action::EnterOperatorPending(Operator::Lowercase)
            }
            KeyCode::Char('U') => {
                Action::EnterOperatorPending(Operator::Uppercase)
            }
            KeyCode::Char('q') => {
                Action::EnterOperatorPending(Operator::Format)
            }
            KeyCode::Char('_') => {
                Action::MoveCursor(Motion::LastNonBlank, 1)
            }
            _ => Action::Nop,
        };
        self.reset();
        Some(action)
    }

    fn process_z_key(&mut self, key: &Key) -> Option<Action> {
        self.z_pending = false;
        let action = match &key.code {
            KeyCode::Char('z') => {
                Action::MoveCursor(Motion::ScreenMiddle, 1)
            }
            KeyCode::Char('t') => {
                Action::MoveCursor(Motion::ScreenTop, 1)
            }
            KeyCode::Char('b') => {
                Action::MoveCursor(Motion::ScreenBottom, 1)
            }
            _ => Action::Nop,
        };
        self.reset();
        Some(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_accumulation() {
        let mut state = NormalModeState::new();
        assert!(state.process_key(&Key::char('3')).is_none());
        let action = state.process_key(&Key::char('j'));
        assert!(matches!(
            action,
            Some(Action::MoveCursor(Motion::Down, 3))
        ));
    }

    #[test]
    fn insert_entry() {
        let mut state = NormalModeState::new();
        let action = state.process_key(&Key::char('i'));
        assert!(matches!(
            action,
            Some(Action::EnterInsert(InsertPosition::BeforeCursor))
        ));
    }

    #[test]
    fn operator_pending() {
        let mut state = NormalModeState::new();
        let action = state.process_key(&Key::char('d'));
        assert!(matches!(
            action,
            Some(Action::EnterOperatorPending(Operator::Delete))
        ));
    }

    #[test]
    fn gg_motion() {
        let mut state = NormalModeState::new();
        assert!(state.process_key(&Key::char('g')).is_none());
        let action = state.process_key(&Key::char('g'));
        assert!(matches!(
            action,
            Some(Action::MoveCursor(Motion::GotoFirstLine, 1))
        ));
    }
}
