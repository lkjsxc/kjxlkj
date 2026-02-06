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
        ":sp" | ":split" => {
            crate::dispatch_windows::dispatch_window_split_horizontal(state)
        }
        ":vsp" | ":vsplit" => {
            crate::dispatch_windows::dispatch_window_split_vertical(state)
        }
        ":close" => {
            crate::dispatch_windows::dispatch_window_close(state)
        }
        ":only" => {
            crate::dispatch_windows::dispatch_window_only(state)
        }
        ":new" => {
            let bid = state.create_buffer();
            state.create_window(bid);
        }
        ":bd" | ":bdelete" => {
            crate::commands_display::dispatch_bdelete(state, false)
        }
        ":bd!" | ":bdelete!" => {
            crate::commands_display::dispatch_bdelete(state, true)
        }
        ":marks" => {
            crate::commands_display::dispatch_show_marks(state)
        }
        ":reg" | ":registers" => {
            crate::commands_display::dispatch_show_registers(state)
        }
        ":jumps" => {
            crate::commands_display::dispatch_show_jumps(state)
        }
        ":changes" => {
            crate::commands_display::dispatch_show_changes(state)
        }
        ":file" | ":f" => {
            crate::commands_display::dispatch_show_file_info(state)
        }
        ":noh" | ":nohlsearch" => {
            state.search_pattern = None;
            state.message = None;
        }
        ":sort" => {
            crate::commands_display::dispatch_sort_lines(state, args)
        }
        ":messages" | ":mes" => {
            // Display message history (simplified)
            if state.message.is_none() {
                state.message = Some("No messages".into());
            }
        }
        ":source" | ":so" => {
            if let Some(path) = args {
                let p = std::path::Path::new(path);
                match crate::config::load_config_file(state, p)
                {
                    Ok(n) => {
                        state.message = Some(format!(
                            "sourced {} lines from {}",
                            n, path
                        ));
                    }
                    Err(e) => {
                        state.message = Some(e);
                    }
                }
            } else {
                state.message =
                    Some("Usage: :source <file>".into());
            }
        }
        ":pwd" => {
            if let Ok(dir) = std::env::current_dir() {
                state.message =
                    Some(dir.to_string_lossy().to_string());
            }
        }
        ":cd" => {
            if let Some(dir) = args {
                if std::env::set_current_dir(dir).is_err() {
                    state.message =
                        Some(format!("Cannot cd to: {}", dir));
                }
            } else {
                if let Ok(dir) = std::env::current_dir() {
                    state.message =
                        Some(dir.to_string_lossy().to_string());
                }
            }
        }
        ":explorer" | ":terminal" | ":find" | ":livegrep"
        | ":undotree" => {
            state.message =
                Some(format!("{}: coming soon", command));
        }
        ":map" | ":nmap" | ":imap" | ":vmap" | ":cmap" | ":omap"
        | ":noremap" | ":nnoremap" | ":inoremap" | ":vnoremap"
        | ":cnoremap" | ":onoremap" => {
            dispatch_map_command(state, command, args);
        }
        ":unmap" | ":nunmap" | ":iunmap" | ":vunmap" | ":cunmap"
        | ":ounmap" => {
            dispatch_unmap_command(state, command, args);
        }
        ":mapclear" | ":nmapclear" | ":imapclear" | ":vmapclear" => {
            dispatch_mapclear(state, command);
        }
        _ => {
            // Try :s/pattern/replacement/[flags]
            // Try :s/pattern/replacement/[flags] with any separator
            if trimmed.starts_with(":s") && trimmed.len() > 2
                && !trimmed.chars().nth(2).unwrap_or(' ').is_alphanumeric()
            {
                crate::commands_substitute::dispatch_substitute(
                    state, trimmed,
                );
            } else if trimmed.starts_with(":g/")
                || trimmed.starts_with(":g!")
            {
                crate::commands_substitute::dispatch_global(
                    state, trimmed,
                );
            } else if trimmed.starts_with(":v/") {
                crate::commands_substitute::dispatch_vglobal(
                    state, trimmed,
                );
            } else {
                dispatch_unknown(state, trimmed, command);
            }
        }
    }
}

fn map_mode(command: &str) -> crate::mappings::MappingMode {
    use crate::mappings::MappingMode;
    match command {
        ":nmap" | ":nnoremap" | ":nunmap" | ":nmapclear" => {
            MappingMode::Normal
        }
        ":imap" | ":inoremap" | ":iunmap" | ":imapclear" => {
            MappingMode::Insert
        }
        ":vmap" | ":vnoremap" | ":vunmap" | ":vmapclear" => {
            MappingMode::Visual
        }
        ":cmap" | ":cnoremap" | ":cunmap" => {
            MappingMode::Command
        }
        ":omap" | ":onoremap" | ":ounmap" => {
            MappingMode::OperatorPending
        }
        _ => MappingMode::All,
    }
}

fn dispatch_map_command(
    state: &mut EditorState,
    command: &str,
    args: Option<&str>,
) {
    let mode = map_mode(command);
    let recursive = !command.contains("noremap");
    match args {
        Some(a) => {
            if let Some((lhs, rhs)) = a.split_once(' ') {
                state.mappings.add(
                    mode,
                    lhs.trim(),
                    rhs.trim(),
                    recursive,
                );
                state.message = Some(format!(
                    "mapped {} → {}",
                    lhs.trim(),
                    rhs.trim()
                ));
            } else {
                // Show mapping for lhs
                if let Some(m) = state.mappings.get(mode, a) {
                    state.message = Some(format!(
                        "{} → {}",
                        m.lhs, m.rhs
                    ));
                } else {
                    state.message =
                        Some(format!("No mapping: {}", a));
                }
            }
        }
        None => {
            let all = state.mappings.list(mode);
            if all.is_empty() {
                state.message =
                    Some("No mappings defined".into());
            } else {
                let lines: Vec<String> = all
                    .iter()
                    .map(|m| {
                        format!("{} → {}", m.lhs, m.rhs)
                    })
                    .collect();
                state.message =
                    Some(lines.join(" | "));
            }
        }
    }
}

fn dispatch_unmap_command(
    state: &mut EditorState,
    command: &str,
    args: Option<&str>,
) {
    let mode = map_mode(command);
    if let Some(lhs) = args {
        if state.mappings.remove(mode, lhs.trim()) {
            state.message =
                Some(format!("unmapped {}", lhs.trim()));
        } else {
            state.message =
                Some(format!("No mapping: {}", lhs.trim()));
        }
    } else {
        state.message =
            Some("Usage: :unmap <key>".into());
    }
}

fn dispatch_mapclear(
    state: &mut EditorState,
    command: &str,
) {
    let mode = map_mode(command);
    state.mappings.clear(mode);
    state.message = Some("mappings cleared".into());
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
    // Handle value assignments like tabstop=8
    if let Some((key, val)) = opt.split_once('=') {
        if let Ok(n) = val.parse::<usize>() {
            match key {
                "tabstop" | "ts" => {
                    state.options.tabstop = n;
                    state.message =
                        Some(format!("tabstop={}", n));
                }
                "shiftwidth" | "sw" => {
                    state.options.shiftwidth = n;
                    state.message =
                        Some(format!("shiftwidth={}", n));
                }
                "scrolloff" | "so" => {
                    state.options.scrolloff = n;
                    state.message =
                        Some(format!("scrolloff={}", n));
                }
                _ => {
                    state.message =
                        Some(format!("unknown option: {}", key));
                }
            }
        } else {
            state.message =
                Some(format!("invalid value: {}", val));
        }
        return;
    }
    // Handle boolean toggles
    match opt {
        "number" | "nu" => {
            state.options.number = true;
            state.message = Some("number on".into());
        }
        "nonumber" | "nonu" => {
            state.options.number = false;
            state.message = Some("number off".into());
        }
        "relativenumber" | "rnu" => {
            state.options.relative_number = true;
            state.message = Some("relativenumber on".into());
        }
        "norelativenumber" | "nornu" => {
            state.options.relative_number = false;
            state.message = Some("relativenumber off".into());
        }
        "wrap" => {
            state.options.wrap = true;
            state.message = Some("wrap on".into());
        }
        "nowrap" => {
            state.options.wrap = false;
            state.message = Some("wrap off".into());
        }
        "ignorecase" | "ic" => {
            state.options.ignorecase = true;
            state.message = Some("ignorecase on".into());
        }
        "noignorecase" | "noic" => {
            state.options.ignorecase = false;
            state.message = Some("ignorecase off".into());
        }
        "smartcase" | "scs" => {
            state.options.smartcase = true;
            state.message = Some("smartcase on".into());
        }
        "nosmartcase" | "noscs" => {
            state.options.smartcase = false;
            state.message = Some("smartcase off".into());
        }
        "hlsearch" | "hls" => {
            state.options.hlsearch = true;
            state.message = Some("hlsearch on".into());
        }
        "nohlsearch" | "nohls" => {
            state.options.hlsearch = false;
            state.message = Some("hlsearch off".into());
        }
        "incsearch" | "is" => {
            state.options.incsearch = true;
            state.message = Some("incsearch on".into());
        }
        "noincsearch" | "nois" => {
            state.options.incsearch = false;
            state.message = Some("incsearch off".into());
        }
        "expandtab" | "et" => {
            state.options.expandtab = true;
            state.message = Some("expandtab on".into());
        }
        "noexpandtab" | "noet" => {
            state.options.expandtab = false;
            state.message = Some("expandtab off".into());
        }
        "autoindent" | "ai" => {
            state.options.autoindent = true;
            state.message = Some("autoindent on".into());
        }
        "noautoindent" | "noai" => {
            state.options.autoindent = false;
            state.message = Some("autoindent off".into());
        }
        "smartindent" | "si" => {
            state.options.smartindent = true;
            state.message = Some("smartindent on".into());
        }
        "nosmartindent" | "nosi" => {
            state.options.smartindent = false;
            state.message = Some("smartindent off".into());
        }
        _ => {
            if opt.ends_with('?') {
                let name = &opt[..opt.len() - 1];
                let val = match name {
                    "number" | "nu" => {
                        format!("{}", state.options.number)
                    }
                    "wrap" => {
                        format!("{}", state.options.wrap)
                    }
                    "tabstop" | "ts" => {
                        format!("{}", state.options.tabstop)
                    }
                    "shiftwidth" | "sw" => {
                        format!("{}", state.options.shiftwidth)
                    }
                    "scrolloff" | "so" => {
                        format!("{}", state.options.scrolloff)
                    }
                    _ => "unknown".into(),
                };
                state.message =
                    Some(format!("{}={}", name, val));
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

// Substitute/global commands moved to commands_substitute.rs
