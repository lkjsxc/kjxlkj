//! Input parser.

use crate::{Key, KeyCodeWrapper, KeyMap, KeySequence};
use kjxlkj_core_edit::{Motion, MotionKind};
use kjxlkj_core_mode::{Intent, IntentKind};
use kjxlkj_core_types::Mode;

/// Input parser for converting keys to intents.
#[derive(Debug, Clone)]
pub struct InputParser {
    /// Current key sequence.
    sequence: KeySequence,
    /// Pending count.
    count: Option<usize>,
    /// Key maps by mode.
    normal_map: KeyMap,
    _insert_map: KeyMap,
}

impl InputParser {
    /// Creates a new input parser.
    pub fn new() -> Self {
        Self {
            sequence: KeySequence::new(),
            count: None,
            normal_map: Self::default_normal_map(),
            _insert_map: Self::default_insert_map(),
        }
    }

    /// Parses a key in the given mode.
    pub fn parse(&mut self, key: Key, mode: Mode) -> Option<Intent> {
        match mode {
            Mode::Normal => self.parse_normal(key),
            Mode::Insert => self.parse_insert(key),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => self.parse_visual(key),
            Mode::Command => self.parse_command(key),
            _ => None,
        }
    }

    /// Parses normal mode input.
    fn parse_normal(&mut self, key: Key) -> Option<Intent> {
        // Handle count accumulation
        if let KeyCodeWrapper::Char(c) = &key.code {
            if c.is_ascii_digit() && (self.count.is_some() || *c != '0') {
                let digit = *c as u8 - b'0';
                let current = self.count.unwrap_or(0);
                self.count = Some(current * 10 + digit as usize);
                return None;
            }
        }

        // Check for escape
        if key.is_esc() {
            self.reset();
            return Some(Intent::noop());
        }

        // Look up in keymap
        if let Some(intent) = self.normal_map.lookup(&key).cloned() {
            let count = self.count.take().unwrap_or(1);
            self.reset();
            return Some(intent.with_count(count));
        }

        // Basic motions
        let intent = match &key.code {
            KeyCodeWrapper::Char('h') => Some(Intent::motion(Motion::new(MotionKind::Left))),
            KeyCodeWrapper::Char('j') => Some(Intent::motion(Motion::new(MotionKind::Down))),
            KeyCodeWrapper::Char('k') => Some(Intent::motion(Motion::new(MotionKind::Up))),
            KeyCodeWrapper::Char('l') => Some(Intent::motion(Motion::new(MotionKind::Right))),
            KeyCodeWrapper::Char('w') => Some(Intent::motion(Motion::new(MotionKind::WordStart))),
            KeyCodeWrapper::Char('b') => Some(Intent::motion(Motion::new(MotionKind::WordBack))),
            KeyCodeWrapper::Char('e') => Some(Intent::motion(Motion::new(MotionKind::WordEnd))),
            KeyCodeWrapper::Char('0') => Some(Intent::motion(Motion::new(MotionKind::LineStart))),
            KeyCodeWrapper::Char('$') => Some(Intent::motion(Motion::new(MotionKind::LineEnd))),
            KeyCodeWrapper::Char('i') => Some(Intent::change_mode(Mode::Insert)),
            KeyCodeWrapper::Char('a') => Some(Intent::change_mode(Mode::Insert)),
            KeyCodeWrapper::Char('v') => Some(Intent::change_mode(Mode::Visual)),
            KeyCodeWrapper::Char('V') => Some(Intent::change_mode(Mode::VisualLine)),
            KeyCodeWrapper::Char(':') => Some(Intent::change_mode(Mode::Command)),
            KeyCodeWrapper::Char('u') => Some(Intent::new(IntentKind::Undo)),
            _ => None,
        };

        if let Some(mut intent) = intent {
            let count = self.count.take().unwrap_or(1);
            intent = intent.with_count(count);
            self.reset();
            Some(intent)
        } else {
            None
        }
    }

    /// Parses insert mode input.
    fn parse_insert(&mut self, key: Key) -> Option<Intent> {
        if key.is_esc() {
            return Some(Intent::change_mode(Mode::Normal));
        }

        match &key.code {
            KeyCodeWrapper::Char(c) => Some(Intent::new(IntentKind::InsertText {
                text: c.to_string(),
            })),
            KeyCodeWrapper::Enter => Some(Intent::new(IntentKind::InsertNewline)),
            KeyCodeWrapper::Backspace => Some(Intent::new(IntentKind::Backspace)),
            KeyCodeWrapper::Delete => Some(Intent::new(IntentKind::DeleteChar)),
            _ => None,
        }
    }

    /// Parses visual mode input.
    fn parse_visual(&mut self, key: Key) -> Option<Intent> {
        if key.is_esc() {
            return Some(Intent::change_mode(Mode::Normal));
        }
        self.parse_normal(key)
    }

    /// Parses command mode input.
    fn parse_command(&mut self, key: Key) -> Option<Intent> {
        if key.is_esc() {
            return Some(Intent::change_mode(Mode::Normal));
        }
        None
    }

    /// Resets the parser state.
    fn reset(&mut self) {
        self.sequence.clear();
        self.count = None;
    }

    /// Default normal mode keymap.
    fn default_normal_map() -> KeyMap {
        KeyMap::new()
    }

    /// Default insert mode keymap.
    fn default_insert_map() -> KeyMap {
        KeyMap::new()
    }
}

impl Default for InputParser {
    fn default() -> Self {
        Self::new()
    }
}
