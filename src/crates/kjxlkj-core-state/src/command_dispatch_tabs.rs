//! Tab and miscellaneous command dispatch.

use kjxlkj_core_types::Action;

/// Dispatch tab, resize, and other miscellaneous commands.
pub fn dispatch_tabs_misc(
    name: &str,
    args: &str,
) -> Option<Action> {
    match name {
        "tab" | "tabnew" | "tabe" | "tabedit" => {
            if args.is_empty() {
                Some(Action::TabNew(None))
            } else {
                Some(Action::TabNew(Some(
                    args.into(),
                )))
            }
        }
        "tabclose" | "tabc" => {
            Some(Action::TabClose)
        }
        "tabonly" | "tabo" => {
            Some(Action::TabOnly)
        }
        "tabnext" | "tabn" => {
            Some(Action::TabNext)
        }
        "tabprevious" | "tabprev" | "tabp" => {
            Some(Action::TabPrev)
        }
        "tabfirst" | "tabfir" | "tabrewind" => {
            Some(Action::TabFirst)
        }
        "tablast" | "tabl" => {
            Some(Action::TabLast)
        }
        "tabmove" | "tabm" => {
            Some(Action::TabMove(args.into()))
        }
        "resize" => {
            Some(Action::ResizeCmd(args.into()))
        }
        "vertical" => dispatch_vertical(args),
        "execute" | "exe" => {
            Some(Action::ExecuteExpr(args.into()))
        }
        "cq" | "cquit" => {
            Some(Action::ForceQuit)
        }
        "SessionSave" => Some(Action::SessionSave),
        "SessionLoad" => Some(Action::SessionLoad),
        _ => dispatch_line_number(name),
    }
}

fn dispatch_vertical(args: &str) -> Option<Action> {
    if args.starts_with("resize") {
        let rest = args
            .strip_prefix("resize")
            .unwrap_or("")
            .trim();
        Some(Action::ResizeCmd(
            format!("v {}", rest),
        ))
    } else if args.starts_with("split") {
        let rest = args
            .strip_prefix("split")
            .unwrap_or("")
            .trim();
        if rest.is_empty() {
            Some(Action::SplitVertical)
        } else {
            Some(Action::VsplitOpen(rest.into()))
        }
    } else if args.starts_with("new") {
        Some(Action::NewVsplit)
    } else {
        Some(Action::Nop)
    }
}

fn dispatch_line_number(
    name: &str,
) -> Option<Action> {
    if let Ok(line) = name.parse::<usize>() {
        Some(Action::MoveCursor(
            kjxlkj_core_types::Motion::GotoLine(
                line.saturating_sub(1),
            ),
            1,
        ))
    } else {
        None
    }
}
