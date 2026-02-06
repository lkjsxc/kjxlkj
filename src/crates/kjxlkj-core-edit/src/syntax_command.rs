/// Syntax command processing — `:syntax on/off`, filetype detection, syntax toggling.
use std::collections::HashMap;
use std::path::Path;

/// Syntax state for a buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxState {
    On,
    Off,
    Manual,
}

/// Result of a syntax command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxCommandResult {
    pub new_state: SyntaxState,
    pub language: Option<String>,
    pub message: Option<String>,
}

/// Known file extension to language mappings.
fn build_extension_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("rs", "rust");
    m.insert("py", "python");
    m.insert("js", "javascript");
    m.insert("ts", "typescript");
    m.insert("c", "c");
    m.insert("h", "c");
    m.insert("cpp", "cpp");
    m.insert("go", "go");
    m.insert("rb", "ruby");
    m.insert("lua", "lua");
    m.insert("sh", "bash");
    m.insert("md", "markdown");
    m.insert("toml", "toml");
    m.insert("json", "json");
    m.insert("yaml", "yaml");
    m.insert("yml", "yaml");
    m.insert("html", "html");
    m.insert("css", "css");
    m.insert("java", "java");
    m.insert("vim", "vim");
    m
}

/// Detect language from a file path extension.
pub fn detect_language(path: &str) -> Option<String> {
    let ext = Path::new(path).extension()?.to_str()?;
    let map = build_extension_map();
    map.get(ext).map(|s| s.to_string())
}

/// Parse a `:syntax` command argument.
pub fn parse_syntax_command(arg: &str) -> SyntaxCommandResult {
    let trimmed = arg.trim();
    match trimmed {
        "on" | "enable" => SyntaxCommandResult {
            new_state: SyntaxState::On,
            language: None,
            message: Some("Syntax highlighting enabled".into()),
        },
        "off" | "disable" => SyntaxCommandResult {
            new_state: SyntaxState::Off,
            language: None,
            message: Some("Syntax highlighting disabled".into()),
        },
        "manual" => SyntaxCommandResult {
            new_state: SyntaxState::Manual,
            language: None,
            message: Some("Syntax set to manual".into()),
        },
        "" => SyntaxCommandResult {
            new_state: SyntaxState::On,
            language: None,
            message: Some("syntax=on".into()),
        },
        _ => SyntaxCommandResult {
            new_state: SyntaxState::On,
            language: None,
            message: Some(format!("Unknown syntax argument: {}", trimmed)),
        },
    }
}

/// Parse a `:setfiletype` or `:set ft=` command.
pub fn parse_filetype_command(arg: &str) -> Option<String> {
    let trimmed = arg.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_lowercase())
}

/// Build a display string showing current syntax info.
pub fn format_syntax_info(state: &SyntaxState, lang: &Option<String>) -> String {
    let state_str = match state {
        SyntaxState::On => "on",
        SyntaxState::Off => "off",
        SyntaxState::Manual => "manual",
    };
    match lang {
        Some(l) => format!("syntax={} filetype={}", state_str, l),
        None => format!("syntax={} filetype=", state_str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_rust() {
        assert_eq!(detect_language("main.rs"), Some("rust".into()));
    }

    #[test]
    fn detect_python() {
        assert_eq!(detect_language("script.py"), Some("python".into()));
    }

    #[test]
    fn detect_unknown_ext() {
        assert_eq!(detect_language("file.xyz"), None);
    }

    #[test]
    fn detect_no_extension() {
        assert_eq!(detect_language("Makefile"), None);
    }

    #[test]
    fn syntax_on() {
        let r = parse_syntax_command("on");
        assert_eq!(r.new_state, SyntaxState::On);
    }

    #[test]
    fn syntax_off() {
        let r = parse_syntax_command("off");
        assert_eq!(r.new_state, SyntaxState::Off);
    }

    #[test]
    fn syntax_manual() {
        let r = parse_syntax_command("manual");
        assert_eq!(r.new_state, SyntaxState::Manual);
    }

    #[test]
    fn filetype_parse() {
        assert_eq!(parse_filetype_command("Rust"), Some("rust".into()));
        assert_eq!(parse_filetype_command(""), None);
    }

    #[test]
    fn format_info_with_lang() {
        let s = format_syntax_info(&SyntaxState::On, &Some("rust".into()));
        assert_eq!(s, "syntax=on filetype=rust");
    }

    #[test]
    fn format_info_no_lang() {
        let s = format_syntax_info(&SyntaxState::Off, &None);
        assert_eq!(s, "syntax=off filetype=");
    }
}
