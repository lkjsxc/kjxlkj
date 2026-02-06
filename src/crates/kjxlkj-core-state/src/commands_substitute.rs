//! Substitute and global commands: :s, :g, :v.

use crate::EditorState;

/// Substitute command: :s/pattern/replacement/[flags]
pub(crate) fn dispatch_substitute(
    state: &mut EditorState,
    cmd: &str,
) {
    let sep = cmd.chars().nth(2).unwrap_or('/');
    let parts: Vec<&str> = cmd[3..].splitn(3, sep).collect();
    if parts.len() < 2 {
        state.message = Some("Usage: :s/pattern/replacement/[flags]".into());
        return;
    }
    let (pattern, replacement) = (parts[0], parts[1]);
    let global = parts.get(2).map_or(false, |f| f.contains('g'));

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
pub(crate) fn dispatch_global(state: &mut EditorState, cmd: &str) {
    let sep = cmd.chars().nth(2).unwrap_or('/');
    let parts: Vec<&str> = cmd[3..].splitn(2, sep).collect();
    if parts.is_empty() || parts[0].is_empty() {
        state.message = Some("Usage: :g/pattern/cmd".into());
        return;
    }
    let pattern = parts[0];
    let subcmd = parts.get(1).copied().unwrap_or("p");
    execute_global(state, pattern, subcmd, false);
}

/// VGlobal command: :v/pattern/cmd.
pub(crate) fn dispatch_vglobal(state: &mut EditorState, cmd: &str) {
    let parts: Vec<&str> = cmd[3..].splitn(2, '/').collect();
    if parts.is_empty() || parts[0].is_empty() {
        state.message = Some("Usage: :v/pattern/cmd".into());
        return;
    }
    let pattern = parts[0];
    let subcmd = parts.get(1).copied().unwrap_or("p");
    execute_global(state, pattern, subcmd, true);
}

/// Execute :g or :v global command.
fn execute_global(
    state: &mut EditorState, pattern: &str,
    subcmd: &str, invert: bool,
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
        state.message = Some(format!("{} lines deleted", matching.len()));
    } else if subcmd_trimmed == "p" || subcmd_trimmed == "print"
        || subcmd_trimmed.is_empty() {
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
