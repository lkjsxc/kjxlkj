//! Unicode input modes: code point entry (Ctrl-V u{hex})
//! and Unicode name search.

/// Unicode input state.
#[derive(Debug, Clone, Default)]
pub struct UnicodeInputState {
    /// Whether Unicode input mode is active.
    pub active: bool,
    /// Accumulated hex digits.
    pub hex_digits: String,
    /// Maximum hex digits allowed.
    pub max_digits: usize,
    /// Whether in named lookup mode.
    pub named_mode: bool,
    /// Name search query.
    pub name_query: String,
}

impl UnicodeInputState {
    pub fn new() -> Self {
        Self {
            max_digits: 8,
            ..Default::default()
        }
    }

    /// Enter unicode hex input mode (Ctrl-V u).
    pub fn start_hex(&mut self) {
        self.active = true;
        self.hex_digits.clear();
        self.named_mode = false;
    }

    /// Enter unicode name lookup mode.
    pub fn start_named(&mut self) {
        self.active = true;
        self.named_mode = true;
        self.name_query.clear();
    }

    /// Add a hex digit. Returns Some(char) if complete.
    pub fn add_hex_digit(&mut self, ch: char) -> Option<char> {
        if !ch.is_ascii_hexdigit() {
            return self.finish();
        }
        self.hex_digits.push(ch);
        if self.hex_digits.len() >= self.max_digits {
            self.finish()
        } else {
            None
        }
    }

    /// Finish hex input and convert to char.
    pub fn finish(&mut self) -> Option<char> {
        if self.hex_digits.is_empty() {
            self.cancel();
            return None;
        }
        let code = u32::from_str_radix(&self.hex_digits, 16).ok()?;
        let ch = char::from_u32(code)?;
        self.cancel();
        Some(ch)
    }

    /// Cancel unicode input.
    pub fn cancel(&mut self) {
        self.active = false;
        self.hex_digits.clear();
        self.named_mode = false;
        self.name_query.clear();
    }

    /// Look up a Unicode character by code point.
    pub fn lookup_codepoint(code: u32) -> Option<char> {
        char::from_u32(code)
    }

    /// Look up common Unicode characters by name fragment.
    pub fn lookup_by_name(query: &str) -> Vec<(char, &'static str)> {
        let q = query.to_uppercase();
        NAMED_CHARS
            .iter()
            .filter(|(_, name)| name.contains(&q))
            .copied()
            .collect()
    }
}

/// Common named Unicode characters for lookup.
static NAMED_CHARS: &[(char, &str)] = &[
    ('©', "COPYRIGHT SIGN"),
    ('®', "REGISTERED SIGN"),
    ('™', "TRADE MARK SIGN"),
    ('€', "EURO SIGN"),
    ('£', "POUND SIGN"),
    ('¥', "YEN SIGN"),
    ('°', "DEGREE SIGN"),
    ('±', "PLUS-MINUS SIGN"),
    ('µ', "MICRO SIGN"),
    ('¶', "PILCROW SIGN"),
    ('·', "MIDDLE DOT"),
    ('×', "MULTIPLICATION SIGN"),
    ('÷', "DIVISION SIGN"),
    ('∞', "INFINITY"),
    ('≠', "NOT EQUAL TO"),
    ('≤', "LESS-THAN OR EQUAL TO"),
    ('≥', "GREATER-THAN OR EQUAL TO"),
    ('α', "GREEK SMALL LETTER ALPHA"),
    ('β', "GREEK SMALL LETTER BETA"),
    ('γ', "GREEK SMALL LETTER GAMMA"),
    ('δ', "GREEK SMALL LETTER DELTA"),
    ('π', "GREEK SMALL LETTER PI"),
    ('λ', "GREEK SMALL LETTER LAMBDA"),
    ('→', "RIGHTWARDS ARROW"),
    ('←', "LEFTWARDS ARROW"),
    ('↑', "UPWARDS ARROW"),
    ('↓', "DOWNWARDS ARROW"),
    ('✓', "CHECK MARK"),
    ('✗', "BALLOT X"),
    ('♠', "BLACK SPADE SUIT"),
    ('♥', "BLACK HEART SUIT"),
    ('♦', "BLACK DIAMOND SUIT"),
    ('♣', "BLACK CLUB SUIT"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_input() {
        let mut state = UnicodeInputState::new();
        state.start_hex();
        assert!(state.active);
        assert!(state.add_hex_digit('0').is_none());
        assert!(state.add_hex_digit('0').is_none());
        assert!(state.add_hex_digit('4').is_none());
        state.add_hex_digit('1');
        // 0x0041 = 'A', need to call finish explicitly
        let result = state.finish();
        assert_eq!(result, Some('A'));
    }

    #[test]
    fn finish_partial() {
        let mut state = UnicodeInputState::new();
        state.start_hex();
        state.add_hex_digit('2');
        state.add_hex_digit('6');
        let result = state.finish();
        // 0x26 = '&'
        assert_eq!(result, Some('&'));
    }

    #[test]
    fn lookup_by_name() {
        let results = UnicodeInputState::lookup_by_name("arrow");
        assert!(results.len() >= 4);
        assert!(results.iter().any(|(c, _)| *c == '→'));
    }

    #[test]
    fn cancel() {
        let mut state = UnicodeInputState::new();
        state.start_hex();
        state.cancel();
        assert!(!state.active);
    }
}
