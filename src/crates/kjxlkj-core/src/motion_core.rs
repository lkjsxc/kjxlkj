//! Motion helper functions for cursor movement.
//!
//! Low-level helpers for executing individual motion operations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Position;

/// Moves cursor left.
pub fn move_left(_buffer: &TextBuffer, pos: Position) -> Position {
    if pos.col > 0 {
        Position::new(pos.line, pos.col - 1)
    } else {
        pos
    }
}

/// Moves cursor right.
pub fn move_right(buffer: &TextBuffer, pos: Position) -> Position {
    let line_len = buffer.line_grapheme_count(pos.line);
    if pos.col + 1 < line_len {
        Position::new(pos.line, pos.col + 1)
    } else {
        pos
    }
}

/// Moves cursor up.
pub fn move_up(_buffer: &TextBuffer, pos: Position) -> Position {
    if pos.line > 0 {
        Position::new(pos.line - 1, pos.col)
    } else {
        pos
    }
}

/// Moves cursor down.
pub fn move_down(buffer: &TextBuffer, pos: Position) -> Position {
    if pos.line + 1 < buffer.line_count() {
        Position::new(pos.line + 1, pos.col)
    } else {
        pos
    }
}

/// Goes to line start.
pub fn line_start(pos: Position) -> Position {
    Position::new(pos.line, 0)
}

/// Goes to line end.
pub fn line_end(buffer: &TextBuffer, pos: Position) -> Position {
    let len = buffer.line_grapheme_count(pos.line);
    if len > 0 {
        Position::new(pos.line, len - 1)
    } else {
        Position::new(pos.line, 0)
    }
}

/// Goes to first non-blank character.
pub fn first_non_blank(buffer: &TextBuffer, pos: Position) -> Position {
    let line = buffer.line(pos.line);
    let col = line
        .chars()
        .position(|c| !c.is_whitespace())
        .unwrap_or(0);
    Position::new(pos.line, col)
}

/// Goes to buffer end.
pub fn buffer_end(buffer: &TextBuffer) -> Position {
    let last_line = buffer.line_count().saturating_sub(1);
    Position::new(last_line, 0)
}

/// Goes to specific line.
pub fn goto_line(buffer: &TextBuffer, line: usize) -> Position {
    let target = line.saturating_sub(1).min(buffer.line_count().saturating_sub(1));
    first_non_blank(buffer, Position::new(target, 0))
}

/// Checks if character is a word character.
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
