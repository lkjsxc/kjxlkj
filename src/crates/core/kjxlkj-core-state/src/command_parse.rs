//! Ex command parser. Tokenizes `:` command strings
//! into typed Actions.
//! See /docs/spec/commands/syntax.md and essential.md.

use kjxlkj_core_types::Action;

/// Parse an ex command string (without leading `:`) into a typed Action.
pub fn parse_ex_command(input: &str) -> Action {
    let input = input.trim();
    if input.is_empty() { return Action::Noop; }
    let (cmd, bang) = if input.ends_with('!') { (&input[..input.len()-1], true) } else { (input, false) };
    let (name, args) = split_cmd_args(cmd);
    match_command(name, args, bang)
}

fn split_cmd_args(s: &str) -> (&str, &str) {
    let i = s.bytes().take_while(|b| b.is_ascii_alphabetic()).count();
    if i == 0 { return (s, ""); }
    (&s[..i], s[i..].trim_start())
}

fn match_command(name: &str, args: &str, bang: bool) -> Action {
    match name {
        "q" | "quit" | "qa" | "qall" => if bang { Action::ForceQuit } else { Action::Quit },
        "cq" => Action::ForceQuit,
        "w" | "write" | "wa" | "wall" => Action::Write,
        "wq" | "x" | "xit" | "exi" | "exit" => Action::WriteQuit,
        "e" | "edit" => if args.is_empty() { Action::Noop } else { Action::OpenFile(args.to_string()) },
        "bn" | "bnext" => Action::NextBuffer,
        "bp" | "bprevious" | "bprev" => Action::PreviousBuffer,
        "bd" | "bdelete" => Action::DeleteBuffer,
        "bf" | "bfirst" => Action::FirstBuffer,
        "bl" | "blast" => Action::LastBuffer,
        "b" | "buffer" => args.parse::<u64>().map_or(Action::Noop, |n| Action::SwitchBuffer(kjxlkj_core_types::BufferId(n))),
        "sp" | "split" | "new" => Action::SplitHorizontal,
        "vsp" | "vsplit" | "vnew" => Action::SplitVertical,
        "clo" | "close" => Action::CloseWindow,
        "on" | "only" => Action::WindowOnly,
        "Explorer" | "Ex" => Action::OpenExplorer,
        "ExplorerClose" => Action::CloseExplorer,
        "terminal" | "term" => Action::OpenTerminal,
        // Register display.
        "registers" | "reg" | "display" | "di" => Action::ShowRegisters,
        // Buffer listing.
        "ls" | "buffers" => Action::ListBuffers,
        // Search highlight clear.
        "nohlsearch" | "noh" => Action::ClearSearchHighlight,
        // Tab commands.
        "tabnew" | "tabe" | "tabedit" => Action::TabNew(if args.is_empty() { None } else { Some(args.to_string()) }),
        "tabclose" | "tabc" => if bang { Action::TabCloseForce } else { Action::TabClose },
        "tabonly" | "tabo" => Action::TabOnly,
        "tabnext" | "tabn" => if args.is_empty() { Action::TabNext } else { parse_tab_goto(args) },
        "tabprevious" | "tabprev" | "tabp" | "tabNext" | "tabN" => Action::TabPrev,
        "tabfirst" | "tabfir" | "tabrewind" | "tabr" => Action::TabFirst,
        "tablast" | "tabl" => Action::TabLast,
        "tabmove" | "tabm" => parse_tab_move(args),
        // Zoom.
        "ZoomToggle" => Action::ZoomToggle,
        "ZoomHeight" => Action::WindowMaxHeight,
        "ZoomWidth" => Action::WindowMaxWidth,
        // Set option.
        "set" | "se" => parse_set_option(args),
        "setlocal" => parse_set_option(args),
        _ => Action::Noop,
    }
}

fn parse_tab_goto(args: &str) -> Action {
    args.trim().parse::<usize>().map_or(Action::Noop, Action::TabGoto)
}

fn parse_tab_move(args: &str) -> Action {
    let a = args.trim();
    if a.is_empty() || a == "$" { return Action::TabMove(i32::MAX); }
    if let Some(n) = a.strip_prefix('+') { return Action::TabMove(n.parse::<i32>().unwrap_or(1)); }
    if let Some(n) = a.strip_prefix('-') { return Action::TabMove(-(n.parse::<i32>().unwrap_or(1))); }
    a.parse::<i32>().map_or(Action::Noop, |n| Action::TabMove(n - (i32::MAX / 2)))
}

fn parse_set_option(args: &str) -> Action {
    let args = args.trim();
    if args.is_empty() { return Action::Noop; }
    if let Some((k, v)) = args.split_once('=') {
        return Action::SetOption(k.to_string(), v.to_string());
    }
    if let Some(name) = args.strip_prefix("no") {
        return Action::SetOption(name.to_string(), "false".to_string());
    }
    Action::SetOption(args.to_string(), "true".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_quit_and_write() {
        assert_eq!(parse_ex_command("q"), Action::Quit);
        assert_eq!(parse_ex_command("quit"), Action::Quit);
        assert_eq!(parse_ex_command("q!"), Action::ForceQuit);
        assert_eq!(parse_ex_command("w"), Action::Write);
    }

    #[test]
    fn parse_wq() {
        assert_eq!(parse_ex_command("wq"), Action::WriteQuit);
    }

    #[test]
    fn parse_split() {
        assert_eq!(parse_ex_command("sp"), Action::SplitHorizontal);
        assert_eq!(parse_ex_command("vsp"), Action::SplitVertical);
    }

    #[test]
    fn parse_edit_file() {
        match parse_ex_command("e myfile.txt") {
            Action::OpenFile(f) => assert_eq!(f, "myfile.txt"),
            _ => panic!("expected OpenFile"),
        }
    }

    #[test]
    fn parse_buffer_cmds() {
        assert_eq!(parse_ex_command("bn"), Action::NextBuffer);
        assert_eq!(parse_ex_command("bp"), Action::PreviousBuffer);
    }

    #[test]
    fn parse_empty() {
        assert_eq!(parse_ex_command(""), Action::Noop);
        assert_eq!(parse_ex_command("  "), Action::Noop);
    }

    #[test]
    fn parse_registers() {
        assert_eq!(parse_ex_command("registers"), Action::ShowRegisters);
        assert_eq!(parse_ex_command("reg"), Action::ShowRegisters);
        assert_eq!(parse_ex_command("display"), Action::ShowRegisters);
        assert_eq!(parse_ex_command("di"), Action::ShowRegisters);
    }

    #[test]
    fn parse_nohlsearch() {
        assert_eq!(parse_ex_command("nohlsearch"), Action::ClearSearchHighlight);
        assert_eq!(parse_ex_command("noh"), Action::ClearSearchHighlight);
    }

    #[test]
    fn parse_set_option_forms() {
        assert_eq!(parse_ex_command("set ignorecase"), Action::SetOption("ignorecase".into(), "true".into()));
        assert_eq!(parse_ex_command("set noignorecase"), Action::SetOption("ignorecase".into(), "false".into()));
        assert_eq!(parse_ex_command("set tabstop=4"), Action::SetOption("tabstop".into(), "4".into()));
    }
}
