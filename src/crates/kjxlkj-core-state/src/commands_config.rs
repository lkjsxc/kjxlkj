//! Configuration-related commands: :set, :map, :autocmd, :filetype.

use crate::EditorState;

/// Set a boolean option to true and return display name.
fn set_bool(opt: &mut bool, name: &str, val: bool) -> String {
    *opt = val;
    format!("{} {}", name, if val { "on" } else { "off" })
}

/// Dispatch :set option.
pub(crate) fn dispatch_set_option(state: &mut EditorState, opt: &str) {
    // Handle value assignments like tabstop=8
    if let Some((key, val)) = opt.split_once('=') {
        if let Ok(n) = val.parse::<usize>() {
            let msg = match key {
                "tabstop" | "ts" => { state.options.tabstop = n; format!("tabstop={}", n) }
                "shiftwidth" | "sw" => { state.options.shiftwidth = n; format!("shiftwidth={}", n) }
                "scrolloff" | "so" => { state.options.scrolloff = n; format!("scrolloff={}", n) }
                "sidescrolloff" | "siso" => { state.options.sidescrolloff = n; format!("sidescrolloff={}", n) }
                _ => format!("unknown option: {}", key),
            };
            state.message = Some(msg);
        } else { state.message = Some(format!("invalid value: {}", val)); }
        return;
    }
    // Handle boolean toggles
    let msg = match opt {
        "number" | "nu" => set_bool(&mut state.options.number, "number", true),
        "nonumber" | "nonu" => set_bool(&mut state.options.number, "number", false),
        "relativenumber" | "rnu" => set_bool(&mut state.options.relative_number, "relativenumber", true),
        "norelativenumber" | "nornu" => set_bool(&mut state.options.relative_number, "relativenumber", false),
        "wrap" => set_bool(&mut state.options.wrap, "wrap", true),
        "nowrap" => set_bool(&mut state.options.wrap, "wrap", false),
        "ignorecase" | "ic" => set_bool(&mut state.options.ignorecase, "ignorecase", true),
        "noignorecase" | "noic" => set_bool(&mut state.options.ignorecase, "ignorecase", false),
        "smartcase" | "scs" => set_bool(&mut state.options.smartcase, "smartcase", true),
        "nosmartcase" | "noscs" => set_bool(&mut state.options.smartcase, "smartcase", false),
        "hlsearch" | "hls" => set_bool(&mut state.options.hlsearch, "hlsearch", true),
        "nohlsearch" | "nohls" => set_bool(&mut state.options.hlsearch, "hlsearch", false),
        "incsearch" | "is" => set_bool(&mut state.options.incsearch, "incsearch", true),
        "noincsearch" | "nois" => set_bool(&mut state.options.incsearch, "incsearch", false),
        "expandtab" | "et" => set_bool(&mut state.options.expandtab, "expandtab", true),
        "noexpandtab" | "noet" => set_bool(&mut state.options.expandtab, "expandtab", false),
        "autoindent" | "ai" => set_bool(&mut state.options.autoindent, "autoindent", true),
        "noautoindent" | "noai" => set_bool(&mut state.options.autoindent, "autoindent", false),
        "smartindent" | "si" => set_bool(&mut state.options.smartindent, "smartindent", true),
        "nosmartindent" | "nosi" => set_bool(&mut state.options.smartindent, "smartindent", false),
        "autopairs" => set_bool(&mut state.options.autopairs, "autopairs", true),
        "noautopairs" => set_bool(&mut state.options.autopairs, "autopairs", false),
        "list" => set_bool(&mut state.options.list, "list", true),
        "nolist" => set_bool(&mut state.options.list, "list", false),
        "cursorline" | "cul" => set_bool(&mut state.options.cursorline, "cursorline", true),
        "nocursorline" | "nocul" => set_bool(&mut state.options.cursorline, "cursorline", false),
        "cursorcolumn" | "cuc" => set_bool(&mut state.options.cursorcolumn, "cursorcolumn", true),
        "nocursorcolumn" | "nocuc" => set_bool(&mut state.options.cursorcolumn, "cursorcolumn", false),
        "showmode" | "smd" => set_bool(&mut state.options.showmode, "showmode", true),
        "noshowmode" | "nosmd" => set_bool(&mut state.options.showmode, "showmode", false),
        "showcmd" | "sc" => set_bool(&mut state.options.showcmd, "showcmd", true),
        "noshowcmd" | "nosc" => set_bool(&mut state.options.showcmd, "showcmd", false),
        "hidden" | "hid" => set_bool(&mut state.options.hidden, "hidden", true),
        "nohidden" | "nohid" => set_bool(&mut state.options.hidden, "hidden", false),
        _ => return dispatch_set_query(state, opt),
    };
    state.message = Some(msg);
}

fn dispatch_set_query(state: &mut EditorState, opt: &str) {
    if opt.ends_with('?') {
        let name = &opt[..opt.len() - 1];
        let val = match name {
            "number" | "nu" => format!("{}", state.options.number),
            "wrap" => format!("{}", state.options.wrap),
            "tabstop" | "ts" => format!("{}", state.options.tabstop),
            "shiftwidth" | "sw" => format!("{}", state.options.shiftwidth),
            "scrolloff" | "so" => format!("{}", state.options.scrolloff),
            "list" => format!("{}", state.options.list),
            "cursorline" | "cul" => format!("{}", state.options.cursorline),
            "showmode" | "smd" => format!("{}", state.options.showmode),
            "showcmd" | "sc" => format!("{}", state.options.showcmd),
            "hidden" | "hid" => format!("{}", state.options.hidden),
            _ => "unknown".into(),
        };
        state.message = Some(format!("{}={}", name, val));
    } else {
        state.message = Some(format!("unknown option: {}", opt));
    }
}
