//! Intent helper functions.

use kjxlkj_core::{EditorState, Mode, Position, SelectionKind};
use kjxlkj_services::fs::FsService;
use std::path::PathBuf;

pub fn apply_enter_insert(state: &mut EditorState, at_line_end: bool, after_cursor: bool) {
    if at_line_end {
        let line_len = state.buffer.line_len(state.cursor.line());
        state.cursor.position.col = line_len;
    } else if after_cursor {
        let line_len = state.buffer.line_len(state.cursor.line());
        if state.cursor.col() < line_len {
            state.cursor.position.col += 1;
        }
    }
    state.set_mode(Mode::Insert);
}

pub fn apply_start_visual(state: &mut EditorState, kind: SelectionKind) {
    let mode = match kind {
        SelectionKind::Char => Mode::Visual,
        SelectionKind::Line => Mode::VisualLine,
        SelectionKind::Block => Mode::VisualBlock,
    };
    state.set_mode(mode);
}

pub fn apply_open_line_below(state: &mut EditorState) {
    let line = state.cursor.line();
    let line_len = state.buffer.line_len(line);
    state.buffer.insert(Position::new(line, line_len), "\n");
    state.cursor.position = Position::new(line + 1, 0);
    state.set_mode(Mode::Insert);
}

pub fn apply_open_line_above(state: &mut EditorState) {
    let line = state.cursor.line();
    state.buffer.insert(Position::new(line, 0), "\n");
    state.cursor.position = Position::new(line, 0);
    state.set_mode(Mode::Insert);
}

pub fn apply_delete_char(state: &mut EditorState) {
    let pos = state.cursor.position;
    let line_len = state.buffer.line_len(pos.line);
    if pos.col < line_len {
        let next = Position::new(pos.line, pos.col + 1);
        state.buffer.delete_range(pos, next);
    }
}

pub fn apply_delete_char_before(state: &mut EditorState) {
    if state.cursor.col() > 0 {
        let pos = state.cursor.position;
        let prev = Position::new(pos.line, pos.col - 1);
        state.buffer.delete_range(prev, pos);
        state.cursor.position = prev;
    }
}

pub fn apply_undo(state: &mut EditorState) {
    if let Some(tx) = state.undo.undo() {
        state.cursor.position = tx.cursor_before;
    }
}

pub fn apply_redo(state: &mut EditorState) {
    if let Some(tx) = state.undo.redo() {
        state.cursor.position = tx.cursor_after;
    }
}

pub fn apply_paste(state: &mut EditorState, before: bool, cursor_at_end: bool) {
    let Some(reg) = state.registers.unnamed() else {
        return;
    };
    let content = reg.content.clone();
    let linewise = reg.linewise;

    if linewise {
        paste_linewise(state, &content, before);
    } else {
        paste_charwise(state, &content, before, cursor_at_end);
    }
}

fn paste_linewise(state: &mut EditorState, content: &str, before: bool) {
    let line = if before {
        state.cursor.line()
    } else {
        state.cursor.line() + 1
    };
    let pos = Position::new(line, 0);
    state.buffer.insert(pos, &format!("{}\n", content.trim_end()));
    state.cursor.position = pos;
}

fn paste_charwise(state: &mut EditorState, content: &str, before: bool, cursor_at_end: bool) {
    let col = if before {
        state.cursor.col()
    } else {
        state.cursor.col() + 1
    };
    let pos = Position::new(state.cursor.line(), col);
    state.buffer.insert(pos, content);
    if cursor_at_end {
        state.cursor.position.col = col + content.len();
    } else {
        state.cursor.position.col = col;
    }
}

pub fn apply_join_lines(state: &mut EditorState, with_space: bool) {
    let line = state.cursor.line();
    if line + 1 >= state.buffer.line_count() {
        return;
    }
    let line_len = state.buffer.line_len(line);
    let next_start = Position::new(line + 1, 0);
    let line_end = Position::new(line, line_len);
    state.buffer.delete_range(line_end, next_start);
    if with_space {
        state.buffer.insert(line_end, " ");
    }
    state.cursor.position = line_end;
}

pub fn apply_replace_char(state: &mut EditorState, c: char) {
    let pos = state.cursor.position;
    let line_len = state.buffer.line_len(pos.line);
    if pos.col < line_len {
        let next = Position::new(pos.line, pos.col + 1);
        state.buffer.delete_range(pos, next);
        state.buffer.insert(pos, &c.to_string());
    }
}

pub fn apply_toggle_case(state: &mut EditorState) {
    let pos = state.cursor.position;
    let Some(line) = state.buffer.line(pos.line) else {
        return;
    };
    let chars: Vec<char> = line.chars().collect();
    if pos.col >= chars.len() {
        return;
    }
    let c = chars[pos.col];
    let toggled = if c.is_uppercase() {
        c.to_lowercase().to_string()
    } else {
        c.to_uppercase().to_string()
    };
    let next = Position::new(pos.line, pos.col + 1);
    state.buffer.delete_range(pos, next);
    state.buffer.insert(pos, &toggled);
    state.cursor.position.col += 1;
}

pub fn apply_search_forward(state: &mut EditorState) {
    state.mode_state.search_forward = true;
    state.mode_state.command_line.clear();
    state.set_mode(Mode::Command);
    state.mode_state.command_line.push('/');
}

pub fn apply_search_backward(state: &mut EditorState) {
    state.mode_state.search_forward = false;
    state.mode_state.command_line.clear();
    state.set_mode(Mode::Command);
    state.mode_state.command_line.push('?');
}

pub fn apply_write(state: &mut EditorState, path: Option<String>) {
    if let Some(ref p) = path {
        state.buffer.set_path(PathBuf::from(p));
    }
    if let Some(p) = state.buffer.path() {
        let content = state.buffer.text();
        if FsService::write_file(p, &content).is_ok() {
            state.buffer.mark_saved();
            state.set_status("Written");
        } else {
            state.set_status("Write failed!");
        }
    } else {
        state.set_status("No file name");
    }
}
