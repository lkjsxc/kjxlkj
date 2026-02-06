//! Configuration-related commands: :set, :map, :autocmd, :filetype.

use crate::EditorState;

/// Dispatch :set option.
pub(crate) fn dispatch_set_option(
    state: &mut EditorState,
    opt: &str,
) {
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
                    state.message = Some(format!(
                        "unknown option: {}",
                        key
                    ));
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
            state.message =
                Some("relativenumber on".into());
        }
        "norelativenumber" | "nornu" => {
            state.options.relative_number = false;
            state.message =
                Some("relativenumber off".into());
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
            state.message =
                Some("ignorecase off".into());
        }
        "smartcase" | "scs" => {
            state.options.smartcase = true;
            state.message = Some("smartcase on".into());
        }
        "nosmartcase" | "noscs" => {
            state.options.smartcase = false;
            state.message =
                Some("smartcase off".into());
        }
        "hlsearch" | "hls" => {
            state.options.hlsearch = true;
            state.message = Some("hlsearch on".into());
        }
        "nohlsearch" | "nohls" => {
            state.options.hlsearch = false;
            state.message =
                Some("hlsearch off".into());
        }
        "incsearch" | "is" => {
            state.options.incsearch = true;
            state.message = Some("incsearch on".into());
        }
        "noincsearch" | "nois" => {
            state.options.incsearch = false;
            state.message =
                Some("incsearch off".into());
        }
        "expandtab" | "et" => {
            state.options.expandtab = true;
            state.message = Some("expandtab on".into());
        }
        "noexpandtab" | "noet" => {
            state.options.expandtab = false;
            state.message =
                Some("expandtab off".into());
        }
        "autoindent" | "ai" => {
            state.options.autoindent = true;
            state.message = Some("autoindent on".into());
        }
        "noautoindent" | "noai" => {
            state.options.autoindent = false;
            state.message =
                Some("autoindent off".into());
        }
        "smartindent" | "si" => {
            state.options.smartindent = true;
            state.message =
                Some("smartindent on".into());
        }
        "nosmartindent" | "nosi" => {
            state.options.smartindent = false;
            state.message =
                Some("smartindent off".into());
        }
        _ => dispatch_set_query(state, opt),
    }
}

fn dispatch_set_query(
    state: &mut EditorState,
    opt: &str,
) {
    if opt.ends_with('?') {
        let name = &opt[..opt.len() - 1];
        let val = match name {
            "number" | "nu" => {
                format!("{}", state.options.number)
            }
            "wrap" => format!("{}", state.options.wrap),
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
        state.message = Some(format!("{}={}", name, val));
    } else {
        state.message =
            Some(format!("unknown option: {}", opt));
    }
}

/// Get the MappingMode for a map-family command.
pub(crate) fn map_mode(
    command: &str,
) -> crate::mappings::MappingMode {
    use crate::mappings::MappingMode;
    match command {
        ":nmap" | ":nnoremap" | ":nunmap"
        | ":nmapclear" => MappingMode::Normal,
        ":imap" | ":inoremap" | ":iunmap"
        | ":imapclear" => MappingMode::Insert,
        ":vmap" | ":vnoremap" | ":vunmap"
        | ":vmapclear" => MappingMode::Visual,
        ":cmap" | ":cnoremap" | ":cunmap" => {
            MappingMode::Command
        }
        ":omap" | ":onoremap" | ":ounmap" => {
            MappingMode::OperatorPending
        }
        _ => MappingMode::All,
    }
}

pub(crate) fn dispatch_map_command(
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
            } else if let Some(m) =
                state.mappings.get(mode, a)
            {
                state.message = Some(format!(
                    "{} → {}",
                    m.lhs, m.rhs
                ));
            } else {
                state.message =
                    Some(format!("No mapping: {}", a));
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
                state.message = Some(lines.join(" | "));
            }
        }
    }
}

pub(crate) fn dispatch_unmap_command(
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
        state.message = Some("Usage: :unmap <key>".into());
    }
}

pub(crate) fn dispatch_mapclear(
    state: &mut EditorState,
    command: &str,
) {
    let mode = map_mode(command);
    state.mappings.clear(mode);
    state.message = Some("mappings cleared".into());
}

pub(crate) fn dispatch_autocmd(
    state: &mut EditorState,
    args: Option<&str>,
) {
    match args {
        Some(a) => {
            let parts: Vec<&str> =
                a.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                if let Some(event) =
                    crate::autocommands::parse_event(
                        parts[0],
                    )
                {
                    state.autocmds.add(
                        event,
                        parts[1],
                        parts[2],
                        None,
                    );
                    state.message = Some(format!(
                        "autocmd {:?} {} {}",
                        event, parts[1], parts[2]
                    ));
                } else {
                    state.message = Some(format!(
                        "unknown event: {}",
                        parts[0]
                    ));
                }
            } else {
                state.message =
                    Some(state.autocmds.display());
            }
        }
        None => {
            state.message = Some(state.autocmds.display());
        }
    }
}

pub(crate) fn dispatch_filetype(
    state: &mut EditorState,
    args: Option<&str>,
) {
    match args {
        Some(ft) => {
            if let Some(buf) = state.active_buffer_mut() {
                let lang =
                    kjxlkj_core_types::LanguageId::from_extension(
                        ft,
                    );
                buf.language = lang;
                state.message =
                    Some(format!("filetype={}", lang));
            }
        }
        None => {
            if let Some(buf) = state.active_buffer() {
                state.message = Some(format!(
                    "filetype={}",
                    buf.language
                ));
            }
        }
    }
}
