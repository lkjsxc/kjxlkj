#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Esc,
    Enter,
    Ctrl(char),
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawPrintableKey {
    pub ch: char,
    pub shift: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedEvent {
    pub raw_byte: u8,
    pub normalized_key: Key,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteStreamDecoder {
    pending: [u8; 4],
    len: usize,
    expected: usize,
}

pub fn normalize_printable(raw: RawPrintableKey) -> char {
    if raw.shift {
        raw.ch.to_ascii_uppercase()
    } else {
        raw.ch
    }
}

pub fn decode_byte(raw_byte: u8) -> DecodedEvent {
    let normalized_key = match raw_byte {
        0x1B => Key::Esc,
        0x0D => Key::Enter,
        0x01..=0x1A => Key::Ctrl((raw_byte + 96) as char),
        0x1C => Key::Ctrl('\\'),
        0x1D => Key::Ctrl(']'),
        0x1E => Key::Ctrl('^'),
        0x1F => Key::Ctrl('_'),
        0x20..=0x7E => {
            let printable = raw_byte as char;
            Key::Char(normalize_printable(RawPrintableKey {
                ch: printable,
                shift: false,
            }))
        }
        _ => Key::Unknown(raw_byte),
    };
    DecodedEvent {
        raw_byte,
        normalized_key,
    }
}

impl ByteStreamDecoder {
    pub fn new() -> Self {
        Self {
            pending: [0; 4],
            len: 0,
            expected: 0,
        }
    }

    pub fn decode_stream_byte(&mut self, raw_byte: u8) -> Option<DecodedEvent> {
        if self.expected == 0 {
            return match utf8_expected_len(raw_byte) {
                Some(expected) => {
                    self.pending[0] = raw_byte;
                    self.len = 1;
                    self.expected = expected;
                    None
                }
                None => Some(decode_byte(raw_byte)),
            };
        }

        if raw_byte & 0b1100_0000 != 0b1000_0000 {
            self.reset();
            return Some(decode_byte(raw_byte));
        }
        self.pending[self.len] = raw_byte;
        self.len += 1;
        if self.len < self.expected {
            return None;
        }

        let normalized = std::str::from_utf8(&self.pending[..self.expected])
            .ok()
            .and_then(|value| value.chars().next())
            .map(Key::Char)
            .unwrap_or(Key::Unknown(raw_byte));
        self.reset();
        Some(DecodedEvent {
            raw_byte,
            normalized_key: normalized,
        })
    }

    fn reset(&mut self) {
        self.len = 0;
        self.expected = 0;
    }
}

impl Default for ByteStreamDecoder {
    fn default() -> Self {
        Self::new()
    }
}

fn utf8_expected_len(raw: u8) -> Option<usize> {
    match raw {
        0xC2..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xF4 => Some(4),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_alpha_normalizes_to_uppercase() {
        let normalized = normalize_printable(RawPrintableKey {
            ch: 'a',
            shift: true,
        });
        assert_eq!(normalized, 'A');
    }

    #[test]
    fn shifted_raw_byte_is_already_a() {
        let decoded = decode_byte(b'A');
        assert_eq!(decoded.normalized_key, Key::Char('A'));
    }

    #[test]
    fn ctrl_w_remains_control_key() {
        let decoded = decode_byte(0x17);
        assert_eq!(decoded.normalized_key, Key::Ctrl('w'));
    }

    #[test]
    fn ctrl_backslash_is_decoded() {
        let decoded = decode_byte(0x1C);
        assert_eq!(decoded.normalized_key, Key::Ctrl('\\'));
    }

    #[test]
    fn stream_decoder_decodes_utf8_char_after_full_sequence() {
        let mut decoder = ByteStreamDecoder::new();
        assert_eq!(decoder.decode_stream_byte(0xE3), None);
        assert_eq!(decoder.decode_stream_byte(0x81), None);
        let decoded = decoder
            .decode_stream_byte(0x82)
            .expect("complete UTF-8 should decode");
        assert_eq!(decoded.normalized_key, Key::Char('あ'));
    }

    #[test]
    fn stream_decoder_recovers_when_sequence_breaks() {
        let mut decoder = ByteStreamDecoder::new();
        assert_eq!(decoder.decode_stream_byte(0xE3), None);
        let decoded = decoder
            .decode_stream_byte(b'a')
            .expect("decoder should recover on ASCII byte");
        assert_eq!(decoded.normalized_key, Key::Char('a'));
    }

    #[test]
    fn stream_decoder_keeps_control_keys_immediate() {
        let mut decoder = ByteStreamDecoder::new();
        let decoded = decoder
            .decode_stream_byte(0x17)
            .expect("ctrl keys should decode immediately");
        assert_eq!(decoded.normalized_key, Key::Ctrl('w'));
    }
}
