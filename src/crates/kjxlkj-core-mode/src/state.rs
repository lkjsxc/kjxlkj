//! Mode state machine.

use kjxlkj_core_types::{Intent, KeyEvent, Mode, Modifier, Position};

/// Mode state with pending input.
#[derive(Debug, Clone)]
pub struct ModeState {
    mode: Mode,
    pending_count: Option<usize>,
    pending_operator: Option<char>,
    pending_register: Option<char>,
    visual_anchor: Option<Position>,
    last_find: Option<(char, bool, bool)>, // (char, forward, till)
    command_line: String,
}

impl ModeState {
    /// Create initial mode state.
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            pending_count: None,
            pending_operator: None,
            pending_register: None,
            visual_anchor: None,
            last_find: None,
            command_line: String::new(),
        }
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Set the mode.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        if !mode.is_visual() {
            self.visual_anchor = None;
        }
    }

    /// Get the pending count.
    pub fn count(&self) -> usize {
        self.pending_count.unwrap_or(1)
    }

    /// Get raw pending count (None if not specified).
    pub fn raw_count(&self) -> Option<usize> {
        self.pending_count
    }

    /// Set pending count.
    pub fn set_count(&mut self, count: usize) {
        self.pending_count = Some(count);
    }

    /// Accumulate a digit to the pending count.
    pub fn accumulate_count(&mut self, digit: char) {
        let d = digit.to_digit(10).unwrap_or(0) as usize;
        let current = self.pending_count.unwrap_or(0);
        self.pending_count = Some(current * 10 + d);
    }

    /// Clear the pending count.
    pub fn clear_count(&mut self) {
        self.pending_count = None;
    }

    /// Get pending operator.
    pub fn operator(&self) -> Option<char> {
        self.pending_operator
    }

    /// Set pending operator.
    pub fn set_operator(&mut self, op: char) {
        self.pending_operator = Some(op);
    }

    /// Clear pending operator.
    pub fn clear_operator(&mut self) {
        self.pending_operator = None;
    }

    /// Set visual anchor.
    pub fn set_visual_anchor(&mut self, pos: Position) {
        self.visual_anchor = Some(pos);
    }

    /// Get visual anchor.
    pub fn visual_anchor(&self) -> Option<Position> {
        self.visual_anchor
    }

    /// Set last find command.
    pub fn set_last_find(&mut self, ch: char, forward: bool, till: bool) {
        self.last_find = Some((ch, forward, till));
    }

    /// Get last find command.
    pub fn last_find(&self) -> Option<(char, bool, bool)> {
        self.last_find
    }

    /// Get pending register.
    pub fn register(&self) -> Option<char> {
        self.pending_register
    }

    /// Set pending register.
    pub fn set_register(&mut self, reg: char) {
        self.pending_register = Some(reg);
    }

    /// Clear pending register.
    pub fn clear_register(&mut self) {
        self.pending_register = None;
    }

    /// Get command line content.
    pub fn command_line(&self) -> &str {
        &self.command_line
    }

    /// Append to command line.
    pub fn command_line_push(&mut self, c: char) {
        self.command_line.push(c);
    }

    /// Remove last char from command line.
    pub fn command_line_pop(&mut self) -> Option<char> {
        self.command_line.pop()
    }

    /// Clear command line.
    pub fn command_line_clear(&mut self) {
        self.command_line.clear();
    }

    /// Set command line content.
    pub fn set_command_line(&mut self, s: String) {
        self.command_line = s;
    }

    /// Reset all pending state.
    pub fn reset_pending(&mut self) {
        self.pending_count = None;
        self.pending_operator = None;
        self.pending_register = None;
    }

    /// Process a key event in the current mode.
    pub fn process_key(&mut self, key: &KeyEvent) -> Option<Intent> {
        match self.mode {
            Mode::Normal => self.process_normal(key),
            Mode::Insert => self.process_insert(key),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                self.process_visual(key)
            }
            Mode::Command | Mode::Search => self.process_command(key),
            Mode::Replace => self.process_replace(key),
        }
    }

    fn process_normal(&mut self, key: &KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.reset_pending();
                Some(Intent::Noop)
            }

            // Digits for count
            KeyEvent::Char(c @ '1'..='9', Modifier { ctrl: false, .. }) => {
                self.accumulate_count(*c);
                None
            }
            KeyEvent::Char('0', Modifier { ctrl: false, .. })
                if self.pending_count.is_some() =>
            {
                self.accumulate_count('0');
                None
            }

            // Basic motions
            KeyEvent::Char('h', Modifier { ctrl: false, .. }) | KeyEvent::Left => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveLeft(c))
            }
            KeyEvent::Char('j', Modifier { ctrl: false, .. }) | KeyEvent::Down => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveDown(c))
            }
            KeyEvent::Char('k', Modifier { ctrl: false, .. }) | KeyEvent::Up => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveUp(c))
            }
            KeyEvent::Char('l', Modifier { ctrl: false, .. }) | KeyEvent::Right => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveRight(c))
            }

            // Line motions
            KeyEvent::Char('0', Modifier { ctrl: false, .. }) => {
                self.reset_pending();
                Some(Intent::MoveToLineStart)
            }
            KeyEvent::Char('^', _) => {
                self.reset_pending();
                Some(Intent::MoveToFirstNonBlank)
            }
            KeyEvent::Char('$', _) => {
                self.reset_pending();
                Some(Intent::MoveToLineEnd)
            }

            // Word motions
            KeyEvent::Char('w', Modifier { ctrl: false, .. }) => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveWordForward(c))
            }
            KeyEvent::Char('b', Modifier { ctrl: false, .. }) => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveWordBackward(c))
            }
            KeyEvent::Char('e', Modifier { ctrl: false, .. }) => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveWordEnd(c))
            }
            KeyEvent::Char('W', _) => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveBigWordForward(c))
            }
            KeyEvent::Char('B', _) => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveBigWordBackward(c))
            }
            KeyEvent::Char('E', _) => {
                let c = self.count();
                self.reset_pending();
                Some(Intent::MoveBigWordEnd(c))
            }

            // Document motions
            KeyEvent::Char('G', _) => {
                let intent = if let Some(n) = self.raw_count() {
                    Intent::MoveToLine(n.saturating_sub(1))
                } else {
                    Intent::MoveToDocumentEnd
                };
                self.reset_pending();
                Some(intent)
            }

            // Scrolling
            KeyEvent::Char('u', Modifier { ctrl: true, .. }) => {
                self.reset_pending();
                Some(Intent::ScrollHalfPageUp)
            }
            KeyEvent::Char('d', Modifier { ctrl: true, .. }) => {
                self.reset_pending();
                Some(Intent::ScrollHalfPageDown)
            }
            KeyEvent::Char('b', Modifier { ctrl: true, .. }) => {
                self.reset_pending();
                Some(Intent::ScrollPageUp)
            }
            KeyEvent::Char('f', Modifier { ctrl: true, .. }) => {
                self.reset_pending();
                Some(Intent::ScrollPageDown)
            }

            // Mode entry
            KeyEvent::Char('i', Modifier { ctrl: false, .. }) => {
                self.reset_pending();
                self.set_mode(Mode::Insert);
                Some(Intent::EnterMode(Mode::Insert))
            }
            KeyEvent::Char('a', Modifier { ctrl: false, .. }) => {
                self.reset_pending();
                self.set_mode(Mode::Insert);
                Some(Intent::MoveRight(1))
            }
            KeyEvent::Char('I', _) => {
                self.reset_pending();
                self.set_mode(Mode::Insert);
                Some(Intent::MoveToFirstNonBlank)
            }
            KeyEvent::Char('A', _) => {
                self.reset_pending();
                self.set_mode(Mode::Insert);
                Some(Intent::MoveToLineEnd)
            }
            KeyEvent::Char('o', Modifier { ctrl: false, .. }) => {
                self.reset_pending();
                self.set_mode(Mode::Insert);
                Some(Intent::InsertNewlineBelow)
            }
            KeyEvent::Char('O', _) => {
                self.reset_pending();
                self.set_mode(Mode::Insert);
                Some(Intent::InsertNewlineAbove)
            }
            KeyEvent::Char('v', Modifier { ctrl: false, .. }) => {
                self.reset_pending();
                self.set_mode(Mode::Visual);
                Some(Intent::StartVisual)
            }
            KeyEvent::Char('V', _) => {
                self.reset_pending();
                self.set_mode(Mode::VisualLine);
                Some(Intent::StartVisualLine)
            }
            KeyEvent::Char('v', Modifier { ctrl: true, .. }) => {
                self.reset_pending();
                self.set_mode(Mode::VisualBlock);
                Some(Intent::StartVisualBlock)
            }
            KeyEvent::Char('R', _) => {
                self.reset_pending();
                self.set_mode(Mode::Replace);
                Some(Intent::EnterMode(Mode::Replace))
            }
            KeyEvent::Char(':', _) => {
                self.reset_pending();
                self.command_line_clear();
                self.set_mode(Mode::Command);
                Some(Intent::OpenCommandLine)
            }
            KeyEvent::Char('/', _) => {
                self.reset_pending();
                self.command_line_clear();
                self.set_mode(Mode::Search);
                Some(Intent::OpenSearchForward)
            }
            KeyEvent::Char('?', _) => {
                self.reset_pending();
                self.command_line_clear();
                self.set_mode(Mode::Search);
                Some(Intent::OpenSearchBackward)
            }

            // Basic editing
            KeyEvent::Char('x', Modifier { ctrl: false, .. }) => {
                self.reset_pending();
                Some(Intent::DeleteChar)
            }
            KeyEvent::Char('X', _) => {
                self.reset_pending();
                Some(Intent::DeleteCharBackward)
            }
            KeyEvent::Char('D', _) => {
                self.reset_pending();
                Some(Intent::DeleteToLineEnd)
            }
            KeyEvent::Char('C', _) => {
                self.reset_pending();
                self.set_mode(Mode::Insert);
                Some(Intent::ChangeToLineEnd)
            }

            // Operators
            KeyEvent::Char('d', Modifier { ctrl: false, .. }) => {
                if self.pending_operator == Some('d') {
                    let c = self.count();
                    self.reset_pending();
                    Some(Intent::DeleteLine(c))
                } else {
                    self.set_operator('d');
                    None
                }
            }
            KeyEvent::Char('y', Modifier { ctrl: false, .. }) => {
                if self.pending_operator == Some('y') {
                    let c = self.count();
                    self.reset_pending();
                    Some(Intent::YankLine(c))
                } else {
                    self.set_operator('y');
                    None
                }
            }
            KeyEvent::Char('c', Modifier { ctrl: false, .. }) => {
                if self.pending_operator == Some('c') {
                    let c = self.count();
                    self.reset_pending();
                    self.set_mode(Mode::Insert);
                    Some(Intent::ChangeLine(c))
                } else {
                    self.set_operator('c');
                    None
                }
            }

            // Paste
            KeyEvent::Char('p', Modifier { ctrl: false, .. }) => {
                self.reset_pending();
                Some(Intent::Paste { after: true })
            }
            KeyEvent::Char('P', _) => {
                self.reset_pending();
                Some(Intent::Paste { after: false })
            }

            // Undo/Redo
            KeyEvent::Char('u', Modifier { ctrl: false, .. }) => {
                self.reset_pending();
                Some(Intent::Undo)
            }
            KeyEvent::Char('r', Modifier { ctrl: true, .. }) => {
                self.reset_pending();
                Some(Intent::Redo)
            }

            // Repeat
            KeyEvent::Char('.', _) => {
                self.reset_pending();
                Some(Intent::RepeatLastChange)
            }

            // Search navigation
            KeyEvent::Char('n', Modifier { ctrl: false, .. }) => {
                self.reset_pending();
                Some(Intent::NextSearchMatch)
            }
            KeyEvent::Char('N', _) => {
                self.reset_pending();
                Some(Intent::PrevSearchMatch)
            }

            // Leader key (Space)
            KeyEvent::Char(' ', Modifier { ctrl: false, .. }) => {
                // Leader key - wait for next key
                None
            }

            _ => {
                self.reset_pending();
                Some(Intent::Noop)
            }
        }
    }

    fn process_insert(&mut self, key: &KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.set_mode(Mode::Normal);
                Some(Intent::EnterMode(Mode::Normal))
            }
            KeyEvent::Enter => Some(Intent::InsertNewline),
            KeyEvent::Backspace => Some(Intent::DeleteCharBackward),
            KeyEvent::Delete => Some(Intent::DeleteChar),
            KeyEvent::Tab => Some(Intent::InsertChar('\t')),
            KeyEvent::Char(c, Modifier { ctrl: false, .. }) => {
                Some(Intent::InsertChar(*c))
            }
            KeyEvent::Char('w', Modifier { ctrl: true, .. }) => {
                Some(Intent::DeleteWordBackward)
            }
            KeyEvent::Char('u', Modifier { ctrl: true, .. }) => {
                Some(Intent::DeleteToLineStart)
            }
            KeyEvent::Left => Some(Intent::MoveLeft(1)),
            KeyEvent::Right => Some(Intent::MoveRight(1)),
            KeyEvent::Up => Some(Intent::MoveUp(1)),
            KeyEvent::Down => Some(Intent::MoveDown(1)),
            KeyEvent::Home => Some(Intent::MoveToLineStart),
            KeyEvent::End => Some(Intent::MoveToLineEnd),
            _ => Some(Intent::Noop),
        }
    }

    fn process_visual(&mut self, key: &KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.set_mode(Mode::Normal);
                Some(Intent::EnterMode(Mode::Normal))
            }
            // Motions work the same as normal mode
            KeyEvent::Char('h', Modifier { ctrl: false, .. }) | KeyEvent::Left => {
                Some(Intent::MoveLeft(1))
            }
            KeyEvent::Char('j', Modifier { ctrl: false, .. }) | KeyEvent::Down => {
                Some(Intent::MoveDown(1))
            }
            KeyEvent::Char('k', Modifier { ctrl: false, .. }) | KeyEvent::Up => {
                Some(Intent::MoveUp(1))
            }
            KeyEvent::Char('l', Modifier { ctrl: false, .. }) | KeyEvent::Right => {
                Some(Intent::MoveRight(1))
            }

            // Operators on selection
            KeyEvent::Char('d', Modifier { ctrl: false, .. })
            | KeyEvent::Char('x', Modifier { ctrl: false, .. }) => {
                self.set_mode(Mode::Normal);
                // The actual range will be computed by the editor
                Some(Intent::DeleteRange(kjxlkj_core_types::Range::default()))
            }
            KeyEvent::Char('y', Modifier { ctrl: false, .. }) => {
                self.set_mode(Mode::Normal);
                Some(Intent::Yank(kjxlkj_core_types::Range::default()))
            }
            KeyEvent::Char('c', Modifier { ctrl: false, .. }) => {
                self.set_mode(Mode::Insert);
                Some(Intent::Change(kjxlkj_core_types::Range::default()))
            }

            _ => Some(Intent::Noop),
        }
    }

    fn process_command(&mut self, key: &KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.command_line_clear();
                self.set_mode(Mode::Normal);
                Some(Intent::EnterMode(Mode::Normal))
            }
            KeyEvent::Enter => {
                let cmd = self.command_line.clone();
                self.command_line_clear();
                self.set_mode(Mode::Normal);
                if self.mode == Mode::Search {
                    Some(Intent::SearchForward(cmd))
                } else {
                    Some(Intent::ExecuteCommand(cmd))
                }
            }
            KeyEvent::Backspace => {
                if self.command_line.is_empty() {
                    self.set_mode(Mode::Normal);
                    Some(Intent::EnterMode(Mode::Normal))
                } else {
                    self.command_line_pop();
                    Some(Intent::Noop)
                }
            }
            KeyEvent::Char(c, Modifier { ctrl: false, .. }) => {
                self.command_line_push(*c);
                Some(Intent::Noop)
            }
            _ => Some(Intent::Noop),
        }
    }

    fn process_replace(&mut self, key: &KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.set_mode(Mode::Normal);
                Some(Intent::EnterMode(Mode::Normal))
            }
            KeyEvent::Char(c, Modifier { ctrl: false, .. }) => {
                // Replace mode: overwrite character
                Some(Intent::InsertChar(*c))
            }
            KeyEvent::Left => Some(Intent::MoveLeft(1)),
            KeyEvent::Right => Some(Intent::MoveRight(1)),
            _ => Some(Intent::Noop),
        }
    }
}

impl Default for ModeState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_mode() {
        let state = ModeState::new();
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn test_enter_insert() {
        let mut state = ModeState::new();
        let intent = state.process_key(&KeyEvent::char('i'));
        assert!(matches!(intent, Some(Intent::EnterMode(Mode::Insert))));
        assert_eq!(state.mode(), Mode::Insert);
    }

    #[test]
    fn test_count_accumulation() {
        let mut state = ModeState::new();
        state.process_key(&KeyEvent::char('2'));
        state.process_key(&KeyEvent::char('3'));
        assert_eq!(state.count(), 23);
    }

    #[test]
    fn test_motion_with_count() {
        let mut state = ModeState::new();
        state.process_key(&KeyEvent::char('5'));
        let intent = state.process_key(&KeyEvent::char('j'));
        assert!(matches!(intent, Some(Intent::MoveDown(5))));
    }

    #[test]
    fn test_escape_clears_pending() {
        let mut state = ModeState::new();
        state.process_key(&KeyEvent::char('3'));
        state.process_key(&KeyEvent::char('d'));
        state.process_key(&KeyEvent::Escape);
        assert!(state.raw_count().is_none());
        assert!(state.operator().is_none());
    }
}
