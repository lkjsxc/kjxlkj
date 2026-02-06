//! Word and WORD boundary detection.

use crate::TextBuffer;
use kjxlkj_core_types::Position;

/// A character is a "word" character (alphanumeric or underscore).
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

/// Classify a character for word motion purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharClass {
    Word,
    Punctuation,
    Whitespace,
}

impl CharClass {
    pub fn of(c: char) -> Self {
        char_class(c)
    }
}

fn char_class(c: char) -> CharClass {
    if is_whitespace(c) {
        CharClass::Whitespace
    } else if is_word_char(c) {
        CharClass::Word
    } else {
        CharClass::Punctuation
    }
}

/// Move to next word start (`w` motion).
pub fn word_start_forward(buf: &TextBuffer, pos: Position) -> Position {
    let max_line = buf.line_count().saturating_sub(1);
    let mut line = pos.line;
    let mut col = pos.col;

    let line_str = buf.line_to_string(line);
    let chars: Vec<char> = line_str.chars().collect();

    if col < chars.len() {
        let start_class = char_class(chars[col]);
        // Skip past current class
        while col < chars.len() && char_class(chars[col]) == start_class {
            col += 1;
        }
        // Skip whitespace
        while col < chars.len() && is_whitespace(chars[col]) {
            col += 1;
        }
        if col < chars.len() {
            return Position::new(line, col);
        }
    }

    // Move to next line
    line += 1;
    while line <= max_line {
        let ls = buf.line_to_string(line);
        let cs: Vec<char> = ls.chars().collect();
        let mut c = 0;
        while c < cs.len() && is_whitespace(cs[c]) {
            c += 1;
        }
        if c < cs.len() {
            return Position::new(line, c);
        }
        line += 1;
    }

    Position::new(max_line, buf.line_len(max_line).saturating_sub(1).max(0))
}

/// Move to previous word start (`b` motion).
pub fn word_start_backward(buf: &TextBuffer, pos: Position) -> Position {
    let mut line = pos.line;
    let mut col = pos.col;

    if col > 0 {
        let line_str = buf.line_to_string(line);
        let chars: Vec<char> = line_str.chars().collect();
        col = col.min(chars.len());

        // Skip whitespace backwards
        while col > 0 && is_whitespace(chars[col - 1]) {
            col -= 1;
        }
        if col == 0 {
            if line == 0 {
                return Position::new(0, 0);
            }
            line -= 1;
            let prev_len = buf.line_len(line);
            return word_end_of_line_backward(buf, line, prev_len);
        }

        // Skip current class backwards
        let target_class = char_class(chars[col - 1]);
        while col > 0 && char_class(chars[col - 1]) == target_class {
            col -= 1;
        }
        return Position::new(line, col);
    }

    if line == 0 {
        return Position::new(0, 0);
    }
    line -= 1;
    let prev_len = buf.line_len(line);
    word_end_of_line_backward(buf, line, prev_len)
}

fn word_end_of_line_backward(buf: &TextBuffer, line: usize, from: usize) -> Position {
    let ls = buf.line_to_string(line);
    let chars: Vec<char> = ls.chars().collect();
    let mut col = from;
    while col > 0 && is_whitespace(chars[col - 1]) {
        col -= 1;
    }
    if col == 0 {
        return Position::new(line, 0);
    }
    let target_class = char_class(chars[col - 1]);
    while col > 0 && char_class(chars[col - 1]) == target_class {
        col -= 1;
    }
    Position::new(line, col)
}

/// Move to word end (`e` motion).
pub fn word_end_forward(buf: &TextBuffer, pos: Position) -> Position {
    let max_line = buf.line_count().saturating_sub(1);
    let mut line = pos.line;
    let mut col = pos.col + 1; // We start from next char

    loop {
        let ls = buf.line_to_string(line);
        let chars: Vec<char> = ls.chars().collect();

        // Skip whitespace
        while col < chars.len() && is_whitespace(chars[col]) {
            col += 1;
        }
        if col < chars.len() {
            let target_class = char_class(chars[col]);
            while col + 1 < chars.len() && char_class(chars[col + 1]) == target_class {
                col += 1;
            }
            return Position::new(line, col);
        }

        if line >= max_line {
            return Position::new(
                max_line,
                buf.line_len(max_line).saturating_sub(1).max(0),
            );
        }
        line += 1;
        col = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_forward_simple() {
        let buf = TextBuffer::from_text("hello world foo");
        let p = word_start_forward(&buf, Position::new(0, 0));
        assert_eq!(p, Position::new(0, 6));
    }

    #[test]
    fn word_backward_simple() {
        let buf = TextBuffer::from_text("hello world");
        let p = word_start_backward(&buf, Position::new(0, 8));
        assert_eq!(p, Position::new(0, 6));
    }

    #[test]
    fn word_end_simple() {
        let buf = TextBuffer::from_text("hello world");
        let p = word_end_forward(&buf, Position::new(0, 0));
        assert_eq!(p, Position::new(0, 4));
    }

    #[test]
    fn word_forward_across_lines() {
        let buf = TextBuffer::from_text("hello\nworld");
        let p = word_start_forward(&buf, Position::new(0, 0));
        assert_eq!(p, Position::new(1, 0));
    }

    #[test]
    fn word_char_classification() {
        assert!(is_word_char('a'));
        assert!(is_word_char('_'));
        assert!(is_word_char('5'));
        assert!(!is_word_char('.'));
        assert!(!is_word_char(' '));
    }
}
