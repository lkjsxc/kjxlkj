//! Motion implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Position;

/// Move cursor left.
pub fn move_left(_buf: &TextBuffer, pos: Position, count: usize) -> Position {
    let new_col = pos.col.saturating_sub(count);
    Position::new(pos.line, new_col)
}

/// Move cursor right.
pub fn move_right(buf: &TextBuffer, pos: Position, count: usize) -> Position {
    let line_len = buf.line_len(pos.line);
    let max_col = if line_len > 0 { line_len - 1 } else { 0 };
    let new_col = (pos.col + count).min(max_col);
    Position::new(pos.line, new_col)
}

/// Move cursor up.
pub fn move_up(buf: &TextBuffer, pos: Position, count: usize) -> Position {
    let new_line = pos.line.saturating_sub(count);
    let line_len = buf.line_len(new_line);
    let new_col = pos.col.min(if line_len > 0 { line_len - 1 } else { 0 });
    Position::new(new_line, new_col)
}

/// Move cursor down.
pub fn move_down(buf: &TextBuffer, pos: Position, count: usize) -> Position {
    let max_line = buf.line_count().saturating_sub(1);
    let new_line = (pos.line + count).min(max_line);
    let line_len = buf.line_len(new_line);
    let new_col = pos.col.min(if line_len > 0 { line_len - 1 } else { 0 });
    Position::new(new_line, new_col)
}

/// Move to start of line.
pub fn move_line_start(_buf: &TextBuffer, pos: Position) -> Position {
    Position::new(pos.line, 0)
}

/// Move to end of line.
pub fn move_line_end(buf: &TextBuffer, pos: Position) -> Position {
    let line_len = buf.line_len(pos.line);
    let col = if line_len > 0 { line_len - 1 } else { 0 };
    Position::new(pos.line, col)
}

/// Move to first non-blank character.
pub fn move_first_non_blank(buf: &TextBuffer, pos: Position) -> Position {
    if let Some(line) = buf.line(pos.line) {
        let col = line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
        Position::new(pos.line, col)
    } else {
        Position::new(pos.line, 0)
    }
}

/// Move to next word start.
pub fn move_word_start(buf: &TextBuffer, pos: Position) -> Position {
    let line = match buf.line(pos.line) {
        Some(l) => l,
        None => return pos,
    };
    let chars: Vec<char> = line.chars().collect();
    let mut col = pos.col;

    // Skip current word
    while col < chars.len() && is_word_char(chars[col]) {
        col += 1;
    }
    // Skip non-word chars
    while col < chars.len() && !is_word_char(chars[col]) && !chars[col].is_whitespace() {
        col += 1;
    }
    // Skip whitespace
    while col < chars.len() && chars[col].is_whitespace() {
        col += 1;
    }

    if col >= chars.len() && pos.line + 1 < buf.line_count() {
        return move_first_non_blank(buf, Position::new(pos.line + 1, 0));
    }

    Position::new(pos.line, col.min(chars.len().saturating_sub(1)))
}

/// Move to previous word start.
pub fn move_word_back(buf: &TextBuffer, pos: Position) -> Position {
    let line = match buf.line(pos.line) {
        Some(l) => l,
        None => return pos,
    };
    let chars: Vec<char> = line.chars().collect();
    let mut col = pos.col;

    if col == 0 && pos.line > 0 {
        let prev_line = pos.line - 1;
        let len = buf.line_len(prev_line);
        return Position::new(prev_line, len.saturating_sub(1));
    }

    col = col.saturating_sub(1);
    // Skip whitespace
    while col > 0 && chars.get(col).map(|c| c.is_whitespace()).unwrap_or(false) {
        col -= 1;
    }
    // Skip to start of word
    while col > 0
        && chars
            .get(col - 1)
            .map(|c| is_word_char(*c))
            .unwrap_or(false)
    {
        col -= 1;
    }

    Position::new(pos.line, col)
}

/// Move to word end.
pub fn move_word_end(buf: &TextBuffer, pos: Position) -> Position {
    let line = match buf.line(pos.line) {
        Some(l) => l,
        None => return pos,
    };
    let chars: Vec<char> = line.chars().collect();
    let mut col = pos.col + 1;

    // Skip whitespace
    while col < chars.len() && chars[col].is_whitespace() {
        col += 1;
    }
    // Move to end of word
    while col < chars.len() && is_word_char(chars[col]) {
        col += 1;
    }

    let result_col = col.saturating_sub(1).min(chars.len().saturating_sub(1));
    Position::new(pos.line, result_col)
}

/// Move to file start.
pub fn move_file_start(_buf: &TextBuffer) -> Position {
    Position::zero()
}

/// Move to file end.
pub fn move_file_end(buf: &TextBuffer) -> Position {
    let last_line = buf.line_count().saturating_sub(1);
    Position::new(last_line, 0)
}

/// Move to specific line.
pub fn move_to_line(buf: &TextBuffer, line: usize) -> Position {
    let line = line.min(buf.line_count().saturating_sub(1));
    move_first_non_blank(buf, Position::new(line, 0))
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_left_right() {
        let buf = TextBuffer::from_text("hello");
        let pos = Position::new(0, 2);
        assert_eq!(move_left(&buf, pos, 1), Position::new(0, 1));
        assert_eq!(move_right(&buf, pos, 1), Position::new(0, 3));
    }

    #[test]
    fn test_move_up_down() {
        let buf = TextBuffer::from_text("abc\ndef\nghi");
        let pos = Position::new(1, 1);
        assert_eq!(move_up(&buf, pos, 1), Position::new(0, 1));
        assert_eq!(move_down(&buf, pos, 1), Position::new(2, 1));
    }

    #[test]
    fn test_move_line_boundaries() {
        let buf = TextBuffer::from_text("  hello world");
        let pos = Position::new(0, 5);
        assert_eq!(move_line_start(&buf, pos), Position::new(0, 0));
        assert_eq!(move_first_non_blank(&buf, pos), Position::new(0, 2));
    }
}
