//! Word navigation utilities.

use kjxlkj_core_text::Rope;
use kjxlkj_core_types::CursorPosition;

/// Find next word start.
pub fn next_word_start(rope: &Rope, pos: CursorPosition, big: bool) -> CursorPosition {
    let total_chars = rope.len_chars();
    let mut char_idx = rope.line_to_char(pos.line) + pos.grapheme;

    if char_idx >= total_chars {
        return pos;
    }

    // Skip current word.
    while char_idx < total_chars {
        let c = rope.char(char_idx);
        if big {
            if c.is_whitespace() {
                break;
            }
        } else if !is_word_char(c) {
            break;
        }
        char_idx += 1;
    }

    // Skip whitespace.
    while char_idx < total_chars && rope.char(char_idx).is_whitespace() {
        char_idx += 1;
    }

    char_to_position(rope, char_idx.min(total_chars.saturating_sub(1)))
}

/// Find previous word start.
pub fn prev_word_start(rope: &Rope, pos: CursorPosition, big: bool) -> CursorPosition {
    let mut char_idx = rope.line_to_char(pos.line) + pos.grapheme;

    if char_idx == 0 {
        return pos;
    }
    char_idx -= 1;

    // Skip whitespace.
    while char_idx > 0 && rope.char(char_idx).is_whitespace() {
        char_idx -= 1;
    }

    // Find start of word.
    while char_idx > 0 {
        let c = rope.char(char_idx - 1);
        if big {
            if c.is_whitespace() {
                break;
            }
        } else if !is_word_char(c) {
            break;
        }
        char_idx -= 1;
    }

    char_to_position(rope, char_idx)
}

/// Find next word end.
pub fn next_word_end(rope: &Rope, pos: CursorPosition, big: bool) -> CursorPosition {
    let total_chars = rope.len_chars();
    let mut char_idx = rope.line_to_char(pos.line) + pos.grapheme;

    if char_idx >= total_chars {
        return pos;
    }
    char_idx += 1;

    // Skip whitespace.
    while char_idx < total_chars && rope.char(char_idx).is_whitespace() {
        char_idx += 1;
    }

    // Find end of word.
    while char_idx < total_chars - 1 {
        let next_c = rope.char(char_idx + 1);
        if big {
            if next_c.is_whitespace() {
                break;
            }
        } else if !is_word_char(next_c) {
            break;
        }
        char_idx += 1;
    }

    char_to_position(rope, char_idx.min(total_chars.saturating_sub(1)))
}

/// Find previous word end.
pub fn prev_word_end(rope: &Rope, pos: CursorPosition, big: bool) -> CursorPosition {
    let mut char_idx = rope.line_to_char(pos.line) + pos.grapheme;

    if char_idx == 0 {
        return pos;
    }
    char_idx -= 1;

    // Skip current word going backward.
    while char_idx > 0 {
        let c = rope.char(char_idx);
        if big {
            if c.is_whitespace() {
                break;
            }
        } else if !is_word_char(c) {
            break;
        }
        char_idx -= 1;
    }

    // Skip whitespace.
    while char_idx > 0 && rope.char(char_idx).is_whitespace() {
        char_idx -= 1;
    }

    char_to_position(rope, char_idx)
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn char_to_position(rope: &Rope, char_idx: usize) -> CursorPosition {
    let line = rope.char_to_line(char_idx);
    let line_start = rope.line_to_char(line);
    let grapheme = char_idx - line_start;
    CursorPosition::new(line, grapheme)
}
