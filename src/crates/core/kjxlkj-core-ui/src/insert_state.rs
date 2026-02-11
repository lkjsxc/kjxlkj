//! Insert-mode sub-state machine.
//! See /docs/spec/modes/insert/input/README.md.

/// Sub-state within Insert mode for multi-key input sequences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsertSubState {
    /// Normal insert typing.
    Normal,
    /// Ctrl-k pressed, awaiting first digraph char.
    DigraphFirst,
    /// First digraph char received, awaiting second.
    DigraphSecond(char),
    /// Ctrl-v pressed, awaiting literal input.
    LiteralPending,
    /// Numeric literal entry (radix, accumulated digits, max digits).
    LiteralNumeric(LiteralRadix, Vec<u8>, usize),
    /// Ctrl-r pressed, awaiting register name.
    RegisterPending,
    /// Ctrl-x pressed, awaiting completion sub-key.
    CtrlXPending,
}

/// Radix for Ctrl-v numeric entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralRadix {
    Decimal,
    Octal,
    Hex,
    Unicode4,
    Unicode8,
}

impl LiteralRadix {
    pub fn max_digits(self) -> usize {
        match self {
            Self::Decimal => 3,
            Self::Octal => 3,
            Self::Hex => 2,
            Self::Unicode4 => 4,
            Self::Unicode8 => 8,
        }
    }

    pub fn base(self) -> u32 {
        match self {
            Self::Decimal => 10,
            Self::Octal => 8,
            Self::Hex => 16,
            Self::Unicode4 | Self::Unicode8 => 16,
        }
    }
}

/// Resolve accumulated digits to a character.
pub fn resolve_literal(radix: LiteralRadix, digits: &[u8]) -> Option<char> {
    if digits.is_empty() { return None; }
    let mut val: u32 = 0;
    for &d in digits {
        val = val.checked_mul(radix.base())?.checked_add(d as u32)?;
    }
    match radix {
        LiteralRadix::Decimal => {
            if val > 255 { None } else { Some(val as u8 as char) }
        }
        LiteralRadix::Octal => {
            if val > 0o377 { None } else { Some(val as u8 as char) }
        }
        LiteralRadix::Hex => {
            if val > 0xFF { None } else { Some(val as u8 as char) }
        }
        LiteralRadix::Unicode4 | LiteralRadix::Unicode8 => {
            char::from_u32(val)
        }
    }
}

/// Parse a digit character in the given radix. Returns None if invalid.
pub fn parse_digit(radix: LiteralRadix, c: char) -> Option<u8> {
    match radix {
        LiteralRadix::Decimal => {
            if c.is_ascii_digit() { Some(c as u8 - b'0') } else { None }
        }
        LiteralRadix::Octal => {
            if ('0'..='7').contains(&c) { Some(c as u8 - b'0') } else { None }
        }
        LiteralRadix::Hex | LiteralRadix::Unicode4 | LiteralRadix::Unicode8 => {
            if c.is_ascii_digit() { Some(c as u8 - b'0') }
            else if ('a'..='f').contains(&c) { Some(c as u8 - b'a' + 10) }
            else if ('A'..='F').contains(&c) { Some(c as u8 - b'A' + 10) }
            else { None }
        }
    }
}

impl Default for InsertSubState {
    fn default() -> Self { Self::Normal }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_resolve() {
        assert_eq!(resolve_literal(LiteralRadix::Decimal, &[6, 5]), Some('A'));
        assert_eq!(resolve_literal(LiteralRadix::Decimal, &[9, 7]), Some('a'));
    }

    #[test]
    fn decimal_overflow() {
        assert_eq!(resolve_literal(LiteralRadix::Decimal, &[2, 5, 6]), None);
    }

    #[test]
    fn octal_resolve() {
        assert_eq!(resolve_literal(LiteralRadix::Octal, &[1, 0, 1]), Some('A'));
    }

    #[test]
    fn hex_resolve() {
        assert_eq!(resolve_literal(LiteralRadix::Hex, &[4, 1]), Some('A'));
    }

    #[test]
    fn unicode4_resolve() {
        assert_eq!(resolve_literal(LiteralRadix::Unicode4, &[0, 0, 6, 5]), Some('e'));
    }

    #[test]
    fn unicode8_emoji() {
        // U+1F600 = Grinning Face
        let digits = vec![0, 1, 0xF, 6, 0, 0];
        assert_eq!(resolve_literal(LiteralRadix::Unicode8, &digits), Some('\u{1F600}'));
    }

    #[test]
    fn parse_digit_decimal() {
        assert_eq!(parse_digit(LiteralRadix::Decimal, '5'), Some(5));
        assert_eq!(parse_digit(LiteralRadix::Decimal, 'a'), None);
    }

    #[test]
    fn parse_digit_hex() {
        assert_eq!(parse_digit(LiteralRadix::Hex, 'f'), Some(15));
        assert_eq!(parse_digit(LiteralRadix::Hex, 'F'), Some(15));
        assert_eq!(parse_digit(LiteralRadix::Hex, 'g'), None);
    }

    #[test]
    fn parse_digit_octal() {
        assert_eq!(parse_digit(LiteralRadix::Octal, '7'), Some(7));
        assert_eq!(parse_digit(LiteralRadix::Octal, '8'), None);
    }

    #[test]
    fn default_is_normal() {
        assert_eq!(InsertSubState::default(), InsertSubState::Normal);
    }

    #[test]
    fn radix_max_digits() {
        assert_eq!(LiteralRadix::Decimal.max_digits(), 3);
        assert_eq!(LiteralRadix::Unicode8.max_digits(), 8);
    }

    #[test]
    fn empty_digits_returns_none() {
        assert_eq!(resolve_literal(LiteralRadix::Decimal, &[]), None);
    }
}
