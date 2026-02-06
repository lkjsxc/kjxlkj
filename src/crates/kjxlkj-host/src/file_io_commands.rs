/// File I/O command processing — open, write, edit, save.
use std::path::{Path, PathBuf};

/// Result of a file I/O operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileIoResult {
    Success(String),
    Error(String),
    NeedForce(String),
}

/// A file I/O command parsed from Ex input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileCommand {
    Write { path: Option<PathBuf>, force: bool },
    Edit { path: PathBuf, force: bool },
    SaveAs { path: PathBuf },
    WriteQuit { path: Option<PathBuf>, force: bool },
    WriteAll,
    Reload { force: bool },
}

/// Parse a file command from Ex-command arguments.
pub fn parse_file_command(cmd: &str, args: &str, force: bool) -> Option<FileCommand> {
    let args = args.trim();
    match cmd {
        "w" | "write" => Some(FileCommand::Write {
            path: if args.is_empty() { None } else { Some(PathBuf::from(args)) }, force,
        }),
        "e" | "edit" => {
            if args.is_empty() { return Some(FileCommand::Reload { force }); }
            Some(FileCommand::Edit { path: PathBuf::from(args), force })
        }
        "saveas" => {
            if args.is_empty() { return None; }
            Some(FileCommand::SaveAs { path: PathBuf::from(args) })
        }
        "wq" | "x" | "exit" => Some(FileCommand::WriteQuit {
            path: if args.is_empty() { None } else { Some(PathBuf::from(args)) }, force,
        }),
        "wa" | "wall" => Some(FileCommand::WriteAll),
        _ => None,
    }
}

/// Validate a write operation before executing.
pub fn validate_write(path: &Path, buf_modified: bool, force: bool) -> FileIoResult {
    if !buf_modified && !force {
        return FileIoResult::Success("No changes to write".into());
    }
    let dir = path.parent();
    if let Some(d) = dir {
        if !d.as_os_str().is_empty() && !d.exists() {
            return FileIoResult::Error(format!("Directory does not exist: {}", d.display()));
        }
    }
    if path.exists() && path.metadata().map(|m| m.permissions().readonly()).unwrap_or(false) {
        if !force {
            return FileIoResult::NeedForce("File is read-only, use ! to force".into());
        }
    }
    FileIoResult::Success(format!("\"{}\" written", path.display()))
}

/// Expand `~` in a path to the home directory.
pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    PathBuf::from(path)
}

/// Compute a display title for a buffer.
pub fn buffer_title(path: &Option<PathBuf>, modified: bool) -> String {
    let name = path.as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "[No Name]".into());
    if modified { format!("{} [+]", name) } else { name }
}

/// Compute a relative path for display.
pub fn display_path(path: &Path, cwd: &Path) -> String {
    path.strip_prefix(cwd).unwrap_or(path).display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_write() {
        let cmd = parse_file_command("w", "", false).unwrap();
        assert_eq!(cmd, FileCommand::Write { path: None, force: false });
    }

    #[test]
    fn parse_write_path() {
        let cmd = parse_file_command("w", "out.txt", false).unwrap();
        assert_eq!(cmd, FileCommand::Write { path: Some("out.txt".into()), force: false });
    }

    #[test]
    fn parse_edit() {
        let cmd = parse_file_command("e", "file.rs", false).unwrap();
        assert_eq!(cmd, FileCommand::Edit { path: "file.rs".into(), force: false });
    }

    #[test]
    fn parse_edit_reload() {
        let cmd = parse_file_command("e", "", true).unwrap();
        assert_eq!(cmd, FileCommand::Reload { force: true });
    }

    #[test]
    fn parse_wq() {
        let cmd = parse_file_command("wq", "", false).unwrap();
        assert_eq!(cmd, FileCommand::WriteQuit { path: None, force: false });
    }

    #[test]
    fn parse_unknown() {
        assert!(parse_file_command("xyz", "", false).is_none());
    }

    #[test]
    fn buffer_title_modified() {
        assert_eq!(buffer_title(&Some("main.rs".into()), true), "main.rs [+]");
        assert_eq!(buffer_title(&Some("main.rs".into()), false), "main.rs");
        assert_eq!(buffer_title(&None, false), "[No Name]");
    }

    #[test]
    fn display_path_relative() {
        let p = Path::new("/home/user/project/src/main.rs");
        let cwd = Path::new("/home/user/project");
        assert_eq!(display_path(p, cwd), "src/main.rs");
    }

    #[test]
    fn expand_tilde_noop() {
        assert_eq!(expand_tilde("/abs/path"), PathBuf::from("/abs/path"));
    }
}
