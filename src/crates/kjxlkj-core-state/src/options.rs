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

// Re-export apply logic from dedicated module.
pub use crate::options_apply::apply_set_action;
