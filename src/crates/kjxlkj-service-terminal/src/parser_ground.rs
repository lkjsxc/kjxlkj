//! Ground state handler for the VT parser.

use crate::parser::Parser;
use crate::screen::Screen;
use unicode_width::UnicodeWidthChar;

impl Parser {
    /// Handle a byte in Ground state.
    pub(crate) fn ground(&mut self, byte: u8, screen: &mut Screen) {
        // UTF-8 continuation
        if self.utf8_remaining > 0 {
            if byte & 0xC0 == 0x80 {
                self.utf8_buf.push(byte);
                self.utf8_remaining -= 1;
                if self.utf8_remaining == 0 {
                    self.emit_utf8(screen);
                }
            } else {
                screen.write_char('\u{FFFD}', 1);
                self.utf8_buf.clear();
                self.utf8_remaining = 0;
                self.ground(byte, screen);
            }
            return;
        }
        match byte {
            0x20..=0x7E => screen.write_char(byte as char, 1),
            0x0A => screen.newline(),
            0x0D => screen.carriage_return(),
            0x08 => screen.backspace(),
            0x09 => {
                let tab = 8 - (screen.cursor_col % 8);
                for _ in 0..tab {
                    screen.write_char(' ', 1);
                }
            }
            0x07 => {} // BEL: ignore
            0xC0..=0xDF => {
                self.utf8_buf.clear();
                self.utf8_buf.push(byte);
                self.utf8_remaining = 1;
            }
            0xE0..=0xEF => {
                self.utf8_buf.clear();
                self.utf8_buf.push(byte);
                self.utf8_remaining = 2;
            }
            0xF0..=0xF7 => {
                self.utf8_buf.clear();
                self.utf8_buf.push(byte);
                self.utf8_remaining = 3;
            }
            _ => {} // Ignore other control chars
        }
    }

    pub(crate) fn emit_utf8(&mut self, screen: &mut Screen) {
        if let Ok(s) = std::str::from_utf8(&self.utf8_buf) {
            for ch in s.chars() {
                let w = ch.width().unwrap_or(0) as u8;
                screen.write_char(ch, w.max(1));
            }
        } else {
            screen.write_char('\u{FFFD}', 1);
        }
        self.utf8_buf.clear();
    }
}
