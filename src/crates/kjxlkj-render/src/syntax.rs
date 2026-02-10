//! Language detection and regex-based syntax highlighting.
//!
//! Provides a fallback highlighter for when tree-sitter
//! grammars are not available. Detects language from file
//! extension and applies regex patterns for common tokens.

use crate::highlight::HighlightGroup;
use crate::lang_keywords::keywords;
use std::path::Path;

/// A highlight span covering a byte range in a line.
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
    pub group: HighlightGroup,
}

/// Detected language identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Lua,
    C,
    Cpp,
    Go,
    Toml,
    Markdown,
    Json,
    Yaml,
    Shell,
    Plain,
}

/// Detect language from file extension.
pub fn detect_language(path: &Path) -> Language {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    match ext {
        "rs" => Language::Rust,
        "py" | "pyi" => Language::Python,
        "js" | "jsx" | "mjs" => Language::JavaScript,
        "ts" | "tsx" => Language::TypeScript,
        "lua" => Language::Lua,
        "c" | "h" => Language::C,
        "cpp" | "cc" | "cxx" | "hpp" => Language::Cpp,
        "go" => Language::Go,
        "toml" => Language::Toml,
        "md" | "markdown" => Language::Markdown,
        "json" => Language::Json,
        "yml" | "yaml" => Language::Yaml,
        "sh" | "bash" | "zsh" => Language::Shell,
        _ => {
            // Check shebang would go here
            Language::Plain
        }
    }
}

/// Highlight a single line of source code.
///
/// Returns spans for keywords, strings, comments, and numbers.
/// This is a simple regex-free lexer for common patterns.
pub fn highlight_line(line: &str, lang: Language) -> Vec<HighlightSpan> {
    if lang == Language::Plain {
        return Vec::new();
    }
    let kw = keywords(lang);
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut spans = Vec::new();
    let mut i = 0;

    while i < len {
        let b = bytes[i];
        // Line comments
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            spans.push(HighlightSpan {
                start: i,
                end: len,
                group: HighlightGroup::Comment,
            });
            break;
        }
        // Python/shell comments
        if b == b'#'
            && matches!(
                lang,
                Language::Python | Language::Shell | Language::Toml | Language::Yaml
            )
        {
            spans.push(HighlightSpan {
                start: i,
                end: len,
                group: HighlightGroup::Comment,
            });
            break;
        }
        // Strings
        if b == b'"' || b == b'\'' {
            let quote = b;
            let start = i;
            i += 1;
            while i < len && bytes[i] != quote {
                if bytes[i] == b'\\' {
                    i += 1; // skip escaped char
                }
                i += 1;
            }
            if i < len {
                i += 1; // closing quote
            }
            spans.push(HighlightSpan {
                start,
                end: i,
                group: HighlightGroup::String,
            });
            continue;
        }
        // Numbers
        if b.is_ascii_digit() {
            let start = i;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.' || bytes[i] == b'_')
            {
                i += 1;
            }
            spans.push(HighlightSpan {
                start,
                end: i,
                group: HighlightGroup::Number,
            });
            continue;
        }
        // Identifiers / keywords
        if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &line[start..i];
            if kw.contains(&word) {
                spans.push(HighlightSpan {
                    start,
                    end: i,
                    group: HighlightGroup::Keyword,
                });
            } else if word == "true" || word == "false" {
                spans.push(HighlightSpan {
                    start,
                    end: i,
                    group: HighlightGroup::Boolean,
                });
            }
            continue;
        }
        i += 1;
    }
    spans
}
