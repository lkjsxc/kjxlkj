//! Motion implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{MotionKind, Position, Range};

/// Apply a motion and return the new cursor position.
pub fn apply_motion(
    buf: &TextBuffer,
    pos: Position,
    motion: MotionKind,
    count: usize,
) -> Position {
    let count = count.max(1);
    match motion {
        MotionKind::Left => move_left(buf, pos, count),
        MotionKind::Right => move_right(buf, pos, count),
        MotionKind::Up => move_up(buf, pos, count),
        MotionKind::Down => move_down(buf, pos, count),
        MotionKind::LineStart => line_start(pos),
        MotionKind::LineEnd => line_end(buf, pos),
        MotionKind::FirstNonBlank => first_non_blank(buf, pos),
        MotionKind::LastNonBlank => last_non_blank(buf, pos),
        MotionKind::FileStart => Position::new(0, 0),
        MotionKind::FileEnd => file_end(buf),
        MotionKind::GotoLine(n) => goto_line(buf, n),
        MotionKind::GotoColumn(n) => goto_column(buf, pos, n),
        MotionKind::WordForward => word_forward(buf, pos, count),
        MotionKind::WordBackward => word_backward(buf, pos, count),
        MotionKind::WordForwardEnd => word_end(buf, pos, count),
        MotionKind::NextParagraph => {
            crate::motion_extra::next_paragraph(buf, pos, count)
        }
        MotionKind::PrevParagraph => {
            crate::motion_extra::prev_paragraph(buf, pos, count)
        }
        MotionKind::MatchingBracket => {
            crate::motion_extra::matching_bracket(buf, pos)
        }
        MotionKind::ScreenTop => Position::new(0, 0),
        MotionKind::ScreenMiddle => Position::new(0, 0),
        MotionKind::ScreenBottom => Position::new(0, 0),
        MotionKind::NextNonBlankLine => {
            crate::motion_extra::next_non_blank_line(buf, pos, count)
        }
        MotionKind::PrevNonBlankLine => {
            crate::motion_extra::prev_non_blank_line(buf, pos, count)
        }
        MotionKind::MiddleOfLine => {
            crate::motion_extra::middle_of_line(buf, pos)
        }
        MotionKind::FindCharForward(c) => {
            crate::motion_extra::find_char_forward(buf, pos, c)
        }
        MotionKind::FindCharBackward(c) => {
            crate::motion_extra::find_char_backward(buf, pos, c)
        }
        MotionKind::TillCharForward(c) => {
            crate::motion_extra::till_char_forward(buf, pos, c)
        }
        MotionKind::TillCharBackward(c) => {
            crate::motion_extra::till_char_backward(buf, pos, c)
        }
        MotionKind::GotoPercent(pct) => {
            crate::motion_extra::goto_percent(buf, pct)
        }
        _ => pos,
    }
}

/// Compute the range that a motion covers from a position.
pub fn compute_motion_range(
    buf: &TextBuffer,
    pos: Position,
    motion: MotionKind,
    count: usize,
) -> Range {
    let target = apply_motion(buf, pos, motion, count);
    if target < pos {
        Range::new(target, pos)
    } else {
        Range::new(pos, Position::new(target.line, target.col + 1))
    }
}

fn move_left(_buf: &TextBuffer, pos: Position, count: usize) -> Position {
    Position::new(pos.line, pos.col.saturating_sub(count))
}

fn move_right(buf: &TextBuffer, pos: Position, count: usize) -> Position {
    let max = buf.line_len(pos.line).saturating_sub(1);
    Position::new(pos.line, (pos.col + count).min(max))
}

fn move_up(buf: &TextBuffer, pos: Position, count: usize) -> Position {
    let new_line = pos.line.saturating_sub(count);
    let max_col = buf.line_len(new_line).saturating_sub(1).max(0);
    Position::new(new_line, pos.col.min(max_col))
}

fn move_down(buf: &TextBuffer, pos: Position, count: usize) -> Position {
    let max_line = buf.line_count().saturating_sub(1);
    let new_line = (pos.line + count).min(max_line);
    let max_col = buf.line_len(new_line).saturating_sub(1).max(0);
    Position::new(new_line, pos.col.min(max_col))
}

fn line_start(pos: Position) -> Position {
    Position::new(pos.line, 0)
}

fn line_end(buf: &TextBuffer, pos: Position) -> Position {
    let len = buf.line_len(pos.line);
    Position::new(pos.line, len.saturating_sub(1).max(0))
}

pub(crate) fn first_non_blank(
    buf: &TextBuffer,
    pos: Position,
) -> Position {
    let line_str = buf.line_to_string(pos.line);
    let col = line_str
        .chars()
        .position(|c| !c.is_whitespace())
        .unwrap_or(0);
    Position::new(pos.line, col)
}

fn last_non_blank(buf: &TextBuffer, pos: Position) -> Position {
    let line_str = buf.line_to_string(pos.line);
    let chars: Vec<char> = line_str.chars().collect();
    let mut col = chars.len().saturating_sub(1);
    while col > 0 && chars[col].is_whitespace() {
        col -= 1;
    }
    Position::new(pos.line, col)
}

fn file_end(buf: &TextBuffer) -> Position {
    let last = buf.line_count().saturating_sub(1);
    first_non_blank(buf, Position::new(last, 0))
}

fn goto_line(buf: &TextBuffer, n: usize) -> Position {
    let line =
        n.saturating_sub(1).min(buf.line_count().saturating_sub(1));
    first_non_blank(buf, Position::new(line, 0))
}

fn goto_column(
    buf: &TextBuffer,
    pos: Position,
    n: usize,
) -> Position {
    let col = n
        .saturating_sub(1)
        .min(buf.line_len(pos.line).saturating_sub(1).max(0));
    Position::new(pos.line, col)
}

fn word_forward(
    buf: &TextBuffer,
    pos: Position,
    count: usize,
) -> Position {
    let mut p = pos;
    for _ in 0..count {
        p = kjxlkj_core_text::word_start_forward(buf, p);
    }
    p
}

fn word_backward(
    buf: &TextBuffer,
    pos: Position,
    count: usize,
) -> Position {
    let mut p = pos;
    for _ in 0..count {
        p = kjxlkj_core_text::word_start_backward(buf, p);
    }
    p
}

fn word_end(
    buf: &TextBuffer,
    pos: Position,
    count: usize,
) -> Position {
    let mut p = pos;
    for _ in 0..count {
        p = kjxlkj_core_text::word_end_forward(buf, p);
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_right() {
        let buf = TextBuffer::from_text("hello");
        assert_eq!(
            apply_motion(&buf, Position::new(0, 2), MotionKind::Left, 1),
            Position::new(0, 1),
        );
        assert_eq!(
            apply_motion(&buf, Position::new(0, 2), MotionKind::Right, 1),
            Position::new(0, 3),
        );
    }

    #[test]
    fn up_down() {
        let buf = TextBuffer::from_text("abc\ndef\nghi");
        assert_eq!(
            apply_motion(&buf, Position::new(1, 1), MotionKind::Up, 1),
            Position::new(0, 1),
        );
        assert_eq!(
            apply_motion(&buf, Position::new(1, 1), MotionKind::Down, 1),
            Position::new(2, 1),
        );
    }

    #[test]
    fn line_start_end() {
        let buf = TextBuffer::from_text("  hello  ");
        assert_eq!(
            apply_motion(&buf, Position::new(0, 4), MotionKind::LineStart, 1),
            Position::new(0, 0),
        );
        assert_eq!(
            apply_motion(&buf, Position::new(0, 0), MotionKind::FirstNonBlank, 1),
            Position::new(0, 2),
        );
    }

    #[test]
    fn file_start_end() {
        let buf = TextBuffer::from_text("abc\ndef\nghi");
        assert_eq!(
            apply_motion(&buf, Position::new(1, 0), MotionKind::FileStart, 1),
            Position::new(0, 0),
        );
        let end = apply_motion(&buf, Position::new(0, 0), MotionKind::FileEnd, 1);
        assert_eq!(end.line, 2);
    }

    #[test]
    fn matching_bracket_test() {
        let buf = TextBuffer::from_text("(hello (world))");
        let r = apply_motion(
            &buf,
            Position::new(0, 0),
            MotionKind::MatchingBracket,
            1,
        );
        assert_eq!(r, Position::new(0, 14));
    }

    #[test]
    fn find_char_test() {
        let buf = TextBuffer::from_text("hello world");
        let r = apply_motion(
            &buf,
            Position::new(0, 0),
            MotionKind::FindCharForward('o'),
            1,
        );
        assert_eq!(r, Position::new(0, 4));
    }
}
