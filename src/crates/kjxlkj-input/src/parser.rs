//! Input parser.

use crate::{CommandLine, Key, KeyCodeWrapper, KeyMap, KeySequence};
use kjxlkj_core_edit::{Motion, MotionKind};
use kjxlkj_core_mode::{Intent, IntentKind, WindowDirection};
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
            normal_map: Self::default_normal_map(),
            _insert_map: Self::default_insert_map(),
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
            Mode::Insert => self.parse_insert(key),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => self.parse_visual(key),
            Mode::Command => self.parse_command(key),
            _ => None,
        }
    }

    /// Parses normal mode input.
    fn parse_normal(&mut self, key: Key) -> Option<Intent> {
        // Handle Ctrl-W window command prefix
        if self.pending_ctrl_w {
            self.pending_ctrl_w = false;
            return self.parse_window_command(key);
        }

        // Check for Ctrl-W prefix
        if key.modifiers.ctrl {
            if let KeyCodeWrapper::Char('w') = &key.code {
                self.pending_ctrl_w = true;
                return None;
            }
        }

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
            KeyCodeWrapper::Char('p') => {
                Some(Intent::new(IntentKind::PutAfter { register: None }))
            }
            KeyCodeWrapper::Char('P') => {
                Some(Intent::new(IntentKind::PutBefore { register: None }))
            }
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

    /// Parses window commands after Ctrl-W prefix.
    fn parse_window_command(&mut self, key: Key) -> Option<Intent> {
        match &key.code {
            // Split commands
            KeyCodeWrapper::Char('s') => Some(Intent::new(IntentKind::SplitHorizontal)),
            KeyCodeWrapper::Char('v') => Some(Intent::new(IntentKind::SplitVertical)),
            
            // Close commands
            KeyCodeWrapper::Char('c') | KeyCodeWrapper::Char('q') => {
                Some(Intent::new(IntentKind::CloseWindow))
            }
            KeyCodeWrapper::Char('o') => Some(Intent::new(IntentKind::OnlyWindow)),
            
            // Navigation
            KeyCodeWrapper::Char('h') => {
                Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Left)))
            }
            KeyCodeWrapper::Char('j') => {
                Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Down)))
            }
            KeyCodeWrapper::Char('k') => {
                Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Up)))
            }
            KeyCodeWrapper::Char('l') => {
                Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Right)))
            }
            KeyCodeWrapper::Left => {
                Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Left)))
            }
            KeyCodeWrapper::Down => {
                Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Down)))
            }
            KeyCodeWrapper::Up => {
                Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Up)))
            }
            KeyCodeWrapper::Right => {
                Some(Intent::new(IntentKind::WindowDirection(WindowDirection::Right)))
            }
            
            // Cycle windows
            KeyCodeWrapper::Char('w') => Some(Intent::new(IntentKind::NextWindow)),
            KeyCodeWrapper::Char('W') => Some(Intent::new(IntentKind::PrevWindow)),
            
            _ => None,
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
            self.cmdline.close();
            return Some(Intent::change_mode(Mode::Normal));
        }

        match &key.code {
            KeyCodeWrapper::Enter => {
                self.cmdline.add_to_history();
                let cmd = self.cmdline.close();
                Some(Intent::new(IntentKind::ExCommand { command: cmd }))
            }
            KeyCodeWrapper::Char(c) => {
                self.cmdline.insert(*c);
                None
            }
            KeyCodeWrapper::Backspace => {
                if !self.cmdline.backspace() && self.cmdline.input().is_empty() {
                    self.cmdline.close();
                    return Some(Intent::change_mode(Mode::Normal));
                }
                None
            }
            KeyCodeWrapper::Delete => {
                self.cmdline.delete();
                None
            }
            KeyCodeWrapper::Left => {
                self.cmdline.move_left();
                None
            }
            KeyCodeWrapper::Right => {
                self.cmdline.move_right();
                None
            }
            KeyCodeWrapper::Home => {
                self.cmdline.move_start();
                None
            }
            KeyCodeWrapper::End => {
                self.cmdline.move_end();
                None
            }
            KeyCodeWrapper::Up => {
                self.cmdline.history_prev();
                None
            }
            KeyCodeWrapper::Down => {
                self.cmdline.history_next();
                None
            }
            _ => None,
        }
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
