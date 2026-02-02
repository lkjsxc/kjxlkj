//! Motion execution for cursor movement.

use kjxlkj_core_edit::{Motion, MotionKind};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Position;

/// Executes a motion and returns the new position.
pub fn execute_motion(
    buffer: &TextBuffer,
    cursor: Position,
    motion: &Motion,
) -> Position {
    let mut pos = cursor;
    for _ in 0..motion.count {
        pos = execute_motion_once(buffer, pos, motion);
    }
    pos
}

/// Executes a motion once.
fn execute_motion_once(
    buffer: &TextBuffer,
    pos: Position,
    motion: &Motion,
) -> Position {
    match motion.kind {
        MotionKind::Left => move_left(buffer, pos),
        MotionKind::Right => move_right(buffer, pos),
        MotionKind::Up => move_up(buffer, pos),
        MotionKind::Down => move_down(buffer, pos),
        MotionKind::LineStart => line_start(pos),
        MotionKind::LineEnd => line_end(buffer, pos),
        MotionKind::FirstNonBlank => first_non_blank(buffer, pos),
        MotionKind::WordStart => word_start(buffer, pos),
        MotionKind::WordEnd => word_end(buffer, pos),
        MotionKind::WordBack => word_back(buffer, pos),
        MotionKind::BufferStart => Position::origin(),
        MotionKind::BufferEnd => buffer_end(buffer),
        MotionKind::GotoLine => goto_line(buffer, motion.count),
        _ => pos,
    }
}

fn move_left(_buffer: &TextBuffer, pos: Position) -> Position {
    if pos.col > 0 {
        Position::new(pos.line, pos.col - 1)
    } else {
        pos
    }
}

fn move_right(buffer: &TextBuffer, pos: Position) -> Position {
    let line_len = buffer.line_grapheme_count(pos.line);
    if pos.col + 1 < line_len {
        Position::new(pos.line, pos.col + 1)
    } else {
        pos
    }
}

fn move_up(_buffer: &TextBuffer, pos: Position) -> Position {
    if pos.line > 0 {
        Position::new(pos.line - 1, pos.col)
    } else {
        pos
    }
}

fn move_down(buffer: &TextBuffer, pos: Position) -> Position {
    if pos.line + 1 < buffer.line_count() {
        Position::new(pos.line + 1, pos.col)
    } else {
        pos
    }
}

fn line_start(pos: Position) -> Position {
    Position::new(pos.line, 0)
}

fn line_end(buffer: &TextBuffer, pos: Position) -> Position {
    let len = buffer.line_grapheme_count(pos.line);
    if len > 0 {
        Position::new(pos.line, len - 1)
    } else {
        Position::new(pos.line, 0)
    }
}

fn first_non_blank(buffer: &TextBuffer, pos: Position) -> Position {
    let line = buffer.line(pos.line);
    let col = line
        .chars()
        .position(|c| !c.is_whitespace())
        .unwrap_or(0);
    Position::new(pos.line, col)
}

fn word_start(buffer: &TextBuffer, pos: Position) -> Position {
    let line = buffer.line(pos.line);
    let chars: Vec<char> = line.chars().collect();
    
    if pos.col >= chars.len() {
        // Move to next line
        if pos.line + 1 < buffer.line_count() {
            return first_non_blank(buffer, Position::new(pos.line + 1, 0));
        }
        return pos;
    }

    // Skip current word
    let mut col = pos.col;
    while col < chars.len() && is_word_char(chars[col]) {
        col += 1;
    }
    // Skip whitespace
    while col < chars.len() && chars[col].is_whitespace() {
        col += 1;
    }

    if col >= chars.len() {
        if pos.line + 1 < buffer.line_count() {
            return first_non_blank(buffer, Position::new(pos.line + 1, 0));
        }
        return Position::new(pos.line, chars.len().saturating_sub(1));
    }

    Position::new(pos.line, col)
}

fn word_end(buffer: &TextBuffer, pos: Position) -> Position {
    let line = buffer.line(pos.line);
    let chars: Vec<char> = line.chars().collect();
    
    if pos.col >= chars.len().saturating_sub(1) {
        if pos.line + 1 < buffer.line_count() {
            let next_line = buffer.line(pos.line + 1);
            let next_chars: Vec<char> = next_line.chars().collect();
            let mut col = 0;
            while col < next_chars.len() && next_chars[col].is_whitespace() {
                col += 1;
            }
            while col < next_chars.len() && is_word_char(next_chars[col]) {
                col += 1;
            }
            return Position::new(pos.line + 1, col.saturating_sub(1));
        }
        return pos;
    }

    let mut col = pos.col + 1;
    // Skip whitespace first
    while col < chars.len() && chars[col].is_whitespace() {
        col += 1;
    }
    // Find end of word
    while col < chars.len() && is_word_char(chars[col]) {
        col += 1;
    }

    Position::new(pos.line, col.saturating_sub(1))
}

fn word_back(buffer: &TextBuffer, pos: Position) -> Position {
    if pos.col == 0 {
        if pos.line > 0 {
            let prev_len = buffer.line_grapheme_count(pos.line - 1);
            return word_back(buffer, Position::new(pos.line - 1, prev_len));
        }
        return pos;
    }

    let line = buffer.line(pos.line);
    let chars: Vec<char> = line.chars().collect();
    
    let mut col = pos.col.saturating_sub(1);
    // Skip whitespace
    while col > 0 && chars[col].is_whitespace() {
        col -= 1;
    }
    // Find start of word
    while col > 0 && is_word_char(chars[col - 1]) {
        col -= 1;
    }

    Position::new(pos.line, col)
}

fn buffer_end(buffer: &TextBuffer) -> Position {
    let last_line = buffer.line_count().saturating_sub(1);
    Position::new(last_line, 0)
}

fn goto_line(buffer: &TextBuffer, line: usize) -> Position {
    let target = line.saturating_sub(1).min(buffer.line_count().saturating_sub(1));
    first_non_blank(buffer, Position::new(target, 0))
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_left() {
        let buf = TextBuffer::from_str("hello");
        assert_eq!(move_left(&buf, Position::new(0, 3)), Position::new(0, 2));
        assert_eq!(move_left(&buf, Position::new(0, 0)), Position::new(0, 0));
    }

    #[test]
    fn test_move_right() {
        let buf = TextBuffer::from_str("hello");
        assert_eq!(move_right(&buf, Position::new(0, 0)), Position::new(0, 1));
    }

    #[test]
    fn test_move_up() {
        let buf = TextBuffer::from_str("line1\nline2");
        assert_eq!(move_up(&buf, Position::new(1, 0)), Position::new(0, 0));
        assert_eq!(move_up(&buf, Position::new(0, 0)), Position::new(0, 0));
    }

    #[test]
    fn test_move_down() {
        let buf = TextBuffer::from_str("line1\nline2");
        assert_eq!(move_down(&buf, Position::new(0, 0)), Position::new(1, 0));
        assert_eq!(move_down(&buf, Position::new(1, 0)), Position::new(1, 0));
    }

    #[test]
    fn test_line_start() {
        assert_eq!(line_start(Position::new(5, 10)), Position::new(5, 0));
    }

    #[test]
    fn test_first_non_blank() {
        let buf = TextBuffer::from_str("  hello");
        assert_eq!(first_non_blank(&buf, Position::new(0, 0)), Position::new(0, 2));
    }
}
