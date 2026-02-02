//! Register operations for editor state.

use kjxlkj_core_state::{EditorState, RegisterContent};
use kjxlkj_core_types::{Cursor, Position};

/// Put (paste) after cursor.
pub fn put_after(state: &mut EditorState, register: Option<char>) {
    let content = get_register_content(state, register);
    let Some(content) = content else { return };

    let Some(window) = state.windows.get_mut(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get_mut(&window.buffer_id) else {
        return;
    };

    let pos = window.cursor.position;
    if content.is_linewise() {
        // Paste on next line
        let next_line = pos.line + 1;
        let insert_pos = Position::new(next_line, 0);

        // Ensure text ends with newline
        let text = if content.text.ends_with('\n') {
            content.text.clone()
        } else {
            format!("{}\n", content.text)
        };

        buffer.text.insert(insert_pos, &text);
        buffer.modified = true;
        window.cursor = Cursor::at(next_line, 0);
    } else {
        // Paste after cursor
        let insert_pos = Position::new(pos.line, pos.col + 1);
        buffer.text.insert(insert_pos, &content.text);
        buffer.modified = true;
        window.cursor = Cursor::at(pos.line, pos.col + content.text.chars().count());
    }
}

/// Put (paste) before cursor.
pub fn put_before(state: &mut EditorState, register: Option<char>) {
    let content = get_register_content(state, register);
    let Some(content) = content else { return };

    let Some(window) = state.windows.get_mut(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get_mut(&window.buffer_id) else {
        return;
    };

    let pos = window.cursor.position;
    if content.is_linewise() {
        // Paste on current line (push existing down)
        let insert_pos = Position::new(pos.line, 0);

        // Ensure text ends with newline
        let text = if content.text.ends_with('\n') {
            content.text.clone()
        } else {
            format!("{}\n", content.text)
        };

        buffer.text.insert(insert_pos, &text);
        buffer.modified = true;
        window.cursor = Cursor::at(pos.line, 0);
    } else {
        // Paste at cursor
        buffer.text.insert(pos, &content.text);
        buffer.modified = true;
    }
}

/// Yank current line.
pub fn yank_line(state: &mut EditorState) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get(&window.buffer_id) else {
        return;
    };

    let line = window.cursor.position.line;
    let line_content = buffer.text.line(line);

    state.registers.yank(RegisterContent::line(line_content));
}

/// Gets register content, defaulting to unnamed register.
fn get_register_content(state: &EditorState, register: Option<char>) -> Option<RegisterContent> {
    let reg = register.unwrap_or('"');
    state.registers.get(reg).cloned()
}
