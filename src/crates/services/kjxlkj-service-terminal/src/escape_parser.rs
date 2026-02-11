//! VT100/xterm escape sequence parser (ANSI X3.64 / ECMA-48).
//! See /docs/spec/features/terminal/escape-parser.md.
use crate::screen::Screen;
use crate::csi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ground, Escape, EscapeIntermediate, CsiEntry, CsiParam,
    CsiIntermediate, CsiIgnore, OscString, DcsEntry, DcsParam,
    DcsPassthrough, DcsIgnore, SosPmApc,
}

pub struct Parser {
    pub state: State,
    params: Vec<u16>,
    current_param: Option<u16>,
    intermediates: Vec<u8>,
    osc_buf: Vec<u8>,
    private_marker: bool,
    utf8_buf: Vec<u8>,
    utf8_remaining: u8,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: State::Ground, params: Vec::new(), current_param: None,
            intermediates: Vec::new(), osc_buf: Vec::new(), private_marker: false,
            utf8_buf: Vec::new(), utf8_remaining: 0,
        }
    }
    fn clear_params(&mut self) {
        self.params.clear();
        self.current_param = None;
        self.intermediates.clear();
        self.private_marker = false;
    }
    fn finish_param(&mut self) {
        self.params.push(self.current_param.unwrap_or(0));
        self.current_param = None;
    }
    pub fn feed(&mut self, byte: u8, screen: &mut Screen) {
        match self.state {
            State::Ground => self.ground(byte, screen),
            State::Escape => self.escape(byte, screen),
            State::EscapeIntermediate => self.esc_intermediate(byte, screen),
            State::CsiEntry => self.csi_entry(byte, screen),
            State::CsiParam => self.csi_param(byte, screen),
            State::CsiIntermediate => self.csi_intermediate(byte, screen),
            State::CsiIgnore => { if (0x40..=0x7E).contains(&byte) { self.state = State::Ground; } }
            State::OscString => self.osc_string(byte, screen),
            State::DcsEntry | State::DcsParam | State::DcsPassthrough
            | State::DcsIgnore | State::SosPmApc => {
                if byte == 0x1B { self.state = State::Escape; }
                else if byte == 0x07 { self.state = State::Ground; }
            }
            _ => {}
        }
    }
    fn ground(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            0x1B => { self.state = State::Escape; self.clear_params(); }
            0x07 => {} // BEL
            0x08 => screen.backspace(),
            0x09 => screen.tab(),
            0x0A | 0x0B | 0x0C => screen.linefeed(),
            0x0D => screen.carriage_return(),
            0x00..=0x1F => {} // other C0 controls ignored
            0xC0..=0xFD if self.utf8_remaining == 0 => {
                self.utf8_buf.clear();
                self.utf8_buf.push(byte);
                self.utf8_remaining = if byte < 0xE0 { 1 } else if byte < 0xF0 { 2 } else { 3 };
            }
            0x80..=0xBF if self.utf8_remaining > 0 => {
                self.utf8_buf.push(byte);
                self.utf8_remaining -= 1;
                if self.utf8_remaining == 0 {
                    if let Ok(s) = std::str::from_utf8(&self.utf8_buf) {
                        for ch in s.chars() { screen.put_char(ch); }
                    } else {
                        screen.put_char('\u{FFFD}');
                    }
                }
            }
            0x20..=0x7E => screen.put_char(byte as char),
            _ => screen.put_char('\u{FFFD}'),
        }
    }
    fn escape(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            b'[' => { self.state = State::CsiEntry; self.clear_params(); }
            b']' => { self.state = State::OscString; self.osc_buf.clear(); }
            b'P' => { self.state = State::DcsEntry; self.clear_params(); }
            b'_' | b'^' | b'X' => { self.state = State::SosPmApc; }
            0x20..=0x2F => { self.intermediates.push(byte); self.state = State::EscapeIntermediate; }
            b'M' => { screen.reverse_index(); self.state = State::Ground; }
            b'D' => { screen.linefeed(); self.state = State::Ground; }
            b'E' => { screen.carriage_return(); screen.linefeed(); self.state = State::Ground; }
            b'7' => { screen.save_cursor(); self.state = State::Ground; }
            b'8' => { screen.restore_cursor(); self.state = State::Ground; }
            b'c' => { screen.reset(); self.state = State::Ground; }
            0x30..=0x7E => { self.state = State::Ground; }
            _ => { self.state = State::Ground; }
        }
    }
    fn esc_intermediate(&mut self, byte: u8, _screen: &mut Screen) {
        match byte {
            0x20..=0x2F => { self.intermediates.push(byte); }
            0x30..=0x7E => { self.state = State::Ground; }
            _ => { self.state = State::Ground; }
        }
    }
    fn csi_entry(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            b'?' => { self.private_marker = true; self.state = State::CsiParam; }
            0x30..=0x39 => {
                self.current_param = Some((byte - b'0') as u16);
                self.state = State::CsiParam;
            }
            b';' => { self.finish_param(); self.state = State::CsiParam; }
            0x20..=0x2F => { self.intermediates.push(byte); self.state = State::CsiIntermediate; }
            0x40..=0x7E => {
                csi::dispatch(&self.params, self.private_marker, byte, screen);
                self.state = State::Ground;
            }
            _ => { self.state = State::CsiIgnore; }
        }
    }
    fn csi_param(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            0x30..=0x39 => {
                let d = (byte - b'0') as u16;
                self.current_param = Some(self.current_param.unwrap_or(0).saturating_mul(10).saturating_add(d));
            }
            b';' => { self.finish_param(); }
            0x20..=0x2F => { self.finish_param(); self.intermediates.push(byte); self.state = State::CsiIntermediate; }
            0x40..=0x7E => {
                self.finish_param();
                csi::dispatch(&self.params, self.private_marker, byte, screen);
                self.state = State::Ground;
            }
            _ => { self.state = State::CsiIgnore; }
        }
    }
    fn csi_intermediate(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            0x20..=0x2F => { self.intermediates.push(byte); }
            0x40..=0x7E => {
                csi::dispatch(&self.params, self.private_marker, byte, screen);
                self.state = State::Ground;
            }
            _ => { self.state = State::CsiIgnore; }
        }
    }
    fn osc_string(&mut self, byte: u8, screen: &mut Screen) {
        match byte {
            0x07 => { self.osc_dispatch(screen); self.state = State::Ground; }
            0x1B => { self.state = State::Escape; self.osc_dispatch(screen); }
            _ => { self.osc_buf.push(byte); }
        }
    }
    fn osc_dispatch(&mut self, screen: &mut Screen) {
        if let Ok(s) = std::str::from_utf8(&self.osc_buf) {
            if let Some(rest) = s.strip_prefix("0;").or_else(|| s.strip_prefix("2;")) {
                screen.set_title(rest.to_string());
            }
        }
    }
}

#[cfg(test)]
#[path = "parser_tests.rs"]
mod tests;
