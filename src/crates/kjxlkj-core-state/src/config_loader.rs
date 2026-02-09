//! Config file loading (config.toml), ftplugin, indent, exrc on startup.
use crate::editor::EditorState;

impl EditorState {
    /// Load configuration from the default config path.
    #[rustfmt::skip]
    pub fn load_config_default(&mut self) {
        for p in [dirs_config("kjxlkj/config.toml"), Some("config.toml".into())].iter().flatten() {
            if std::path::Path::new(p).exists() { self.load_config_file(p); return; }
        }
    }
    /// Load ftplugin file for the given filetype.
    #[rustfmt::skip]
    pub fn load_ftplugin(&mut self, ft: &str) {
        for p in [dirs_config(&format!("kjxlkj/ftplugin/{ft}.vim")), Some(format!("ftplugin/{ft}.vim"))].iter().flatten() {
            if std::path::Path::new(p).exists() { self.handle_source(p); return; }
        }
    }
    /// Load indent plugin then fall back to built-in defaults.
    #[rustfmt::skip]
    pub fn load_indent_plugin(&mut self, ft: &str) {
        for p in [dirs_config(&format!("kjxlkj/indent/{ft}.vim")), Some(format!("indent/{ft}.vim")),
            dirs_config(&format!("kjxlkj/ftplugin/{ft}_indent.vim")), Some(format!("ftplugin/{ft}_indent.vim"))].iter().flatten() {
            if std::path::Path::new(p).exists() { self.handle_source(p); return; }
        }
        self.apply_builtin_indent(ft);
    }
    #[rustfmt::skip]
    fn apply_builtin_indent(&mut self, ft: &str) {
        use crate::options::OptionValue;
        let (sw, ts, et) = match ft {
            "rust" | "go" | "c" | "cpp" | "java" | "python" | "yaml" | "json" | "toml" | "sh" | "vim" | "markdown" => (4, 4, true),
            "javascript" | "typescript" | "html" | "css" | "jsx" | "typescriptreact" | "javascriptreact" |
            "ruby" | "elixir" | "lua" | "dart" | "kotlin" | "swift" | "haskell" | "ocaml" | "julia" | "r" => (2, 2, true),
            "makefile" => (8, 8, false),
            _ => return,
        };
        self.options.set("shiftwidth", OptionValue::Int(sw));
        self.options.set("tabstop", OptionValue::Int(ts));
        self.options.set("expandtab", OptionValue::Bool(et));
        if let Some(cs) = commentstring_for_filetype(ft) { self.options.set("commentstring", OptionValue::Str(cs.into())); }
    }
    pub fn try_auto_restore_session(&mut self) { if std::path::Path::new("Session.vim").exists() { self.handle_source("Session.vim"); } }
    /// Load local exrc; when `secure` option is set, skip dangerous commands.
    pub fn load_local_exrc(&mut self) {
        if !self.options.get_bool("exrc") { return; }
        let secure = self.options.get_bool("secure");
        for name in &[".exrc", ".vimrc", ".nvimrc"] {
            if std::path::Path::new(name).exists() {
                if secure { self.handle_source_secure(name); } else { self.handle_source(name); }
                return;
            }
        }
    }
    /// Source file in secure mode — skip dangerous commands (!, autocmd, write, wq, source).
    #[rustfmt::skip]
    fn handle_source_secure(&mut self, path: &str) {
        let content = match std::fs::read_to_string(path) { Ok(c) => c, Err(_) => return };
        let dangerous = ["!", "autocmd", "write", "wq", "source", "call system("];
        for line in content.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('"') { continue; }
            if dangerous.iter().any(|d| t.starts_with(d) || t.contains(&format!(":{d}"))) { continue; }
            self.execute_ex_command(t);
        }
    }
    /// Load config file with [section] headers; keys become "section.key".
    pub fn load_config_file(&mut self, path: &str) {
        let content = match std::fs::read_to_string(path) { Ok(c) => c, Err(_) => return };
        let mut section = String::new();
        for line in content.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') { continue; }
            if t.starts_with('[') && t.ends_with(']') { section = t[1..t.len()-1].trim().to_string(); continue; }
            if let Some((key, value)) = t.split_once('=') {
                let (key, value) = (key.trim(), value.trim());
                let full_key = if section.is_empty() { key.to_string() } else { format!("{}.{}", section, key) };
                self.apply_config_value(&full_key, value);
            }
        }
    }
    #[rustfmt::skip]
    fn apply_config_value(&mut self, key: &str, value: &str) {
        use crate::options::OptionValue;
        if value == "true" || value == "false" { self.options.set(key, OptionValue::Bool(value == "true")); return; }
        if let Ok(n) = value.parse::<usize>() { self.options.set(key, OptionValue::Int(n)); return; }
        if value.starts_with('[') && value.ends_with(']') { let inner = &value[1..value.len()-1]; self.options.set(key, OptionValue::Str(inner.split(',').map(|s| s.trim()).collect::<Vec<_>>().join(","))); return; }
        let s = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')).unwrap_or(value);
        self.options.set(key, OptionValue::Str(s.to_string()));
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
