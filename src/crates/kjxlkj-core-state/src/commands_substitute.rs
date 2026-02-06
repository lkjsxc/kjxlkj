//! Substitute and global commands: :s, :g, :v.

use crate::EditorState;

/// Substitute command: :s/pattern/replacement/[flags]
pub(crate) fn dispatch_substitute(
    state: &mut EditorState,
    cmd: &str,
) {
    let sep = cmd.chars().nth(2).unwrap_or('/');
    let rest = &cmd[3..]; // skip ":s/"
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
    let line = win.cursor_line;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let text = buf.text.line_to_string(line);
        let new_text = if global {
            text.replace(pattern, replacement)
        } else {
            text.replacen(pattern, replacement, 1)
        };
        if new_text != text {
            use kjxlkj_core_types::{Position, Range};
            let end = text.len();
            buf.text.delete_range(Range::new(
                Position::new(line, 0),
                Position::new(line, end),
            ));
            buf.text
                .insert_text(Position::new(line, 0), &new_text);
            buf.modified = true;
            state.message = None;
        } else {
            state.message =
                Some(format!("Pattern not found: {}", pattern));
        }
    }
}

/// Global command: :g/pattern/cmd.
pub(crate) fn dispatch_global(
    state: &mut EditorState,
    cmd: &str,
) {
    let sep = cmd.chars().nth(2).unwrap_or('/');
    let rest = &cmd[3..]; // skip ":g/"
    let parts: Vec<&str> = rest.splitn(2, sep).collect();
    if parts.is_empty() || parts[0].is_empty() {
        state.message =
            Some("Usage: :g/pattern/cmd".into());
        return;
    }
    let pattern = parts[0];
    let subcmd = if parts.len() > 1 { parts[1] } else { "p" };
    execute_global(state, pattern, subcmd, false);
}

/// VGlobal command: :v/pattern/cmd.
pub(crate) fn dispatch_vglobal(
    state: &mut EditorState,
    cmd: &str,
) {
    let rest = &cmd[3..]; // skip ":v/"
    let parts: Vec<&str> = rest.splitn(2, '/').collect();
    if parts.is_empty() || parts[0].is_empty() {
        state.message =
            Some("Usage: :v/pattern/cmd".into());
        return;
    }
    let pattern = parts[0];
    let subcmd = if parts.len() > 1 { parts[1] } else { "p" };
    execute_global(state, pattern, subcmd, true);
}

/// Execute :g or :v global command.
fn execute_global(
    state: &mut EditorState,
    pattern: &str,
    subcmd: &str,
    invert: bool,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let buf = match state.buffers.get(&bid) {
        Some(b) => b,
        None => return,
    };
    let line_count = buf.text.line_count();
    let matching: Vec<usize> = (0..line_count)
        .filter(|&i| {
            let text = buf.text.line_to_string(i);
            let has_match = text.contains(pattern);
            if invert { !has_match } else { has_match }
        })
        .collect();
    let subcmd_trimmed = subcmd.trim();
    if subcmd_trimmed == "d" || subcmd_trimmed == "delete" {
        let buf = state.buffers.get_mut(&bid).unwrap();
        for &line in matching.iter().rev() {
            use kjxlkj_core_types::{Position, Range};
            if line + 1 < buf.text.line_count() {
                buf.text.delete_range(Range::new(
                    Position::new(line, 0),
                    Position::new(line + 1, 0),
                ));
            } else if line > 0 {
                let prev_len = buf.text.line_len(line - 1);
                buf.text.delete_range(Range::new(
                    Position::new(line - 1, prev_len),
                    Position::new(
                        line,
                        buf.text.line_len(line),
                    ),
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
        state.message =
            Some(format!("{} lines deleted", matching.len()));
    } else if subcmd_trimmed == "p"
        || subcmd_trimmed == "print"
        || subcmd_trimmed.is_empty()
    {
        let buf = state.buffers.get(&bid).unwrap();
        let lines: Vec<String> = matching
            .iter()
            .take(20)
            .map(|&i| {
                format!(
                    "{:>4}: {}",
                    i + 1,
                    buf.text.line_to_string(i)
                )
            })
            .collect();
        state.message = Some(lines.join("\n"));
    } else {
        state.message = Some(format!(
            ":g sub-command not supported: {}",
            subcmd_trimmed,
        ));
    }
}

/// Range-aware substitute: respects LineRange if provided.
pub(crate) fn dispatch_substitute_range(
    state: &mut EditorState,
    cmd: &str,
    range: Option<crate::commands_range::LineRange>,
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
    for line in start..=end.min(buf.text.line_count().saturating_sub(1)) {
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
            buf.text.insert_text(Position::new(line, 0), &new_text);
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
        state.message = Some(format!("Pattern not found: {}", pattern));
    }
}

/// Range delete: delete lines in range.
pub(crate) fn dispatch_range_delete(
    state: &mut EditorState,
    range: Option<crate::commands_range::LineRange>,
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
        let cur_last = buf.text.line_count().saturating_sub(1);
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
    let new_last = buf.text.line_count().saturating_sub(1);
    if win.cursor_line > new_last {
        win.cursor_line = new_last;
    }
}

/// Range yank: yank lines in range to unnamed register.
pub(crate) fn dispatch_range_yank(
    state: &mut EditorState,
    range: Option<crate::commands_range::LineRange>,
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
    fn substitute_replaces() {
        let mut s = setup("hello world");
        dispatch_substitute(&mut s, ":s/hello/hi");
        let buf = s.active_buffer().unwrap();
        assert!(buf.text.line_to_string(0).starts_with("hi"));
    }

    #[test]
    fn substitute_global_flag() {
        let mut s = setup("aa bb aa");
        dispatch_substitute(&mut s, ":s/aa/cc/g");
        let buf = s.active_buffer().unwrap();
        assert_eq!(
            buf.text.line_to_string(0).trim(),
            "cc bb cc"
        );
    }

    #[test]
    fn global_delete() {
        let mut s = setup("keep\nremove this\nkeep too");
        dispatch_global(&mut s, ":g/remove/d");
        let buf = s.active_buffer().unwrap();
        assert_eq!(buf.text.line_count(), 2);
    }

    #[test]
    fn vglobal_delete() {
        let mut s = setup("keep\nremove\nkeep");
        dispatch_vglobal(&mut s, ":v/keep/d");
        let buf = s.active_buffer().unwrap();
        assert_eq!(buf.text.line_count(), 2);
    }
}
