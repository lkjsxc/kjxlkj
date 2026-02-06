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
        MotionKind::WordBackwardEnd => {
            word_end_backward(buf, pos, count)
        }
        MotionKind::WORDForward => {
            big_word_forward(buf, pos, count)
        }
        MotionKind::WORDBackward => {
            big_word_backward(buf, pos, count)
        }
        MotionKind::WORDForwardEnd => {
            big_word_end(buf, pos, count)
        }
        MotionKind::WORDBackwardEnd => {
            big_word_end_backward(buf, pos, count)
        }
        MotionKind::NextSentence => sentence_forward(buf, pos, count),
        MotionKind::PrevSentence => sentence_backward(buf, pos, count),
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

fn goto_column(buf: &TextBuffer, pos: Position, n: usize) -> Position {
    let col = n.saturating_sub(1).min(buf.line_len(pos.line).saturating_sub(1).max(0));
    Position::new(pos.line, col)
}

fn repeat_motion(buf: &TextBuffer, pos: Position, count: usize, f: fn(&TextBuffer, Position) -> Position) -> Position {
    let mut p = pos;
    for _ in 0..count { p = f(buf, p); }
    p
}

fn word_forward(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::word_start_forward) }
fn word_backward(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::word_start_backward) }
fn word_end(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::word_end_forward) }
fn word_end_backward(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::word_end_backward) }
fn big_word_forward(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::big_word_start_forward) }
fn big_word_backward(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::big_word_start_backward) }
fn big_word_end(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::big_word_end_forward) }
fn big_word_end_backward(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::big_word_end_backward) }
fn sentence_forward(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::next_sentence) }
fn sentence_backward(b: &TextBuffer, p: Position, c: usize) -> Position { repeat_motion(b, p, c, kjxlkj_core_text::prev_sentence) }
