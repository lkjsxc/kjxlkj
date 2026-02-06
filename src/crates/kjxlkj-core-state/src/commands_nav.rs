//! Buffer navigation and session commands: :q, :bn, :bp, :ls, :NUM, :!

use crate::EditorState;

pub(crate) fn dispatch_quit(
    state: &mut EditorState,
    force: bool,
) {
    if !force && state.has_unsaved_changes() {
        state.message = Some(
            "No write since last change (add ! to override)"
                .into(),
        );
    } else {
        state.should_quit = true;
    }
}

pub(crate) fn dispatch_quit_all(
    state: &mut EditorState,
    force: bool,
) {
    if !force && state.has_unsaved_changes() {
        state.message = Some(
            "No write since last change (add ! to override)"
                .into(),
        );
    } else {
        state.should_quit = true;
    }
}

pub(crate) fn dispatch_list_buffers(
    state: &mut EditorState,
) {
    let mut lines = Vec::new();
    for (id, buf) in &state.buffers {
        let name =
            buf.file_path.as_deref().unwrap_or("[No Name]");
        let modified = if buf.modified { "+" } else { "" };
        lines.push(format!(
            "{:3} {}{}",
            id.0, modified, name
        ));
    }
    if lines.is_empty() {
        state.message = Some("No buffers".into());
    } else {
        state.message = Some(lines.join(" | "));
    }
}

pub(crate) fn dispatch_bnext(state: &mut EditorState) {
    if let Some(wid) = state.active_window {
        let win = state.windows.get(&wid).unwrap();
        let current = win.buffer_id;
        let mut ids: Vec<_> =
            state.buffers.keys().cloned().collect();
        ids.sort_by_key(|b| b.0);
        if let Some(pos) =
            ids.iter().position(|b| *b == current)
        {
            let next = ids[(pos + 1) % ids.len()];
            let w = state.windows.get_mut(&wid).unwrap();
            w.buffer_id = next;
            w.cursor_line = 0;
            w.cursor_col = 0;
        }
    }
}

pub(crate) fn dispatch_bprev(state: &mut EditorState) {
    if let Some(wid) = state.active_window {
        let win = state.windows.get(&wid).unwrap();
        let current = win.buffer_id;
        let mut ids: Vec<_> =
            state.buffers.keys().cloned().collect();
        ids.sort_by_key(|b| b.0);
        if let Some(pos) =
            ids.iter().position(|b| *b == current)
        {
            let prev = if pos == 0 {
                ids.len() - 1
            } else {
                pos - 1
            };
            let w = state.windows.get_mut(&wid).unwrap();
            w.buffer_id = ids[prev];
            w.cursor_line = 0;
            w.cursor_col = 0;
        }
    }
}

pub(crate) fn dispatch_unknown(
    state: &mut EditorState,
    trimmed: &str,
    command: &str,
) {
    if command.starts_with(":!") {
        let shell_cmd = trimmed[2..].trim();
        state.message = Some(format!(
            "shell command not yet implemented: {}",
            shell_cmd
        ));
    } else if let Some(num) = command.strip_prefix(':') {
        if let Ok(n) = num.parse::<usize>() {
            use kjxlkj_core_edit::apply_motion;
            use kjxlkj_core_types::{MotionKind, Position};
            if let Some(wid) = state.active_window {
                let win =
                    state.windows.get(&wid).unwrap();
                let bid = win.buffer_id;
                if let Some(buf) =
                    state.buffers.get(&bid)
                {
                    let pos = Position::new(
                        win.cursor_line,
                        win.cursor_col,
                    );
                    let new_pos = apply_motion(
                        &buf.text,
                        pos,
                        MotionKind::GotoLine(n),
                        1,
                    );
                    let win = state
                        .windows
                        .get_mut(&wid)
                        .unwrap();
                    win.cursor_line = new_pos.line;
                    win.cursor_col = new_pos.col;
                    win.ensure_cursor_visible();
                }
            }
            return;
        }
        state.message =
            Some(format!("unknown command: {}", command));
    } else {
        state.message =
            Some(format!("unknown command: {}", command));
    }
}
