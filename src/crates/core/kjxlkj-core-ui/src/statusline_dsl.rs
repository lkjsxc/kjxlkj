//! Statusline DSL parser and renderer.
//!
//! See /docs/spec/features/ui/statusline/statusline-dsl.md.

/// A parsed token from a statusline format string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DslToken {
    Literal(String), Separator,
    FilePath, FilePathAbsolute,
    Modified, ReadOnly,
    Line, Column, Percent, FileType,
    Highlight(String),
}

/// Variable values for rendering.
#[derive(Debug, Clone, Default)]
pub struct DslVars {
    pub file_path: String,
    pub file_path_abs: String,
    pub modified: bool,
    pub readonly: bool,
    pub line: usize,
    pub column: usize,
    pub percent: u8,
    pub file_type: String,
}

/// Parse a statusline format string into tokens.
pub fn parse_format(fmt: &str) -> Vec<DslToken> {
    let mut tokens = Vec::new();
    let mut chars = fmt.chars().peekable();
    let mut lit = String::new();
    while let Some(c) = chars.next() {
        if c != '%' { lit.push(c); continue; }
        match chars.peek() {
            None => lit.push('%'),
            Some(&'%') => { chars.next(); lit.push('%'); }
            Some(&'=') => {
                flush(&mut lit, &mut tokens);
                chars.next();
                tokens.push(DslToken::Separator);
            }
            Some(&'#') => {
                flush(&mut lit, &mut tokens);
                chars.next();
                let mut g = String::new();
                while let Some(&gc) = chars.peek() {
                    if gc == '#' { chars.next(); break; }
                    g.push(gc); chars.next();
                }
                tokens.push(DslToken::Highlight(g));
            }
            Some(&ch) => {
                if let Some(tok) = variable_token(ch) {
                    flush(&mut lit, &mut tokens);
                    chars.next();
                    tokens.push(tok);
                } else {
                    lit.push('%');
                }
            }
        }
    }
    if !lit.is_empty() {
        tokens.push(DslToken::Literal(lit));
    }
    tokens
}

fn variable_token(c: char) -> Option<DslToken> {
    match c {
        'f' => Some(DslToken::FilePath),
        'F' => Some(DslToken::FilePathAbsolute),
        'm' => Some(DslToken::Modified),
        'r' => Some(DslToken::ReadOnly),
        'l' => Some(DslToken::Line),
        'c' => Some(DslToken::Column),
        'p' => Some(DslToken::Percent),
        'y' => Some(DslToken::FileType),
        _ => None,
    }
}

fn flush(lit: &mut String, tokens: &mut Vec<DslToken>) {
    if !lit.is_empty() {
        tokens.push(DslToken::Literal(lit.clone()));
        lit.clear();
    }
}

/// Render parsed tokens to a string. Separator becomes `\x00`.
pub fn render_tokens(tokens: &[DslToken], vars: &DslVars) -> String {
    let mut out = String::new();
    for tok in tokens {
        match tok {
            DslToken::Literal(s) => out.push_str(s),
            DslToken::Separator => out.push('\x00'),
            DslToken::FilePath => out.push_str(&vars.file_path),
            DslToken::FilePathAbsolute => out.push_str(&vars.file_path_abs),
            DslToken::Modified => { if vars.modified { out.push_str("[+]"); } }
            DslToken::ReadOnly => { if vars.readonly { out.push_str("[-]"); } }
            DslToken::Line => out.push_str(&vars.line.to_string()),
            DslToken::Column => out.push_str(&vars.column.to_string()),
            DslToken::Percent => out.push_str(&vars.percent.to_string()),
            DslToken::FileType => out.push_str(&vars.file_type),
            DslToken::Highlight(_) => {}
        }
    }
    out
}

#[cfg(test)]
#[path = "statusline_dsl_tests.rs"]
mod tests;
