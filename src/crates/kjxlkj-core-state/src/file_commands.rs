//! File operation helpers for write, edit, and path utilities.

use std::path::Path;

/// File operation command kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileCommand {
    Write,
    Edit,
    SaveAs,
    WriteQuit,
    WriteAll,
    Reload,
}

/// Parse a command string into a FileCommand.
pub fn parse_file_command(input: &str) -> Option<FileCommand> {
    match input.trim() {
        "w" | "write" => Some(FileCommand::Write),
        "e" | "edit" => Some(FileCommand::Edit),
        "saveas" => Some(FileCommand::SaveAs),
        "wq" | "x" | "exit" => Some(FileCommand::WriteQuit),
        "wa" | "wall" => Some(FileCommand::WriteAll),
        "e!" => Some(FileCommand::Reload),
        _ => None,
    }
}

/// Validate that a path is writable.
pub fn validate_write(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("no file name".to_string());
    }
    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(format!("directory does not exist: {}", parent.display()));
        }
    }
    if p.exists() && p.is_dir() {
        return Err(format!("is a directory: {path}"));
    }
    Ok(())
}

/// Expand ~ to home directory in a path.
pub fn expand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{home}/{rest}");
        }
    }
    if path == "~" {
        if let Ok(home) = std::env::var("HOME") {
            return home;
        }
    }
    path.to_string()
}

/// Generate a display title for a buffer.
pub fn buffer_title(path: Option<&str>) -> String {
    match path {
        Some(p) => {
            let p = Path::new(p);
            p.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "[No Name]".to_string())
        }
        None => "[No Name]".to_string(),
    }
}

/// Shorten a path for display.
pub fn display_path(path: &str) -> String {
    if let Ok(home) = std::env::var("HOME") {
        if let Some(rest) = path.strip_prefix(&home) {
            return format!("~{rest}");
        }
    }
    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_write() {
        assert_eq!(parse_file_command("w"), Some(FileCommand::Write));
        assert_eq!(parse_file_command("wq"), Some(FileCommand::WriteQuit));
    }

    #[test]
    fn validate_empty_path() {
        assert!(validate_write("").is_err());
    }

    #[test]
    fn buffer_title_none() {
        assert_eq!(buffer_title(None), "[No Name]");
    }

    #[test]
    fn buffer_title_path() {
        assert_eq!(buffer_title(Some("/foo/bar.rs")), "bar.rs");
    }

    #[test]
    fn expand_tilde_plain() {
        assert_eq!(expand_tilde("/foo/bar"), "/foo/bar");
    }
}
