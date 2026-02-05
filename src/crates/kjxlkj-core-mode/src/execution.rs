//! Command execution module.
//!
//! Implements `:source`, `:execute`, and `:normal` commands.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Error types for command execution.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecError {
    /// File not found.
    FileNotFound(PathBuf),
    /// Failed to read file.
    ReadError(String),
    /// Command parse error.
    ParseError(String),
    /// Command execution error.
    CommandError(String),
    /// Recursion limit exceeded.
    RecursionLimit,
    /// User aborted.
    Aborted,
}

impl std::fmt::Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound(path) => write!(f, "File not found: {}", path.display()),
            Self::ReadError(msg) => write!(f, "Read error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::CommandError(msg) => write!(f, "Command error: {}", msg),
            Self::RecursionLimit => write!(f, "Recursion limit exceeded"),
            Self::Aborted => write!(f, "Aborted"),
        }
    }
}

impl std::error::Error for ExecError {}

/// Configuration for command execution.
#[derive(Debug, Clone)]
pub struct ExecConfig {
    /// Maximum recursion depth.
    pub max_recursion: usize,
    /// Continue on error (like :source!).
    pub continue_on_error: bool,
    /// Runtime paths for :runtime command.
    pub runtime_paths: Vec<PathBuf>,
}

impl Default for ExecConfig {
    fn default() -> Self {
        Self {
            max_recursion: 100,
            continue_on_error: false,
            runtime_paths: Vec::new(),
        }
    }
}

/// A single command to execute.
#[derive(Debug, Clone)]
pub struct ExCommand {
    /// The command name.
    pub name: String,
    /// Range specification (if any).
    pub range: Option<Range>,
    /// Bang modifier (!).
    pub bang: bool,
    /// Command arguments.
    pub args: String,
}

impl ExCommand {
    /// Parse an Ex command from a string.
    pub fn parse(line: &str) -> Result<Self, ExecError> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('"') {
            // Empty line or comment
            return Ok(Self {
                name: String::new(),
                range: None,
                bang: false,
                args: String::new(),
            });
        }

        let (range, rest) = Range::parse_from(line)?;
        let (name, bang, args) = parse_command_name(rest);

        Ok(Self {
            name,
            range,
            bang,
            args,
        })
    }

    /// Check if this is an empty/comment command.
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
}

/// A range specification for Ex commands.
#[derive(Debug, Clone)]
pub struct Range {
    /// Start of range.
    pub start: RangeSpec,
    /// End of range (if different from start).
    pub end: Option<RangeSpec>,
}

/// A single line specification in a range.
#[derive(Debug, Clone)]
pub enum RangeSpec {
    /// Current line (.).
    Current,
    /// Last line ($).
    Last,
    /// Specific line number.
    Line(usize),
    /// Pattern match (/).
    Pattern(String),
    /// Relative offset (+n or -n).
    Offset(i64),
    /// Mark ('a).
    Mark(char),
}

impl Range {
    /// Parse a range from the beginning of a command.
    fn parse_from(line: &str) -> Result<(Option<Self>, &str), ExecError> {
        let line = line.trim_start();

        // Check for common range prefixes
        if let Some(rest) = line.strip_prefix('%') {
            // Whole file
            return Ok((
                Some(Range {
                    start: RangeSpec::Line(1),
                    end: Some(RangeSpec::Last),
                }),
                rest,
            ));
        }

        if let Some(rest) = line.strip_prefix('.') {
            if let Some(rest2) = rest.strip_prefix(',') {
                let (end, rest3) = Self::parse_spec(rest2)?;
                return Ok((
                    Some(Range {
                        start: RangeSpec::Current,
                        end: Some(end),
                    }),
                    rest3,
                ));
            }
            return Ok((
                Some(Range {
                    start: RangeSpec::Current,
                    end: None,
                }),
                rest,
            ));
        }

        if let Some(rest) = line.strip_prefix('$') {
            return Ok((
                Some(Range {
                    start: RangeSpec::Last,
                    end: None,
                }),
                rest,
            ));
        }

        // Check for line number
        if let Some(c) = line.chars().next() {
            if c.is_ascii_digit() {
                let num_end = line.find(|c: char| !c.is_ascii_digit()).unwrap_or(line.len());
                if let Ok(num) = line[..num_end].parse::<usize>() {
                    let rest = &line[num_end..];
                    if let Some(rest2) = rest.strip_prefix(',') {
                        let (end, rest3) = Self::parse_spec(rest2)?;
                        return Ok((
                            Some(Range {
                                start: RangeSpec::Line(num),
                                end: Some(end),
                            }),
                            rest3,
                        ));
                    }
                    return Ok((
                        Some(Range {
                            start: RangeSpec::Line(num),
                            end: None,
                        }),
                        rest,
                    ));
                }
            }
        }

        // No range found
        Ok((None, line))
    }

    /// Parse a single range spec.
    fn parse_spec(line: &str) -> Result<(RangeSpec, &str), ExecError> {
        let line = line.trim_start();

        if let Some(rest) = line.strip_prefix('.') {
            return Ok((RangeSpec::Current, rest));
        }
        if let Some(rest) = line.strip_prefix('$') {
            return Ok((RangeSpec::Last, rest));
        }

        // Line number
        if let Some(c) = line.chars().next() {
            if c.is_ascii_digit() {
                let num_end = line.find(|c: char| !c.is_ascii_digit()).unwrap_or(line.len());
                if let Ok(num) = line[..num_end].parse::<usize>() {
                    return Ok((RangeSpec::Line(num), &line[num_end..]));
                }
            }
        }

        // Default to current
        Ok((RangeSpec::Current, line))
    }
}

/// Parse command name, bang, and arguments.
fn parse_command_name(line: &str) -> (String, bool, String) {
    let line = line.trim_start();
    if line.is_empty() {
        return (String::new(), false, String::new());
    }

    let mut chars = line.chars().peekable();
    let mut name = String::new();

    // Collect command name (letters and some special chars)
    while let Some(&c) = chars.peek() {
        if c.is_alphabetic() || c == '_' {
            name.push(c);
            chars.next();
        } else {
            break;
        }
    }

    // Check for bang
    let bang = chars.peek() == Some(&'!');
    if bang {
        chars.next();
    }

    // Rest is arguments
    let args: String = chars.collect();
    let args = args.trim().to_string();

    (name, bang, args)
}

/// Context for command execution.
#[derive(Debug)]
pub struct ExecContext {
    /// Configuration.
    pub config: ExecConfig,
    /// Current recursion depth.
    pub depth: usize,
    /// Variables defined during execution.
    pub variables: HashMap<String, String>,
    /// Commands executed (for logging/debugging).
    pub log: Vec<String>,
}

impl ExecContext {
    /// Create a new execution context.
    pub fn new() -> Self {
        Self {
            config: ExecConfig::default(),
            depth: 0,
            variables: HashMap::new(),
            log: Vec::new(),
        }
    }

    /// Create with custom config.
    pub fn with_config(config: ExecConfig) -> Self {
        Self {
            config,
            depth: 0,
            variables: HashMap::new(),
            log: Vec::new(),
        }
    }

    /// Check if we can recurse deeper.
    pub fn can_recurse(&self) -> bool {
        self.depth < self.config.max_recursion
    }

    /// Enter a nested execution.
    pub fn enter(&mut self) -> Result<(), ExecError> {
        if !self.can_recurse() {
            return Err(ExecError::RecursionLimit);
        }
        self.depth += 1;
        Ok(())
    }

    /// Exit a nested execution.
    pub fn exit(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
}

impl Default for ExecContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of executing a command.
#[derive(Debug, Clone)]
pub enum ExecResult {
    /// Command executed successfully.
    Ok,
    /// Command returned output.
    Output(String),
    /// Command produced multiple commands to run.
    Commands(Vec<String>),
    /// Error occurred.
    Error(ExecError),
}

/// Source command executor.
#[derive(Debug)]
pub struct SourceCommand;

impl SourceCommand {
    /// Execute :source command.
    pub fn execute(
        path: &Path,
        ctx: &mut ExecContext,
    ) -> Result<Vec<String>, ExecError> {
        ctx.enter()?;

        // Expand ~ to home directory
        let path = Self::expand_path(path);

        // Read the file
        let content = std::fs::read_to_string(&path)
            .map_err(|e| ExecError::ReadError(e.to_string()))?;

        // Parse into commands
        let commands = Self::parse_script(&content)?;

        ctx.exit();
        Ok(commands)
    }

    /// Expand path (handle ~).
    fn expand_path(path: &Path) -> PathBuf {
        let path_str = path.to_string_lossy();
        if path_str.starts_with('~') {
            if let Some(home) = dirs_next::home_dir() {
                return home.join(&path_str[2..]);
            }
        }
        path.to_path_buf()
    }

    /// Parse a script into individual commands.
    fn parse_script(content: &str) -> Result<Vec<String>, ExecError> {
        let mut commands = Vec::new();
        let mut continuation = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('"') {
                continue;
            }

            // Handle line continuation (\)
            if let Some(without_backslash) = trimmed.strip_suffix('\\') {
                continuation.push_str(without_backslash);
                continue;
            }

            if !continuation.is_empty() {
                continuation.push_str(trimmed);
                commands.push(std::mem::take(&mut continuation));
            } else {
                commands.push(trimmed.to_string());
            }
        }

        if !continuation.is_empty() {
            commands.push(continuation);
        }

        Ok(commands)
    }
}

/// Execute command executor.
#[derive(Debug)]
pub struct ExecuteCommand;

impl ExecuteCommand {
    /// Execute :execute command.
    pub fn execute(
        args: &str,
        ctx: &mut ExecContext,
    ) -> Result<Vec<String>, ExecError> {
        ctx.enter()?;

        // Evaluate the expression to get command(s)
        let command = Self::evaluate(args, &ctx.variables)?;
        let commands = vec![command];

        ctx.log.push(format!(":execute {}", args));
        ctx.exit();
        Ok(commands)
    }

    /// Evaluate an expression.
    fn evaluate(
        expr: &str,
        variables: &HashMap<String, String>,
    ) -> Result<String, ExecError> {
        let expr = expr.trim();

        // Handle string concatenation with . first
        // Need to find . that's outside of quotes
        if let Some(parts) = Self::split_concat(expr) {
            let mut result = String::new();
            for part in parts {
                result.push_str(&Self::evaluate(part.trim(), variables)?);
            }
            return Ok(result);
        }

        // Handle string literals
        if expr.starts_with('"') && expr.ends_with('"') && expr.len() > 1 {
            let inner = &expr[1..expr.len() - 1];
            return Ok(Self::unescape(inner));
        }
        if expr.starts_with('\'') && expr.ends_with('\'') && expr.len() > 1 {
            let inner = &expr[1..expr.len() - 1];
            return Ok(inner.to_string());
        }

        // Handle variable references
        if expr.starts_with("g:") || expr.starts_with("l:") || expr.starts_with("s:") {
            let name = &expr[2..];
            return Ok(variables.get(name).cloned().unwrap_or_default());
        }

        // Plain string
        Ok(expr.to_string())
    }

    /// Split expression by concatenation operator, respecting quotes.
    fn split_concat(expr: &str) -> Option<Vec<&str>> {
        let mut parts = Vec::new();
        let mut start = 0;
        let mut in_double_quote = false;
        let mut in_single_quote = false;
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '"' if !in_single_quote => in_double_quote = !in_double_quote,
                '\'' if !in_double_quote => in_single_quote = !in_single_quote,
                '.' if !in_double_quote && !in_single_quote => {
                    // Check for " . " pattern
                    if i > 0 && i + 1 < chars.len() && chars[i - 1] == ' ' && chars[i + 1] == ' ' {
                        let part = &expr[start..i - 1];
                        parts.push(part.trim());
                        start = i + 2;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        if parts.is_empty() {
            return None;
        }

        parts.push(expr[start..].trim());
        Some(parts)
    }

    /// Unescape a string.
    fn unescape(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(&next) = chars.peek() {
                    match next {
                        'n' => {
                            result.push('\n');
                            chars.next();
                        }
                        't' => {
                            result.push('\t');
                            chars.next();
                        }
                        'r' => {
                            result.push('\r');
                            chars.next();
                        }
                        '\\' => {
                            result.push('\\');
                            chars.next();
                        }
                        '"' => {
                            result.push('"');
                            chars.next();
                        }
                        '<' => {
                            // Handle special key notation like \<CR>
                            chars.next(); // consume '<'
                            let mut key = String::new();
                            while let Some(&c) = chars.peek() {
                                if c == '>' {
                                    chars.next();
                                    break;
                                }
                                key.push(c);
                                chars.next();
                            }
                            match key.to_uppercase().as_str() {
                                "CR" | "ENTER" => result.push('\r'),
                                "ESC" | "ESCAPE" => result.push('\x1b'),
                                "TAB" => result.push('\t'),
                                "BS" | "BACKSPACE" => result.push('\x08'),
                                "SPACE" => result.push(' '),
                                _ => {
                                    result.push('<');
                                    result.push_str(&key);
                                    result.push('>');
                                }
                            }
                        }
                        _ => {
                            result.push('\\');
                        }
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(c);
            }
        }

        result
    }
}

/// Normal command executor.
#[derive(Debug)]
pub struct NormalCommand;

impl NormalCommand {
    /// Execute :normal command.
    /// Returns the keys to feed into normal mode.
    pub fn execute(
        args: &str,
        bang: bool,
        _ctx: &mut ExecContext,
    ) -> Result<NormalExec, ExecError> {
        let keys = Self::parse_keys(args)?;

        Ok(NormalExec {
            keys,
            ignore_mappings: bang,
        })
    }

    /// Parse key notation into actual keys.
    fn parse_keys(args: &str) -> Result<Vec<char>, ExecError> {
        let mut keys = Vec::new();
        let mut chars = args.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '<' {
                // Special key notation
                let mut key = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '>' {
                        chars.next();
                        break;
                    }
                    key.push(c);
                    chars.next();
                }
                match key.to_uppercase().as_str() {
                    "CR" | "ENTER" => keys.push('\r'),
                    "ESC" | "ESCAPE" => keys.push('\x1b'),
                    "TAB" => keys.push('\t'),
                    "BS" | "BACKSPACE" => keys.push('\x08'),
                    "SPACE" => keys.push(' '),
                    "LT" => keys.push('<'),
                    "GT" => keys.push('>'),
                    "BAR" => keys.push('|'),
                    _ if key.starts_with("C-") || key.starts_with("c-") => {
                        // Control character
                        if let Some(ch) = key.chars().nth(2) {
                            let ctrl = (ch.to_ascii_uppercase() as u8) - b'@';
                            keys.push(ctrl as char);
                        }
                    }
                    _ => {
                        // Unknown, just include literally
                        keys.push('<');
                        for ch in key.chars() {
                            keys.push(ch);
                        }
                        keys.push('>');
                    }
                }
            } else {
                keys.push(c);
            }
        }

        Ok(keys)
    }
}

/// Result of :normal command execution.
#[derive(Debug, Clone)]
pub struct NormalExec {
    /// Keys to execute.
    pub keys: Vec<char>,
    /// Whether to ignore mappings (!).
    pub ignore_mappings: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_empty() {
        let cmd = ExCommand::parse("").unwrap();
        assert!(cmd.is_empty());
    }

    #[test]
    fn test_parse_command_comment() {
        let cmd = ExCommand::parse("\" this is a comment").unwrap();
        assert!(cmd.is_empty());
    }

    #[test]
    fn test_parse_command_simple() {
        let cmd = ExCommand::parse("quit").unwrap();
        assert_eq!(cmd.name, "quit");
        assert!(!cmd.bang);
        assert!(cmd.args.is_empty());
    }

    #[test]
    fn test_parse_command_bang() {
        let cmd = ExCommand::parse("write!").unwrap();
        assert_eq!(cmd.name, "write");
        assert!(cmd.bang);
    }

    #[test]
    fn test_parse_command_args() {
        let cmd = ExCommand::parse("edit foo.txt").unwrap();
        assert_eq!(cmd.name, "edit");
        assert_eq!(cmd.args, "foo.txt");
    }

    #[test]
    fn test_parse_command_range_percent() {
        let cmd = ExCommand::parse("%s/foo/bar/g").unwrap();
        assert!(cmd.range.is_some());
        let range = cmd.range.unwrap();
        assert!(matches!(range.start, RangeSpec::Line(1)));
        assert!(matches!(range.end, Some(RangeSpec::Last)));
        assert_eq!(cmd.name, "s");
    }

    #[test]
    fn test_parse_command_range_line() {
        let cmd = ExCommand::parse("10d").unwrap();
        assert!(cmd.range.is_some());
        let range = cmd.range.unwrap();
        assert!(matches!(range.start, RangeSpec::Line(10)));
        assert_eq!(cmd.name, "d");
    }

    #[test]
    fn test_parse_command_range_lines() {
        let cmd = ExCommand::parse("1,10d").unwrap();
        assert!(cmd.range.is_some());
        let range = cmd.range.unwrap();
        assert!(matches!(range.start, RangeSpec::Line(1)));
        assert!(matches!(range.end, Some(RangeSpec::Line(10))));
    }

    #[test]
    fn test_exec_context_recursion() {
        let config = ExecConfig {
            max_recursion: 3,
            ..Default::default()
        };
        let mut ctx = ExecContext::with_config(config);

        assert!(ctx.can_recurse());
        ctx.enter().unwrap();
        ctx.enter().unwrap();
        ctx.enter().unwrap();
        assert!(!ctx.can_recurse());

        let result = ctx.enter();
        assert!(matches!(result, Err(ExecError::RecursionLimit)));
    }

    #[test]
    fn test_source_parse_script() {
        let content = r#"
            " Comment line
            set number
            set relativenumber
            
            map j gj
        "#;
        let commands = SourceCommand::parse_script(content).unwrap();
        assert_eq!(commands.len(), 3);
        assert_eq!(commands[0], "set number");
        assert_eq!(commands[1], "set relativenumber");
        assert_eq!(commands[2], "map j gj");
    }

    #[test]
    fn test_source_parse_continuation() {
        let content = "echo 'hello \\\n  world'";
        let commands = SourceCommand::parse_script(content).unwrap();
        assert_eq!(commands.len(), 1);
        // Line continuation joins trimmed lines
        assert_eq!(commands[0], "echo 'hello world'");
    }

    #[test]
    fn test_execute_string_literal() {
        let vars = HashMap::new();
        let result = ExecuteCommand::evaluate("\"echo 'hello'\"", &vars).unwrap();
        assert_eq!(result, "echo 'hello'");
    }

    #[test]
    fn test_execute_concatenation() {
        let vars = HashMap::new();
        let result = ExecuteCommand::evaluate("\"echo \" . \"'hello'\"", &vars).unwrap();
        assert_eq!(result, "echo 'hello'");
    }

    #[test]
    fn test_execute_special_keys() {
        let vars = HashMap::new();
        let result = ExecuteCommand::evaluate("\"normal! \\<CR>\"", &vars).unwrap();
        assert_eq!(result, "normal! \r");
    }

    #[test]
    fn test_normal_parse_keys() {
        let keys = NormalCommand::parse_keys("jjk").unwrap();
        assert_eq!(keys, vec!['j', 'j', 'k']);
    }

    #[test]
    fn test_normal_parse_special_keys() {
        let keys = NormalCommand::parse_keys("<CR><Esc>").unwrap();
        assert_eq!(keys, vec!['\r', '\x1b']);
    }

    #[test]
    fn test_normal_parse_ctrl_keys() {
        let keys = NormalCommand::parse_keys("<C-a><C-x>").unwrap();
        assert_eq!(keys, vec!['\x01', '\x18']);
    }

    #[test]
    fn test_normal_execute_bang() {
        let mut ctx = ExecContext::new();
        let result = NormalCommand::execute("dd", true, &mut ctx).unwrap();
        assert!(result.ignore_mappings);
        assert_eq!(result.keys, vec!['d', 'd']);
    }

    #[test]
    fn test_normal_execute_no_bang() {
        let mut ctx = ExecContext::new();
        let result = NormalCommand::execute("dd", false, &mut ctx).unwrap();
        assert!(!result.ignore_mappings);
    }
}
