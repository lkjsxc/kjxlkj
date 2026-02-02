//! Text width calculation.
//!
//! Calculates display widths of characters and strings.

/// Character display width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharWidth {
    /// Zero width (combining, control).
    Zero,
    /// Normal width.
    Single,
    /// Double width (CJK, emoji).
    Double,
}

impl CharWidth {
    /// Returns the numeric width.
    pub fn width(&self) -> usize {
        match self {
            Self::Zero => 0,
            Self::Single => 1,
            Self::Double => 2,
        }
    }
}

/// Calculates the display width of a character.
pub fn char_width(ch: char) -> CharWidth {
    // Zero-width characters
    if ch == '\0' || ch == '\x7F' {
        return CharWidth::Zero;
    }

    // Combining characters
    if is_combining(ch) {
        return CharWidth::Zero;
    }

    // CJK and other wide characters
    if is_wide(ch) {
        return CharWidth::Double;
    }

    CharWidth::Single
}

/// Returns whether a character is combining.
fn is_combining(ch: char) -> bool {
    let cp = ch as u32;
    // Common combining character ranges
    (0x0300..=0x036F).contains(&cp)  // Combining Diacritical Marks
        || (0x1AB0..=0x1AFF).contains(&cp) // Combining Diacritical Marks Extended
        || (0x1DC0..=0x1DFF).contains(&cp) // Combining Diacritical Marks Supplement
        || (0x20D0..=0x20FF).contains(&cp) // Combining Diacritical Marks for Symbols
        || (0xFE20..=0xFE2F).contains(&cp) // Combining Half Marks
}

/// Returns whether a character is wide (double-width).
fn is_wide(ch: char) -> bool {
    let cp = ch as u32;
    // CJK and other wide character ranges
    (0x1100..=0x115F).contains(&cp)  // Hangul Jamo
        || (0x2E80..=0x9FFF).contains(&cp)  // CJK
        || (0xAC00..=0xD7A3).contains(&cp)  // Hangul Syllables
        || (0xF900..=0xFAFF).contains(&cp)  // CJK Compatibility Ideographs
        || (0xFE10..=0xFE1F).contains(&cp)  // Vertical Forms
        || (0xFF00..=0xFF60).contains(&cp)  // Fullwidth Forms
        || (0xFFE0..=0xFFE6).contains(&cp)  // Fullwidth Forms
        || (0x20000..=0x2FFFF).contains(&cp) // CJK Extension B+
        || (0x30000..=0x3FFFF).contains(&cp) // CJK Extension G+
}

/// Calculates the display width of a string.
pub fn str_width(s: &str) -> usize {
    s.chars().map(|c| char_width(c).width()).sum()
}

/// Calculates width up to a byte position.
pub fn width_to_byte(s: &str, byte_pos: usize) -> usize {
    let clamped = byte_pos.min(s.len());
    s[..clamped]
        .chars()
        .map(|c| char_width(c).width())
        .sum()
}

/// Finds byte position for display column.
pub fn byte_for_column(s: &str, target_col: usize) -> usize {
    let mut col = 0;
    for (byte_pos, ch) in s.char_indices() {
        if col >= target_col {
            return byte_pos;
        }
        col += char_width(ch).width();
    }
    s.len()
}

/// Truncates string to display width.
pub fn truncate_to_width(s: &str, max_width: usize) -> &str {
    let byte_pos = byte_for_column(s, max_width);
    &s[..byte_pos]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_width_ascii() {
        assert_eq!(char_width('a').width(), 1);
        assert_eq!(char_width('Z').width(), 1);
    }

    #[test]
    fn test_char_width_cjk() {
        assert_eq!(char_width('中').width(), 2);
        assert_eq!(char_width('日').width(), 2);
    }

    #[test]
    fn test_str_width() {
        assert_eq!(str_width("hello"), 5);
        assert_eq!(str_width("中文"), 4);
    }

    #[test]
    fn test_str_width_mixed() {
        assert_eq!(str_width("a中b"), 4);
    }

    #[test]
    fn test_byte_for_column() {
        let s = "a中b";
        assert_eq!(byte_for_column(s, 0), 0);
        assert_eq!(byte_for_column(s, 1), 1);
    }

    #[test]
    fn test_truncate_to_width() {
        assert_eq!(truncate_to_width("hello", 3), "hel");
    }
}
