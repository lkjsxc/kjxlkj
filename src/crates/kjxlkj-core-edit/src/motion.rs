//! Motion definitions and execution.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Cursor, CursorDirection};

/// Execute a cursor motion.
pub fn execute_motion(
    buffer: &TextBuffer,
    cursor: &mut Cursor,
    direction: CursorDirection,
    count: u32,
) {
    for _ in 0..count {
        execute_single_motion(buffer, cursor, direction);
    }
}

fn execute_single_motion(buffer: &TextBuffer, cursor: &mut Cursor, direction: CursorDirection) {
    let line_count = buffer.line_count();
    if line_count == 0 {
        return;
    }

    match direction {
        CursorDirection::Left => {
            if cursor.col() > 0 {
                cursor.move_horizontal(cursor.col() - 1);
            }
        }
        CursorDirection::Right => {
            let line_len = buffer.line_len(cursor.line() as usize);
            let max_col = line_len.saturating_sub(1) as u32;
            if cursor.col() < max_col {
                cursor.move_horizontal(cursor.col() + 1);
            }
        }
        CursorDirection::Up => {
            if cursor.line() > 0 {
                let new_line = cursor.line() - 1;
                let line_len = buffer.line_len(new_line as usize);
                let max_col = line_len.saturating_sub(1).max(0) as u32;
                cursor.move_vertical(new_line, max_col);
            }
        }
        CursorDirection::Down => {
            if (cursor.line() as usize) < line_count - 1 {
                let new_line = cursor.line() + 1;
                let line_len = buffer.line_len(new_line as usize);
                let max_col = line_len.saturating_sub(1).max(0) as u32;
                cursor.move_vertical(new_line, max_col);
            }
        }
        CursorDirection::WordForward => {
            if let Some(line) = buffer.line(cursor.line() as usize) {
                let new_col = kjxlkj_core_text::next_word_start(&line, cursor.col() as usize);
                if new_col < kjxlkj_core_text::grapheme_count(&line) {
                    cursor.move_horizontal(new_col as u32);
                } else if (cursor.line() as usize) < line_count - 1 {
                    cursor.move_to(cursor.line() + 1, 0);
                }
            }
        }
        CursorDirection::WordBackward => {
            if cursor.col() > 0 {
                if let Some(line) = buffer.line(cursor.line() as usize) {
                    let new_col = kjxlkj_core_text::prev_word_start(&line, cursor.col() as usize);
                    cursor.move_horizontal(new_col as u32);
                }
            } else if cursor.line() > 0 {
                let new_line = cursor.line() - 1;
                let line_len = buffer.line_len(new_line as usize);
                cursor.move_to(new_line, line_len.saturating_sub(1) as u32);
            }
        }
        CursorDirection::WordEndForward => {
            if let Some(line) = buffer.line(cursor.line() as usize) {
                let current = cursor.col() as usize;
                let end = kjxlkj_core_text::word_end(&line, current + 1);
                if end > current {
                    cursor.move_horizontal((end.saturating_sub(1)) as u32);
                } else if (cursor.line() as usize) < line_count - 1 {
                    cursor.move_to(cursor.line() + 1, 0);
                }
            }
        }
    }
}

/// Move cursor to line start.
pub fn move_to_line_start(cursor: &mut Cursor) {
    cursor.move_horizontal(0);
}

/// Move cursor to first non-blank character.
pub fn move_to_first_non_blank(buffer: &TextBuffer, cursor: &mut Cursor) {
    if let Some(line) = buffer.line(cursor.line() as usize) {
        let col = line
            .chars()
            .position(|c| !c.is_whitespace())
            .unwrap_or(0) as u32;
        cursor.move_horizontal(col);
    }
}

/// Move cursor to line end.
pub fn move_to_line_end(buffer: &TextBuffer, cursor: &mut Cursor) {
    let line_len = buffer.line_len(cursor.line() as usize);
    let col = line_len.saturating_sub(1) as u32;
    cursor.move_horizontal(col);
}

/// Move cursor to specific line.
pub fn move_to_line(buffer: &TextBuffer, cursor: &mut Cursor, line: u32) {
    let target = (line as usize).min(buffer.line_count().saturating_sub(1));
    let line_len = buffer.line_len(target);
    let max_col = line_len.saturating_sub(1).max(0) as u32;
    cursor.move_to(target as u32, cursor.desired_col.min(max_col));
}

/// Move cursor to first line.
pub fn move_to_file_start(cursor: &mut Cursor) {
    cursor.move_to(0, 0);
}

/// Move cursor to last line.
pub fn move_to_file_end(buffer: &TextBuffer, cursor: &mut Cursor) {
    let last_line = buffer.line_count().saturating_sub(1) as u32;
    move_to_line(buffer, cursor, last_line);
}

/// Clamp cursor to valid position within buffer.
/// Uses normal mode rules (cursor on last char, not past it).
pub fn clamp_cursor(buffer: &TextBuffer, cursor: &mut Cursor) {
    clamp_cursor_for_mode(buffer, cursor, kjxlkj_core_types::Mode::Normal);
}

/// Clamp cursor for a specific mode.
/// In insert mode, cursor can be past the last character.
pub fn clamp_cursor_for_mode(buffer: &TextBuffer, cursor: &mut Cursor, mode: kjxlkj_core_types::Mode) {
    let line_count = buffer.line_count();
    if line_count == 0 {
        cursor.move_to(0, 0);
        return;
    }

    let line = (cursor.line() as usize).min(line_count - 1) as u32;
    let line_len = buffer.line_len(line as usize);
    
    // In insert mode, cursor can be at position line_len (after last char)
    // In normal mode, cursor must be on a character (max line_len - 1)
    let max_col = if mode == kjxlkj_core_types::Mode::Insert || mode == kjxlkj_core_types::Mode::Replace {
        line_len as u32
    } else {
        line_len.saturating_sub(1).max(0) as u32
    };
    
    let col = cursor.col().min(max_col);
    cursor.position = kjxlkj_core_types::Position::new(line, col);
}
