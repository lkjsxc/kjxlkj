//! VT escape sequence parser.

/// Parser state.
#[derive(Debug, Default)]
pub struct Parser {
    /// Current state.
    state: ParserState,
    /// Parameter buffer.
    params: Vec<u8>,
}

/// Parser state machine state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ParserState {
    /// Ground state.
    #[default]
    Ground,
    /// Escape sequence started.
    Escape,
    /// CSI sequence.
    Csi,
    /// OSC sequence.
    Osc,
}

impl Parser {
    /// Create a new parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a byte and return any actions.
    pub fn parse(&mut self, byte: u8) -> Vec<ParseAction> {
        let mut actions = Vec::new();

        match self.state {
            ParserState::Ground => {
                if byte == 0x1b {
                    self.state = ParserState::Escape;
                } else if byte >= 0x20 && byte < 0x7f {
                    actions.push(ParseAction::Print(byte as char));
                } else if byte == b'\n' {
                    actions.push(ParseAction::Newline);
                } else if byte == b'\r' {
                    actions.push(ParseAction::CarriageReturn);
                } else if byte == 0x08 {
                    actions.push(ParseAction::Backspace);
                } else if byte == 0x07 {
                    actions.push(ParseAction::Bell);
                }
            }
            ParserState::Escape => {
                if byte == b'[' {
                    self.state = ParserState::Csi;
                    self.params.clear();
                } else if byte == b']' {
                    self.state = ParserState::Osc;
                    self.params.clear();
                } else {
                    self.state = ParserState::Ground;
                }
            }
            ParserState::Csi => {
                if byte >= 0x30 && byte <= 0x3f {
                    self.params.push(byte);
                } else if byte >= 0x40 && byte <= 0x7e {
                    // Final byte.
                    actions.push(ParseAction::CsiDispatch(byte as char));
                    self.state = ParserState::Ground;
                } else {
                    self.state = ParserState::Ground;
                }
            }
            ParserState::Osc => {
                if byte == 0x07 || byte == 0x1b {
                    actions.push(ParseAction::OscDispatch);
                    self.state = ParserState::Ground;
                } else {
                    self.params.push(byte);
                }
            }
        }

        actions
    }
}

/// Parser action.
#[derive(Debug, Clone)]
pub enum ParseAction {
    /// Print a character.
    Print(char),
    /// Newline.
    Newline,
    /// Carriage return.
    CarriageReturn,
    /// Backspace.
    Backspace,
    /// Bell.
    Bell,
    /// CSI dispatch.
    CsiDispatch(char),
    /// OSC dispatch.
    OscDispatch,
}
