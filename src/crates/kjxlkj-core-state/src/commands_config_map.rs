//! Mapping, autocmd, and filetype commands.

use crate::EditorState;

/// Get the MappingMode for a map-family command.
pub(crate) fn map_mode(cmd: &str) -> crate::mappings::MappingMode {
    use crate::mappings::MappingMode;
    match cmd {
        ":nmap" | ":nnoremap" | ":nunmap" | ":nmapclear" => MappingMode::Normal,
        ":imap" | ":inoremap" | ":iunmap" | ":imapclear" => MappingMode::Insert,
        ":vmap" | ":vnoremap" | ":vunmap" | ":vmapclear" => MappingMode::Visual,
        ":cmap" | ":cnoremap" | ":cunmap" => MappingMode::Command,
        ":omap" | ":onoremap" | ":ounmap" => MappingMode::OperatorPending,
        _ => MappingMode::All,
    }
}

pub(crate) fn dispatch_map_command(
    state: &mut EditorState, command: &str, args: Option<&str>,
) {
    let mode = map_mode(command);
    let recursive = !command.contains("noremap");
    match args {
        Some(a) => {
            if let Some((lhs, rhs)) = a.split_once(' ') {
                state.mappings.add(mode, lhs.trim(), rhs.trim(), recursive);
                state.message = Some(format!("mapped {} → {}", lhs.trim(), rhs.trim()));
            } else if let Some(m) = state.mappings.get(mode, a) {
                state.message = Some(format!("{} → {}", m.lhs, m.rhs));
            } else {
                state.message = Some(format!("No mapping: {}", a));
            }
        }
        None => {
            let all = state.mappings.list(mode);
            if all.is_empty() {
                state.message = Some("No mappings defined".into());
            } else {
                let lines: Vec<String> = all.iter().map(|m| format!("{} → {}", m.lhs, m.rhs)).collect();
                state.message = Some(lines.join(" | "));
            }
        }
    }
}

pub(crate) fn dispatch_unmap_command(
    state: &mut EditorState, command: &str, args: Option<&str>,
) {
    let mode = map_mode(command);
    if let Some(lhs) = args {
        if state.mappings.remove(mode, lhs.trim()) {
            state.message = Some(format!("unmapped {}", lhs.trim()));
        } else {
            state.message = Some(format!("No mapping: {}", lhs.trim()));
        }
    } else {
        state.message = Some("Usage: :unmap <key>".into());
    }
}

pub(crate) fn dispatch_mapclear(state: &mut EditorState, command: &str) {
    let mode = map_mode(command);
    state.mappings.clear(mode);
    state.message = Some("mappings cleared".into());
}

pub(crate) fn dispatch_autocmd(state: &mut EditorState, args: Option<&str>) {
    match args {
        Some(a) => {
            let parts: Vec<&str> = a.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                if let Some(event) = crate::autocommands::parse_event(parts[0]) {
                    state.autocmds.add(event, parts[1], parts[2], None);
                    state.message = Some(format!("autocmd {:?} {} {}", event, parts[1], parts[2]));
                } else {
                    state.message = Some(format!("unknown event: {}", parts[0]));
                }
            } else {
                state.message = Some(state.autocmds.display());
            }
        }
        None => { state.message = Some(state.autocmds.display()); }
    }
}

pub(crate) fn dispatch_filetype(state: &mut EditorState, args: Option<&str>) {
    match args {
        Some(ft) => {
            if let Some(buf) = state.active_buffer_mut() {
                let lang = kjxlkj_core_types::LanguageId::from_extension(ft);
                buf.language = lang;
                state.message = Some(format!("filetype={}", lang));
            }
        }
        None => {
            if let Some(buf) = state.active_buffer() {
                state.message = Some(format!("filetype={}", buf.language));
            }
        }
    }
}
