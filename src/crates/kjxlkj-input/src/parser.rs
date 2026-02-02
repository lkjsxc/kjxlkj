//! Input parser.

use crate::parser_modes::{parse_command, parse_insert, parse_window_command};
use crate::{CommandLine, Key, KeyCodeWrapper, KeyMap, KeySequence};
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
    /// Command line buffer.
    cmdline: CommandLine,
    /// Pending Ctrl-W for window commands.
    pending_ctrl_w: bool,
}

impl InputParser {
    /// Creates a new input parser.
    pub fn new() -> Self {
        Self {
            sequence: KeySequence::new(),
            count: None,
            normal_map: KeyMap::new(),
            _insert_map: KeyMap::new(),
            cmdline: CommandLine::new(),
            pending_ctrl_w: false,
        }
    }

    /// Returns the command line.
    pub fn cmdline(&self) -> &CommandLine {
        &self.cmdline
    }

    /// Parses a key in the given mode.
    pub fn parse(&mut self, key: Key, mode: Mode) -> Option<Intent> {
        match mode {
            Mode::Normal => self.parse_normal(key),
            Mode::Insert => parse_insert(&key),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => self.parse_visual(key),
            Mode::Command => parse_command(&key, &mut self.cmdline),
            _ => None,
        }
    }

    /// Parses normal mode input.
    fn parse_normal(&mut self, key: Key) -> Option<Intent> {
        if self.pending_ctrl_w {
            self.pending_ctrl_w = false;
            return parse_window_command(&key);
        }

        if key.modifiers.ctrl {
            if let KeyCodeWrapper::Char('w') = &key.code {
                self.pending_ctrl_w = true;
                return None;
            }
        }

        if let KeyCodeWrapper::Char(c) = &key.code {
            if c.is_ascii_digit() && (self.count.is_some() || *c != '0') {
                let digit = *c as u8 - b'0';
                let current = self.count.unwrap_or(0);
                self.count = Some(current * 10 + digit as usize);
                return None;
            }
        }

        if key.is_esc() {
            self.reset();
            return Some(Intent::noop());
        }

        if let Some(intent) = self.normal_map.lookup(&key).cloned() {
            let count = self.count.take().unwrap_or(1);
            self.reset();
            return Some(intent.with_count(count));
        }

        let intent = self.parse_basic_motions(&key);

        if let Some(mut intent) = intent {
            let count = self.count.take().unwrap_or(1);
            intent = intent.with_count(count);
            self.reset();
            Some(intent)
        } else {
            None
        }
    }

    fn parse_basic_motions(&mut self, key: &Key) -> Option<Intent> {
        match &key.code {
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
            KeyCodeWrapper::Char('p') => Some(Intent::new(IntentKind::PutAfter { register: None })),
            KeyCodeWrapper::Char('P') => Some(Intent::new(IntentKind::PutBefore { register: None })),
            KeyCodeWrapper::Char('/') => {
                self.cmdline.open('/');
                Some(Intent::change_mode(Mode::Command))
            }
            KeyCodeWrapper::Char('?') => {
                self.cmdline.open('?');
                Some(Intent::change_mode(Mode::Command))
            }
            KeyCodeWrapper::Char('n') => Some(Intent::new(IntentKind::NextMatch)),
            KeyCodeWrapper::Char('N') => Some(Intent::new(IntentKind::PrevMatch)),
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

    /// Opens command line with prompt.
    pub fn open_cmdline(&mut self, prompt: char) {
        self.cmdline.open(prompt);
    }

    /// Resets the parser state.
    fn reset(&mut self) {
        self.sequence.clear();
        self.count = None;
    }
}

impl Default for InputParser {
    fn default() -> Self {
        Self::new()
    }
}
