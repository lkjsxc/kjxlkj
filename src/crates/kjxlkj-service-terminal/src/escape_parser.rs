//! Terminal escape sequence parser (subset of VT100/xterm).

use kjxlkj_core_types::{CellAttrs, Color};

/// Parser state machine for terminal escape sequences.
#[derive(Debug, Default)]
pub struct EscapeParser {
    /// Current state.
    state: ParseState,
    /// Parameter accumulator.
    params: Vec<u16>,
    /// Current parameter being built.
    current_param: u16,
    /// Intermediate characters.
    intermediates: Vec<u8>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    #[default]
    Ground,
    Escape,
    CsiEntry,
    CsiParam,
    OscString,
}

/// Parsed action from the escape parser.
#[derive(Debug, Clone)]
pub enum ParseAction {
    /// Print a character.
    Print(char),
    /// Execute a C0 control code.
    Execute(u8),
    /// CSI sequence with command char.
    CsiDispatch(char, Vec<u16>),
    /// OSC string.
    OscDispatch(String),
    /// Set foreground color.
    SetFg(Color),
    /// Set background color.
    SetBg(Color),
    /// Set/reset attributes.
    SetAttrs(CellAttrs),
    /// Reset all attributes.
    Reset,
}

impl EscapeParser {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed a byte to the parser and collect actions.
    pub fn feed(&mut self, byte: u8, actions: &mut Vec<ParseAction>) {
        match self.state {
            ParseState::Ground => self.ground(byte, actions),
            ParseState::Escape => self.escape(byte, actions),
            ParseState::CsiEntry => self.csi_entry(byte, actions),
            ParseState::CsiParam => self.csi_param(byte, actions),
            ParseState::OscString => self.osc_string(byte, actions),
        }
    }

    fn ground(&mut self, byte: u8, actions: &mut Vec<ParseAction>) {
        match byte {
            0x1b => self.state = ParseState::Escape,
            0x00..=0x1f => {
                actions.push(ParseAction::Execute(byte));
            }
            _ => {
                if let Some(ch) = char::from_u32(byte as u32) {
                    actions.push(ParseAction::Print(ch));
                }
            }
        }
    }

    fn escape(&mut self, byte: u8, actions: &mut Vec<ParseAction>) {
        match byte {
            b'[' => {
                self.state = ParseState::CsiEntry;
                self.params.clear();
                self.current_param = 0;
                self.intermediates.clear();
            }
            b']' => {
                self.state = ParseState::OscString;
                self.intermediates.clear();
            }
            _ => {
                self.state = ParseState::Ground;
            }
        }
    }

    fn csi_entry(&mut self, byte: u8, actions: &mut Vec<ParseAction>) {
        match byte {
            b'0'..=b'9' => {
                self.current_param =
                    (byte - b'0') as u16;
                self.state = ParseState::CsiParam;
            }
            b';' => {
                self.params.push(0);
                self.state = ParseState::CsiParam;
            }
            b'?' | b'>' | b'!' => {
                self.intermediates.push(byte);
            }
            0x40..=0x7e => {
                let cmd = byte as char;
                actions.push(ParseAction::CsiDispatch(
                    cmd,
                    self.params.clone(),
                ));
                self.state = ParseState::Ground;
            }
            _ => {
                self.state = ParseState::Ground;
            }
        }
    }

    fn csi_param(&mut self, byte: u8, actions: &mut Vec<ParseAction>) {
        match byte {
            b'0'..=b'9' => {
                self.current_param = self
                    .current_param
                    .saturating_mul(10)
                    .saturating_add((byte - b'0') as u16);
            }
            b';' => {
                self.params.push(self.current_param);
                self.current_param = 0;
            }
            0x40..=0x7e => {
                self.params.push(self.current_param);
                let cmd = byte as char;
                actions.push(ParseAction::CsiDispatch(
                    cmd,
                    self.params.clone(),
                ));
                self.state = ParseState::Ground;
            }
            _ => {
                self.state = ParseState::Ground;
            }
        }
    }

    fn osc_string(&mut self, byte: u8, actions: &mut Vec<ParseAction>) {
        match byte {
            0x07 | 0x1b => {
                let s = String::from_utf8_lossy(
                    &self.intermediates,
                )
                .to_string();
                actions.push(ParseAction::OscDispatch(s));
                self.state = ParseState::Ground;
            }
            _ => {
                self.intermediates.push(byte);
            }
        }
    }

    /// Reset parser state.
    pub fn reset(&mut self) {
        self.state = ParseState::Ground;
        self.params.clear();
        self.current_param = 0;
        self.intermediates.clear();
    }
}
