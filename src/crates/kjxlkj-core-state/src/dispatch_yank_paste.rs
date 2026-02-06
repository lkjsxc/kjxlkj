//! Yank, paste, and delete-to-end dispatchers.

use crate::EditorState;
use kjxlkj_core_types::{Position, Range};

/// Yank entire lines (yy / Y with count).
pub(crate) fn dispatch_yank_line(state: &mut EditorState, count: usize) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    if let Some(buf) = state.buffers.get(&win.buffer_id) {
        let start = win.cursor_line;
        let end = (start + count).min(buf.text.line_count());
        let start_pos = Position::new(start, 0);
        let end_pos = if end >= buf.text.line_count() {
            let last = buf.text.line_count() - 1;
            Position::new(last, buf.text.line_len(last))
        } else {
            Position::new(end, 0)
        };
        let text = buf.text.text_in_range(start_pos, end_pos);
        state.registers.yank(&text, true);
    }
}

/// Delete from cursor to end of line (D / d$).
pub(crate) fn dispatch_delete_to_end(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let (line, col) = (win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let line_len = buf.text.line_len(line);
        if col < line_len {
            let start = Position::new(line, col);
            let end = Position::new(line, line_len);
            let text = buf.text.text_in_range(start, end);
            buf.text.delete_range(Range::new(start, end));
            buf.modified = true;
            state.registers.delete(&text, false);
        }
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        if let Some(buf) = state.buffers.get(&bid) {
            let new_len = buf.text.line_len(win.cursor_line);
            if win.cursor_col >= new_len && new_len > 0 {
                win.cursor_col = new_len.saturating_sub(1);
            }
        }
    }
}

/// Paste from unnamed register (p / P).
pub(crate) fn dispatch_paste(
    state: &mut EditorState,
    paste_pos: kjxlkj_core_types::PastePosition,
) {
    let text = match state.registers.unnamed_text() {
        Some(t) => t.to_string(),
        None => return,
    };
    let linewise = state.registers.unnamed_type()
        == Some(kjxlkj_core_types::RegisterType::Linewise);
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    use kjxlkj_core_types::PastePosition;
    if linewise {
        let line = match paste_pos {
            PastePosition::After | PastePosition::AfterCursorEnd => win.cursor_line + 1,
            PastePosition::Before | PastePosition::BeforeCursorEnd => win.cursor_line,
        };
        let pos = Position::new(line, 0);
        if let Some(buf) = state.buffers.get_mut(&bid) {
            let insert_text = if text.ends_with('\n') { text.clone() } else { format!("{}\n", text) };
            buf.text.insert_text(pos, &insert_text);
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_line = line;
        win.cursor_col = 0;
    } else {
        let col = match paste_pos {
            PastePosition::After | PastePosition::AfterCursorEnd => win.cursor_col + 1,
            PastePosition::Before | PastePosition::BeforeCursorEnd => win.cursor_col,
        };
        let pos = Position::new(win.cursor_line, col);
        if let Some(buf) = state.buffers.get_mut(&bid) {
            buf.text.insert_text(pos, &text);
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_col = col + text.len().saturating_sub(1);
    }
}
