//! Grapheme, display-width, and word-motion helpers.

use unicode_width::UnicodeWidthChar;

/// Display width of a string (tabs count as 8).
pub fn display_width(s: &str) -> usize {
    s.chars()
        .map(|c| if c == '\t' { 8 } else { c.width().unwrap_or(0) })
        .sum()
}

/// Convert a char index within a line to a display column.
pub fn char_to_col(line: &str, char_idx: usize) -> usize {
    line.chars()
        .take(char_idx)
        .map(|c| if c == '\t' { 8 } else { c.width().unwrap_or(0) })
        .sum()
}

/// Convert a display column to the nearest char index.
pub fn col_to_char(line: &str, col: usize) -> usize {
    let mut accum = 0usize;
    for (i, c) in line.chars().enumerate() {
        if accum >= col {
            return i;
        }
        accum += if c == '\t' { 8 } else { c.width().unwrap_or(0) };
    }
    line.chars().count()
}

/// True if `c` is a word character (alphanumeric or underscore).
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Index of the first non-blank character on the line.
pub fn first_non_blank(line: &str) -> usize {
    line.chars().position(|c| !c.is_whitespace()).unwrap_or(0)
}

/// Index of the last non-blank character on the line.
pub fn last_non_blank(line: &str) -> usize {
    let len = line.chars().count();
    if len == 0 {
        return 0;
    }
    for (i, c) in line.chars().rev().enumerate() {
        if !c.is_whitespace() {
            return len - 1 - i;
        }
    }
    0
}

// ---------------------------------------------------------------------------
// Word motions
// ---------------------------------------------------------------------------

/// Classify a character for word-motion purposes.
fn char_class(c: char) -> u8 {
    if c.is_whitespace() {
        0
    } else if is_word_char(c) {
        1
    } else {
        2 // punctuation / symbol
    }
}

/// `w` motion: next word start, forward.
pub fn word_start_forward(line: &str, pos: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    if pos >= len {
        return len;
    }
    let cls = char_class(chars[pos]);
    let mut i = pos + 1;
    // skip same class
    while i < len && char_class(chars[i]) == cls {
        i += 1;
    }
    // skip whitespace
    while i < len && chars[i].is_whitespace() {
        i += 1;
    }
    i
}

/// `b` motion: previous word start, backward.
pub fn word_start_backward(line: &str, pos: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    if pos == 0 || chars.is_empty() {
        return 0;
    }
    let mut i = pos.min(chars.len()) - 1;
    // skip whitespace
    while i > 0 && chars[i].is_whitespace() {
        i -= 1;
    }
    let cls = char_class(chars[i]);
    while i > 0 && char_class(chars[i - 1]) == cls {
        i -= 1;
    }
    i
}

/// `e` motion: next word end, forward.
pub fn word_end_forward(line: &str, pos: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    if len == 0 {
        return 0;
    }
    let mut i = if pos + 1 < len {
        pos + 1
    } else {
        return len.saturating_sub(1);
    };
    // skip whitespace
    while i < len && chars[i].is_whitespace() {
        i += 1;
    }
    if i >= len {
        return len.saturating_sub(1);
    }
    let cls = char_class(chars[i]);
    while i + 1 < len && char_class(chars[i + 1]) == cls {
        i += 1;
    }
    i
}

/// `ge` motion: previous word end, backward.
pub fn prev_word_end(line: &str, pos: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    if pos == 0 || chars.is_empty() {
        return 0;
    }
    let mut i = pos.min(chars.len()) - 1;
    // skip whitespace
    while i > 0 && chars[i].is_whitespace() {
        i -= 1;
    }
    if i == 0 {
        return 0;
    }
    let cls = char_class(chars[i]);
    while i > 0 && char_class(chars[i - 1]) == cls {
        i -= 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_width_basic() {
        assert_eq!(display_width("abc"), 3);
        assert_eq!(display_width("a\tb"), 10); // 1 + 8 + 1
    }

    #[test]
    fn word_forward() {
        let s = "hello world";
        assert_eq!(word_start_forward(s, 0), 6);
    }

    #[test]
    fn word_backward() {
        let s = "hello world";
        assert_eq!(word_start_backward(s, 8), 6);
    }

    #[test]
    fn first_last_non_blank() {
        assert_eq!(first_non_blank("  abc  "), 2);
        assert_eq!(last_non_blank("  abc  "), 4);
    }
}
