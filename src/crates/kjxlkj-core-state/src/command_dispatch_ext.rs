//! Extended command dispatch (overflow).

use kjxlkj_core_types::Action;

use crate::command_dispatch_tabs::dispatch_tabs_misc;
use crate::command_parse::full_with_range;

/// Dispatch commands not covered by the primary match.
pub fn dispatch_extended(
    name: &str,
    args: &str,
    cmd: &str,
) -> Option<Action> {
    match name {
        "d" | "delete" => {
            Some(Action::RangeDelete(
                full_with_range(cmd),
            ))
        }
        "y" | "yank" => {
            Some(Action::RangeYank(
                full_with_range(cmd),
            ))
        }
        "t" | "copy" => {
            Some(Action::RangeCopy(
                full_with_range(cmd),
            ))
        }
        "m" | "move" => {
            Some(Action::RangeMove(
                full_with_range(cmd),
            ))
        }
        "normal" | "norm" => {
            Some(Action::RangeNormal(
                full_with_range(cmd),
            ))
        }
        "set" => {
            Some(Action::SetOption(args.into()))
        }
        "reg" | "registers" | "marks"
        | "history" => Some(Action::Nop),
        "read" | "r" => dispatch_read(args),
        "source" | "so" => dispatch_source(args),
        "filetype" | "ft" | "autocmd" | "au" => {
            Some(Action::Nop)
        }
        "map" | "noremap" | "nmap" | "nnoremap"
        | "imap" | "inoremap" | "vmap" | "vnoremap"
        | "cmap" | "cnoremap" | "tmap" | "tnoremap"
        | "omap" | "onoremap" => {
            Some(Action::MapCommand(
                name.into(),
                args.into(),
            ))
        }
        "unmap" | "nunmap" | "iunmap" | "vunmap"
        | "cunmap" | "tunmap" | "ounmap" => {
            Some(Action::UnmapCommand(
                name.into(),
                args.into(),
            ))
        }
        "command" => {
            Some(Action::UserCommand(args.into()))
        }
        "cnext" | "cn" | "cprev" | "cp"
        | "cprevious" | "cfirst" | "cfir"
        | "clast" | "cla" | "copen"
        | "cclose" | "ccl" => Some(Action::Nop),
        "grep" | "vimgrep" => dispatch_grep(args),
        "make" => dispatch_make(args),
        "new" => dispatch_new(args),
        "vnew" => dispatch_vnew(args),
        _ => dispatch_tabs_misc(name, args),
    }
}

fn dispatch_read(args: &str) -> Option<Action> {
    if !args.is_empty() {
        Some(Action::ReadFile(args.into()))
    } else {
        Some(Action::Nop)
    }
}

fn dispatch_source(args: &str) -> Option<Action> {
    if !args.is_empty() {
        Some(Action::SourceFile(args.into()))
    } else {
        Some(Action::Nop)
    }
}

fn dispatch_grep(args: &str) -> Option<Action> {
    if !args.is_empty() {
        Some(Action::ShellCommand(
            format!("grep -rn {}", args),
        ))
    } else {
        Some(Action::Nop)
    }
}

fn dispatch_make(args: &str) -> Option<Action> {
    Some(Action::ShellCommand(if args.is_empty() {
        "make".into()
    } else {
        format!("make {}", args)
    }))
}

fn dispatch_new(args: &str) -> Option<Action> {
    if !args.is_empty() {
        Some(Action::SplitOpen(args.into()))
    } else {
        Some(Action::NewSplit)
    }
}

fn dispatch_vnew(args: &str) -> Option<Action> {
    if !args.is_empty() {
        Some(Action::VsplitOpen(args.into()))
    } else {
        Some(Action::NewVsplit)
    }
}
