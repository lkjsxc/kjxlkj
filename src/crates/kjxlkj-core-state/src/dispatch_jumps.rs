//! Jump list and change list navigation.

use crate::EditorState;
use kjxlkj_core_types::Position;

/// Push current position onto the jump list.
pub(crate) fn push_jump(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    // Don't push duplicates
    if state.jump_list.last() == Some(&(bid, pos)) {
        return;
    }
    // Truncate any forward entries
    state.jump_list.truncate(state.jump_list_idx);
    state.jump_list.push((bid, pos));
    state.jump_list_idx = state.jump_list.len();
    // Limit jump list size
    if state.jump_list.len() > 100 {
        state.jump_list.remove(0);
        state.jump_list_idx = state.jump_list.len();
    }
}

pub(crate) fn dispatch_jump_back(state: &mut EditorState) {
    if state.jump_list_idx == 0 || state.jump_list.is_empty() {
        state.message = Some("At bottom of jump list".into());
        return;
    }
    // Save current position before jumping
    if state.jump_list_idx == state.jump_list.len() {
        push_jump(state);
        state.jump_list_idx -= 1;
    }
    state.jump_list_idx = state.jump_list_idx.saturating_sub(1);
    let (bid, pos) = state.jump_list[state.jump_list_idx];
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            win.buffer_id = bid;
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    }
}

pub(crate) fn dispatch_jump_forward(state: &mut EditorState) {
    if state.jump_list_idx + 1 >= state.jump_list.len() {
        state.message = Some("At top of jump list".into());
        return;
    }
    state.jump_list_idx += 1;
    let (bid, pos) = state.jump_list[state.jump_list_idx];
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            win.buffer_id = bid;
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    }
}

/// Push a change position onto the change list.
pub(crate) fn push_change(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    state.change_list.push((bid, pos));
    state.change_list_idx = state.change_list.len();
    if state.change_list.len() > 100 {
        state.change_list.remove(0);
        state.change_list_idx = state.change_list.len();
    }
}

pub(crate) fn dispatch_change_older(
    state: &mut EditorState,
) {
    if state.change_list_idx == 0
        || state.change_list.is_empty()
    {
        state.message = Some("At oldest change".into());
        return;
    }
    state.change_list_idx -= 1;
    let (bid, pos) = state.change_list[state.change_list_idx];
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            win.buffer_id = bid;
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    }
}

pub(crate) fn dispatch_change_newer(
    state: &mut EditorState,
) {
    if state.change_list_idx + 1 >= state.change_list.len() {
        state.message = Some("At newest change".into());
        return;
    }
    state.change_list_idx += 1;
    let (bid, pos) = state.change_list[state.change_list_idx];
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            win.buffer_id = bid;
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    }
}
