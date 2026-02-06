//! File I/O Ex commands: :w, :e, :wa, reload.

use crate::EditorState;

pub(crate) fn dispatch_write(
    state: &mut EditorState,
    path: Option<&str>,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let target = path
            .map(|p| p.to_string())
            .or_else(|| buf.file_path.clone());
        match target {
            Some(p) => {
                let text = buf.text.text();
                match std::fs::write(&p, &text) {
                    Ok(()) => {
                        buf.file_path = Some(p.clone());
                        buf.modified = false;
                        let bytes = text.len();
                        let lines = buf.text.line_count();
                        state.message = Some(format!(
                            "\"{}\" {}L, {}B written",
                            p, lines, bytes
                        ));
                    }
                    Err(e) => {
                        state.message = Some(format!(
                            "Error writing {}: {}",
                            p, e
                        ));
                    }
                }
            }
            None => {
                state.message =
                    Some("No file name (use :w <file>)".into());
            }
        }
    }
}

pub(crate) fn dispatch_write_all(state: &mut EditorState) {
    let mut count = 0;
    let bids: Vec<_> = state.buffers.keys().copied().collect();
    for bid in bids {
        if let Some(buf) = state.buffers.get(&bid) {
            if buf.modified {
                if let Some(path) = buf.file_path.clone() {
                    let text = buf.text.text();
                    if std::fs::write(&path, &text).is_ok() {
                        if let Some(buf) =
                            state.buffers.get_mut(&bid)
                        {
                            buf.modified = false;
                        }
                        count += 1;
                    }
                }
            }
        }
    }
    state.message =
        Some(format!("{} buffer(s) written", count));
}

pub(crate) fn dispatch_edit(
    state: &mut EditorState,
    args: Option<&str>,
    force: bool,
) {
    match args {
        Some(path) => dispatch_edit_file(state, path),
        None => {
            if force {
                dispatch_reload_buffer(state);
            } else {
                state.message =
                    Some("Usage: :e <file>".into());
            }
        }
    }
}

pub(crate) fn dispatch_edit_file(
    state: &mut EditorState,
    path: &str,
) {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let bid = state.create_buffer_from_text(&content);
            if let Some(buf) = state.buffers.get_mut(&bid) {
                buf.file_path = Some(path.to_string());
                buf.modified = false;
            }
            if let Some(wid) = state.active_window {
                if let Some(win) = state.windows.get_mut(&wid) {
                    win.buffer_id = bid;
                    win.cursor_line = 0;
                    win.cursor_col = 0;
                    win.top_line = 0;
                }
            } else {
                state.create_window(bid);
            }
            state.message =
                Some(format!("\"{}\" opened", path));
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let bid = state.create_buffer();
                if let Some(buf) = state.buffers.get_mut(&bid) {
                    buf.file_path = Some(path.to_string());
                }
                if let Some(wid) = state.active_window {
                    if let Some(win) =
                        state.windows.get_mut(&wid)
                    {
                        win.buffer_id = bid;
                        win.cursor_line = 0;
                        win.cursor_col = 0;
                        win.top_line = 0;
                    }
                } else {
                    state.create_window(bid);
                }
                state.message =
                    Some(format!("\"{}\" [New file]", path));
            } else {
                state.message = Some(format!(
                    "Error reading {}: {}",
                    path, e
                ));
            }
        }
    }
}

pub(crate) fn dispatch_reload_buffer(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    if let Some(buf) = state.buffers.get(&bid) {
        if let Some(path) = buf.file_path.clone() {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    if let Some(buf) =
                        state.buffers.get_mut(&bid)
                    {
                        buf.text =
                            kjxlkj_core_text::TextBuffer::from_text(
                                &content,
                            );
                        buf.modified = false;
                        buf.undo =
                            kjxlkj_core_undo::UndoTree::new();
                    }
                    state.message = Some(format!(
                        "\"{}\" reloaded",
                        path
                    ));
                }
                Err(e) => {
                    state.message = Some(format!(
                        "Cannot reload: {}",
                        e
                    ));
                }
            }
        }
    }
}
