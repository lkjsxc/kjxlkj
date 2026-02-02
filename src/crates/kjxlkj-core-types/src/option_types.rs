//! Option types for buffer-local options.
//!
//! Types for representing option scope and values.

/// Option scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionScope {
    /// Global option.
    Global,
    /// Buffer-local option.
    Buffer,
    /// Window-local option.
    Window,
}

/// A typed option value.
#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue {
    /// Boolean option.
    Bool(bool),
    /// Integer option.
    Int(i64),
    /// String option.
    String(String),
}

impl OptionValue {
    /// Returns the boolean value if this is a Bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            OptionValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the integer value if this is an Int.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            OptionValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the string value if this is a String.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            OptionValue::String(v) => Some(v),
            _ => None,
        }
    }
}
