//! Buffer-local variables.
//!
//! Variables scoped to individual buffers (b: prefix).

use std::collections::HashMap;

/// Variable value.
#[derive(Debug, Clone, PartialEq)]
pub enum VarValue {
    /// String value.
    String(String),
    /// Integer value.
    Int(i64),
    /// Float value.
    Float(f64),
    /// Boolean value.
    Bool(bool),
    /// List value.
    List(Vec<VarValue>),
    /// Dictionary value.
    Dict(HashMap<String, VarValue>),
}

impl VarValue {
    /// Creates a string value.
    pub fn string(s: &str) -> Self {
        Self::String(s.to_string())
    }

    /// Returns as string if possible.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns as int if possible.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns as bool if possible.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

/// Buffer variables.
#[derive(Debug, Clone, Default)]
pub struct BufferVars {
    /// Variables.
    vars: HashMap<String, VarValue>,
}

impl BufferVars {
    /// Creates new buffer variables.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a variable.
    pub fn set(&mut self, name: &str, value: VarValue) {
        self.vars.insert(name.to_string(), value);
    }

    /// Gets a variable.
    pub fn get(&self, name: &str) -> Option<&VarValue> {
        self.vars.get(name)
    }

    /// Removes a variable.
    pub fn remove(&mut self, name: &str) -> Option<VarValue> {
        self.vars.remove(name)
    }

    /// Lists all variable names.
    pub fn names(&self) -> Vec<&str> {
        self.vars.keys().map(|s| s.as_str()).collect()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }

    /// Returns count.
    pub fn len(&self) -> usize {
        self.vars.len()
    }
}

/// Window variables.
#[derive(Debug, Clone, Default)]
pub struct WindowVars {
    /// Variables.
    vars: HashMap<String, VarValue>,
}

impl WindowVars {
    /// Creates new window variables.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a variable.
    pub fn set(&mut self, name: &str, value: VarValue) {
        self.vars.insert(name.to_string(), value);
    }

    /// Gets a variable.
    pub fn get(&self, name: &str) -> Option<&VarValue> {
        self.vars.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_value_string() {
        let v = VarValue::string("hello");
        assert_eq!(v.as_str(), Some("hello"));
    }

    #[test]
    fn test_var_value_int() {
        let v = VarValue::Int(42);
        assert_eq!(v.as_int(), Some(42));
    }

    #[test]
    fn test_var_value_bool() {
        let v = VarValue::Bool(true);
        assert_eq!(v.as_bool(), Some(true));
    }

    #[test]
    fn test_buffer_vars_set_get() {
        let mut vars = BufferVars::new();
        vars.set("foo", VarValue::string("bar"));
        assert_eq!(vars.get("foo").and_then(|v| v.as_str()), Some("bar"));
    }

    #[test]
    fn test_buffer_vars_remove() {
        let mut vars = BufferVars::new();
        vars.set("foo", VarValue::Int(1));
        assert!(vars.remove("foo").is_some());
        assert!(vars.get("foo").is_none());
    }

    #[test]
    fn test_buffer_vars_len() {
        let mut vars = BufferVars::new();
        vars.set("a", VarValue::Int(1));
        vars.set("b", VarValue::Int(2));
        assert_eq!(vars.len(), 2);
    }
}
