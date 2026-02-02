//! Text editing operations for editor state.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Cursor, Position, Range};

/// Inserts text at cursor position.
pub fn insert_text(state: &mut EditorState, text: &str) {
    let Some(window) = state.windows.get_mut(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get_mut(&window.buffer_id) else {
        return;
    };

    let pos = window.cursor.position;
    buffer.text.insert(pos, text);
    buffer.modified = true;

    // Move cursor after inserted text
    if text == "\n" {
        window.cursor = Cursor::at(pos.line + 1, 0);
    } else {
        window.cursor = Cursor::at(pos.line, pos.col + text.chars().count());
    }
}

/// Deletes character before cursor.
pub fn backspace(state: &mut EditorState) {
    let Some(window) = state.windows.get_mut(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get_mut(&window.buffer_id) else {
        return;
    };

    let pos = window.cursor.position;
    if pos.col > 0 {
        let start = Position::new(pos.line, pos.col - 1);
        buffer.text.delete(Range::new(start, pos));
        buffer.modified = true;
        window.cursor = Cursor::new(start);
    } else if pos.line > 0 {
        let prev_len = buffer.text.line(pos.line - 1).chars().count();
        let start = Position::new(pos.line - 1, prev_len);
        buffer.text.delete(Range::new(start, pos));
        buffer.modified = true;
        window.cursor = Cursor::new(start);
    }
}

/// Deletes character at cursor.
pub fn delete_char(state: &mut EditorState) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get_mut(&window.buffer_id) else {
        return;
    };

    let pos = window.cursor.position;
    let line_content = buffer.text.line(pos.line);
    if pos.col < line_content.chars().count() {
        let end = Position::new(pos.line, pos.col + 1);
        buffer.text.delete(Range::new(pos, end));
        buffer.modified = true;
    }
}

/// Undo last change.
pub fn undo(state: &mut EditorState) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get_mut(&window.buffer_id) else {
        return;
    };
    buffer.undo.undo();
}

/// Redo last undone change.
pub fn redo(state: &mut EditorState) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get_mut(&window.buffer_id) else {
        return;
    };
    buffer.undo.redo();
}

/// Executes a motion on the current state.
pub fn execute_motion_on_state(
    state: &mut EditorState,
    motion: &kjxlkj_core_edit::Motion,
    count: usize,
) {
    let Some(window) = state.windows.get_mut(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get(&window.buffer_id) else {
        return;
    };

    for _ in 0..count {
        let new_pos = crate::execute_motion(&buffer.text, window.cursor.position, motion);
        window.cursor = Cursor::new(new_pos);
    }
}
