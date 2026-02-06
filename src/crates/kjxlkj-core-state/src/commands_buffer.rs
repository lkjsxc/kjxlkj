//! Buffer management commands: :sort, :bdelete.

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
