//! Range-aware editing commands: :s with range, :d, :y.

use crate::commands_range::LineRange;
use crate::EditorState;

/// Range-aware substitute: respects LineRange if provided.
pub(crate) fn dispatch_substitute_range(
    state: &mut EditorState,
    cmd: &str,
    range: Option<LineRange>,
) {
    let sep = cmd.chars().nth(2).unwrap_or('/');
    let rest = &cmd[3..];
    let parts: Vec<&str> = rest.splitn(3, sep).collect();
    if parts.len() < 2 {
        state.message =
            Some("Usage: :s/pattern/replacement/[flags]".into());
        return;
    }
    let pattern = parts[0];
    let replacement = parts[1];
    let flags = if parts.len() > 2 { parts[2] } else { "" };
    let global = flags.contains('g');

    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let cursor = win.cursor_line;

    let (start, end) = match range {
        Some(r) => (r.start, r.end),
        None => (cursor, cursor),
    };

    let buf = match state.buffers.get_mut(&bid) {
        Some(b) => b,
        None => return,
    };
    let mut count = 0usize;
    let last = buf.text.line_count().saturating_sub(1);
    for line in start..=end.min(last) {
        let text = buf.text.line_to_string(line);
        let new_text = if global {
            text.replace(pattern, replacement)
        } else {
            text.replacen(pattern, replacement, 1)
        };
        if new_text != text {
            use kjxlkj_core_types::{Position, Range};
            let end_col = text.len();
            buf.text.delete_range(Range::new(
                Position::new(line, 0),
                Position::new(line, end_col),
            ));
            buf.text
                .insert_text(Position::new(line, 0), &new_text);
            count += 1;
        }
    }
    if count > 0 {
        buf.modified = true;
        state.message = Some(format!(
            "{} substitution{} on {} line{}",
            count,
            if count > 1 { "s" } else { "" },
            count,
            if count > 1 { "s" } else { "" },
        ));
    } else {
        state.message =
            Some(format!("Pattern not found: {}", pattern));
    }
}

/// Range delete: delete lines in range.
pub(crate) fn dispatch_range_delete(
    state: &mut EditorState,
    range: Option<LineRange>,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let cursor = win.cursor_line;
    let (start, end) = match range {
        Some(r) => (r.start, r.end),
        None => (cursor, cursor),
    };
    let buf = match state.buffers.get_mut(&bid) {
        Some(b) => b,
        None => return,
    };
    let last = buf.text.line_count().saturating_sub(1);
    let end = end.min(last);
    let count = end - start + 1;
    for _ in 0..count {
        use kjxlkj_core_types::{Position, Range};
        let cur_last =
            buf.text.line_count().saturating_sub(1);
        let line = start.min(cur_last);
        if line < cur_last {
            buf.text.delete_range(Range::new(
                Position::new(line, 0),
                Position::new(line + 1, 0),
            ));
        } else if line > 0 {
            let prev_len = buf.text.line_len(line - 1);
            buf.text.delete_range(Range::new(
                Position::new(line - 1, prev_len),
                Position::new(line, buf.text.line_len(line)),
            ));
        } else {
            let len = buf.text.line_len(0);
            buf.text.delete_range(Range::new(
                Position::new(0, 0),
                Position::new(0, len),
            ));
        }
    }
    buf.modified = true;
    state.message = Some(format!("{} lines deleted", count));
    // Adjust cursor
    let win = state.windows.get_mut(&wid).unwrap();
    let new_last =
        buf.text.line_count().saturating_sub(1);
    if win.cursor_line > new_last {
        win.cursor_line = new_last;
    }
}

/// Range yank: yank lines in range to unnamed register.
pub(crate) fn dispatch_range_yank(
    state: &mut EditorState,
    range: Option<LineRange>,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let cursor = win.cursor_line;
    let (start, end) = match range {
        Some(r) => (r.start, r.end),
        None => (cursor, cursor),
    };
    let buf = match state.buffers.get(&bid) {
        Some(b) => b,
        None => return,
    };
    let last = buf.text.line_count().saturating_sub(1);
    let end = end.min(last);
    let mut yanked = String::new();
    for line in start..=end {
        yanked.push_str(&buf.text.line_to_string(line));
        yanked.push('\n');
    }
    let count = end - start + 1;
    use kjxlkj_core_types::{RegisterContent, RegisterName};
    state.registers.set(
        RegisterName::Unnamed,
        RegisterContent::linewise(&yanked),
    );
    state.message = Some(format!("{} lines yanked", count));
}
