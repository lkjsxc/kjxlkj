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

    /// Load ftplugin file for the given filetype (e.g., `ftplugin/rust.vim`).
    /// Searches XDG config, then current directory, then runtime path.
    pub fn load_ftplugin(&mut self, filetype: &str) {
        let candidates = [
            dirs_config(&format!("kjxlkj/ftplugin/{filetype}.vim")),
            Some(format!("ftplugin/{filetype}.vim")),
        ];
        for p in candidates.iter().flatten() {
            if std::path::Path::new(p).exists() {
                self.handle_source(p);
                return;
            }
        }
    }

    /// Load indent plugin for the given filetype from ftplugin/indent directory.
    /// Sets indent-related options (shiftwidth, tabstop, expandtab, etc.).
    #[rustfmt::skip]
    pub fn load_indent_plugin(&mut self, filetype: &str) {
        let candidates = [
            dirs_config(&format!("kjxlkj/indent/{filetype}.vim")),
            Some(format!("indent/{filetype}.vim")),
            dirs_config(&format!("kjxlkj/ftplugin/{filetype}_indent.vim")),
            Some(format!("ftplugin/{filetype}_indent.vim")),
        ];
        for p in candidates.iter().flatten() {
            if std::path::Path::new(p).exists() { self.handle_source(p); return; }
        }
        // Apply built-in indent defaults for common filetypes.
        self.apply_builtin_indent(filetype);
    }

    #[rustfmt::skip]
    fn apply_builtin_indent(&mut self, filetype: &str) {
        use crate::options::OptionValue;
        let (sw, ts, et) = match filetype {
            "rust" | "go" | "c" | "cpp" | "java" => (4, 4, true),
            "python" | "yaml" | "json" | "toml" => (4, 4, true),
            "javascript" | "typescript" | "html" | "css" | "jsx" |
            "typescriptreact" | "javascriptreact" => (2, 2, true),
            "ruby" | "elixir" | "lua" | "dart" | "kotlin" | "swift" => (2, 2, true),
            "haskell" | "ocaml" | "julia" | "r" => (2, 2, true),
            "sh" | "vim" | "markdown" => (4, 4, true),
            "makefile" => (8, 8, false),
            _ => return,
        };
        self.options.set("shiftwidth", OptionValue::Int(sw));
        self.options.set("tabstop", OptionValue::Int(ts));
        self.options.set("expandtab", OptionValue::Bool(et));
        if let Some(cs) = commentstring_for_filetype(filetype) {
            self.options.set("commentstring", OptionValue::Str(cs.into()));
        }
    }

    /// Try to auto-restore a session: load Session.vim if present in cwd.
    pub fn try_auto_restore_session(&mut self) {
        if std::path::Path::new("Session.vim").exists() { self.handle_source("Session.vim"); }
    }

    /// Load local vimrc (.exrc) from current directory if `exrc` option is set.
    /// Also checks for `.vimrc` and `.nvimrc` in cwd.
    pub fn load_local_exrc(&mut self) {
        if !self.options.get_bool("exrc") { return; }
        for name in &[".exrc", ".vimrc", ".nvimrc"] {
            if std::path::Path::new(name).exists() {
                self.handle_source(name);
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

/// Returns the commentstring for a given filetype (format: `// %s`, `# %s`, etc.).
#[rustfmt::skip]
pub fn commentstring_for_filetype(ft: &str) -> Option<&'static str> {
    Some(match ft {
        "rust" | "c" | "cpp" | "java" | "javascript" | "typescript" | "go" | "dart" |
        "swift" | "kotlin" | "zig" | "css" | "typescriptreact" | "javascriptreact" => "// %s",
        "python" | "ruby" | "sh" | "yaml" | "toml" | "elixir" | "r" | "julia" | "csv" => "# %s",
        "lua" | "haskell" | "sql" => "-- %s",
        "vim" => "\" %s", "lisp" | "clojure" => "; %s",
        "html" | "xml" => "<!-- %s -->", "erlang" | "ocaml" => "(* %s *)",
        _ => return None,
    })
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
