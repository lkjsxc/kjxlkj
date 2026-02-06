//! Window management dispatch: split, close, focus, resize.

use crate::EditorState;
use kjxlkj_core_types::{MotionKind, WindowId};

/// Split the current window horizontally (:split).
pub(crate) fn dispatch_window_split_horizontal(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = match state.windows.get(&wid) { Some(w) => w, None => return };
    let (bid, cursor_line, cursor_col, top_line) = (win.buffer_id, win.cursor_line, win.cursor_col, win.top_line);
    let new_wid = state.create_window(bid);
    if let Some(new_win) = state.windows.get_mut(&new_wid) {
        new_win.cursor_line = cursor_line;
        new_win.cursor_col = cursor_col;
        new_win.top_line = top_line;
        new_win.height = state.size.height as usize / 2;
    }
    if let Some(orig) = state.windows.get_mut(&wid) { orig.height = state.size.height as usize / 2; }
    state.active_window = Some(new_wid);
    state.message = Some(format!("Split: {} windows", state.windows.len()));
}

/// Split the current window vertically (:vsplit).
pub(crate) fn dispatch_window_split_vertical(state: &mut EditorState) {
    dispatch_window_split_horizontal(state);
}

/// Close the current window (:close).
pub(crate) fn dispatch_window_close(state: &mut EditorState) {
    if state.windows.len() <= 1 { state.message = Some("Cannot close last window".into()); return; }
    let wid = match state.active_window { Some(w) => w, None => return };
    state.windows.remove(&wid);
    state.active_window = state.windows.keys().next().cloned();
}

/// Close all other windows (:only).
pub(crate) fn dispatch_window_only(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let keep: Vec<WindowId> = state.windows.keys().filter(|id| **id != wid).cloned().collect();
    for id in keep { state.windows.remove(&id); }
    if let Some(win) = state.windows.get_mut(&wid) { win.height = state.size.height as usize; }
}

/// Focus the next window (Ctrl-w w).
pub(crate) fn dispatch_window_focus_next(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let mut ids: Vec<WindowId> = state.windows.keys().cloned().collect();
    ids.sort_by_key(|w| w.0);
    if let Some(pos) = ids.iter().position(|w| *w == wid) {
        state.active_window = Some(ids[(pos + 1) % ids.len()]);
    }
}

/// Focus the previous window.
pub(crate) fn dispatch_window_focus_prev(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let mut ids: Vec<WindowId> = state.windows.keys().cloned().collect();
    ids.sort_by_key(|w| w.0);
    if let Some(pos) = ids.iter().position(|w| *w == wid) {
        let prev = if pos == 0 { ids.len() - 1 } else { pos - 1 };
        state.active_window = Some(ids[prev]);
    }
}

/// Focus window in a direction (h/j/k/l).
pub(crate) fn dispatch_window_focus_direction(state: &mut EditorState, direction: MotionKind) {
    match direction {
        MotionKind::Down | MotionKind::Right => dispatch_window_focus_next(state),
        MotionKind::Up | MotionKind::Left => dispatch_window_focus_prev(state),
        _ => {}
    }
}

/// Equalize window sizes.
pub(crate) fn dispatch_window_equal_size(state: &mut EditorState) {
    let count = state.windows.len();
    if count == 0 { return; }
    let each = state.size.height as usize / count;
    for win in state.windows.values_mut() { win.height = each; }
}

/// Rotate windows (Ctrl-w r).
pub(crate) fn dispatch_window_rotate(state: &mut EditorState) {
    let mut ids: Vec<WindowId> = state.windows.keys().cloned().collect();
    ids.sort_by_key(|w| w.0);
    if ids.len() < 2 { return; }
    let mut bufs: Vec<_> = ids.iter().map(|id| state.windows[id].buffer_id).collect();
    bufs.rotate_right(1);
    for (id, bid) in ids.iter().zip(bufs.iter()) {
        if let Some(win) = state.windows.get_mut(id) { win.buffer_id = *bid; }
    }
}
