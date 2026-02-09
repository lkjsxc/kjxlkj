//! Editing helper features: auto-pairs, comments, surround, etc.
//!
//! Per docs/spec/features/editing/ normative specs.

/// Auto-pair configuration for a specific pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoPair {
    /// Opening character.
    pub open: char,
    /// Closing character.
    pub close: char,
    /// Whether to auto-close on type.
    pub auto_close: bool,
    /// Whether to skip over closing char if next char matches.
    pub skip_close: bool,
}

/// Default auto-pairs for most languages.
pub fn default_pairs() -> Vec<AutoPair> {
    vec![
        AutoPair {
            open: '(',
            close: ')',
            auto_close: true,
            skip_close: true,
        },
        AutoPair {
            open: '[',
            close: ']',
            auto_close: true,
            skip_close: true,
        },
        AutoPair {
            open: '{',
            close: '}',
            auto_close: true,
            skip_close: true,
        },
        AutoPair {
            open: '"',
            close: '"',
            auto_close: true,
            skip_close: true,
        },
        AutoPair {
            open: '\'',
            close: '\'',
            auto_close: true,
            skip_close: true,
        },
        AutoPair {
            open: '`',
            close: '`',
            auto_close: true,
            skip_close: true,
        },
    ]
}

/// Check if a character should trigger auto-pairing.
pub fn should_auto_pair(ch: char, next_char: Option<char>, pairs: &[AutoPair]) -> Option<char> {
    for pair in pairs {
        if pair.open == ch && pair.auto_close {
            // Only auto-pair if next char is whitespace, closing bracket, or end of line
            match next_char {
                None | Some(' ') | Some('\t') | Some('\n') | Some(')') | Some(']') | Some('}') => {
                    return Some(pair.close)
                }
                _ => {}
            }
        }
    }
    None
}

/// Check if we should skip over a closing character.
pub fn should_skip_close(ch: char, next_char: Option<char>, pairs: &[AutoPair]) -> bool {
    for pair in pairs {
        if pair.close == ch && pair.skip_close && next_char == Some(ch) {
            return true;
        }
    }
    false
}

/// Comment toggle configuration.
#[derive(Debug, Clone)]
pub struct CommentConfig {
    /// Single-line comment prefix (e.g., "//", "#", "--").
    pub line_prefix: String,
    /// Block comment open (e.g., "/*").
    pub block_open: Option<String>,
    /// Block comment close (e.g., "*/").
    pub block_close: Option<String>,
}

impl CommentConfig {
    /// Get comment config for a file extension.
    pub fn for_extension(ext: &str) -> Self {
        match ext {
            "rs" | "c" | "cpp" | "h" | "hpp" | "java" | "js" | "ts" | "go" | "swift" | "kt" => {
                Self {
                    line_prefix: "//".to_string(),
                    block_open: Some("/*".to_string()),
                    block_close: Some("*/".to_string()),
                }
            }
            "py" | "rb" | "sh" | "bash" | "zsh" | "yaml" | "yml" | "toml" => Self {
                line_prefix: "#".to_string(),
                block_open: None,
                block_close: None,
            },
            "lua" | "sql" | "hs" => Self {
                line_prefix: "--".to_string(),
                block_open: Some("--[[".to_string()),
                block_close: Some("]]".to_string()),
            },
            "html" | "xml" | "svg" => Self {
                line_prefix: "<!--".to_string(),
                block_open: Some("<!--".to_string()),
                block_close: Some("-->".to_string()),
            },
            "css" | "scss" | "less" => Self {
                line_prefix: "//".to_string(),
                block_open: Some("/*".to_string()),
                block_close: Some("*/".to_string()),
            },
            "vim" => Self {
                line_prefix: "\"".to_string(),
                block_open: None,
                block_close: None,
            },
            _ => Self {
                line_prefix: "#".to_string(),
                block_open: None,
                block_close: None,
            },
        }
    }

    /// Toggle comment on a single line.
    pub fn toggle_line_comment(&self, line: &str) -> String {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];

        if let Some(rest) = trimmed.strip_prefix(&self.line_prefix) {
            // Remove comment
            let rest = rest.strip_prefix(' ').unwrap_or(rest);
            format!("{indent}{rest}")
        } else {
            // Add comment
            format!("{indent}{} {trimmed}", self.line_prefix)
        }
    }
}
