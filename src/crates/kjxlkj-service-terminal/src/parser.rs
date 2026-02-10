//! VT escape sequence parser state machine.
//!
//! Implementation split across parser_ground.rs and parser_esc.rs.

use crate::screen::Screen;

/// Parser state per ECMA-48.
#[derive(Debug, Clone, PartialEq)]
pub enum ParserState {
    Ground,
    Escape,
    EscapeIntermediate,
    CsiEntry,
    CsiParam,
    CsiIntermediate,
    CsiIgnore,
    OscString,
}

/// VT parser that feeds a Screen.
pub struct Parser {
    pub state: ParserState,
    pub params: Vec<u16>,
    pub current_param: u16,
    pub intermediates: Vec<u8>,
    pub osc_string: String,
    pub private_marker: bool,
    pub(crate) utf8_buf: Vec<u8>,
    pub(crate) utf8_remaining: u8,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Ground,
            params: Vec::new(),
            current_param: 0,
            intermediates: Vec::new(),
            osc_string: String::new(),
            private_marker: false,
            utf8_buf: Vec::new(),
            utf8_remaining: 0,
        }
    }

    /// Process a single byte of input.
    pub fn feed(&mut self, byte: u8, screen: &mut Screen) {
        if byte == 0x1B && self.state != ParserState::OscString {
            self.state = ParserState::Escape;
            self.clear();
            return;
        }
        match self.state {
            ParserState::Ground => self.ground(byte, screen),
            ParserState::Escape => self.escape(byte, screen),
            ParserState::EscapeIntermediate => self.escape_intermediate(byte, screen),
            ParserState::CsiEntry => self.csi_entry(byte, screen),
            ParserState::CsiParam => self.csi_param(byte, screen),
            ParserState::CsiIntermediate => self.csi_intermediate(byte, screen),
            ParserState::CsiIgnore => self.csi_ignore(byte),
            ParserState::OscString => self.osc_string_state(byte, screen),
        }
    }

    /// Process a slice of bytes.
    pub fn feed_bytes(&mut self, data: &[u8], screen: &mut Screen) {
        for &b in data {
            self.feed(b, screen);
        }
    }

    pub(crate) fn clear(&mut self) {
        self.params.clear();
        self.current_param = 0;
        self.intermediates.clear();
        self.private_marker = false;
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
