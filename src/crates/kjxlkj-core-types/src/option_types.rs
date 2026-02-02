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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_scope() {
        assert_ne!(OptionScope::Global, OptionScope::Buffer);
        assert_ne!(OptionScope::Buffer, OptionScope::Window);
    }

    #[test]
    fn test_option_value_accessors() {
        let b = OptionValue::Bool(true);
        let i = OptionValue::Int(42);
        let s = OptionValue::String("test".to_string());

        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        assert_eq!(i.as_int(), Some(42));
        assert_eq!(i.as_string(), None);
        assert_eq!(s.as_string(), Some("test"));
        assert_eq!(s.as_bool(), None);
    }
}
