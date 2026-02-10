//! Motion application.

use kjxlkj_core_edit::Motion;
use kjxlkj_core_state::Buffer;
use kjxlkj_core_types::CursorPosition;

/// Apply a motion to a cursor position.
pub fn apply_motion(buffer: &Buffer, cursor: CursorPosition, motion: &Motion) -> CursorPosition {
    let line_count = buffer.line_count();
    let line_len = buffer.line_grapheme_count(cursor.line);

    match motion {
        Motion::Left => {
            if cursor.grapheme > 0 {
                CursorPosition::new(cursor.line, cursor.grapheme - 1)
            } else {
                cursor
            }
        }
        Motion::Right => {
            let max = if line_len > 0 { line_len - 1 } else { 0 };
            if cursor.grapheme < max {
                CursorPosition::new(cursor.line, cursor.grapheme + 1)
            } else {
                cursor
            }
        }
        Motion::Up => {
            if cursor.line > 0 {
                let new_line = cursor.line - 1;
                let new_len = buffer.line_grapheme_count(new_line);
                let new_grapheme = cursor.grapheme.min(new_len.saturating_sub(1));
                CursorPosition::new(new_line, new_grapheme)
            } else {
                cursor
            }
        }
        Motion::Down => {
            if cursor.line + 1 < line_count {
                let new_line = cursor.line + 1;
                let new_len = buffer.line_grapheme_count(new_line);
                let new_grapheme = cursor.grapheme.min(new_len.saturating_sub(1));
                CursorPosition::new(new_line, new_grapheme)
            } else {
                cursor
            }
        }
        Motion::LineStart => CursorPosition::new(cursor.line, 0),
        Motion::LineEnd => {
            let max = if line_len > 0 { line_len - 1 } else { 0 };
            CursorPosition::new(cursor.line, max)
        }
        Motion::DocumentStart => CursorPosition::new(0, 0),
        Motion::DocumentEnd => {
            let last_line = line_count.saturating_sub(1);
            CursorPosition::new(last_line, 0)
        }
        _ => cursor,
    }
}
