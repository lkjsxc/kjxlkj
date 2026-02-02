//! Validation types.

/// Validation result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    /// Input is valid.
    Valid,
    /// Input is invalid with reason.
    Invalid(String),
}

impl ValidationResult {
    /// Returns whether the result is valid.
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Returns the error message if invalid.
    pub fn error(&self) -> Option<&str> {
        match self {
            Self::Invalid(msg) => Some(msg),
            Self::Valid => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_valid() {
        let r = ValidationResult::Valid;
        assert!(r.is_valid());
        assert!(r.error().is_none());
    }

    #[test]
    fn test_validation_result_invalid() {
        let r = ValidationResult::Invalid("error".into());
        assert!(!r.is_valid());
        assert_eq!(r.error(), Some("error"));
    }
}
