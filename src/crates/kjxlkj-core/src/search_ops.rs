//! Search operations for editor state.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::Cursor;

/// Searches forward for a pattern.
pub fn search_forward(state: &mut EditorState, pattern: &str) {
    state.registers.set_search(pattern.to_string());
    do_search(state, pattern, true);
}

/// Searches backward for a pattern.
pub fn search_backward(state: &mut EditorState, pattern: &str) {
    state.registers.set_search(pattern.to_string());
    do_search(state, pattern, false);
}

/// Navigates to the next search match.
pub fn next_match(state: &mut EditorState) {
    if let Some(pattern) = state.registers.search().map(|s| s.to_string()) {
        do_search(state, &pattern, true);
    }
}

/// Navigates to the previous search match.
pub fn prev_match(state: &mut EditorState) {
    if let Some(pattern) = state.registers.search().map(|s| s.to_string()) {
        do_search(state, &pattern, false);
    }
}

/// Performs the actual search.
fn do_search(state: &mut EditorState, pattern: &str, forward: bool) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let Some(buffer) = state.buffers.get(&window.buffer_id) else {
        return;
    };

    let pos = window.cursor.position;
    let text = buffer.text.to_string();

    if forward {
        if let Some((line, col)) = find_next(&text, pattern, pos.line, pos.col + 1) {
            if let Some(window) = state.windows.get_mut(&state.layout.active) {
                window.cursor = Cursor::at(line, col);
            }
        }
    } else if let Some((line, col)) = find_prev(&text, pattern, pos.line, pos.col) {
        if let Some(window) = state.windows.get_mut(&state.layout.active) {
            window.cursor = Cursor::at(line, col);
        }
    }
}

/// Finds the next occurrence.
fn find_next(text: &str, pattern: &str, start_line: usize, start_col: usize) -> Option<(usize, usize)> {
    let lines: Vec<&str> = text.lines().collect();

    // Search from current position
    for (idx, line) in lines.iter().enumerate().skip(start_line) {
        let search_start = if idx == start_line { start_col } else { 0 };
        if let Some(col) = line.get(search_start..)?.find(pattern) {
            return Some((idx, search_start + col));
        }
    }

    // Wrap around
    for (idx, line) in lines.iter().enumerate().take(start_line + 1) {
        if let Some(col) = line.find(pattern) {
            return Some((idx, col));
        }
    }

    None
}

/// Finds the previous occurrence.
fn find_prev(text: &str, pattern: &str, start_line: usize, start_col: usize) -> Option<(usize, usize)> {
    let lines: Vec<&str> = text.lines().collect();

    // Search backward from current position
    for idx in (0..=start_line).rev() {
        let line = lines.get(idx)?;
        let search_end = if idx == start_line { start_col } else { line.len() };
        if let Some(col) = line.get(..search_end)?.rfind(pattern) {
            return Some((idx, col));
        }
    }

    // Wrap around
    for idx in (start_line..lines.len()).rev() {
        if let Some(col) = lines.get(idx)?.rfind(pattern) {
            return Some((idx, col));
        }
    }

    None
}
