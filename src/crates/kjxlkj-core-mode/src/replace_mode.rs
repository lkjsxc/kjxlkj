//! Replace mode semantics: single-char and continuous character replacement.
//!
//! In Replace mode, typed characters overwrite existing text rather than inserting.

/// Replace mode state.
#[derive(Debug, Clone)]
pub struct ReplaceState {
    /// Original characters that were replaced (for undo on Backspace).
    originals: Vec<Option<char>>,
    /// Column where replace mode was entered.
    start_col: usize,
    /// Whether this is single-char replace (r) or continuous (R).
    pub single: bool,
}

impl ReplaceState {
    /// Create a new replace state for continuous Replace mode (R).
    pub fn continuous(col: usize) -> Self {
        Self { originals: Vec::new(), start_col: col, single: false }
    }

    /// Create a new replace state for single-char replace (r).
    pub fn single_char(col: usize) -> Self {
        Self { originals: Vec::new(), start_col: col, single: true }
    }

    /// The column where replace mode started.
    pub fn start_col(&self) -> usize { self.start_col }

    /// Number of characters replaced so far.
    pub fn replaced_count(&self) -> usize { self.originals.len() }

    /// Push the original char that will be overwritten (None if at EOL / extending).
    pub fn push_original(&mut self, ch: Option<char>) { self.originals.push(ch); }

    /// Pop the last original char (for Backspace undo in Replace mode).
    pub fn pop_original(&mut self) -> Option<Option<char>> { self.originals.pop() }
}

/// Apply a replacement character at the given position in the line.
/// Returns the new line and the original char that was overwritten.
pub fn replace_char_at(line: &str, col: usize, replacement: char) -> (String, Option<char>) {
    let chars: Vec<char> = line.chars().collect();
    let original = chars.get(col).copied();
    let mut result: Vec<char> = chars;
    if col < result.len() {
        result[col] = replacement;
    } else {
        // Extending past end of line
        while result.len() < col { result.push(' '); }
        result.push(replacement);
    }
    (result.into_iter().collect(), original)
}

/// Undo one replacement using the stored original character.
/// If original is None, the char was appended past EOL and should be removed.
pub fn undo_replace_at(line: &str, col: usize, original: Option<char>) -> String {
    let mut chars: Vec<char> = line.chars().collect();
    match original {
        Some(ch) if col < chars.len() => { chars[col] = ch; }
        None if col < chars.len() => { chars.remove(col); }
        _ => {}
    }
    chars.into_iter().collect()
}

/// Apply a single-char replace (r command): replace char at cursor, stay in Normal.
pub fn apply_single_replace(line: &str, col: usize, replacement: char) -> String {
    let (new_line, _) = replace_char_at(line, col, replacement);
    new_line
}

/// Check if a character is a valid replacement target (not newline).
pub fn is_valid_replacement(ch: char) -> bool {
    ch != '\n' && ch != '\r'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_in_middle() {
        let (result, orig) = replace_char_at("hello", 2, 'X');
        assert_eq!(result, "heXlo");
        assert_eq!(orig, Some('l'));
    }

    #[test]
    fn replace_at_end() {
        let (result, orig) = replace_char_at("hello", 4, 'X');
        assert_eq!(result, "hellX");
        assert_eq!(orig, Some('o'));
    }

    #[test]
    fn replace_past_end() {
        let (result, orig) = replace_char_at("hi", 5, 'X');
        assert_eq!(result, "hi   X");
        assert_eq!(orig, None);
    }

    #[test]
    fn undo_replace() {
        let line = "heXlo";
        let restored = undo_replace_at(line, 2, Some('l'));
        assert_eq!(restored, "hello");
    }

    #[test]
    fn undo_replace_appended() {
        let line = "hi   X";
        let restored = undo_replace_at(line, 5, None);
        assert_eq!(restored, "hi   ");
    }

    #[test]
    fn single_replace() {
        let result = apply_single_replace("world", 0, 'W');
        assert_eq!(result, "World");
    }

    #[test]
    fn replace_state_push_pop() {
        let mut state = ReplaceState::continuous(5);
        state.push_original(Some('a'));
        state.push_original(Some('b'));
        state.push_original(None);
        assert_eq!(state.replaced_count(), 3);
        assert_eq!(state.pop_original(), Some(None));
        assert_eq!(state.pop_original(), Some(Some('b')));
        assert_eq!(state.replaced_count(), 1);
    }

    #[test]
    fn single_char_state() {
        let state = ReplaceState::single_char(10);
        assert!(state.single);
        assert_eq!(state.start_col(), 10);
    }

    #[test]
    fn valid_replacement_chars() {
        assert!(is_valid_replacement('a'));
        assert!(is_valid_replacement(' '));
        assert!(!is_valid_replacement('\n'));
    }
}
