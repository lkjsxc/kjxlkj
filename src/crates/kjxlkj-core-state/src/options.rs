//! Editor options storage and :set command support.
//!
//! Options are typed values (bool, usize, String) stored
//! by name. The `:set` command modifies them at runtime.

use std::collections::HashMap;

/// A single option value.
#[derive(Debug, Clone)]
pub enum OptionValue {
    Bool(bool),
    Int(usize),
    Str(String),
}

impl std::fmt::Display for OptionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{v}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Str(v) => write!(f, "{v}"),
        }
    }
}

/// Storage for editor options.
#[derive(Debug, Clone)]
pub struct OptionStore {
    values: HashMap<String, OptionValue>,
}

impl OptionStore {
    pub fn new() -> Self {
        let mut s = Self {
            values: HashMap::new(),
        };
        s.set_defaults();
        s
    }

    fn set_defaults(&mut self) {
        self.set("tabstop", OptionValue::Int(8));
        self.set("shiftwidth", OptionValue::Int(8));
        self.set("expandtab", OptionValue::Bool(false));
        self.set("number", OptionValue::Bool(false));
        self.set("relativenumber", OptionValue::Bool(false));
        self.set("wrap", OptionValue::Bool(true));
        self.set("ignorecase", OptionValue::Bool(false));
        self.set("smartcase", OptionValue::Bool(false));
        self.set("hlsearch", OptionValue::Bool(true));
        self.set("incsearch", OptionValue::Bool(true));
        self.set("autoindent", OptionValue::Bool(true));
        self.set("scrolloff", OptionValue::Int(0));
        self.set("textwidth", OptionValue::Int(79));
        self.set("formatoptions", OptionValue::Str("tcq".to_string()));
        self.set("filetype", OptionValue::Str(String::new()));
    }

    /// Set an option value.
    pub fn set(&mut self, name: &str, value: OptionValue) {
        self.values.insert(name.to_string(), value);
    }

    /// Get an option value.
    pub fn get(&self, name: &str) -> Option<&OptionValue> {
        self.values.get(name)
    }

    /// Get a boolean option (default false).
    pub fn get_bool(&self, name: &str) -> bool {
        match self.get(name) {
            Some(OptionValue::Bool(v)) => *v,
            _ => false,
        }
    }

    /// Get an integer option (default 0).
    pub fn get_int(&self, name: &str) -> usize {
        match self.get(name) {
            Some(OptionValue::Int(v)) => *v,
            _ => 0,
        }
    }

    /// Get a string option (default empty).
    pub fn get_str(&self, name: &str) -> &str {
        match self.get(name) {
            Some(OptionValue::Str(v)) => v.as_str(),
            _ => "",
        }
    }

    /// Check if an option name is known.
    pub fn is_known(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// List all option names and values.
    pub fn list(&self) -> Vec<(String, OptionValue)> {
        let mut items: Vec<_> = self
            .values
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        items
    }
}

impl Default for OptionStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse and execute a `:set` command argument.
/// Returns a human-readable result or error message.
pub fn parse_set_command(opts: &mut OptionStore, args: &str) -> Result<Option<String>, String> {
    let args = args.trim();
    if args.is_empty() || args == "all" {
        let items = opts.list();
        let lines: Vec<String> = items.iter().map(|(k, v)| format!("  {k}={v}")).collect();
        return Ok(Some(lines.join("\n")));
    }
    // :set option?
    if let Some(name) = args.strip_suffix('?') {
        let name = name.trim();
        return match opts.get(name) {
            Some(v) => Ok(Some(format!("  {name}={v}"))),
            None => Err(format!("E518: Unknown option: {name}")),
        };
    }
    // :set nooption (unset boolean)
    if let Some(name) = args.strip_prefix("no") {
        if opts.is_known(name) {
            opts.set(name, OptionValue::Bool(false));
            return Ok(None);
        }
    }
    // :set option=value
    if let Some((name, val_str)) = args.split_once('=') {
        let name = name.trim();
        if !opts.is_known(name) {
            return Err(format!("E518: Unknown option: {name}"));
        }
        let val_str = val_str.trim();
        if let Ok(n) = val_str.parse::<usize>() {
            opts.set(name, OptionValue::Int(n));
        } else {
            opts.set(name, OptionValue::Str(val_str.to_string()));
        }
        return Ok(None);
    }
    // :set option (enable boolean)
    let name = args.trim();
    if opts.is_known(name) {
        opts.set(name, OptionValue::Bool(true));
        return Ok(None);
    }
    Err(format!("E518: Unknown option: {name}"))
}
