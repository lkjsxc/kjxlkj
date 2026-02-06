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
        ":bn" | ":bnext" => {
            state.message =
                Some("bnext: not yet implemented".into());
        }
        ":bp" | ":bprev" | ":bprevious" => {
            state.message =
                Some("bprev: not yet implemented".into());
        }
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
        _ => dispatch_unknown(state, trimmed, command),
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
