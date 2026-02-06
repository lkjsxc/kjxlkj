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
                "sidescrolloff" | "siso" => {
                    state.options.sidescrolloff = n;
                    state.message =
                        Some(format!("sidescrolloff={}", n));
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
        "autopairs" => {
            state.options.autopairs = true;
            state.message = Some("autopairs on".into());
        }
        "noautopairs" => {
            state.options.autopairs = false;
            state.message = Some("autopairs off".into());
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
