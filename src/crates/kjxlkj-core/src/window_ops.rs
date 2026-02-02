//! Window operations for editor state.

use kjxlkj_core_mode::WindowDirection;
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::WindowId;

/// Splits window horizontally.
pub fn split_horizontal(state: &mut EditorState) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let buffer_id = window.buffer_id;
    let new_window_id = state.create_window(buffer_id);
    state.layout.split_horizontal(new_window_id);
}

/// Splits window vertically.
pub fn split_vertical(state: &mut EditorState) {
    let Some(window) = state.windows.get(&state.layout.active) else {
        return;
    };
    let buffer_id = window.buffer_id;
    let new_window_id = state.create_window(buffer_id);
    state.layout.split_vertical(new_window_id);
}

/// Closes the current window.
pub fn close_window(state: &mut EditorState) {
    let active = state.layout.active;
    if state.layout.close_active() {
        state.windows.remove(&active);
    }
}

/// Keeps only the current window.
pub fn only_window(state: &mut EditorState) {
    let active = state.layout.active;
    let ids: Vec<WindowId> = state.windows.keys().copied().collect();

    for id in ids {
        if id != active {
            state.windows.remove(&id);
        }
    }

    // Reset layout to single window
    state.layout = kjxlkj_core_ui::Layout::new(active);
}

/// Navigates to the next window.
pub fn next_window(state: &mut EditorState) {
    let ids = state.layout.window_ids();
    if ids.len() <= 1 {
        return;
    }

    let current_idx = ids.iter().position(|&id| id == state.layout.active);
    if let Some(idx) = current_idx {
        let next_idx = (idx + 1) % ids.len();
        state.layout.active = ids[next_idx];
    }
}

/// Navigates to the previous window.
pub fn prev_window(state: &mut EditorState) {
    let ids = state.layout.window_ids();
    if ids.len() <= 1 {
        return;
    }

    let current_idx = ids.iter().position(|&id| id == state.layout.active);
    if let Some(idx) = current_idx {
        let prev_idx = if idx == 0 { ids.len() - 1 } else { idx - 1 };
        state.layout.active = ids[prev_idx];
    }
}

/// Navigates window by direction.
pub fn window_direction(state: &mut EditorState, direction: WindowDirection) {
    // For now, treat left/up as prev and right/down as next
    match direction {
        WindowDirection::Left | WindowDirection::Up => prev_window(state),
        WindowDirection::Right | WindowDirection::Down => next_window(state),
    }
}
