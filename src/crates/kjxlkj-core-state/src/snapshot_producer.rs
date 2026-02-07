//! Produce an EditorSnapshot from EditorState for rendering.

use kjxlkj_core_types::Mode;
use kjxlkj_core_ui::snapshot::{
    BufferSnapshot, CursorShape, CursorState, EditorSnapshot, WindowSnap,
};

use crate::editor_state::EditorState;
use crate::viewport;

/// Build a complete EditorSnapshot from the current editor state.
pub fn produce_snapshot(state: &EditorState) -> EditorSnapshot {
    let mode = state.mode.current();
    let active_id = state.active_buffer_id();
    let win = state.active_window();
    let buf = state.active_buffer();

    // Extract visible lines from active buffer
    let (start, end) = win.visible_range();
    let lines: Vec<String> = (start..end.min(buf.line_count()))
        .map(|i| buf.line(i).unwrap_or_default())
        .collect();

    let buf_snap = BufferSnapshot {
        id: active_id,
        name: buf.name().to_string(),
        lines,
        line_count: buf.line_count(),
        modified: buf.is_modified(),
        filetype: detect_filetype(buf.path()),
        version: buf.version(),
    };

    let window_snaps: Vec<WindowSnap> = state
        .windows
        .iter()
        .map(|w| WindowSnap {
            id: w.id,
            buffer_id: w.buffer_id,
            cursor: w.cursor,
            top_line: w.viewport_top,
            left_col: w.viewport_left,
            width: w.width,
            height: w.height,
        })
        .collect();

    let cursor_shape = match mode {
        Mode::Insert => CursorShape::Line,
        Mode::Replace => CursorShape::Underline,
        _ => CursorShape::Block,
    };

    let status_line = build_status_line(state);
    let command_line = if state.command_line.active {
        state.command_line.full_text()
    } else {
        String::new()
    };

    EditorSnapshot {
        mode,
        buffers: vec![buf_snap],
        active_buffer: active_id,
        windows: window_snaps,
        status_line,
        command_line,
        message: state.message.clone(),
        cursor: CursorState {
            position: win.cursor,
            shape: cursor_shape,
            visible: true,
            blink: mode == Mode::Insert,
        },
        tab_line: None,
        terminal_width: state.terminal_size.0,
        terminal_height: state.terminal_size.1,
    }
}

fn build_status_line(state: &EditorState) -> String {
    let buf = state.active_buffer();
    let name = buf.path().unwrap_or(buf.name());
    let modified = if buf.is_modified() { " [+]" } else { "" };
    let pos = state.active_window().cursor;
    let mode = state.mode.current();
    let pct = viewport::scroll_percent(pos.line, buf.line_count());
    format!(" {mode} | {name}{modified} | {}:{} | {pct}%", pos.line + 1, pos.col + 1)
}

fn detect_filetype(path: Option<&str>) -> String {
    path.and_then(crate::syntax_cmd::detect_language)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_from_new_state() {
        let state = EditorState::new();
        let snap = produce_snapshot(&state);
        assert_eq!(snap.mode, Mode::Normal);
        assert_eq!(snap.buffers.len(), 1);
        assert!(snap.cursor.visible);
    }
}
