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

pub(crate) fn dispatch_qf_next(state: &mut EditorState) {
    match state.quickfix.next() {
        Some(e) => state.message = Some(format!("{}:{}: {}", e.file, e.line, e.text)),
        None => state.message = Some("No more items".into()),
    }
}

pub(crate) fn dispatch_qf_prev(state: &mut EditorState) {
    match state.quickfix.prev() {
        Some(e) => state.message = Some(format!("{}:{}: {}", e.file, e.line, e.text)),
        None => state.message = Some("At first item".into()),
    }
}

pub(crate) fn dispatch_qf_list(state: &mut EditorState) {
    if state.quickfix.is_empty() { state.message = Some("Quickfix list is empty".into()); return; }
    let items: Vec<_> = state.quickfix.entries.iter()
        .map(|e| format!("{}:{}:{}: {}", e.file, e.line, e.col, e.text)).collect();
    state.message = Some(items.join("\n"));
}

fn open_scratch_panel(state: &mut EditorState, title: &str, text: String) {
    let bid = state.create_buffer_from_text(&text);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.scratch = true; buf.readonly = true; buf.listed = false;
        buf.file_path = Some(format!("[{title}]"));
    }
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            state.alternate_file = Some(win.buffer_id);
            win.buffer_id = bid; win.cursor_line = 0; win.cursor_col = 0; win.top_line = 0;
        }
    } else { state.create_window(bid); }
}

pub(crate) fn dispatch_explorer(state: &mut EditorState) {
    let mut rows = Vec::new();
    if let Ok(rd) = std::fs::read_dir(".") {
        for e in rd.flatten().take(200) { rows.push(e.file_name().to_string_lossy().into_owned()); }
        rows.sort();
    }
    if rows.is_empty() { rows.push("(empty directory)".into()); }
    open_scratch_panel(state, "Explorer", rows.join("\n"));
    state.message = Some("Explorer opened".into());
}

pub(crate) fn dispatch_terminal(state: &mut EditorState) {
    let text = "Terminal panel\n\nInteractive PTY wiring is handled by the terminal service.\nUse :q or buffer switch to leave this panel.".to_string();
    open_scratch_panel(state, "Terminal", text);
    state.message = Some("Terminal opened".into());
}
