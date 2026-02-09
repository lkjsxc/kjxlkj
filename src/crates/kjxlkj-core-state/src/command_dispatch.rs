//! Command dispatch: parse and execute ex commands.

use kjxlkj_core_types::Action;

use crate::command_dispatch_ext::dispatch_extended;
use crate::command_parse::split_command;

/// Parse and dispatch an ex command string.
pub fn dispatch_command(cmd: &str) -> Option<Action> {
    let cmd = cmd.trim();
    if cmd.is_empty() {
        return None;
    }
    if cmd.starts_with('!') {
        return Some(Action::ShellCommand(cmd[1..].to_string()));
    }
    let (name, args) = split_command(cmd);
    let force = name.ends_with('!');
    let name_clean = if force {
        &name[..name.len() - 1]
    } else {
        &name
    };
    dispatch_name(name_clean, args, force, cmd)
}

fn dispatch_name(name: &str, args: &str, force: bool, cmd: &str) -> Option<Action> {
    match name {
        "q" | "quit" => dispatch_quit(force),
        "qa" | "qall" => Some(Action::QuitAll),
        "w" | "write" => Some(Action::Write),
        "wq" | "x" | "xit" => Some(Action::WriteQuit),
        "wa" | "wall" => Some(Action::WriteAll),
        "wqa" | "xall" => Some(Action::WriteAllQuit),
        "e" | "edit" => dispatch_edit(args),
        "bn" | "bnext" => Some(Action::NextBuffer),
        "bp" | "bprevious" | "bprev" => Some(Action::PrevBuffer),
        "bd" | "bdelete" => Some(Action::DeleteBuffer),
        "b" | "buffer" => dispatch_buffer(args),
        "sp" | "split" => dispatch_split(args),
        "vs" | "vsplit" => dispatch_vsplit(args),
        "close" => Some(Action::CloseWindow),
        "on" | "only" => Some(Action::OnlyWindow),
        "hide" => Some(Action::HideWindow),
        "terminal" | "term" => Some(Action::SpawnTerminal),
        "noh" | "nohlsearch" => Some(Action::Nop),
        "ls" | "buffers" | "files" => Some(Action::Nop),
        "s" | "substitute" => Some(Action::Substitute(args.into())),
        "g" | "global" => Some(Action::GlobalCommand(args.into())),
        "v" | "vglobal" => Some(Action::VglobalCommand(args.into())),
        "sort" => Some(Action::SortLines(args.into())),
        _ => dispatch_extended(name, args, cmd),
    }
}

fn dispatch_quit(force: bool) -> Option<Action> {
    if force {
        Some(Action::ForceQuit)
    } else {
        Some(Action::Quit)
    }
}

fn dispatch_edit(args: &str) -> Option<Action> {
    if !args.is_empty() {
        Some(Action::OpenFile(std::path::PathBuf::from(args)))
    } else {
        None
    }
}

fn dispatch_buffer(args: &str) -> Option<Action> {
    if !args.is_empty() {
        Some(Action::SwitchBuffer(args.into()))
    } else {
        None
    }
}

fn dispatch_split(args: &str) -> Option<Action> {
    if !args.is_empty() {
        Some(Action::SplitOpen(args.into()))
    } else {
        Some(Action::SplitHorizontal)
    }
}

fn dispatch_vsplit(args: &str) -> Option<Action> {
    if !args.is_empty() {
        Some(Action::VsplitOpen(args.into()))
    } else {
        Some(Action::SplitVertical)
    }
}
