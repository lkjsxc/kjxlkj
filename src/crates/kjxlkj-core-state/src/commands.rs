//! Ex command dispatch: :w, :e, :q, :ls, :set, etc.

use crate::EditorState;
use kjxlkj_core_types::MotionKind;

/// Dispatch an Ex command string against editor state.
pub(crate) fn dispatch_ex_command(state: &mut EditorState, cmd: &str) {
    let trimmed = cmd.trim();
    let (command, args) = match trimmed.split_once(' ') {
        Some((c, a)) => (c, Some(a.trim())),
        None => (trimmed, None),
    };
    match command {
        ":q" | ":quit" => dispatch_quit(state, false),
        ":q!" | ":quit!" => dispatch_quit(state, true),
        ":qa" => dispatch_quit_all(state, false),
        ":qa!" => dispatch_quit_all(state, true),
        ":w" | ":write" | ":w!" | ":write!" => {
            crate::commands_file::dispatch_write(state, args);
        }
        ":wq" | ":x" => {
            crate::commands_file::dispatch_write(state, None);
            state.should_quit = true;
        }
        ":wa" | ":wall" => {
            crate::commands_file::dispatch_write_all(state)
        }
        ":e" | ":edit" => {
            crate::commands_file::dispatch_edit(state, args, false)
        }
        ":e!" | ":edit!" => {
            crate::commands_file::dispatch_edit(state, args, true)
        }
        ":bn" | ":bnext" => dispatch_bnext(state),
        ":bp" | ":bprev" | ":bprevious" => dispatch_bprev(state),
        ":ls" | ":buffers" => dispatch_list_buffers(state),
        ":set" => match args {
            Some(opt) => dispatch_set_option(state, opt),
            None => {
                state.message =
                    Some("Usage: :set <option>".into());
            }
        },
        ":sp" | ":split" | ":vsp" | ":vsplit" | ":close"
        | ":only" => {
            state.message =
                Some(format!("{}: not yet implemented", command));
        }
        ":new" => {
            let bid = state.create_buffer();
            state.create_window(bid);
        }
        ":explorer" | ":terminal" | ":find" | ":livegrep"
        | ":undotree" => {
            state.message =
                Some(format!("{}: coming soon", command));
        }
        _ => {
            // Try :s/pattern/replacement/[flags]
            if trimmed.starts_with(":s/") || trimmed.starts_with(":s!") {
                dispatch_substitute(state, trimmed);
            } else {
                dispatch_unknown(state, trimmed, command);
            }
        }
    }
}

fn dispatch_quit(state: &mut EditorState, force: bool) {
    if !force && state.has_unsaved_changes() {
        state.message = Some(
            "No write since last change (add ! to override)".into(),
        );
    } else {
        state.should_quit = true;
    }
}

fn dispatch_quit_all(state: &mut EditorState, force: bool) {
    if !force && state.has_unsaved_changes() {
        state.message = Some(
            "No write since last change (add ! to override)".into(),
        );
    } else {
        state.should_quit = true;
    }
}

fn dispatch_list_buffers(state: &mut EditorState) {
    let mut lines = Vec::new();
    for (id, buf) in &state.buffers {
        let name = buf.file_path.as_deref().unwrap_or("[No Name]");
        let modified = if buf.modified { "+" } else { "" };
        lines.push(format!("{:3} {}{}", id.0, modified, name));
    }
    if lines.is_empty() {
        state.message = Some("No buffers".into());
    } else {
        state.message = Some(lines.join(" | "));
    }
}

fn dispatch_set_option(state: &mut EditorState, opt: &str) {
    match opt {
        "number" | "nu" => {
            state.message = Some("number on".into());
        }
        "nonumber" | "nonu" => {
            state.message = Some("number off".into());
        }
        "wrap" => {
            state.message = Some("wrap on".into());
        }
        "nowrap" => {
            state.message = Some("wrap off".into());
        }
        _ => {
            if opt.ends_with('?') {
                state.message = Some(format!(
                    "option not found: {}",
                    opt
                ));
            } else {
                state.message =
                    Some(format!("unknown option: {}", opt));
            }
        }
    }
}

fn dispatch_unknown(
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
            use kjxlkj_core_types::Position;
            if let Some(wid) = state.active_window {
                let win = state.windows.get(&wid).unwrap();
                let bid = win.buffer_id;
                if let Some(buf) = state.buffers.get(&bid) {
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
                    let win =
                        state.windows.get_mut(&wid).unwrap();
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

fn dispatch_bnext(state: &mut EditorState) {
    if let Some(wid) = state.active_window {
        let win = state.windows.get(&wid).unwrap();
        let current = win.buffer_id;
        let mut ids: Vec<_> = state.buffers.keys().cloned().collect();
        ids.sort_by_key(|b| b.0);
        if let Some(pos) = ids.iter().position(|b| *b == current) {
            let next = ids[(pos + 1) % ids.len()];
            let w = state.windows.get_mut(&wid).unwrap();
            w.buffer_id = next;
            w.cursor_line = 0;
            w.cursor_col = 0;
        }
    }
}

fn dispatch_bprev(state: &mut EditorState) {
    if let Some(wid) = state.active_window {
        let win = state.windows.get(&wid).unwrap();
        let current = win.buffer_id;
        let mut ids: Vec<_> = state.buffers.keys().cloned().collect();
        ids.sort_by_key(|b| b.0);
        if let Some(pos) = ids.iter().position(|b| *b == current) {
            let prev = if pos == 0 { ids.len() - 1 } else { pos - 1 };
            let w = state.windows.get_mut(&wid).unwrap();
            w.buffer_id = ids[prev];
            w.cursor_line = 0;
            w.cursor_col = 0;
        }
    }
}

/// Substitute command: :s/pattern/replacement/[flags]
fn dispatch_substitute(state: &mut EditorState, cmd: &str) {
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
            buf.text.insert_text(Position::new(line, 0), &new_text);
            buf.modified = true;
            state.message = None;
        } else {
            state.message =
                Some(format!("Pattern not found: {}", pattern));
        }
    }
}
