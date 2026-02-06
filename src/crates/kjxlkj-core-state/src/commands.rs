//! Ex command dispatch: :w, :e, :q, :ls, :set, etc.

use crate::EditorState;

pub(crate) fn dispatch_ex_command(state: &mut EditorState, cmd: &str) {
    let trimmed = cmd.trim();
    let (range, range_cmd) = crate::commands_range::parse_range(state, trimmed);
    let effective = if range.is_some() { &range_cmd } else { trimmed };
    if let Some(ref rng) = range {
        if effective.trim().trim_start_matches(':').is_empty() {
            if let Some(wid) = state.active_window {
                let win = state.windows.get_mut(&wid).unwrap();
                win.cursor_line = rng.end;
                win.cursor_col = 0;
                win.ensure_cursor_visible();
            }
            return;
        }
    }
    let (command, args) = match effective.split_once(' ') {
        Some((c, a)) => (c, Some(a.trim())),
        None => (effective, None),
    };
    use crate::{commands_config as cfg, commands_display as disp, commands_file as file};
    use crate::{commands_line_ops as lops, commands_nav as nav, commands_range_ops as rops};
    match command {
        ":q" | ":quit" => nav::dispatch_quit(state, false),
        ":q!" | ":quit!" => nav::dispatch_quit(state, true),
        ":qa" => nav::dispatch_quit_all(state, false),
        ":qa!" => nav::dispatch_quit_all(state, true),
        ":w" | ":write" | ":w!" | ":write!" => file::dispatch_write(state, args),
        ":wq" | ":x" => { file::dispatch_write(state, None); state.should_quit = true; }
        ":wa" | ":wall" => file::dispatch_write_all(state),
        ":e" | ":edit" => file::dispatch_edit(state, args, false),
        ":e!" | ":edit!" => file::dispatch_edit(state, args, true),
        ":bn" | ":bnext" => nav::dispatch_bnext(state),
        ":bp" | ":bprev" | ":bprevious" => nav::dispatch_bprev(state),
        ":ls" | ":buffers" => nav::dispatch_list_buffers(state),
        ":set" => match args {
            Some(opt) => cfg::dispatch_set_option(state, opt),
            None => { state.message = Some("Usage: :set <option>".into()); }
        },
        ":sp" | ":split" => crate::dispatch_windows::dispatch_window_split_horizontal(state),
        ":vsp" | ":vsplit" => crate::dispatch_windows::dispatch_window_split_vertical(state),
        ":close" => crate::dispatch_windows::dispatch_window_close(state),
        ":only" => crate::dispatch_windows::dispatch_window_only(state),
        ":new" => { let bid = state.create_buffer(); state.create_window(bid); }
        ":bd" | ":bdelete" => disp::dispatch_bdelete(state, false),
        ":bd!" | ":bdelete!" => disp::dispatch_bdelete(state, true),
        ":marks" => disp::dispatch_show_marks(state),
        ":reg" | ":registers" => disp::dispatch_show_registers(state),
        ":jumps" => disp::dispatch_show_jumps(state),
        ":changes" => disp::dispatch_show_changes(state),
        ":file" | ":f" => disp::dispatch_show_file_info(state),
        ":noh" | ":nohlsearch" => { state.search_pattern = None; state.message = None; }
        ":sort" => disp::dispatch_sort_lines(state, args),
        ":messages" | ":mes" => {
            if state.message.is_none() { state.message = Some("No messages".into()); }
        }
        ":source" | ":so" => dispatch_source(state, args),
        ":pwd" => {
            if let Ok(d) = std::env::current_dir() {
                state.message = Some(d.to_string_lossy().to_string());
            }
        }
        ":cd" => match args {
            Some(dir) => {
                if std::env::set_current_dir(dir).is_err() {
                    state.message = Some(format!("Cannot cd to: {}", dir));
                }
            }
            None => {
                if let Ok(d) = std::env::current_dir() {
                    state.message = Some(d.to_string_lossy().to_string());
                }
            }
        },
        ":explorer" | ":terminal" | ":find" | ":livegrep" | ":undotree" => {
            state.message = Some(format!("{}: coming soon", command));
        }
        ":map" | ":nmap" | ":imap" | ":vmap" | ":cmap" | ":omap"
        | ":noremap" | ":nnoremap" | ":inoremap" | ":vnoremap"
        | ":cnoremap" | ":onoremap" => cfg::dispatch_map_command(state, command, args),
        ":unmap" | ":nunmap" | ":iunmap" | ":vunmap"
        | ":cunmap" | ":ounmap" => cfg::dispatch_unmap_command(state, command, args),
        ":mapclear" | ":nmapclear" | ":imapclear"
        | ":vmapclear" => cfg::dispatch_mapclear(state, command),
        ":autocmd" | ":au" => cfg::dispatch_autocmd(state, args),
        ":autocmd!" | ":au!" => {
            state.autocmds.clear_all();
            state.message = Some("all autocommands cleared".into());
        }
        ":d" | ":delete" => rops::dispatch_range_delete(state, range),
        ":y" | ":yank" => rops::dispatch_range_yank(state, range),
        ":t" | ":copy" => lops::dispatch_copy_lines(state, range, args),
        ":m" | ":move" => lops::dispatch_move_lines(state, range, args),
        ":r" | ":read" => lops::dispatch_read_file(state, args),
        ":filetype" | ":ft" => cfg::dispatch_filetype(state, args),
        _ => dispatch_fallback(state, effective, trimmed, command, range),
    }
}

fn dispatch_source(state: &mut EditorState, args: Option<&str>) {
    if let Some(path) = args {
        let p = std::path::Path::new(path);
        match crate::config::load_config_file(state, p) {
            Ok(n) => state.message = Some(format!("sourced {} lines from {}", n, path)),
            Err(e) => state.message = Some(e),
        }
    } else {
        state.message = Some("Usage: :source <file>".into());
    }
}

fn dispatch_fallback(
    state: &mut EditorState, eff: &str, trimmed: &str, command: &str,
    range: Option<crate::commands_range::LineRange>,
) {
    if eff.starts_with(":s") && eff.len() > 2
        && !eff.chars().nth(2).unwrap_or(' ').is_alphanumeric()
    {
        crate::commands_range_ops::dispatch_substitute_range(state, eff, range);
    } else if eff.starts_with(":g/") || eff.starts_with(":g!") {
        crate::commands_substitute::dispatch_global(state, eff);
    } else if eff.starts_with(":v/") {
        crate::commands_substitute::dispatch_vglobal(state, eff);
    } else {
        crate::commands_nav::dispatch_unknown(state, trimmed, command);
    }
}
