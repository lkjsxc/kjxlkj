//! Completion item kind.

use serde::{Deserialize, Serialize};

/// Completion item kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionKind {
    /// Text completion.
    Text,
    /// Method/function.
    Method,
    /// Function.
    Function,
    /// Constructor.
    Constructor,
    /// Field.
    Field,
    /// Variable.
    Variable,
    /// Class.
    Class,
    /// Interface.
    Interface,
    /// Module.
    Module,
    /// Property.
    Property,
    /// Unit.
    Unit,
    /// Value.
    Value,
    /// Enum.
    Enum,
    /// Keyword.
    Keyword,
    /// Snippet.
    Snippet,
    /// Color.
    Color,
    /// File.
    File,
    /// Reference.
    Reference,
    /// Folder.
    Folder,
    /// Enum member.
    EnumMember,
    /// Constant.
    Constant,
    /// Struct.
    Struct,
    /// Event.
    Event,
    /// Operator.
    Operator,
    /// Type parameter.
    TypeParameter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_kind_eq() {
        assert_eq!(CompletionKind::Function, CompletionKind::Function);
        assert_ne!(CompletionKind::Function, CompletionKind::Method);
    }

    #[test]
    fn test_completion_kind_clone() {
        let kind = CompletionKind::Variable;
        let cloned = kind;
        assert_eq!(kind, cloned);
    }
}
