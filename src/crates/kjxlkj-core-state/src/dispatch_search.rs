//! Search dispatch: forward/backward search, next/prev, word search.

use crate::EditorState;
use kjxlkj_core_types::Position;

/// Search forward from current position for `pattern`.
pub(crate) fn dispatch_search_forward(
    state: &mut EditorState,
    pattern: &str,
) {
    if pattern.is_empty() {
        return;
    }
    state.search_pattern = Some(pattern.to_string());
    state.search_forward = true;
    search_next_impl(state, true);
}

/// Search backward from current position for `pattern`.
pub(crate) fn dispatch_search_backward(
    state: &mut EditorState,
    pattern: &str,
) {
    if pattern.is_empty() {
        return;
    }
    state.search_pattern = Some(pattern.to_string());
    state.search_forward = false;
    search_next_impl(state, false);
}

/// Repeat search in same direction.
pub(crate) fn dispatch_search_next(state: &mut EditorState) {
    let forward = state.search_forward;
    search_next_impl(state, forward);
}

/// Repeat search in opposite direction.
pub(crate) fn dispatch_search_prev(state: &mut EditorState) {
    let forward = !state.search_forward;
    search_next_impl(state, forward);
}

/// Search for word under cursor forward.
pub(crate) fn dispatch_search_word_forward(
    state: &mut EditorState,
) {
    if let Some(word) = word_under_cursor(state) {
        state.search_pattern = Some(word);
        state.search_forward = true;
        search_next_impl(state, true);
    }
}

/// Search for word under cursor backward.
pub(crate) fn dispatch_search_word_backward(
    state: &mut EditorState,
) {
    if let Some(word) = word_under_cursor(state) {
        state.search_pattern = Some(word);
        state.search_forward = false;
        search_next_impl(state, false);
    }
}

/// Get the word under the cursor.
fn word_under_cursor(state: &EditorState) -> Option<String> {
    let wid = state.active_window?;
    let win = state.windows.get(&wid)?;
    let buf = state.buffers.get(&win.buffer_id)?;
    let line = buf.text.line_to_string(win.cursor_line);
    let chars: Vec<char> = line.chars().collect();
    if win.cursor_col >= chars.len() {
        return None;
    }
    if !chars[win.cursor_col].is_alphanumeric()
        && chars[win.cursor_col] != '_'
    {
        return None;
    }
    let mut start = win.cursor_col;
    while start > 0
        && (chars[start - 1].is_alphanumeric()
            || chars[start - 1] == '_')
    {
        start -= 1;
    }
    let mut end = win.cursor_col;
    while end < chars.len()
        && (chars[end].is_alphanumeric() || chars[end] == '_')
    {
        end += 1;
    }
    Some(chars[start..end].iter().collect())
}

/// Core search implementation: find next occurrence in direction.
fn search_next_impl(state: &mut EditorState, forward: bool) {
    let pattern = match &state.search_pattern {
        Some(p) => p.clone(),
        None => {
            state.message = Some("No previous search".into());
            return;
        }
    };
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let cur_line = win.cursor_line;
    let cur_col = win.cursor_col;

    let buf = match state.buffers.get(&bid) {
        Some(b) => b,
        None => return,
    };
    let line_count = buf.text.line_count();

    if forward {
        // Search forward: start from current col+1, wrap around
        for offset in 0..line_count {
            let line_idx = (cur_line + offset) % line_count;
            let line_text = buf.text.line_to_string(line_idx);
            let start_col = if offset == 0 { cur_col + 1 } else { 0 };
            if let Some(found) =
                find_in_line(&line_text, &pattern, start_col)
            {
                let win = state.windows.get_mut(&wid).unwrap();
                win.set_cursor(Position::new(line_idx, found));
                win.ensure_cursor_visible();
                if offset >= line_count / 2 {
                    state.message =
                        Some("search wrapped".into());
                }
                return;
            }
        }
    } else {
        // Search backward
        for offset in 0..line_count {
            let line_idx = if offset <= cur_line {
                cur_line - offset
            } else {
                line_count - (offset - cur_line)
            };
            let line_text = buf.text.line_to_string(line_idx);
            let max_col = if offset == 0 {
                cur_col.saturating_sub(1)
            } else {
                line_text.len()
            };
            if let Some(found) =
                rfind_in_line(&line_text, &pattern, max_col)
            {
                let win = state.windows.get_mut(&wid).unwrap();
                win.set_cursor(Position::new(line_idx, found));
                win.ensure_cursor_visible();
                if offset >= line_count / 2 {
                    state.message =
                        Some("search wrapped".into());
                }
                return;
            }
        }
    }
    state.message =
        Some(format!("Pattern not found: {}", pattern));
}

/// Find pattern in a line starting from `from_col`.
fn find_in_line(
    line: &str,
    pattern: &str,
    from_col: usize,
) -> Option<usize> {
    if from_col >= line.len() {
        return None;
    }
    line[from_col..].find(pattern).map(|i| i + from_col)
}

/// Reverse-find pattern in a line, ending before `max_col`.
fn rfind_in_line(
    line: &str,
    pattern: &str,
    max_col: usize,
) -> Option<usize> {
    let end = max_col.min(line.len());
    if end == 0 {
        return None;
    }
    line[..end].rfind(pattern)
}
