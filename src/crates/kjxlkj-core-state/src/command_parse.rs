//! Command-line parsing helpers and tests.

/// Split a command string into (name, args).
///
/// Handles range prefixes (digits, %, $, etc).
pub fn split_command(cmd: &str) -> (String, &str) {
    let cmd = cmd.trim();

    // Skip range prefix: digits, commas, dots, $, %, '
    let mut pos = 0;
    let bytes = cmd.as_bytes();
    while pos < bytes.len() {
        let b = bytes[pos];
        if b == b'%'
            || b == b'.'
            || b == b'$'
            || b == b','
            || b == b'\''
            || b.is_ascii_digit()
        {
            pos += 1;
        } else {
            break;
        }
    }

    let range_end = pos;
    if range_end > 0 && range_end < cmd.len() {
        let rest = &cmd[range_end..];
        let name_end = rest
            .find(|c: char| {
                c.is_whitespace() || c == '/'
            })
            .unwrap_or(rest.len());
        let name = &rest[..name_end];
        if !name.is_empty()
            && name
                .chars()
                .next()
                .unwrap()
                .is_alphabetic()
        {
            let args = rest[name_end..].trim_start();
            return (name.to_string(), args);
        }
    }

    // Line-number-only commands.
    if cmd
        .chars()
        .next()
        .map_or(false, |c| c.is_ascii_digit())
        && range_end == cmd.len()
    {
        let end = cmd
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(cmd.len());
        let (num, rest) = cmd.split_at(end);
        return (num.to_string(), rest.trim());
    }

    let name_end = cmd
        .find(|c: char| {
            c.is_whitespace() || c == '/'
        })
        .unwrap_or(cmd.len());
    let (name, rest) = cmd.split_at(name_end);
    (name.to_string(), rest.trim_start())
}

/// Return the full command string preserving range.
pub fn full_with_range(cmd: &str) -> String {
    cmd.to_string()
}

#[cfg(test)]
mod tests {
    use crate::dispatch_command;
    use kjxlkj_core_types::Action;

    #[test]
    fn quit_command() {
        assert!(matches!(
            dispatch_command("q"),
            Some(Action::Quit)
        ));
        assert!(matches!(
            dispatch_command("q!"),
            Some(Action::ForceQuit)
        ));
    }

    #[test]
    fn write_command() {
        assert!(matches!(
            dispatch_command("w"),
            Some(Action::Write)
        ));
    }

    #[test]
    fn edit_command() {
        let action = dispatch_command("e test.txt");
        assert!(matches!(
            action,
            Some(Action::OpenFile(_))
        ));
    }

    #[test]
    fn line_number_command() {
        let action = dispatch_command("42");
        assert!(matches!(
            action,
            Some(Action::MoveCursor(
                kjxlkj_core_types::Motion::GotoLine(
                    41
                ),
                1
            ))
        ));
    }

    #[test]
    fn substitute_command() {
        let action =
            dispatch_command("s/foo/bar/g");
        assert!(matches!(
            action,
            Some(Action::Substitute(_))
        ));
    }

    #[test]
    fn unknown_command() {
        assert!(dispatch_command("foobar").is_none());
    }

    #[test]
    fn range_delete_command() {
        let action = dispatch_command("1,5d");
        assert!(matches!(
            action,
            Some(Action::RangeDelete(_))
        ));
    }

    #[test]
    fn range_normal_command() {
        let action = dispatch_command("%normal @a");
        assert!(matches!(
            action,
            Some(Action::RangeNormal(_))
        ));
    }

    #[test]
    fn tab_commands() {
        assert!(matches!(
            dispatch_command("tabnew"),
            Some(Action::TabNew(None))
        ));
        assert!(matches!(
            dispatch_command("tabnew foo.rs"),
            Some(Action::TabNew(Some(_)))
        ));
        assert!(matches!(
            dispatch_command("tabclose"),
            Some(Action::TabClose)
        ));
        assert!(matches!(
            dispatch_command("tabnext"),
            Some(Action::TabNext)
        ));
    }

    #[test]
    fn set_command() {
        assert!(matches!(
            dispatch_command("set number"),
            Some(Action::SetOption(_))
        ));
    }

    #[test]
    fn source_command() {
        assert!(matches!(
            dispatch_command("source init.vim"),
            Some(Action::SourceFile(_))
        ));
    }

    #[test]
    fn mapping_command() {
        assert!(matches!(
            dispatch_command("nnoremap jk <Esc>"),
            Some(Action::MapCommand(_, _))
        ));
    }
}
