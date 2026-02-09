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
    pub fn load_config_file(&mut self, path: &str) {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return,
        };
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = trimmed.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                self.apply_config_value(key, value);
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
