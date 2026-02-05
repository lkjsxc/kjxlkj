//! Script file loading and execution for kjxlkj editor.
//!
//! Implements script file handling as specified in `/docs/spec/scripting/script-files.md`.
//!
//! This module provides:
//! - Script file location discovery
//! - Script parsing and validation
//! - Script execution environment
//! - Auto-load configuration handling

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Locations where script files can be found.
#[derive(Debug, Clone)]
pub struct ScriptLocations {
    /// User configuration directory.
    pub user_config_dir: Option<PathBuf>,
    /// User data directory.
    pub user_data_dir: Option<PathBuf>,
    /// Project-local config (relative to project root).
    pub project_local: Option<PathBuf>,
    /// Runtime script directory.
    pub runtime_dir: Option<PathBuf>,
}

impl Default for ScriptLocations {
    fn default() -> Self {
        Self {
            user_config_dir: dirs_next::config_dir().map(|d| d.join("kjxlkj")),
            user_data_dir: dirs_next::data_local_dir().map(|d| d.join("kjxlkj")),
            project_local: None, // Set when opening a project
            runtime_dir: None,   // Set at runtime
        }
    }
}

impl ScriptLocations {
    /// Create locations with a project root.
    pub fn with_project(project_root: &Path) -> Self {
        Self {
            project_local: Some(project_root.join(".kjxlkj")),
            ..Default::default()
        }
    }

    /// Get the primary user config file path.
    pub fn user_config_file(&self) -> Option<PathBuf> {
        self.user_config_dir.as_ref().map(|d| d.join("config.toml"))
    }

    /// Get the primary project config file path.
    pub fn project_config_file(&self) -> Option<PathBuf> {
        self.project_local.as_ref().map(|d| d.join("config.toml"))
    }

    /// Get all script directories in load order.
    pub fn script_dirs(&self) -> Vec<&Path> {
        let mut dirs = Vec::new();
        if let Some(d) = &self.runtime_dir {
            dirs.push(d.as_path());
        }
        if let Some(d) = &self.user_config_dir {
            dirs.push(d.as_path());
        }
        if let Some(d) = &self.user_data_dir {
            dirs.push(d.as_path());
        }
        if let Some(d) = &self.project_local {
            dirs.push(d.as_path());
        }
        dirs
    }
}

/// A parsed script file.
#[derive(Debug, Clone, Default)]
pub struct Script {
    /// Source path (if loaded from file).
    pub source: Option<PathBuf>,
    /// Settings section.
    pub settings: HashMap<String, SettingValue>,
    /// Commands to execute.
    pub commands: Vec<String>,
    /// Mappings to define.
    pub mappings: Vec<MappingDef>,
    /// Variables.
    pub variables: HashMap<String, String>,
}

impl Script {
    /// Create an empty script.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a script from TOML content.
    pub fn parse(content: &str) -> Result<Self, ScriptError> {
        let mut script = Script::new();

        // Simple line-based parsing for commands
        for line in content.lines() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Variable assignment
            if let Some(eq_pos) = line.find('=') {
                if !line[..eq_pos].contains(' ') {
                    let key = line[..eq_pos].trim();
                    let value = line[eq_pos + 1..].trim();
                    let value = value.trim_matches('"').trim_matches('\'');
                    script.variables.insert(key.to_string(), value.to_string());
                    continue;
                }
            }

            // Setting with :set
            if let Some(rest) = line.strip_prefix("set ") {
                let rest = rest.trim();
                if let Some(eq_pos) = rest.find('=') {
                    let key = rest[..eq_pos].trim();
                    let value = rest[eq_pos + 1..].trim();
                    script.settings.insert(
                        key.to_string(),
                        SettingValue::String(value.to_string()),
                    );
                } else if let Some(name) = rest.strip_prefix("no") {
                    script.settings.insert(
                        name.to_string(),
                        SettingValue::Bool(false),
                    );
                } else {
                    script.settings.insert(rest.to_string(), SettingValue::Bool(true));
                }
                continue;
            }

            // Mapping with map/nmap/imap etc.
            if line.starts_with("map ")
                || line.starts_with("nmap ")
                || line.starts_with("imap ")
                || line.starts_with("vmap ")
                || line.starts_with("nnoremap ")
                || line.starts_with("inoremap ")
                || line.starts_with("vnoremap ")
                || line.starts_with("noremap ")
            {
                if let Some(mapping) = parse_mapping_line(line) {
                    script.mappings.push(mapping);
                }
                continue;
            }

            // Otherwise treat as command
            script.commands.push(line.to_string());
        }

        Ok(script)
    }

    /// Load a script from a file path.
    pub fn load(path: &Path) -> Result<Self, ScriptError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ScriptError::Io(e.to_string()))?;
        let mut script = Self::parse(&content)?;
        script.source = Some(path.to_path_buf());
        Ok(script)
    }

    /// Get the number of items in the script.
    pub fn len(&self) -> usize {
        self.settings.len() + self.commands.len() + self.mappings.len()
    }

    /// Check if the script is empty.
    pub fn is_empty(&self) -> bool {
        self.settings.is_empty() && self.commands.is_empty() && self.mappings.is_empty()
    }
}

/// A setting value.
#[derive(Debug, Clone, PartialEq)]
pub enum SettingValue {
    /// Boolean setting.
    Bool(bool),
    /// Integer setting.
    Int(i64),
    /// String setting.
    String(String),
}

impl SettingValue {
    /// Get as bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            Self::Int(i) => Some(*i != 0),
            Self::String(s) => match s.as_str() {
                "true" | "yes" | "on" | "1" => Some(true),
                "false" | "no" | "off" | "0" => Some(false),
                _ => None,
            },
        }
    }

    /// Get as string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }
}

/// A mapping definition.
#[derive(Debug, Clone)]
pub struct MappingDef {
    /// Mapping mode (n, i, v, etc.).
    pub mode: char,
    /// Left-hand side (trigger).
    pub lhs: String,
    /// Right-hand side (replacement).
    pub rhs: String,
    /// Whether the mapping is recursive.
    pub recursive: bool,
}

/// Parse a mapping line.
fn parse_mapping_line(line: &str) -> Option<MappingDef> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let cmd = parts[0];
    let (mode, recursive) = match cmd {
        "map" => ('n', true),
        "nmap" => ('n', true),
        "nnoremap" => ('n', false),
        "imap" => ('i', true),
        "inoremap" => ('i', false),
        "vmap" => ('v', true),
        "vnoremap" => ('v', false),
        _ => ('n', true),
    };

    Some(MappingDef {
        mode,
        lhs: parts[1].to_string(),
        rhs: parts[2..].join(" "),
        recursive,
    })
}

/// Script loading and execution error.
#[derive(Debug, Clone)]
pub enum ScriptError {
    /// IO error.
    Io(String),
    /// Parse error.
    Parse(String),
    /// Execution error.
    Execution(String),
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "Script IO error: {}", msg),
            Self::Parse(msg) => write!(f, "Script parse error: {}", msg),
            Self::Execution(msg) => write!(f, "Script execution error: {}", msg),
        }
    }
}

impl std::error::Error for ScriptError {}

/// Script loader managing script discovery and loading.
#[derive(Debug, Default)]
pub struct ScriptLoader {
    /// Known locations.
    locations: ScriptLocations,
    /// Loaded scripts.
    loaded: HashMap<PathBuf, Script>,
}

impl ScriptLoader {
    /// Create a new script loader.
    pub fn new(locations: ScriptLocations) -> Self {
        Self {
            locations,
            loaded: HashMap::new(),
        }
    }

    /// Load a script by path.
    pub fn load(&mut self, path: &Path) -> Result<&Script, ScriptError> {
        if !self.loaded.contains_key(path) {
            let script = Script::load(path)?;
            self.loaded.insert(path.to_path_buf(), script);
        }
        Ok(self.loaded.get(path).unwrap())
    }

    /// Load the user config file if it exists.
    pub fn load_user_config(&mut self) -> Option<Result<&Script, ScriptError>> {
        let path = self.locations.user_config_file()?;
        if path.exists() {
            Some(self.load(&path))
        } else {
            None
        }
    }

    /// Get all loaded scripts.
    pub fn loaded_scripts(&self) -> impl Iterator<Item = (&Path, &Script)> {
        self.loaded.iter().map(|(p, s)| (p.as_path(), s))
    }

    /// Check if a script is already loaded.
    pub fn is_loaded(&self, path: &Path) -> bool {
        self.loaded.contains_key(path)
    }

    /// Get the locations.
    pub fn locations(&self) -> &ScriptLocations {
        &self.locations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_locations_default() {
        let loc = ScriptLocations::default();
        assert!(loc.user_config_dir.is_some());
        assert!(loc.user_data_dir.is_some());
        assert!(loc.project_local.is_none());
    }

    #[test]
    fn test_script_locations_with_project() {
        let loc = ScriptLocations::with_project(Path::new("/project"));
        assert!(loc.project_local.is_some());
        assert!(loc.project_local.as_ref().unwrap().ends_with(".kjxlkj"));
    }

    #[test]
    fn test_script_locations_user_config_file() {
        let loc = ScriptLocations::default();
        let path = loc.user_config_file();
        assert!(path.is_some());
        assert!(path.unwrap().ends_with("config.toml"));
    }

    #[test]
    fn test_script_new() {
        let script = Script::new();
        assert!(script.is_empty());
        assert_eq!(script.len(), 0);
    }

    #[test]
    fn test_script_parse_empty() {
        let script = Script::parse("").unwrap();
        assert!(script.is_empty());
    }

    #[test]
    fn test_script_parse_comments() {
        let script = Script::parse("# comment\n# another").unwrap();
        assert!(script.is_empty());
    }

    #[test]
    fn test_script_parse_variable() {
        let script = Script::parse("myvar=hello").unwrap();
        assert_eq!(script.variables.get("myvar"), Some(&"hello".to_string()));
    }

    #[test]
    fn test_script_parse_setting() {
        let script = Script::parse("set number").unwrap();
        assert_eq!(
            script.settings.get("number"),
            Some(&SettingValue::Bool(true))
        );
    }

    #[test]
    fn test_script_parse_setting_no() {
        let script = Script::parse("set nonumber").unwrap();
        assert_eq!(
            script.settings.get("number"),
            Some(&SettingValue::Bool(false))
        );
    }

    #[test]
    fn test_script_parse_setting_value() {
        let script = Script::parse("set tabstop=4").unwrap();
        assert_eq!(
            script.settings.get("tabstop"),
            Some(&SettingValue::String("4".to_string()))
        );
    }

    #[test]
    fn test_script_parse_command() {
        let script = Script::parse("echo hello").unwrap();
        assert_eq!(script.commands.len(), 1);
        assert_eq!(script.commands[0], "echo hello");
    }

    #[test]
    fn test_script_parse_mapping() {
        let script = Script::parse("nmap jj <Esc>").unwrap();
        assert_eq!(script.mappings.len(), 1);
        assert_eq!(script.mappings[0].mode, 'n');
        assert_eq!(script.mappings[0].lhs, "jj");
        assert_eq!(script.mappings[0].rhs, "<Esc>");
    }

    #[test]
    fn test_setting_value_as_bool() {
        assert_eq!(SettingValue::Bool(true).as_bool(), Some(true));
        assert_eq!(SettingValue::Int(1).as_bool(), Some(true));
        assert_eq!(SettingValue::Int(0).as_bool(), Some(false));
        assert_eq!(SettingValue::String("yes".to_string()).as_bool(), Some(true));
        assert_eq!(SettingValue::String("no".to_string()).as_bool(), Some(false));
    }

    #[test]
    fn test_setting_value_as_str() {
        assert_eq!(
            SettingValue::String("test".to_string()).as_str(),
            Some("test")
        );
        assert_eq!(SettingValue::Bool(true).as_str(), None);
    }

    #[test]
    fn test_mapping_def_recursive() {
        let script = Script::parse("nmap jj :echo test").unwrap();
        assert_eq!(script.mappings.len(), 1);
        assert!(script.mappings[0].recursive);
    }

    #[test]
    fn test_mapping_def_nonrecursive() {
        let script = Script::parse("nnoremap jj :echo test").unwrap();
        assert_eq!(script.mappings.len(), 1);
        assert!(!script.mappings[0].recursive);
    }

    #[test]
    fn test_script_error_display() {
        let err = ScriptError::Io("file not found".to_string());
        assert!(err.to_string().contains("file not found"));

        let err = ScriptError::Parse("bad syntax".to_string());
        assert!(err.to_string().contains("bad syntax"));
    }

    #[test]
    fn test_script_loader_new() {
        let loader = ScriptLoader::new(ScriptLocations::default());
        assert!(loader.loaded_scripts().count() == 0);
    }

    #[test]
    fn test_script_loader_is_loaded() {
        let loader = ScriptLoader::new(ScriptLocations::default());
        assert!(!loader.is_loaded(Path::new("/nonexistent")));
    }

    #[test]
    fn test_script_len() {
        let mut script = Script::new();
        script.commands.push("echo test".to_string());
        script.settings.insert("num".to_string(), SettingValue::Bool(true));
        assert_eq!(script.len(), 2);
        assert!(!script.is_empty());
    }
}
