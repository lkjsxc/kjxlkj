//! Editing operations.

use crate::motion::apply_motion;
use kjxlkj_core_edit::{Direction, Motion};
use kjxlkj_core_mode::InsertPosition;
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{CursorPosition, WindowContent};

/// Enter insert mode at position.
pub fn enter_insert(state: &mut EditorState, pos: InsertPosition) {
    if let Some(window) = state.windows.focused_mut() {
        if let WindowContent::Buffer(buffer_id) = &window.content {
            if let Some(buffer) = state.buffers.get(*buffer_id) {
                let line = window.cursor.line;
                let max_grapheme = buffer.line_grapheme_count(line);

                match pos {
                    InsertPosition::Before => {}
                    InsertPosition::After => {
                        window.cursor.grapheme = (window.cursor.grapheme + 1).min(max_grapheme);
                    }
                    InsertPosition::EndOfLine => {
                        window.cursor.grapheme = max_grapheme;
                    }
                    InsertPosition::FirstNonBlank => {
                        window.cursor.grapheme = 0;
                    }
                    InsertPosition::NewLineBelow => {
                        let cursor = CursorPosition::new(line, max_grapheme);
                        if let Some(buffer) = state.buffers.get_mut(*buffer_id) {
                            buffer.insert(cursor, "\n");
                        }
                        window.cursor.line += 1;
                        window.cursor.grapheme = 0;
                    }
                    InsertPosition::NewLineAbove => {
                        let cursor = CursorPosition::new(line, 0);
                        if let Some(buffer) = state.buffers.get_mut(*buffer_id) {
                            buffer.insert(cursor, "\n");
                        }
                        window.cursor.grapheme = 0;
                    }
                }
            }
        }
    }
}

/// Insert text at cursor.
pub fn insert_text(state: &mut EditorState, text: &str) {
    if let Some(window) = state.windows.focused_mut() {
        if let WindowContent::Buffer(buffer_id) = window.content {
            let cursor = window.cursor;
            if let Some(buffer) = state.buffers.get_mut(buffer_id) {
                buffer.insert(cursor, text);
                if text == "\n" {
                    window.cursor.line += 1;
                    window.cursor.grapheme = 0;
                } else {
                    window.cursor.grapheme += kjxlkj_core_text::grapheme_count(text);
                }
            }
        }
    }
}

/// Delete at cursor.
pub fn delete_at_cursor(state: &mut EditorState, direction: Direction) {
    if let Some(window) = state.windows.focused_mut() {
        if let WindowContent::Buffer(buffer_id) = window.content {
            let cursor = window.cursor;
            if let Some(buffer) = state.buffers.get_mut(buffer_id) {
                match direction {
                    Direction::Backward => {
                        if cursor.grapheme > 0 {
                            let start = CursorPosition::new(cursor.line, cursor.grapheme - 1);
                            buffer.delete(start, cursor);
                            window.cursor.grapheme -= 1;
                        } else if cursor.line > 0 {
                            let prev_line = cursor.line - 1;
                            let prev_len = buffer.line_grapheme_count(prev_line);
                            let end = CursorPosition::new(cursor.line, 0);
                            let start = CursorPosition::new(prev_line, prev_len);
                            buffer.delete(start, end);
                            window.cursor.line = prev_line;
                            window.cursor.grapheme = prev_len;
                        }
                    }
                    Direction::Forward => {
                        let line_len = buffer.line_grapheme_count(cursor.line);
                        if cursor.grapheme < line_len {
                            let end = CursorPosition::new(cursor.line, cursor.grapheme + 1);
                            buffer.delete(cursor, end);
                        }
                    }
                }
            }
        }
    }
}

/// Move cursor.
pub fn move_cursor(state: &mut EditorState, motion: Motion, count: usize) {
    if let Some(window) = state.windows.focused_mut() {
        if let WindowContent::Buffer(buffer_id) = window.content {
            if let Some(buffer) = state.buffers.get(buffer_id) {
                let mut cursor = window.cursor;
                for _ in 0..count {
                    cursor = apply_motion(buffer, cursor, &motion);
                }
                window.cursor = cursor;
            }
        }
    }
}

/// Clamp cursor to valid range.
pub fn clamp_cursor(state: &mut EditorState) {
    if let Some(window) = state.windows.focused_mut() {
        if let WindowContent::Buffer(buffer_id) = window.content {
            if let Some(buffer) = state.buffers.get(buffer_id) {
                let max_line = buffer.line_count().saturating_sub(1);
                window.cursor.line = window.cursor.line.min(max_line);

                let line_len = buffer.line_grapheme_count(window.cursor.line);
                let max_grapheme = line_len.saturating_sub(1);
                window.cursor.grapheme = window.cursor.grapheme.min(max_grapheme);
            }
        }
    }
}
