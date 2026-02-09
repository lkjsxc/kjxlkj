//! Config file loading (config.toml) on startup.
//!
//! Reads a simple key = value TOML-like format and applies
//! settings to the OptionStore.

use crate::editor::EditorState;

impl EditorState {
    /// Load configuration from the default config path.
    pub fn load_config_default(&mut self) {
        let paths = [
            dirs_config("kjxlkj/config.toml"),
            Some("config.toml".to_string()),
        ];
        for p in paths.iter().flatten() {
            if std::path::Path::new(p).exists() {
                self.load_config_file(p);
                return;
            }
        }
    }

    /// Load configuration from a specific path.
    /// Supports [section] headers; keys become "section.key".
    pub fn load_config_file(&mut self, path: &str) {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return,
        };
        let mut section = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // Section header: [section]
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                section = trimmed[1..trimmed.len() - 1].trim().to_string();
                continue;
            }
            if let Some((key, value)) = trimmed.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                let full_key = if section.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", section, key)
                };
                self.apply_config_value(&full_key, value);
            }
        }
    }

    fn apply_config_value(&mut self, key: &str, value: &str) {
        // Try bool.
        if value == "true" || value == "false" {
            let b = value == "true";
            self.options.set(key, crate::options::OptionValue::Bool(b));
            return;
        }
        // Try integer.
        if let Ok(n) = value.parse::<usize>() {
            self.options.set(key, crate::options::OptionValue::Int(n));
            return;
        }
        // Array value: [a, b, c] → stored as comma-separated string.
        if value.starts_with('[') && value.ends_with(']') {
            let inner = &value[1..value.len() - 1];
            let items: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            let joined = items.join(",");
            self.options
                .set(key, crate::options::OptionValue::Str(joined));
            return;
        }
        // Strip quotes for string values.
        let s = value
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .unwrap_or(value);
        self.options
            .set(key, crate::options::OptionValue::Str(s.to_string()));
    }
}

fn dirs_config(sub: &str) -> Option<String> {
    std::env::var("XDG_CONFIG_HOME")
        .ok()
        .or_else(|| std::env::var("HOME").ok().map(|h| format!("{}/.config", h)))
        .map(|base| format!("{}/{}", base, sub))
}

/// Detect filetype from a file path based on extension.
#[rustfmt::skip]
pub fn detect_filetype(path: &str) -> Option<&'static str> {
    let ext = std::path::Path::new(path).extension()?.to_str()?;
    Some(match ext {
        "rs" => "rust", "py" => "python", "js" => "javascript",
        "ts" => "typescript", "tsx" => "typescriptreact",
        "jsx" => "javascriptreact", "rb" => "ruby",
        "go" => "go", "c" | "h" => "c", "cpp" | "cc" | "cxx" | "hpp" => "cpp",
        "java" => "java", "lua" => "lua", "sh" | "bash" | "zsh" => "sh",
        "json" => "json", "toml" => "toml", "yaml" | "yml" => "yaml",
        "md" | "markdown" => "markdown", "html" | "htm" => "html",
        "css" => "css", "xml" => "xml", "sql" => "sql",
        "vim" => "vim", "el" => "lisp", "hs" => "haskell",
        "ml" | "mli" => "ocaml", "ex" | "exs" => "elixir",
        "erl" => "erlang", "zig" => "zig", "dart" => "dart",
        "swift" => "swift", "kt" | "kts" => "kotlin",
        "r" | "R" => "r", "jl" => "julia",
        "txt" => "text", "csv" => "csv",
        _ => return None,
    })
}
