//! Input parser.

use crate::parser_modes::{parse_command, parse_insert, parse_window_command};
use crate::parser_operators::{parse_normal_key, parse_operator_argument, parse_operator_key};
use crate::{CommandLine, Key, KeyCodeWrapper, KeyMap, KeySequence};
use kjxlkj_core_edit::OperatorKind;
use kjxlkj_core_mode::{Intent, IntentKind};
use kjxlkj_core_types::Mode;

/// Input parser for converting keys to intents.
#[derive(Debug, Clone)]
pub struct InputParser {
    sequence: KeySequence,
    count: Option<usize>,
    normal_map: KeyMap,
    _insert_map: KeyMap,
    cmdline: CommandLine,
    pending_ctrl_w: bool,
    pending_leader: bool,
    pending_operator: Option<OperatorKind>,
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
            pending_leader: false,
            pending_operator: None,
        }
    }

    /// Returns the command line.
    pub fn cmdline(&self) -> &CommandLine { &self.cmdline }

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

    fn parse_normal(&mut self, key: Key) -> Option<Intent> {
        if self.pending_leader {
            self.pending_leader = false;
            return self.parse_leader_command(&key);
        }

        if let Some(op_kind) = self.pending_operator.take() {
            let result = parse_operator_argument(&key, op_kind);
            if key.is_esc() { self.reset(); }
            return result;
        }

        if self.pending_ctrl_w {
            self.pending_ctrl_w = false;
            return parse_window_command(&key);
        }

        if let KeyCodeWrapper::Char(' ') = &key.code {
            if !key.modifiers.ctrl && !key.modifiers.alt {
                self.pending_leader = true;
                return None;
            }
        }

        if key.modifiers.ctrl {
            if let KeyCodeWrapper::Char('w') = &key.code {
                self.pending_ctrl_w = true;
                return None;
            }
        }

        if let KeyCodeWrapper::Char(c) = &key.code {
            if c.is_ascii_digit() && (self.count.is_some() || *c != '0') {
                self.count = Some(self.count.unwrap_or(0) * 10 + (*c as u8 - b'0') as usize);
                return None;
            }
        }

        if key.is_esc() {
            self.reset();
            return Some(Intent::noop());
        }

        if let Some(op_kind) = parse_operator_key(&key) {
            self.pending_operator = Some(op_kind);
            return None;
        }

        if let Some(intent) = self.normal_map.lookup(&key).cloned() {
            let count = self.count.take().unwrap_or(1);
            self.reset();
            return Some(intent.with_count(count));
        }

        if let Some(mut intent) = parse_normal_key(&key, &mut self.cmdline) {
            let count = self.count.take().unwrap_or(1);
            intent = intent.with_count(count);
            self.reset();
            Some(intent)
        } else {
            None
        }
    }

    fn parse_leader_command(&self, key: &Key) -> Option<Intent> {
        match &key.code {
            KeyCodeWrapper::Char('e') => Some(Intent::new(IntentKind::ToggleFileExplorer)),
            KeyCodeWrapper::Char('E') => Some(Intent::new(IntentKind::FocusFileExplorer)),
            KeyCodeWrapper::Char('t') => Some(Intent::new(IntentKind::ToggleTerminal)),
            KeyCodeWrapper::Char('w') => Some(Intent::new(IntentKind::Save)),
            KeyCodeWrapper::Char('q') => Some(Intent::new(IntentKind::Quit)),
            _ => None,
        }
    }

    fn parse_visual(&mut self, key: Key) -> Option<Intent> {
        if key.is_esc() { return Some(Intent::change_mode(Mode::Normal)); }
        self.parse_normal(key)
    }

    /// Opens command line with prompt.
    pub fn open_cmdline(&mut self, prompt: char) { self.cmdline.open(prompt); }

    fn reset(&mut self) {
        self.sequence.clear();
        self.count = None;
        self.pending_leader = false;
        self.pending_operator = None;
    }
}

impl Default for InputParser {
    fn default() -> Self { Self::new() }
}
