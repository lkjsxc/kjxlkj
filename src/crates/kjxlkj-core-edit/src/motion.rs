//! Motion application.

use kjxlkj_core_text::grapheme::line_graphemes;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::MotionAction;

use crate::cursor::CursorPosition;
use crate::word_motion::{move_word_backward, move_word_forward};

/// Apply a motion to a cursor position.
pub fn apply_motion(
    motion: &MotionAction,
    cursor: &mut CursorPosition,
    buffer: &TextBuffer,
    count: usize,
) {
    let count = if count == 0 { 1 } else { count };
    match motion {
        MotionAction::Left => {
            cursor.grapheme_offset = cursor.grapheme_offset.saturating_sub(count);
        }
        MotionAction::Right => {
            let line = buffer.line(cursor.line).unwrap_or_default();
            let g = line_graphemes(&line).len();
            let max = if g == 0 { 0 } else { g - 1 };
            cursor.grapheme_offset = (cursor.grapheme_offset + count).min(max);
        }
        MotionAction::Up => {
            cursor.line = cursor.line.saturating_sub(count);
            clamp_to_line(cursor, buffer);
        }
        MotionAction::Down => {
            let max_line = buffer.line_count().saturating_sub(1);
            cursor.line = (cursor.line + count).min(max_line);
            clamp_to_line(cursor, buffer);
        }
        MotionAction::LineStart => {
            cursor.grapheme_offset = 0;
        }
        MotionAction::LineEnd => {
            let line = buffer.line(cursor.line).unwrap_or_default();
            let g = line_graphemes(&line).len();
            cursor.grapheme_offset = if g == 0 { 0 } else { g - 1 };
        }
        MotionAction::FirstNonBlank => {
            let line = buffer.line(cursor.line).unwrap_or_default();
            let graphemes = line_graphemes(&line);
            let offset = graphemes
                .iter()
                .position(|g: &&str| !g.chars().all(|c| c == ' ' || c == '\t'))
                .unwrap_or(0);
            cursor.grapheme_offset = offset;
        }
        MotionAction::WordForward => {
            for _ in 0..count {
                move_word_forward(cursor, buffer);
            }
        }
        MotionAction::WordBackward => {
            for _ in 0..count {
                move_word_backward(cursor, buffer);
            }
        }
        MotionAction::GoToFirstLine => {
            cursor.line = 0;
            cursor.grapheme_offset = 0;
        }
        MotionAction::GoToLastLine => {
            cursor.line = buffer.line_count().saturating_sub(1);
            cursor.grapheme_offset = 0;
        }
        MotionAction::GoToLine(n) => {
            let target = (*n).saturating_sub(1);
            cursor.line = target.min(buffer.line_count().saturating_sub(1));
            cursor.grapheme_offset = 0;
        }
        MotionAction::PageDown => {
            let page = 20;
            let max_line = buffer.line_count().saturating_sub(1);
            cursor.line = (cursor.line + page * count).min(max_line);
            clamp_to_line(cursor, buffer);
        }
        MotionAction::PageUp => {
            let page = 20;
            cursor.line = cursor.line.saturating_sub(page * count);
            clamp_to_line(cursor, buffer);
        }
        MotionAction::HalfPageDown => {
            let half = 10;
            let max_line = buffer.line_count().saturating_sub(1);
            cursor.line = (cursor.line + half * count).min(max_line);
            clamp_to_line(cursor, buffer);
        }
        MotionAction::HalfPageUp => {
            let half = 10;
            cursor.line = cursor.line.saturating_sub(half * count);
            clamp_to_line(cursor, buffer);
        }
        _ => {
            // Other motions: no-op for now
        }
    }
}

fn clamp_to_line(cursor: &mut CursorPosition, buffer: &TextBuffer) {
    let line = buffer.line(cursor.line).unwrap_or_default();
    let g = line_graphemes(&line).len();
    let max = if g == 0 { 0 } else { g - 1 };
    if cursor.grapheme_offset > max {
        cursor.grapheme_offset = max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_text::TextBuffer;
    use kjxlkj_core_types::BufferId;

    fn make_buf(text: &str) -> TextBuffer {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, text);
        buf
    }

    #[test]
    fn test_motion_left_right() {
        let buf = make_buf("hello\n");
        let mut cur = CursorPosition::new(0, 2);
        apply_motion(&MotionAction::Left, &mut cur, &buf, 1);
        assert_eq!(cur.grapheme_offset, 1);
        apply_motion(&MotionAction::Right, &mut cur, &buf, 1);
        assert_eq!(cur.grapheme_offset, 2);
    }

    #[test]
    fn test_motion_up_down() {
        let buf = make_buf("hello\nworld\n");
        let mut cur = CursorPosition::new(0, 0);
        apply_motion(&MotionAction::Down, &mut cur, &buf, 1);
        assert_eq!(cur.line, 1);
        apply_motion(&MotionAction::Up, &mut cur, &buf, 1);
        assert_eq!(cur.line, 0);
    }
}
