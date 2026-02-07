//! Syntax command parsing and filetype detection.

/// Syntax toggle actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxAction {
    On,
    Off,
    Manual,
    Enable,
    Disable,
}

/// Parse a :syntax command argument.
pub fn parse_syntax_command(arg: &str) -> Option<SyntaxAction> {
    match arg.trim().to_lowercase().as_str() {
        "on" | "enable" => Some(SyntaxAction::On),
        "off" | "disable" => Some(SyntaxAction::Off),
        "manual" => Some(SyntaxAction::Manual),
        "" => Some(SyntaxAction::Enable),
        _ => None,
    }
}

/// Detect language from file extension.
pub fn detect_language(path: &str) -> Option<String> {
    let ext = path.rsplit('.').next()?;
    let lang = match ext.to_lowercase().as_str() {
        "rs" => "rust",
        "py" => "python",
        "js" => "javascript",
        "ts" => "typescript",
        "tsx" => "typescriptreact",
        "jsx" => "javascriptreact",
        "c" => "c",
        "cpp" | "cc" | "cxx" => "cpp",
        "h" | "hpp" => "cpp",
        "go" => "go",
        "rb" => "ruby",
        "java" => "java",
        "lua" => "lua",
        "sh" | "bash" => "bash",
        "zsh" => "zsh",
        "toml" => "toml",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "md" | "markdown" => "markdown",
        "html" | "htm" => "html",
        "css" => "css",
        _ => return None,
    };
    Some(lang.to_string())
}

/// Parse a :filetype / :ft command argument.
pub fn parse_filetype_command(arg: &str) -> Option<String> {
    let arg = arg.trim();
    if arg.is_empty() {
        return None;
    }
    Some(arg.to_string())
}

/// Format syntax status info string.
pub fn format_syntax_info(enabled: bool, filetype: &str) -> String {
    let status = if enabled { "on" } else { "off" };
    if filetype.is_empty() {
        format!("syntax {status}")
    } else {
        format!("syntax {status}  filetype={filetype}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_on_off() {
        assert_eq!(parse_syntax_command("on"), Some(SyntaxAction::On));
        assert_eq!(parse_syntax_command("off"), Some(SyntaxAction::Off));
        assert_eq!(parse_syntax_command("manual"), Some(SyntaxAction::Manual));
    }

    #[test]
    fn detect_common_langs() {
        assert_eq!(detect_language("main.rs"), Some("rust".into()));
        assert_eq!(detect_language("app.py"), Some("python".into()));
        assert_eq!(detect_language("index.tsx"), Some("typescriptreact".into()));
        assert_eq!(detect_language("data.unknown"), None);
    }

    #[test]
    fn format_info() {
        assert_eq!(format_syntax_info(true, "rust"), "syntax on  filetype=rust");
        assert_eq!(format_syntax_info(false, ""), "syntax off");
    }

    #[test]
    fn parse_filetype() {
        assert_eq!(parse_filetype_command("rust"), Some("rust".into()));
        assert_eq!(parse_filetype_command(""), None);
    }
}
