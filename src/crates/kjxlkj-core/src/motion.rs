//! Motion execution for cursor movement.

use kjxlkj_core_edit::{Motion, MotionKind};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Position;

use crate::motion_core::{
    buffer_end, first_non_blank, goto_line, is_word_char, line_end, line_start,
    move_down, move_left, move_right, move_up,
};

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

fn word_start(buffer: &TextBuffer, pos: Position) -> Position {
    let line = buffer.line(pos.line);
    let chars: Vec<char> = line.chars().collect();
    
    if pos.col >= chars.len() {
        if pos.line + 1 < buffer.line_count() {
            return first_non_blank(buffer, Position::new(pos.line + 1, 0));
        }
        return pos;
    }

    let mut col = pos.col;
    while col < chars.len() && is_word_char(chars[col]) {
        col += 1;
    }
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
    while col < chars.len() && chars[col].is_whitespace() {
        col += 1;
    }
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
    while col > 0 && chars[col].is_whitespace() {
        col -= 1;
    }
    while col > 0 && is_word_char(chars[col - 1]) {
        col -= 1;
    }

    Position::new(pos.line, col)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motion_core::{move_left, move_right, move_up, move_down, line_start, first_non_blank};

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
