//! Extended Ex command parsing: additional commands and complex parsers.

use kjxlkj_core_types::EditorError;

use crate::commands::ExCommand;

/// Parse extended commands that didn't match in the main parser.
pub fn parse_extended(cmd: &str, args: Option<&str>) -> Result<ExCommand, EditorError> {
    let arg_s = args.unwrap_or("").to_string();
    match cmd {
        "marks" => Ok(ExCommand::Marks),
        "reg" | "registers" => Ok(ExCommand::Registers),
        "jumps" | "ju" => Ok(ExCommand::Jumps),
        "changes" => Ok(ExCommand::Changes),
        "dig" | "digraphs" => Ok(ExCommand::Digraphs),
        "file" | "f" => Ok(ExCommand::FileInfo),
        "sort" => Ok(ExCommand::Sort),
        "noh" | "nohlsearch" => Ok(ExCommand::NoHlSearch),
        "cn" | "cnext" => Ok(ExCommand::CNext),
        "cp" | "cprev" | "cprevious" => Ok(ExCommand::CPrev),
        "copen" | "cope" => Ok(ExCommand::COpen),
        "mes" | "messages" => Ok(ExCommand::Messages),
        "source" | "so" => Ok(ExCommand::Source(req_arg(args, "source")?)),
        "execute" | "exe" => Ok(ExCommand::Execute(req_arg(args, "execute")?)),
        "normal" | "norm" => Ok(ExCommand::Normal(req_arg(args, "normal")?)),
        "syntax" | "syn" => Ok(ExCommand::SyntaxCmd(arg_s)),
        "highlight" | "hi" => Ok(ExCommand::Highlight(arg_s)),
        "map" | "nmap" | "imap" | "vmap" => Ok(ExCommand::Map(arg_s)),
        "unmap" | "nunmap" => Ok(ExCommand::Unmap(arg_s)),
        "mapclear" => Ok(ExCommand::MapClear),
        "autocmd" | "au" => Ok(ExCommand::AutoCmd(arg_s)),
        "d" | "delete" => Ok(ExCommand::Delete(args.map(|s| s.to_string()))),
        "y" | "yank" => Ok(ExCommand::Yank(args.map(|s| s.to_string()))),
        "t" | "copy" => Ok(ExCommand::Copy(None, arg_s)),
        "m" | "move" => Ok(ExCommand::Move(None, arg_s)),
        "r" | "read" => Ok(ExCommand::Read(arg_s)),
        "put" | "pu" => {
            let reg = arg_s.chars().next();
            Ok(ExCommand::Put(reg))
        }
        "ft" | "filetype" => Ok(ExCommand::FileType(arg_s)),
        "cd" | "chdir" => Ok(ExCommand::Cd(arg_s)),
        "pwd" => Ok(ExCommand::Pwd),
        "mksession" | "mks" => Ok(ExCommand::MkSession(args.map(|s| s.to_string()))),
        "oldfiles" | "old" => Ok(ExCommand::OldFiles),
        "explorer" | "Explore" | "Ex" => Ok(ExCommand::Explorer),
        "terminal" | "term" => Ok(ExCommand::Terminal),
        "find" | "fin" => Ok(ExCommand::Find),
        "livegrep" | "lg" => Ok(ExCommand::LiveGrep),
        "undotree" => Ok(ExCommand::UndoTree),
        _ => Err(EditorError::InvalidCommand(format!("unknown command: {cmd}"))),
    }
}

fn req_arg(args: Option<&str>, cmd_name: &str) -> Result<String, EditorError> {
    args.map(|s| s.to_string())
        .ok_or_else(|| EditorError::InvalidCommand(format!("{cmd_name}: argument required")))
}

/// Parse :s/pattern/replacement/flags
pub fn parse_substitute(input: &str) -> Result<ExCommand, EditorError> {
    let input = input.strip_prefix('s').unwrap_or(input);
    let sep = input.chars().next().unwrap_or('/');
    let parts: Vec<&str> = input[1..].splitn(3, sep).collect();
    let pattern = parts.first().unwrap_or(&"").to_string();
    let replacement = parts.get(1).unwrap_or(&"").to_string();
    let flags = parts.get(2).unwrap_or(&"").to_string();
    if pattern.is_empty() {
        return Err(EditorError::InvalidCommand("substitute: empty pattern".into()));
    }
    Ok(ExCommand::Substitute(pattern, replacement, flags))
}

/// Parse :g/pattern/cmd or :v/pattern/cmd
pub fn parse_global(input: &str, inverted: bool) -> Result<ExCommand, EditorError> {
    let prefix = if inverted { 'v' } else { 'g' };
    let input = input.strip_prefix(prefix).unwrap_or(input);
    let sep = input.chars().next().unwrap_or('/');
    let rest = &input[1..];
    if let Some(idx) = rest.find(sep) {
        let pattern = rest[..idx].to_string();
        let cmd = rest[idx + 1..].to_string();
        if inverted {
            Ok(ExCommand::VGlobal(pattern, cmd))
        } else {
            Ok(ExCommand::Global(pattern, cmd))
        }
    } else {
        Err(EditorError::InvalidCommand("global: missing separator".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sub() {
        let cmd = parse_substitute("s/foo/bar/g").unwrap();
        assert_eq!(cmd, ExCommand::Substitute("foo".into(), "bar".into(), "g".into()));
    }

    #[test]
    fn parse_sub_empty_pattern() {
        assert!(parse_substitute("s//bar/g").is_err());
    }

    #[test]
    fn parse_global_cmd() {
        let cmd = parse_global("g/pattern/d", false).unwrap();
        assert_eq!(cmd, ExCommand::Global("pattern".into(), "d".into()));
    }

    #[test]
    fn parse_vglobal() {
        let cmd = parse_global("v/pattern/d", true).unwrap();
        assert_eq!(cmd, ExCommand::VGlobal("pattern".into(), "d".into()));
    }

    #[test]
    fn parse_extended_marks() {
        assert_eq!(parse_extended("marks", None).unwrap(), ExCommand::Marks);
    }

    #[test]
    fn parse_extended_unknown() {
        assert!(parse_extended("zzz", None).is_err());
    }
}
