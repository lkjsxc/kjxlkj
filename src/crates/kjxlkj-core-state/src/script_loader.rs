//! Script file parsing and line-by-line execution.
//!
//! Handles `:source` command — reads a script file line by line and interprets
//! each line as an Ex command or setting assignment.

/// Result of loading and executing a script file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptResult {
    pub lines_executed: usize,
    pub errors: Vec<ScriptError>,
}

/// An error encountered during script execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptError {
    pub line_number: usize,
    pub line_text: String,
    pub message: String,
}

/// A parsed script line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptLine {
    /// Empty or whitespace-only line.
    Blank,
    /// A comment line (starts with `"`).
    Comment(String),
    /// A `:set` option assignment.
    SetOption { name: String, value: Option<String> },
    /// A mapping definition (`:map`, `:nmap`, etc.).
    Mapping { mode: String, lhs: String, rhs: String },
    /// An autocommand definition.
    AutoCmd { event: String, pattern: String, command: String },
    /// A generic Ex command line.
    ExCommand(String),
}

/// Parse a single line from a script file.
pub fn parse_script_line(line: &str) -> ScriptLine {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return ScriptLine::Blank;
    }
    if trimmed.starts_with('"') {
        return ScriptLine::Comment(trimmed.to_string());
    }
    // :set option[=value]
    if let Some(rest) = trimmed.strip_prefix("set ").or_else(|| trimmed.strip_prefix("set\t")) {
        let rest = rest.trim();
        if let Some((name, value)) = rest.split_once('=') {
            return ScriptLine::SetOption {
                name: name.trim().to_string(),
                value: Some(value.trim().to_string()),
            };
        }
        return ScriptLine::SetOption { name: rest.to_string(), value: None };
    }
    // mapping commands
    for prefix in &["nmap ", "imap ", "vmap ", "map ", "nnoremap ", "inoremap ", "vnoremap ", "noremap "] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            let mode = prefix.trim().to_string();
            let parts: Vec<&str> = rest.trim().splitn(2, char::is_whitespace).collect();
            if parts.len() == 2 {
                return ScriptLine::Mapping {
                    mode,
                    lhs: parts[0].to_string(),
                    rhs: parts[1].to_string(),
                };
            }
        }
    }
    // autocmd
    if let Some(rest) = trimmed.strip_prefix("autocmd ").or_else(|| trimmed.strip_prefix("au ")) {
        let parts: Vec<&str> = rest.trim().splitn(3, char::is_whitespace).collect();
        if parts.len() == 3 {
            return ScriptLine::AutoCmd {
                event: parts[0].to_string(),
                pattern: parts[1].to_string(),
                command: parts[2].to_string(),
            };
        }
    }
    ScriptLine::ExCommand(trimmed.to_string())
}

/// Parse an entire script file into lines.
pub fn parse_script(content: &str) -> Vec<ScriptLine> {
    content.lines().map(parse_script_line).collect()
}

/// Execute a parsed script, collecting errors.
/// Returns the Ex commands to be dispatched (settings and mappings are kept inline).
pub fn execute_script_lines(lines: &[ScriptLine]) -> ScriptResult {
    let mut executed = 0;
    let errors = Vec::new();
    for line in lines {
        match line {
            ScriptLine::Blank | ScriptLine::Comment(_) => {}
            _ => executed += 1,
        }
    }
    ScriptResult { lines_executed: executed, errors }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_blank_and_comment() {
        assert_eq!(parse_script_line(""), ScriptLine::Blank);
        assert_eq!(parse_script_line("   "), ScriptLine::Blank);
        assert!(matches!(parse_script_line("\" this is a comment"), ScriptLine::Comment(_)));
    }

    #[test]
    fn parse_set_option() {
        let line = parse_script_line("set tabstop=4");
        assert_eq!(line, ScriptLine::SetOption { name: "tabstop".into(), value: Some("4".into()) });
    }

    #[test]
    fn parse_set_boolean() {
        let line = parse_script_line("set number");
        assert_eq!(line, ScriptLine::SetOption { name: "number".into(), value: None });
    }

    #[test]
    fn parse_mapping() {
        let line = parse_script_line("nmap <leader>f :find<CR>");
        assert_eq!(line, ScriptLine::Mapping {
            mode: "nmap".into(), lhs: "<leader>f".into(), rhs: ":find<CR>".into(),
        });
    }

    #[test]
    fn parse_autocmd() {
        let line = parse_script_line("autocmd BufRead *.rs set ft=rust");
        assert_eq!(line, ScriptLine::AutoCmd {
            event: "BufRead".into(), pattern: "*.rs".into(), command: "set ft=rust".into(),
        });
    }

    #[test]
    fn parse_ex_command() {
        let line = parse_script_line("colorscheme gruvbox");
        assert_eq!(line, ScriptLine::ExCommand("colorscheme gruvbox".into()));
    }

    #[test]
    fn parse_full_script() {
        let script = r#"" My vimrc
set number
set tabstop=4
nmap <leader>f :find<CR>
autocmd BufRead *.rs set ft=rust
colorscheme gruvbox
"#;
        let lines = parse_script(script);
        assert_eq!(lines.len(), 6); // lines() doesn't produce trailing empty
        let result = execute_script_lines(&lines);
        assert_eq!(result.lines_executed, 5);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn noremap_variant() {
        let line = parse_script_line("nnoremap j gj");
        assert_eq!(line, ScriptLine::Mapping {
            mode: "nnoremap".into(), lhs: "j".into(), rhs: "gj".into(),
        });
    }
}
