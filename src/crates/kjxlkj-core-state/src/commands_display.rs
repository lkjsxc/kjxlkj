//! Display commands: :marks, :registers, :jumps, :changes, :file.

use crate::EditorState;

/// Display marks (:marks).
pub(crate) fn dispatch_show_marks(state: &mut EditorState) {
    if state.marks.is_empty() {
        state.message = Some("No marks set".into());
        return;
    }
    let mut entries: Vec<_> = state
        .marks
        .iter()
        .map(|(c, (bid, pos))| {
            format!(
                " {} {:>5} {:>3}  buffer {}",
                c, pos.line + 1, pos.col, bid.0
            )
        })
        .collect();
    entries.sort();
    state.message = Some(format!(
        "mark line  col  file\n{}",
        entries.join("\n")
    ));
}

/// Display registers (:registers/:reg).
pub(crate) fn dispatch_show_registers(state: &mut EditorState) {
    let display = state.registers.display();
    if display.is_empty() {
        state.message = Some("No registers".into());
    } else {
        state.message = Some(display);
    }
}

/// Display jump list (:jumps).
pub(crate) fn dispatch_show_jumps(state: &mut EditorState) {
    if state.jump_list.is_empty() {
        state.message = Some("No jumps".into());
        return;
    }
    let lines: Vec<String> = state
        .jump_list
        .iter()
        .enumerate()
        .rev()
        .take(10)
        .map(|(i, (bid, pos))| {
            let marker =
                if i == state.jump_list_idx { ">" } else { " " };
            format!(
                "{}{:>3} {:>5} {:>3}  buf {}",
                marker,
                i,
                pos.line + 1,
                pos.col,
                bid.0,
            )
        })
        .collect();
    state.message =
        Some(format!("jump line  col file\n{}", lines.join("\n")));
}

/// Display change list (:changes).
pub(crate) fn dispatch_show_changes(state: &mut EditorState) {
    if state.change_list.is_empty() {
        state.message = Some("No changes".into());
        return;
    }
    let lines: Vec<String> = state
        .change_list
        .iter()
        .enumerate()
        .rev()
        .take(10)
        .map(|(i, (bid, pos))| {
            let marker = if i == state.change_list_idx {
                ">"
            } else {
                " "
            };
            format!(
                "{}{:>3} {:>5} {:>3}  buf {}",
                marker,
                i,
                pos.line + 1,
                pos.col,
                bid.0,
            )
        })
        .collect();
    state.message = Some(format!(
        "change line  col\n{}",
        lines.join("\n"),
    ));
}

/// Display file info (:file / Ctrl-g).
pub(crate) fn dispatch_show_file_info(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = match state.windows.get(&wid) {
        Some(w) => w,
        None => return,
    };
    let bid = win.buffer_id;
    if let Some(buf) = state.buffers.get(&bid) {
        let name = buf
            .file_path
            .as_deref()
            .unwrap_or("[No Name]");
        let modified = if buf.modified { "[+]" } else { "" };
        let lines = buf.text.line_count();
        let cursor = win.cursor_line + 1;
        let pct = if lines > 0 {
            cursor * 100 / lines
        } else {
            0
        };
        state.message = Some(format!(
            "\"{}\" {} {} lines --{}%--",
            name, modified, lines, pct,
        ));
    }
}

/// Sort lines in buffer (:sort).
pub(crate) fn dispatch_sort_lines(
    state: &mut EditorState,
    args: Option<&str>,
) {
    use kjxlkj_core_types::{Position, Range};
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let line_count = buf.text.line_count();
        let mut lines: Vec<String> = (0..line_count)
            .map(|i| buf.text.line_to_string(i))
            .collect();
        let reverse = args.map_or(false, |a| a.contains('!'));
        let unique = args.map_or(false, |a| a.contains('u'));
        lines.sort();
        if reverse {
            lines.reverse();
        }
        if unique {
            lines.dedup();
        }
        // Replace entire buffer
        let end_line = line_count.saturating_sub(1);
        let end_col = buf.text.line_len(end_line);
        buf.text.delete_range(Range::new(
            Position::new(0, 0),
            Position::new(end_line, end_col),
        ));
        let text = lines.join("\n");
        buf.text.insert_text(Position::new(0, 0), &text);
        buf.modified = true;
        state.message =
            Some(format!("{} lines sorted", lines.len()));
    }
}

/// Delete buffer (:bdelete).
pub(crate) fn dispatch_bdelete(
    state: &mut EditorState,
    force: bool,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    if !force {
        if let Some(buf) = state.buffers.get(&bid) {
            if buf.modified {
                state.message = Some(
                    "No write since last change (add ! to override)"
                        .into(),
                );
                return;
            }
        }
    }
    // If only one buffer, can't delete it
    if state.buffers.len() <= 1 {
        state.message = Some("Cannot delete last buffer".into());
        return;
    }
    state.buffers.remove(&bid);
    // Switch window to another buffer
    if let Some(other_bid) =
        state.buffers.keys().next().cloned()
    {
        let w = state.windows.get_mut(&wid).unwrap();
        w.buffer_id = other_bid;
        w.cursor_line = 0;
        w.cursor_col = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditorState;
    use kjxlkj_core_types::Size;

    fn setup(text: &str) -> EditorState {
        let mut s = EditorState::new(Size::new(80, 24));
        let bid = s.create_buffer_from_text(text);
        s.create_window(bid);
        s
    }

    #[test]
    fn show_file_info() {
        let mut s = setup("hello\nworld");
        dispatch_show_file_info(&mut s);
        let msg = s.message.unwrap();
        assert!(msg.contains("2 lines"));
    }

    #[test]
    fn sort_lines() {
        let mut s = setup("banana\napple\ncherry");
        dispatch_sort_lines(&mut s, None);
        let buf = s.active_buffer().unwrap();
        assert!(buf.text.line_to_string(0).starts_with("apple"));
    }

    #[test]
    fn show_marks_empty() {
        let mut s = setup("hello");
        dispatch_show_marks(&mut s);
        assert_eq!(s.message.as_deref(), Some("No marks set"));
    }

    #[test]
    fn bdelete_rejects_last() {
        let mut s = setup("hello");
        dispatch_bdelete(&mut s, true);
        assert_eq!(
            s.message.as_deref(),
            Some("Cannot delete last buffer")
        );
    }
}
