//! Editor options and :set command parsing.

use serde::{Deserialize, Serialize};

/// Global editor options (all Vim-compatible defaults).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorOptions {
    pub number: bool,
    pub relative_number: bool,
    pub wrap: bool,
    pub tabstop: usize,
    pub shiftwidth: usize,
    pub expandtab: bool,
    pub scrolloff: usize,
    pub sidescrolloff: usize,
    pub ignorecase: bool,
    pub smartcase: bool,
    pub hlsearch: bool,
    pub incsearch: bool,
    pub autoindent: bool,
    pub smartindent: bool,
    pub autopairs: bool,
    pub syntax: bool,
    pub ruler: bool,
    pub showmode: bool,
    pub showcmd: bool,
    pub laststatus: u8,
    pub mouse: bool,
    pub cursorline: bool,
    pub cursorcolumn: bool,
}

impl Default for EditorOptions {
    fn default() -> Self {
        Self {
            number: true,
            relative_number: false,
            wrap: true,
            tabstop: 8,
            shiftwidth: 8,
            expandtab: false,
            scrolloff: 5,
            sidescrolloff: 0,
            ignorecase: false,
            smartcase: false,
            hlsearch: true,
            incsearch: true,
            autoindent: true,
            smartindent: false,
            autopairs: true,
            syntax: true,
            ruler: true,
            showmode: true,
            showcmd: true,
            laststatus: 2,
            mouse: false,
            cursorline: false,
            cursorcolumn: false,
        }
    }
}

/// Action resulting from parsing a :set argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetAction {
    ShowAll,
    Query(String),
    SetBool(String, bool),
    SetInt(String, usize),
    SetStr(String, String),
    Invalid(String),
}

/// Parse a single :set argument string.
pub fn parse_set_arg(arg: &str) -> SetAction {
    let arg = arg.trim();
    if arg.is_empty() || arg == "all" {
        return SetAction::ShowAll;
    }
    if let Some(name) = arg.strip_prefix("no") {
        if is_bool_option(name) {
            return SetAction::SetBool(name.to_string(), false);
        }
    }
    if let Some(name) = arg.strip_suffix('?') {
        return SetAction::Query(name.to_string());
    }
    if let Some((name, value)) = arg.split_once('=') {
        if let Ok(n) = value.parse::<usize>() {
            return SetAction::SetInt(name.to_string(), n);
        }
        return SetAction::SetStr(name.to_string(), value.to_string());
    }
    if is_bool_option(arg) {
        return SetAction::SetBool(arg.to_string(), true);
    }
    SetAction::Invalid(arg.to_string())
}

fn is_bool_option(name: &str) -> bool {
    matches!(
        name,
        "number"
            | "relativenumber"
            | "wrap"
            | "expandtab"
            | "ignorecase"
            | "smartcase"
            | "hlsearch"
            | "incsearch"
            | "autoindent"
            | "smartindent"
            | "autopairs"
            | "syntax"
            | "ruler"
            | "showmode"
            | "showcmd"
            | "mouse"
            | "cursorline"
            | "cursorcolumn"
    )
}

/// Apply a set action to editor options. Returns Ok(message) or Err(error).
pub fn apply_set_action(opts: &mut EditorOptions, action: SetAction) -> Result<String, String> {
    match action {
        SetAction::ShowAll => Ok(format_all_options(opts)),
        SetAction::Query(ref name) => query_option(opts, name),
        SetAction::SetBool(ref name, val) => set_bool(opts, name, val),
        SetAction::SetInt(ref name, val) => set_int(opts, name, val),
        SetAction::SetStr(ref name, _) => Err(format!("unknown option: {name}")),
        SetAction::Invalid(ref s) => Err(format!("invalid argument: {s}")),
    }
}

fn set_bool(opts: &mut EditorOptions, name: &str, val: bool) -> Result<String, String> {
    match name {
        "number" => opts.number = val,
        "relativenumber" => opts.relative_number = val,
        "wrap" => opts.wrap = val,
        "expandtab" => opts.expandtab = val,
        "ignorecase" => opts.ignorecase = val,
        "smartcase" => opts.smartcase = val,
        "hlsearch" => opts.hlsearch = val,
        "incsearch" => opts.incsearch = val,
        "autoindent" => opts.autoindent = val,
        "smartindent" => opts.smartindent = val,
        "autopairs" => opts.autopairs = val,
        "syntax" => opts.syntax = val,
        "ruler" => opts.ruler = val,
        "showmode" => opts.showmode = val,
        "showcmd" => opts.showcmd = val,
        "mouse" => opts.mouse = val,
        "cursorline" => opts.cursorline = val,
        "cursorcolumn" => opts.cursorcolumn = val,
        _ => return Err(format!("unknown bool option: {name}")),
    }
    Ok(String::new())
}

fn set_int(opts: &mut EditorOptions, name: &str, val: usize) -> Result<String, String> {
    match name {
        "tabstop" => opts.tabstop = val,
        "shiftwidth" => opts.shiftwidth = val,
        "scrolloff" => opts.scrolloff = val,
        "sidescrolloff" => opts.sidescrolloff = val,
        "laststatus" => opts.laststatus = val as u8,
        _ => return Err(format!("unknown int option: {name}")),
    }
    Ok(String::new())
}

fn query_option(opts: &EditorOptions, name: &str) -> Result<String, String> {
    let val = match name {
        "number" => format!("{}", opts.number),
        "tabstop" => format!("{}", opts.tabstop),
        "shiftwidth" => format!("{}", opts.shiftwidth),
        "expandtab" => format!("{}", opts.expandtab),
        "scrolloff" => format!("{}", opts.scrolloff),
        "wrap" => format!("{}", opts.wrap),
        _ => return Err(format!("unknown option: {name}")),
    };
    Ok(format!("{name}={val}"))
}

fn format_all_options(opts: &EditorOptions) -> String {
    format!(
        "number={} wrap={} ts={} sw={} et={}",
        opts.number, opts.wrap, opts.tabstop, opts.shiftwidth, opts.expandtab
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_set_bool() {
        assert_eq!(
            parse_set_arg("number"),
            SetAction::SetBool("number".into(), true)
        );
        assert_eq!(
            parse_set_arg("nonumber"),
            SetAction::SetBool("number".into(), false)
        );
    }

    #[test]
    fn parse_set_int() {
        assert_eq!(
            parse_set_arg("tabstop=4"),
            SetAction::SetInt("tabstop".into(), 4)
        );
    }

    #[test]
    fn parse_query() {
        assert_eq!(parse_set_arg("number?"), SetAction::Query("number".into()));
    }

    #[test]
    fn apply_bool_option() {
        let mut opts = EditorOptions::default();
        apply_set_action(&mut opts, SetAction::SetBool("wrap".into(), false)).unwrap();
        assert!(!opts.wrap);
    }
}
