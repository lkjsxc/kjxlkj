//! Normal mode handler.

use super::Action;
use kjxlkj_core::edit::*;
use kjxlkj_core::types::{Mode, Position};
use kjxlkj_core::EditorState;
use kjxlkj_input::{Key, KeyCode};
use kjxlkj_services::Services;

/// Handle normal mode keys.
pub fn handle_normal(state: &mut EditorState, key: Key, services: &Services) -> Action {
    // Handle count prefix
    if let KeyCode::Char(c) = key.code {
        if c.is_ascii_digit() && (c != '0' || state.pending_count.is_some()) {
            let digit = c.to_digit(10).unwrap() as usize;
            let count = state.pending_count.unwrap_or(0) * 10 + digit;
            state.pending_count = Some(count);
            return Action::Continue;
        }
    }

    let count = state.count();
    let result = handle_normal_key(state, key, count, services);
    state.clear_pending();
    result
}

fn handle_normal_key(
    state: &mut EditorState,
    key: Key,
    count: usize,
    _services: &Services,
) -> Action {
    match (key.code, key.mods.ctrl) {
        // Movement
        (KeyCode::Char('h') | KeyCode::Left | KeyCode::Backspace, false) => {
            state.cursor.pos = move_left(&state.buffer, state.cursor.pos, count);
        }
        (KeyCode::Char('l') | KeyCode::Right | KeyCode::Char(' '), false) => {
            state.cursor.pos = move_right(&state.buffer, state.cursor.pos, count);
        }
        (KeyCode::Char('j') | KeyCode::Down, false) => {
            state.cursor.pos = move_down(&state.buffer, state.cursor.pos, count);
        }
        (KeyCode::Char('k') | KeyCode::Up, false) => {
            state.cursor.pos = move_up(&state.buffer, state.cursor.pos, count);
        }
        (KeyCode::Char('0'), false) => {
            state.cursor.pos = move_line_start(&state.buffer, state.cursor.pos);
        }
        (KeyCode::Char('^'), false) => {
            state.cursor.pos = move_first_non_blank(&state.buffer, state.cursor.pos);
        }
        (KeyCode::Char('$'), false) => {
            state.cursor.pos = move_line_end(&state.buffer, state.cursor.pos);
        }
        (KeyCode::Char('w'), false) => {
            for _ in 0..count {
                state.cursor.pos = move_word_start(&state.buffer, state.cursor.pos);
            }
        }
        (KeyCode::Char('b'), false) => {
            for _ in 0..count {
                state.cursor.pos = move_word_back(&state.buffer, state.cursor.pos);
            }
        }
        (KeyCode::Char('e'), false) => {
            for _ in 0..count {
                state.cursor.pos = move_word_end(&state.buffer, state.cursor.pos);
            }
        }
        (KeyCode::Char('g'), false) => {
            state.cursor.pos = move_file_start(&state.buffer);
        }
        (KeyCode::Char('G'), false) => {
            if count > 1 {
                state.cursor.pos = move_to_line(&state.buffer, count - 1);
            } else {
                state.cursor.pos = move_file_end(&state.buffer);
            }
        }

        // Mode transitions
        (KeyCode::Char('i'), false) => {
            state.mode.transition(Mode::Insert);
        }
        (KeyCode::Char('I'), false) => {
            state.cursor.pos = move_first_non_blank(&state.buffer, state.cursor.pos);
            state.mode.transition(Mode::Insert);
        }
        (KeyCode::Char('a'), false) => {
            let line_len = state.buffer.line_len(state.cursor.pos.line);
            if line_len > 0 {
                state.cursor.pos.col = (state.cursor.pos.col + 1).min(line_len);
            }
            state.mode.transition(Mode::Insert);
        }
        (KeyCode::Char('A'), false) => {
            let line_len = state.buffer.line_len(state.cursor.pos.line);
            state.cursor.pos.col = line_len;
            state.mode.transition(Mode::Insert);
        }
        (KeyCode::Char('o'), false) => {
            let line = state.cursor.pos.line;
            let line_len = state.buffer.line_len(line);
            let pos = Position::new(line, line_len);
            insert_text(
                &mut state.buffer,
                pos,
                "\n",
                &mut state.history,
                state.cursor.pos,
            );
            state.cursor.pos = Position::new(line + 1, 0);
            state.mode.transition(Mode::Insert);
            state.modified = true;
        }
        (KeyCode::Char('O'), false) => {
            let pos = Position::new(state.cursor.pos.line, 0);
            insert_text(
                &mut state.buffer,
                pos,
                "\n",
                &mut state.history,
                state.cursor.pos,
            );
            state.cursor.pos = pos;
            state.mode.transition(Mode::Insert);
            state.modified = true;
        }
        (KeyCode::Char('v'), false) => {
            state.cursor.start_selection();
            state.mode.transition(Mode::Visual);
        }
        (KeyCode::Char('V'), false) => {
            state.cursor.start_selection();
            state.mode.transition(Mode::VisualLine);
        }
        (KeyCode::Char('v'), true) => {
            state.cursor.start_selection();
            state.mode.transition(Mode::VisualBlock);
        }
        (KeyCode::Char('R'), false) => {
            state.mode.transition(Mode::Replace);
        }
        (KeyCode::Char(':'), false) => {
            state.mode.transition(Mode::Command);
        }

        // Editing
        (KeyCode::Char('x'), false) => {
            delete_char_under_cursor(state);
        }
        (KeyCode::Char('X'), false) => {
            delete_char_before_cursor(state);
        }
        (KeyCode::Char('r'), false) => {
            // Would need next char - simplified
        }
        (KeyCode::Char('D'), false) => {
            delete_to_end_of_line(state);
        }
        (KeyCode::Char('C'), false) => {
            delete_to_end_of_line(state);
            state.mode.transition(Mode::Insert);
        }
        (KeyCode::Char('s'), false) => {
            delete_char_under_cursor(state);
            state.mode.transition(Mode::Insert);
        }
        (KeyCode::Char('S'), false) => {
            delete_entire_line_content(state);
            state.mode.transition(Mode::Insert);
        }
        (KeyCode::Char('J'), false) => {
            for _ in 0..count {
                join_lines(
                    &mut state.buffer,
                    state.cursor.pos.line,
                    true,
                    &mut state.history,
                );
            }
            state.modified = true;
        }

        // Yank and paste
        (KeyCode::Char('p'), false) => {
            paste_after(state);
        }
        (KeyCode::Char('P'), false) => {
            paste_before(state);
        }

        // Undo/redo
        (KeyCode::Char('u'), false) => {
            if let Some(pos) = undo(&mut state.buffer, &mut state.history) {
                state.cursor.pos = pos;
                state.modified = true;
            }
        }
        (KeyCode::Char('r'), true) => {
            if let Some(pos) = redo(&mut state.buffer, &mut state.history) {
                state.cursor.pos = pos;
                state.modified = true;
            }
        }

        // Search
        (KeyCode::Char('/'), false) => {
            state.search_forward = true;
            state.mode.transition(Mode::Command);
            state.cmdline = "/".to_string();
        }
        (KeyCode::Char('?'), false) => {
            state.search_forward = false;
            state.mode.transition(Mode::Command);
            state.cmdline = "?".to_string();
        }
        (KeyCode::Char('n'), false) => {
            search_next(state, state.search_forward);
        }
        (KeyCode::Char('N'), false) => {
            search_next(state, !state.search_forward);
        }

        // Scroll
        (KeyCode::Char('d'), true) => {
            let half = state.viewport.height / 2;
            state.viewport.scroll_down(half, state.buffer.line_count());
            state.cursor.pos = move_down(&state.buffer, state.cursor.pos, half);
        }
        (KeyCode::Char('u'), true) => {
            let half = state.viewport.height / 2;
            state.viewport.scroll_up(half);
            state.cursor.pos = move_up(&state.buffer, state.cursor.pos, half);
        }
        (KeyCode::Char('f'), true) => {
            let page = state.viewport.height;
            state.viewport.scroll_down(page, state.buffer.line_count());
            state.cursor.pos = move_down(&state.buffer, state.cursor.pos, page);
        }
        (KeyCode::Char('b'), true) => {
            let page = state.viewport.height;
            state.viewport.scroll_up(page);
            state.cursor.pos = move_up(&state.buffer, state.cursor.pos, page);
        }

        // Quit shortcuts
        (KeyCode::Char('Z'), false) => {
            // Would need to handle ZZ/ZQ sequences
        }

        _ => {}
    }
    Action::Continue
}

fn delete_char_under_cursor(state: &mut EditorState) {
    let pos = state.cursor.pos;
    let line_len = state.buffer.line_len(pos.line);
    if pos.col < line_len {
        let end = Position::new(pos.line, pos.col + 1);
        let deleted = state.buffer.delete(pos, end);
        state.yank(deleted, false);
        state.modified = true;
    }
}

fn delete_char_before_cursor(state: &mut EditorState) {
    if state.cursor.pos.col > 0 {
        let start = Position::new(state.cursor.pos.line, state.cursor.pos.col - 1);
        let end = state.cursor.pos;
        let deleted = state.buffer.delete(start, end);
        state.yank(deleted, false);
        state.cursor.pos.col -= 1;
        state.modified = true;
    }
}

fn delete_to_end_of_line(state: &mut EditorState) {
    let pos = state.cursor.pos;
    let line_len = state.buffer.line_len(pos.line);
    if pos.col < line_len {
        let end = Position::new(pos.line, line_len);
        let deleted = state.buffer.delete(pos, end);
        state.yank(deleted, false);
        state.modified = true;
    }
}

fn delete_entire_line_content(state: &mut EditorState) {
    let line = state.cursor.pos.line;
    let line_len = state.buffer.line_len(line);
    if line_len > 0 {
        let start = Position::new(line, 0);
        let end = Position::new(line, line_len);
        let deleted = state.buffer.delete(start, end);
        state.yank(deleted, true);
        state.cursor.pos.col = 0;
        state.modified = true;
    }
}

fn paste_after(state: &mut EditorState) {
    if let Some(reg) = state.paste().cloned() {
        if reg.linewise {
            let line = state.cursor.pos.line;
            let line_len = state.buffer.line_len(line);
            let pos = Position::new(line, line_len);
            let text = format!("\n{}", reg.content.trim_end_matches('\n'));
            state.buffer.insert(pos, &text);
            state.cursor.pos = Position::new(line + 1, 0);
        } else {
            let pos = Position::new(state.cursor.pos.line, state.cursor.pos.col + 1);
            state.buffer.insert(pos, &reg.content);
            state.cursor.pos.col += reg.content.len();
        }
        state.modified = true;
    }
}

fn paste_before(state: &mut EditorState) {
    if let Some(reg) = state.paste().cloned() {
        if reg.linewise {
            let pos = Position::new(state.cursor.pos.line, 0);
            let text = format!("{}\n", reg.content.trim_end_matches('\n'));
            state.buffer.insert(pos, &text);
        } else {
            state.buffer.insert(state.cursor.pos, &reg.content);
        }
        state.modified = true;
    }
}

fn search_next(state: &mut EditorState, forward: bool) {
    if state.search_pattern.is_empty() {
        return;
    }
    let pattern = &state.search_pattern;
    let total_lines = state.buffer.line_count();
    let start_line = state.cursor.pos.line;
    let start_col = state.cursor.pos.col + 1;

    if forward {
        for offset in 0..total_lines {
            let line_idx = (start_line + offset) % total_lines;
            if let Some(line) = state.buffer.line(line_idx) {
                let search_start = if offset == 0 { start_col } else { 0 };
                if let Some(col) = line[search_start..].find(pattern) {
                    state.cursor.pos = Position::new(line_idx, search_start + col);
                    return;
                }
            }
        }
    } else {
        for offset in 0..total_lines {
            let line_idx = (start_line + total_lines - offset) % total_lines;
            if let Some(line) = state.buffer.line(line_idx) {
                let search_end = if offset == 0 {
                    state.cursor.pos.col
                } else {
                    line.len()
                };
                if let Some(col) = line[..search_end].rfind(pattern) {
                    state.cursor.pos = Position::new(line_idx, col);
                    return;
                }
            }
        }
    }
}
