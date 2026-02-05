//! Mode state machine.

use kjxlkj_core_edit::{Motion, Operator};
use kjxlkj_core_types::{KeyEvent, Mode, Modifier};

/// Intent produced from key processing.
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    /// No operation.
    Noop,
    /// Insert a character.
    InsertChar(char),
    /// Delete character before cursor.
    Backspace,
    /// Delete character at cursor.
    DeleteChar,
    /// Insert newline.
    Newline,
    /// Move cursor.
    Motion(Motion, usize),
    /// Apply operator with motion.
    OperatorMotion(Operator, Motion, usize),
    /// Apply operator on line.
    OperatorLine(Operator, usize),
    /// Enter mode.
    EnterMode(Mode),
    /// Append (enter insert after cursor).
    Append,
    /// Open line below.
    OpenBelow,
    /// Open line above.
    OpenAbove,
    /// Undo.
    Undo,
    /// Redo.
    Redo,
    /// Execute command.
    ExecuteCommand(String),
    /// Append to command line.
    CommandLineAppend(char),
    /// Command line backspace.
    CommandLineBackspace,
    /// Cancel (escape).
    Cancel,
    /// Quit.
    Quit,
}

/// Mode state for the editor.
#[derive(Debug, Clone)]
pub struct ModeState {
    mode: Mode,
    pending_operator: Option<Operator>,
    count: Option<usize>,
    command_line: String,
}

impl Default for ModeState {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeState {
    /// Create a new mode state.
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            pending_operator: None,
            count: None,
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
        if mode != Mode::Command {
            self.command_line.clear();
        }
    }

    /// Get the command line content.
    pub fn command_line(&self) -> &str {
        &self.command_line
    }

    /// Get the effective count.
    pub fn effective_count(&self) -> usize {
        self.count.unwrap_or(1)
    }

    /// Process a key event and return an intent.
    pub fn process_key(&mut self, key: KeyEvent) -> Option<Intent> {
        match self.mode {
            Mode::Normal => self.process_normal(key),
            Mode::Insert => self.process_insert(key),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => self.process_visual(key),
            Mode::Command => self.process_command(key),
            Mode::Replace => self.process_replace(key),
            Mode::Search => self.process_search(key),
        }
    }

    fn process_normal(&mut self, key: KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Char(c, Modifier { ctrl: false, .. }) => {
                // Count handling
                if c.is_ascii_digit() && (self.count.is_some() || c != '0') {
                    let digit = c.to_digit(10).unwrap() as usize;
                    self.count = Some(self.count.unwrap_or(0) * 10 + digit);
                    return Some(Intent::Noop);
                }

                let count = self.effective_count();
                self.count = None;

                match c {
                    'h' => Some(Intent::Motion(Motion::Left, count)),
                    'l' => Some(Intent::Motion(Motion::Right, count)),
                    'j' => Some(Intent::Motion(Motion::Down, count)),
                    'k' => Some(Intent::Motion(Motion::Up, count)),
                    '0' => Some(Intent::Motion(Motion::FirstColumn, 1)),
                    '$' => Some(Intent::Motion(Motion::LineEnd, 1)),
                    '^' => Some(Intent::Motion(Motion::LineStart, 1)),
                    'w' => Some(Intent::Motion(Motion::WordForward, count)),
                    'b' => Some(Intent::Motion(Motion::WordBackward, count)),
                    'e' => Some(Intent::Motion(Motion::WordEnd, count)),
                    'G' => Some(Intent::Motion(Motion::FileEnd, 1)),
                    'i' => {
                        self.set_mode(Mode::Insert);
                        Some(Intent::EnterMode(Mode::Insert))
                    }
                    'a' => {
                        self.set_mode(Mode::Insert);
                        Some(Intent::Append)
                    }
                    'o' => {
                        self.set_mode(Mode::Insert);
                        Some(Intent::OpenBelow)
                    }
                    'O' => {
                        self.set_mode(Mode::Insert);
                        Some(Intent::OpenAbove)
                    }
                    'v' => {
                        self.set_mode(Mode::Visual);
                        Some(Intent::EnterMode(Mode::Visual))
                    }
                    'V' => {
                        self.set_mode(Mode::VisualLine);
                        Some(Intent::EnterMode(Mode::VisualLine))
                    }
                    'R' => {
                        self.set_mode(Mode::Replace);
                        Some(Intent::EnterMode(Mode::Replace))
                    }
                    ':' => {
                        self.set_mode(Mode::Command);
                        self.command_line.clear();
                        Some(Intent::EnterMode(Mode::Command))
                    }
                    'x' => Some(Intent::DeleteChar),
                    'u' => Some(Intent::Undo),
                    'd' => {
                        if self.pending_operator == Some(Operator::Delete) {
                            self.pending_operator = None;
                            Some(Intent::OperatorLine(Operator::Delete, count))
                        } else {
                            self.pending_operator = Some(Operator::Delete);
                            Some(Intent::Noop)
                        }
                    }
                    'c' => {
                        if self.pending_operator == Some(Operator::Change) {
                            self.pending_operator = None;
                            self.set_mode(Mode::Insert);
                            Some(Intent::OperatorLine(Operator::Change, count))
                        } else {
                            self.pending_operator = Some(Operator::Change);
                            Some(Intent::Noop)
                        }
                    }
                    'y' => {
                        if self.pending_operator == Some(Operator::Yank) {
                            self.pending_operator = None;
                            Some(Intent::OperatorLine(Operator::Yank, count))
                        } else {
                            self.pending_operator = Some(Operator::Yank);
                            Some(Intent::Noop)
                        }
                    }
                    'g' => Some(Intent::Noop), // Prefix, wait for next
                    _ => Some(Intent::Noop),
                }
            }
            KeyEvent::Char('r', Modifier { ctrl: true, .. }) => Some(Intent::Redo),
            KeyEvent::Escape => {
                self.pending_operator = None;
                self.count = None;
                Some(Intent::Cancel)
            }
            _ => Some(Intent::Noop),
        }
    }

    fn process_insert(&mut self, key: KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.set_mode(Mode::Normal);
                Some(Intent::EnterMode(Mode::Normal))
            }
            KeyEvent::Char(c, Modifier { ctrl: false, .. }) => Some(Intent::InsertChar(c)),
            KeyEvent::Backspace => Some(Intent::Backspace),
            KeyEvent::Enter => Some(Intent::Newline),
            KeyEvent::Left => Some(Intent::Motion(Motion::Left, 1)),
            KeyEvent::Right => Some(Intent::Motion(Motion::Right, 1)),
            KeyEvent::Up => Some(Intent::Motion(Motion::Up, 1)),
            KeyEvent::Down => Some(Intent::Motion(Motion::Down, 1)),
            _ => Some(Intent::Noop),
        }
    }

    fn process_visual(&mut self, key: KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.set_mode(Mode::Normal);
                Some(Intent::EnterMode(Mode::Normal))
            }
            KeyEvent::Char('h', _) => Some(Intent::Motion(Motion::Left, 1)),
            KeyEvent::Char('l', _) => Some(Intent::Motion(Motion::Right, 1)),
            KeyEvent::Char('j', _) => Some(Intent::Motion(Motion::Down, 1)),
            KeyEvent::Char('k', _) => Some(Intent::Motion(Motion::Up, 1)),
            KeyEvent::Char('d', _) | KeyEvent::Char('x', _) => {
                self.set_mode(Mode::Normal);
                Some(Intent::OperatorMotion(Operator::Delete, Motion::Right, 1))
            }
            KeyEvent::Char('y', _) => {
                self.set_mode(Mode::Normal);
                Some(Intent::OperatorMotion(Operator::Yank, Motion::Right, 1))
            }
            _ => Some(Intent::Noop),
        }
    }

    fn process_command(&mut self, key: KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.set_mode(Mode::Normal);
                self.command_line.clear();
                Some(Intent::Cancel)
            }
            KeyEvent::Enter => {
                let cmd = std::mem::take(&mut self.command_line);
                self.set_mode(Mode::Normal);
                Some(Intent::ExecuteCommand(cmd))
            }
            KeyEvent::Backspace => {
                if self.command_line.is_empty() {
                    self.set_mode(Mode::Normal);
                    Some(Intent::Cancel)
                } else {
                    self.command_line.pop();
                    Some(Intent::CommandLineBackspace)
                }
            }
            KeyEvent::Char(c, _) => {
                self.command_line.push(c);
                Some(Intent::CommandLineAppend(c))
            }
            _ => Some(Intent::Noop),
        }
    }

    fn process_replace(&mut self, key: KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.set_mode(Mode::Normal);
                Some(Intent::EnterMode(Mode::Normal))
            }
            KeyEvent::Char(c, _) => Some(Intent::InsertChar(c)),
            _ => Some(Intent::Noop),
        }
    }

    fn process_search(&mut self, key: KeyEvent) -> Option<Intent> {
        match key {
            KeyEvent::Escape => {
                self.set_mode(Mode::Normal);
                Some(Intent::Cancel)
            }
            KeyEvent::Enter => {
                self.set_mode(Mode::Normal);
                Some(Intent::Noop)
            }
            _ => Some(Intent::Noop),
        }
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
        let intent = state.process_key(KeyEvent::Char('i', Modifier::NONE));
        assert_eq!(state.mode(), Mode::Insert);
        assert_eq!(intent, Some(Intent::EnterMode(Mode::Insert)));
    }

    #[test]
    fn test_insert_escape() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Insert);
        let intent = state.process_key(KeyEvent::Escape);
        assert_eq!(state.mode(), Mode::Normal);
        assert_eq!(intent, Some(Intent::EnterMode(Mode::Normal)));
    }
}
