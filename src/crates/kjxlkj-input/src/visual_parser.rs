//! Visual mode input parsing.

use crossterm::event::KeyCode;
use kjxlkj_core_types::mode::VisualMode;
use kjxlkj_core_types::motion::Motion;

/// Result from parsing visual mode input.
#[derive(Debug, Clone)]
pub enum VisualResult {
    /// Motion to extend selection.
    Motion(Motion),
    /// Apply operator and exit visual.
    Delete,
    /// Yank selection and exit visual.
    Yank,
    /// Change selection (delete and enter insert).
    Change,
    /// Exit visual mode.
    Exit,
    /// Switch visual mode variant.
    Switch(VisualMode),
    /// Indent selection.
    Indent,
    /// Outdent selection.
    Outdent,
    /// Uppercase selection.
    Uppercase,
    /// Lowercase selection.
    Lowercase,
    /// Join lines in selection.
    JoinLines,
    /// No complete command yet.
    Incomplete,
}

/// Parser for visual mode input.
#[derive(Debug, Clone, Default)]
pub struct VisualParser {
    /// Count prefix.
    count: Option<u32>,
    /// g-prefix pending.
    g_prefix: bool,
}

impl VisualParser {
    /// Creates a new visual parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the parser state.
    pub fn reset(&mut self) {
        self.count = None;
        self.g_prefix = false;
    }

    /// Parses a key in visual mode.
    pub fn parse(&mut self, code: KeyCode) -> VisualResult {
        match code {
            // Escape exits visual mode
            KeyCode::Esc => {
                self.reset();
                VisualResult::Exit
            }
            // Operators
            KeyCode::Char('d') | KeyCode::Char('x') => {
                self.reset();
                VisualResult::Delete
            }
            KeyCode::Char('y') => {
                self.reset();
                VisualResult::Yank
            }
            KeyCode::Char('c') | KeyCode::Char('s') => {
                self.reset();
                VisualResult::Change
            }
            // Mode switching
            KeyCode::Char('v') => {
                self.reset();
                VisualResult::Switch(VisualMode::Char)
            }
            KeyCode::Char('V') => {
                self.reset();
                VisualResult::Switch(VisualMode::Line)
            }
            // Indent/outdent
            KeyCode::Char('>') => {
                self.reset();
                VisualResult::Indent
            }
            KeyCode::Char('<') => {
                self.reset();
                VisualResult::Outdent
            }
            // Case changes
            KeyCode::Char('U') => {
                self.reset();
                VisualResult::Uppercase
            }
            KeyCode::Char('u') => {
                self.reset();
                VisualResult::Lowercase
            }
            // Join
            KeyCode::Char('J') => {
                self.reset();
                VisualResult::JoinLines
            }
            // g prefix
            KeyCode::Char('g') if !self.g_prefix => {
                self.g_prefix = true;
                VisualResult::Incomplete
            }
            // Motions
            KeyCode::Char('h') | KeyCode::Left => {
                self.reset();
                VisualResult::Motion(Motion::Left)
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.reset();
                VisualResult::Motion(Motion::Down)
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.reset();
                VisualResult::Motion(Motion::Up)
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.reset();
                VisualResult::Motion(Motion::Right)
            }
            KeyCode::Char('w') => {
                self.reset();
                VisualResult::Motion(Motion::WordForward)
            }
            KeyCode::Char('b') => {
                self.reset();
                VisualResult::Motion(Motion::WordBackward)
            }
            KeyCode::Char('e') => {
                self.reset();
                VisualResult::Motion(Motion::WordEnd)
            }
            KeyCode::Char('0') if self.count.is_none() => {
                self.reset();
                VisualResult::Motion(Motion::LineStart)
            }
            KeyCode::Char('$') | KeyCode::End => {
                self.reset();
                VisualResult::Motion(Motion::LineEnd)
            }
            KeyCode::Char('G') => {
                self.reset();
                VisualResult::Motion(Motion::DocumentEnd)
            }
            KeyCode::Char('g') if self.g_prefix => {
                self.reset();
                VisualResult::Motion(Motion::DocumentStart)
            }
            // Count prefix
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let digit = c.to_digit(10).unwrap();
                self.count = Some(self.count.unwrap_or(0) * 10 + digit);
                VisualResult::Incomplete
            }
            _ => {
                self.reset();
                VisualResult::Incomplete
            }
        }
    }
}
