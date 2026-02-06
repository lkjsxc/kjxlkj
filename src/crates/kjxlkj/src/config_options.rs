/// Config option parsing, defaults, scopes, and option value types.
use std::collections::HashMap;

/// Scope of an option setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptionScope { Global, Buffer, Window }

/// Value of a configuration option.
#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue {
    Bool(bool),
    Int(i64),
    Str(String),
}

impl OptionValue {
    pub fn as_bool(&self) -> Option<bool> { if let OptionValue::Bool(v) = self { Some(*v) } else { None } }
    pub fn as_int(&self) -> Option<i64> { if let OptionValue::Int(v) = self { Some(*v) } else { None } }
    pub fn as_str(&self) -> Option<&str> { if let OptionValue::Str(v) = self { Some(v) } else { None } }
}

/// An option definition.
#[derive(Debug, Clone)]
pub struct OptionDef {
    pub name: String,
    pub scope: OptionScope,
    pub default: OptionValue,
    pub short: Option<String>,
}

/// Configuration store.
#[derive(Debug, Clone)]
pub struct ConfigStore {
    defs: HashMap<String, OptionDef>,
    values: HashMap<(OptionScope, String), OptionValue>,
}

impl ConfigStore {
    pub fn new() -> Self { Self { defs: HashMap::new(), values: HashMap::new() } }

    pub fn define(&mut self, name: &str, scope: OptionScope, default: OptionValue, short: Option<&str>) {
        self.defs.insert(name.to_string(), OptionDef {
            name: name.to_string(), scope, default, short: short.map(|s| s.to_string()),
        });
    }

    pub fn get(&self, name: &str, scope: OptionScope) -> Option<&OptionValue> {
        self.values.get(&(scope, name.to_string()))
            .or_else(|| self.defs.get(name).map(|d| &d.default))
    }

    pub fn set(&mut self, name: &str, scope: OptionScope, value: OptionValue) -> bool {
        if !self.defs.contains_key(name) { return false; }
        self.values.insert((scope, name.to_string()), value);
        true
    }

    pub fn resolve(&self, name: &str) -> Option<&str> {
        for d in self.defs.values() {
            if d.name == name { return Some(&d.name); }
            if d.short.as_deref() == Some(name) { return Some(&d.name); }
        }
        None
    }

    pub fn all_names(&self) -> Vec<&str> { self.defs.keys().map(|s| s.as_str()).collect() }
}

/// Parse a `:set` command argument.
pub fn parse_set_arg(arg: &str) -> SetAction {
    let trimmed = arg.trim();
    if trimmed.is_empty() { return SetAction::ShowAll; }
    if let Some(name) = trimmed.strip_prefix("no") {
        if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric()) {
            return SetAction::SetBool(name.to_string(), false);
        }
    }
    if trimmed.ends_with('?') {
        return SetAction::Query(trimmed.trim_end_matches('?').to_string());
    }
    if let Some((name, val)) = trimmed.split_once('=') {
        if let Ok(n) = val.parse::<i64>() {
            return SetAction::SetInt(name.to_string(), n);
        }
        return SetAction::SetStr(name.to_string(), val.to_string());
    }
    if trimmed.chars().all(|c| c.is_alphanumeric()) {
        return SetAction::SetBool(trimmed.to_string(), true);
    }
    SetAction::Invalid(trimmed.to_string())
}

/// Result of parsing a `:set` argument.
#[derive(Debug, Clone, PartialEq)]
pub enum SetAction {
    ShowAll,
    Query(String),
    SetBool(String, bool),
    SetInt(String, i64),
    SetStr(String, String),
    Invalid(String),
}

/// Build default editor options.
pub fn build_defaults(store: &mut ConfigStore) {
    store.define("number", OptionScope::Window, OptionValue::Bool(true), Some("nu"));
    store.define("relativenumber", OptionScope::Window, OptionValue::Bool(false), Some("rnu"));
    store.define("wrap", OptionScope::Window, OptionValue::Bool(false), None);
    store.define("scrolloff", OptionScope::Global, OptionValue::Int(8), Some("so"));
    store.define("tabstop", OptionScope::Buffer, OptionValue::Int(4), Some("ts"));
    store.define("shiftwidth", OptionScope::Buffer, OptionValue::Int(4), Some("sw"));
    store.define("expandtab", OptionScope::Buffer, OptionValue::Bool(true), Some("et"));
    store.define("ignorecase", OptionScope::Global, OptionValue::Bool(false), Some("ic"));
    store.define("smartcase", OptionScope::Global, OptionValue::Bool(true), Some("scs"));
    store.define("clipboard", OptionScope::Global, OptionValue::Str("unnamedplus".into()), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn define_and_get_default() {
        let mut s = ConfigStore::new();
        s.define("wrap", OptionScope::Window, OptionValue::Bool(false), None);
        assert_eq!(s.get("wrap", OptionScope::Window).unwrap().as_bool(), Some(false));
    }

    #[test]
    fn set_and_get() {
        let mut s = ConfigStore::new();
        s.define("scrolloff", OptionScope::Global, OptionValue::Int(8), Some("so"));
        s.set("scrolloff", OptionScope::Global, OptionValue::Int(5));
        assert_eq!(s.get("scrolloff", OptionScope::Global).unwrap().as_int(), Some(5));
    }

    #[test]
    fn resolve_short_name() {
        let mut s = ConfigStore::new();
        s.define("number", OptionScope::Window, OptionValue::Bool(true), Some("nu"));
        assert_eq!(s.resolve("nu"), Some("number"));
    }

    #[test]
    fn set_unknown_fails() {
        let mut s = ConfigStore::new();
        assert!(!s.set("unknown", OptionScope::Global, OptionValue::Bool(true)));
    }

    #[test]
    fn parse_set_bool_on() { assert_eq!(parse_set_arg("number"), SetAction::SetBool("number".into(), true)); }

    #[test]
    fn parse_set_bool_off() { assert_eq!(parse_set_arg("nonumber"), SetAction::SetBool("number".into(), false)); }

    #[test]
    fn parse_set_int() { assert_eq!(parse_set_arg("scrolloff=5"), SetAction::SetInt("scrolloff".into(), 5)); }

    #[test]
    fn parse_set_str() { assert_eq!(parse_set_arg("clipboard=unnamed"), SetAction::SetStr("clipboard".into(), "unnamed".into())); }

    #[test]
    fn parse_query() { assert_eq!(parse_set_arg("wrap?"), SetAction::Query("wrap".into())); }

    #[test]
    fn build_defaults_count() {
        let mut s = ConfigStore::new();
        build_defaults(&mut s);
        assert!(s.all_names().len() >= 10);
    }
}
