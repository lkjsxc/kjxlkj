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
}
