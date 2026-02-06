//! Line-transfer commands: :t/:copy, :m/:move, :r/:read.

use crate::commands_range::LineRange;
use crate::EditorState;
use kjxlkj_core_types::{BufferId, Position, Range};

fn parse_dest(state: &EditorState, s: &str) -> Result<usize, String> {
    let s = s.trim();
    if s.is_empty() { return Err("Destination required".into()); }
    let cursor = crate::commands_range::current_line(state);
    let last = crate::commands_range::last_line_idx(state);
    let (addr, _) = crate::commands_range::parse_address(s, cursor, last);
    addr.ok_or_else(|| format!("Invalid address: {}", s))
}

/// Collect text of lines [start..=end] with trailing newlines.
fn collect_lines(state: &EditorState, bid: BufferId, start: usize, end: usize) -> String {
    let buf = state.buffers.get(&bid).unwrap();
    let last = buf.text.line_count().saturating_sub(1);
    let mut text = String::new();
    for l in start..=end.min(last) {
        text.push_str(&buf.text.line_to_string(l));
        text.push('\n');
    }
    text
}

/// Insert `text` (with trailing newline) after `dest` line in buffer.
fn insert_after_line(state: &mut EditorState, bid: BufferId, dest: usize, text: &str) {
    let buf = state.buffers.get_mut(&bid).unwrap();
    let last = buf.text.line_count().saturating_sub(1);
    let insert_line = dest + 1;
    if insert_line > last {
        let eof = buf.text.line_count().saturating_sub(1);
        let col = buf.text.line_len(eof);
        let mut t = String::from("\n");
        t.push_str(text);
        if t.ends_with('\n') { t.pop(); }
        buf.text.insert_text(Position::new(eof, col), &t);
    } else {
        buf.text.insert_text(Position::new(insert_line, 0), text);
    }
    buf.modified = true;
}

/// Delete line at `idx`, handling edge cases (only line, last line).
fn delete_line_at(state: &mut EditorState, bid: BufferId, idx: usize) {
    let buf = state.buffers.get_mut(&bid).unwrap();
    let last = buf.text.line_count().saturating_sub(1);
    let line = idx.min(last);
    if line < last {
        buf.text.delete_range(Range::new(Position::new(line, 0), Position::new(line + 1, 0)));
    } else if line > 0 {
        let prev_len = buf.text.line_len(line - 1);
        buf.text.delete_range(Range::new(
            Position::new(line - 1, prev_len), Position::new(line, buf.text.line_len(line)),
        ));
    } else {
        let len = buf.text.line_len(0);
        buf.text.delete_range(Range::new(Position::new(0, 0), Position::new(0, len)));
    }
}

fn get_range_or_cursor(state: &EditorState, range: Option<LineRange>) -> (BufferId, usize, usize) {
    let wid = state.active_window.unwrap();
    let win = state.windows.get(&wid).unwrap();
    let cursor = win.cursor_line;
    match range {
        Some(r) => (win.buffer_id, r.start, r.end),
        None => (win.buffer_id, cursor, cursor),
    }
}

/// :t / :copy — copy lines from range to after destination.
pub(crate) fn dispatch_copy_lines(
    state: &mut EditorState, range: Option<LineRange>, args: Option<&str>,
) {
    let dest = match args.and_then(|a| parse_dest(state, a).ok()) {
        Some(d) => d,
        None => { state.message = Some("Usage: :t {address}".into()); return; }
    };
    if state.active_window.is_none() { return; }
    let (bid, start, end) = get_range_or_cursor(state, range);
    let end = end.min(state.buffers.get(&bid).unwrap().text.line_count().saturating_sub(1));
    let count = end - start + 1;
    let text = collect_lines(state, bid, start, end);
    insert_after_line(state, bid, dest, &text);
    let win = state.windows.get_mut(&state.active_window.unwrap()).unwrap();
    win.cursor_line = dest + count; win.cursor_col = 0; win.ensure_cursor_visible();
    state.message = Some(format!("{} line{} copied", count, if count > 1 { "s" } else { "" }));
}

/// :m / :move — move lines from range to after destination.
pub(crate) fn dispatch_move_lines(
    state: &mut EditorState, range: Option<LineRange>, args: Option<&str>,
) {
    let dest = match args.and_then(|a| parse_dest(state, a).ok()) {
        Some(d) => d,
        None => { state.message = Some("Usage: :m {address}".into()); return; }
    };
    if state.active_window.is_none() { return; }
    let (bid, start, end) = get_range_or_cursor(state, range);
    let end = end.min(state.buffers.get(&bid).unwrap().text.line_count().saturating_sub(1));
    let count = end - start + 1;
    if dest >= start && dest < end { state.message = Some("Move lines into themselves".into()); return; }
    let text = collect_lines(state, bid, start, end);
    for _ in 0..count { delete_line_at(state, bid, start); }
    state.buffers.get_mut(&bid).unwrap().modified = true;
    let adj_dest = if dest >= end + 1 { dest - count } else { dest };
    insert_after_line(state, bid, adj_dest, &text);
    let buf_last = state.buffers.get(&bid).unwrap().text.line_count().saturating_sub(1);
    let win = state.windows.get_mut(&state.active_window.unwrap()).unwrap();
    win.cursor_line = (adj_dest + count).min(buf_last); win.cursor_col = 0; win.ensure_cursor_visible();
    state.message = Some(format!("{} line{} moved", count, if count > 1 { "s" } else { "" }));
}

/// :r / :read — read file contents and insert below cursor.
pub(crate) fn dispatch_read_file(state: &mut EditorState, args: Option<&str>) {
    let filename = match args {
        Some(f) if !f.trim().is_empty() => f.trim(),
        _ => { state.message = Some("Usage: :r {filename}".into()); return; }
    };
    let content = match std::fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => { state.message = Some(format!("Can't open \"{}\": {}", filename, e)); return; }
    };
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let cursor = win.cursor_line;
    let lc = content.lines().count();
    let mut text = content;
    if !text.ends_with('\n') { text.push('\n'); }
    insert_after_line(state, bid, cursor, &text);
    let win = state.windows.get_mut(&wid).unwrap();
    win.cursor_line = cursor + lc;
    win.cursor_col = 0;
    win.ensure_cursor_visible();
    state.message = Some(format!("\"{}\" {} line{}", filename, lc, if lc != 1 { "s" } else { "" }));
}

/// :{range}!{cmd} — filter lines in range through an external command.
pub(crate) fn dispatch_filter_lines(
    state: &mut EditorState, range: Option<crate::commands_range::LineRange>, cmd: &str,
) {
    if cmd.trim().is_empty() {
        state.message = Some("E471: Argument required".into());
        return;
    }
    let wid = match state.active_window { Some(w) => w, None => return };
    let (bid, start, end) = get_range_or_cursor(state, range);
    let buf = match state.buffers.get(&bid) { Some(b) => b, None => return };
    let last = buf.text.line_count().saturating_sub(1);
    let end = end.min(last);
    // Collect input lines
    let mut input = String::new();
    for l in start..=end {
        input.push_str(&buf.text.line_to_string(l));
        input.push('\n');
    }
    // Run command
    use std::io::Write;
    use std::process::{Command, Stdio};
    let child = Command::new("sh").arg("-c").arg(cmd)
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();
    let output = match child {
        Ok(mut c) => {
            if let Some(mut stdin) = c.stdin.take() { let _ = stdin.write_all(input.as_bytes()); }
            match c.wait_with_output() { Ok(o) => o, Err(e) => { state.message = Some(format!("filter error: {}", e)); return; } }
        }
        Err(e) => { state.message = Some(format!("filter error: {}", e)); return; }
    };
    let result = String::from_utf8_lossy(&output.stdout).to_string();
    // Delete original range
    let count = end - start + 1;
    for _ in 0..count { delete_line_at(state, bid, start); }
    // Insert filtered output
    let dest = if start > 0 { start - 1 } else { 0 };
    if !result.is_empty() {
        let text = if result.ends_with('\n') { result.clone() } else { format!("{}\n", result) };
        if start == 0 {
            let buf = state.buffers.get_mut(&bid).unwrap();
            buf.text.insert_text(Position::new(0, 0), &text);
        } else {
            insert_after_line(state, bid, dest, &text);
        }
    }
    state.buffers.get_mut(&bid).unwrap().modified = true;
    let new_count = result.lines().count();
    let win = state.windows.get_mut(&wid).unwrap();
    win.cursor_line = start;
    win.cursor_col = 0;
    win.ensure_cursor_visible();
    state.message = Some(format!("{} line{} filtered", new_count, if new_count != 1 { "s" } else { "" }));
}
