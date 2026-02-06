//! Buffer management commands: :sort, :bdelete, alternate buffer.

use crate::EditorState;
use kjxlkj_core_types::{Position, Range};

/// Sort lines in buffer (:sort).
pub(crate) fn dispatch_sort_lines(state: &mut EditorState, args: Option<&str>) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let line_count = buf.text.line_count();
        let mut lines: Vec<String> = (0..line_count).map(|i| buf.text.line_to_string(i)).collect();
        let reverse = args.map_or(false, |a| a.contains('!'));
        let unique = args.map_or(false, |a| a.contains('u'));
        lines.sort();
        if reverse { lines.reverse(); }
        if unique { lines.dedup(); }
        let end_line = line_count.saturating_sub(1);
        let end_col = buf.text.line_len(end_line);
        buf.text.delete_range(Range::new(Position::new(0, 0), Position::new(end_line, end_col)));
        let text = lines.join("\n");
        buf.text.insert_text(Position::new(0, 0), &text);
        buf.modified = true;
        state.message = Some(format!("{} lines sorted", lines.len()));
    }
}

/// Delete buffer (:bdelete).
pub(crate) fn dispatch_bdelete(state: &mut EditorState, force: bool) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    if !force {
        if let Some(buf) = state.buffers.get(&bid) {
            if buf.modified {
                state.message = Some("No write since last change (add ! to override)".into());
                return;
            }
        }
    }
    if state.buffers.len() <= 1 {
        state.message = Some("Cannot delete last buffer".into());
        return;
    }
    state.buffers.remove(&bid);
    if let Some(other_bid) = state.buffers.keys().next().cloned() {
        let w = state.windows.get_mut(&wid).unwrap();
        w.buffer_id = other_bid;
        w.cursor_line = 0;
        w.cursor_col = 0;
    }
}

/// Switch to alternate buffer (Ctrl-^, :b#).
pub(crate) fn dispatch_switch_alternate(state: &mut EditorState) {
    let alt = match state.alternate_file {
        Some(bid) if state.buffers.contains_key(&bid) => bid,
        _ => { state.message = Some("No alternate file".into()); return; }
    };
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get_mut(&wid).unwrap();
    let old_bid = win.buffer_id;
    win.buffer_id = alt;
    win.cursor_line = 0;
    win.cursor_col = 0;
    state.alternate_file = Some(old_bid);
}

/// Switch to a specific buffer by number (:b N).
pub(crate) fn dispatch_switch_buffer(state: &mut EditorState, args: &str) {
    let args = args.trim();
    if args == "#" {
        dispatch_switch_alternate(state);
        return;
    }
    if let Ok(n) = args.parse::<u64>() {
        let bid = kjxlkj_core_types::BufferId(n);
        if !state.buffers.contains_key(&bid) {
            state.message = Some(format!("Buffer {} does not exist", n));
            return;
        }
        let wid = match state.active_window { Some(w) => w, None => return };
        let win = state.windows.get_mut(&wid).unwrap();
        let old_bid = win.buffer_id;
        win.buffer_id = bid;
        win.cursor_line = 0;
        win.cursor_col = 0;
        state.alternate_file = Some(old_bid);
    } else {
        state.message = Some("Usage: :b[uffer] {N|#}".into());
    }
}
