//! Whitespace display (listchars).
//!
//! Visual representation of whitespace characters.

/// Whitespace display configuration.
#[derive(Debug, Clone)]
pub struct ListChars {
    /// Tab character display.
    pub tab: (char, char),
    /// Space character (use '\0' to not display).
    pub space: char,
    /// Non-breaking space.
    pub nbsp: char,
    /// Trailing space.
    pub trail: char,
    /// End of line.
    pub eol: char,
    /// Extends past screen.
    pub extends: char,
    /// Precedes screen.
    pub precedes: char,
}

impl Default for ListChars {
    fn default() -> Self {
        Self {
            tab: ('>', ' '),
            space: '\0',
            nbsp: '+',
            trail: '-',
            eol: '$',
            extends: '>',
            precedes: '<',
        }
    }
}

impl ListChars {
    /// Creates new list chars with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the tab display string.
    pub fn tab_display(&self, width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let mut s = String::with_capacity(width);
        s.push(self.tab.0);
        for _ in 1..width {
            s.push(self.tab.1);
        }
        s
    }

    /// Returns whether space should be displayed.
    pub fn show_space(&self) -> bool {
        self.space != '\0'
    }

    /// Returns the space display char.
    pub fn space_char(&self) -> Option<char> {
        if self.space != '\0' {
            Some(self.space)
        } else {
            None
        }
    }
}

/// Whitespace display state.
#[derive(Debug, Clone, Default)]
pub struct ListMode {
    /// Whether list mode is enabled.
    pub enabled: bool,
    /// Characters to display.
    pub chars: ListChars,
}

impl ListMode {
    /// Creates new list mode.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables list mode.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables list mode.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Renders a character in list mode.
    pub fn render(&self, ch: char, is_trailing: bool, tab_width: usize) -> String {
        if !self.enabled {
            return ch.to_string();
        }

        match ch {
            '\t' => self.chars.tab_display(tab_width),
            ' ' if is_trailing => self.chars.trail.to_string(),
            ' ' if self.chars.show_space() => self.chars.space.to_string(),
            '\u{00A0}' => self.chars.nbsp.to_string(), // NBSP
            _ => ch.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_chars_default() {
        let lc = ListChars::default();
        assert_eq!(lc.tab.0, '>');
    }

    #[test]
    fn test_list_chars_tab_display() {
        let lc = ListChars::new();
        assert_eq!(lc.tab_display(4), ">   ");
    }

    #[test]
    fn test_list_chars_show_space() {
        let lc = ListChars::new();
        assert!(!lc.show_space());
    }

    #[test]
    fn test_list_mode_new() {
        let lm = ListMode::new();
        assert!(!lm.enabled);
    }

    #[test]
    fn test_list_mode_render_tab() {
        let mut lm = ListMode::new();
        lm.enable();
        let s = lm.render('\t', false, 4);
        assert_eq!(s, ">   ");
    }

    #[test]
    fn test_list_mode_render_trailing() {
        let mut lm = ListMode::new();
        lm.enable();
        let s = lm.render(' ', true, 4);
        assert_eq!(s, "-");
    }
}
