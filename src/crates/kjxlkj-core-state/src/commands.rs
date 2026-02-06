//! Ex command dispatch: :w, :e, :q, :ls, :set, etc.

use crate::EditorState;

/// Dispatch an Ex command string against editor state.
pub(crate) fn dispatch_ex_command(state: &mut EditorState, cmd: &str) {
    let trimmed = cmd.trim();
    let (command, args) = match trimmed.split_once(' ') {
        Some((c, a)) => (c, Some(a.trim())),
        None => (trimmed, None),
    };
    match command {
        ":q" | ":quit" => {
            crate::commands_nav::dispatch_quit(state, false)
        }
        ":q!" | ":quit!" => {
            crate::commands_nav::dispatch_quit(state, true)
        }
        ":qa" => {
            crate::commands_nav::dispatch_quit_all(state, false)
        }
        ":qa!" => {
            crate::commands_nav::dispatch_quit_all(state, true)
        }
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
            crate::commands_nav::dispatch_bnext(state)
        }
        ":bp" | ":bprev" | ":bprevious" => {
            crate::commands_nav::dispatch_bprev(state)
        }
        ":ls" | ":buffers" => {
            crate::commands_nav::dispatch_list_buffers(state)
        }
        ":set" => match args {
            Some(opt) => crate::commands_config::dispatch_set_option(state, opt),
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
            crate::commands_config::dispatch_map_command(state, command, args);
        }
        ":unmap" | ":nunmap" | ":iunmap" | ":vunmap" | ":cunmap"
        | ":ounmap" => {
            crate::commands_config::dispatch_unmap_command(state, command, args);
        }
        ":mapclear" | ":nmapclear" | ":imapclear" | ":vmapclear" => {
            crate::commands_config::dispatch_mapclear(state, command);
        }
        ":autocmd" | ":au" => {
            crate::commands_config::dispatch_autocmd(state, args);
        }
        ":autocmd!" | ":au!" => {
            state.autocmds.clear_all();
            state.message =
                Some("all autocommands cleared".into());
        }
        ":filetype" | ":ft" => {
            crate::commands_config::dispatch_filetype(state, args);
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
                crate::commands_nav::dispatch_unknown(
                    state, trimmed, command,
                );
            }
        }
    }
}

// Navigation/session commands moved to commands_nav.rs
// Config commands moved to commands_config.rs
// Substitute/global commands moved to commands_substitute.rs
