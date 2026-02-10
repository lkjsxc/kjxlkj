//! Escape and CSI state handlers for the VT parser.

use crate::parser::{Parser, ParserState};
use crate::screen::Screen;

impl Parser {
    pub(crate) fn escape(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            b'[' => {
                self.state = ParserState::CsiEntry;
                self.clear();
            }
            b']' => {
                self.state = ParserState::OscString;
                self.osc_string.clear();
            }
            b'7' => {
                screen.saved_cursor = Some((screen.cursor_row, screen.cursor_col));
                self.state = ParserState::Ground;
            }
            b'8' => {
                if let Some((r, c)) = screen.saved_cursor {
                    screen.move_cursor(r, c);
                }
                self.state = ParserState::Ground;
            }
            b'M' => {
                if screen.cursor_row == screen.scroll_top {
                    screen.scroll_down(1);
                } else {
                    screen.cursor_row = screen.cursor_row.saturating_sub(1);
                }
                self.state = ParserState::Ground;
            }
            0x20..=0x2F => {
                self.intermediates.push(byte);
                self.state = ParserState::EscapeIntermediate;
            }
            0x30..=0x7E => self.state = ParserState::Ground,
            _ => self.state = ParserState::Ground,
        }
    }

    pub(crate) fn escape_intermediate(&mut self, byte: u8, _screen: &mut Screen) {
        match byte {
            0x20..=0x2F => self.intermediates.push(byte),
            0x30..=0x7E => self.state = ParserState::Ground,
            _ => self.state = ParserState::Ground,
        }
    }

    pub(crate) fn csi_entry(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            b'?' => {
                self.private_marker = true;
                self.state = ParserState::CsiParam;
            }
            0x30..=0x39 => {
                self.current_param = (byte - b'0') as u16;
                self.state = ParserState::CsiParam;
            }
            b';' => {
                self.params.push(0);
                self.state = ParserState::CsiParam;
            }
            0x40..=0x7E => {
                self.params.push(self.current_param);
                crate::csi::dispatch(byte, self, screen);
                self.state = ParserState::Ground;
            }
            _ => self.state = ParserState::CsiIgnore,
        }
    }

    pub(crate) fn csi_param(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            0x30..=0x39 => {
                self.current_param = self.current_param * 10 + (byte - b'0') as u16;
            }
            b';' => {
                self.params.push(self.current_param);
                self.current_param = 0;
            }
            0x20..=0x2F => {
                self.params.push(self.current_param);
                self.current_param = 0;
                self.intermediates.push(byte);
                self.state = ParserState::CsiIntermediate;
            }
            0x40..=0x7E => {
                self.params.push(self.current_param);
                self.current_param = 0;
                crate::csi::dispatch(byte, self, screen);
                self.state = ParserState::Ground;
            }
            _ => self.state = ParserState::CsiIgnore,
        }
    }

    pub(crate) fn csi_intermediate(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            0x20..=0x2F => self.intermediates.push(byte),
            0x40..=0x7E => {
                crate::csi::dispatch(byte, self, screen);
                self.state = ParserState::Ground;
            }
            _ => self.state = ParserState::CsiIgnore,
        }
    }

    pub(crate) fn csi_ignore(&mut self, byte: u8) {
        if (0x40..=0x7E).contains(&byte) {
            self.state = ParserState::Ground;
        }
    }

    pub(crate) fn osc_string_state(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            0x07 => {
                self.dispatch_osc(screen);
                self.state = ParserState::Ground;
            }
            0x1B => {
                self.dispatch_osc(screen);
                self.state = ParserState::Escape;
            }
            _ => {
                if byte >= 0x20 {
                    self.osc_string.push(byte as char);
                }
            }
        }
    }

    fn dispatch_osc(&mut self, screen: &mut Screen) {
        if self.osc_string.starts_with("2;") {
            screen.title = self.osc_string[2..].to_string();
        }
        self.osc_string.clear();
    }
}
