//! Substitute command types.

use std::collections::HashMap;

/// Substitute flags.
#[derive(Debug, Clone, Default)]
pub struct SubstituteFlags {
    /// Global (replace all on line).
    pub global: bool,
    /// Case insensitive.
    pub ignore_case: bool,
    /// Confirm each replacement.
    pub confirm: bool,
    /// Print changed lines.
    pub print: bool,
    /// Use previous pattern.
    pub use_previous: bool,
}

impl SubstituteFlags {
    /// Parses flags from a string.
    pub fn parse(flags: &str) -> Self {
        let mut result = Self::default();
        for ch in flags.chars() {
            match ch {
                'g' => result.global = true,
                'i' | 'I' => result.ignore_case = true,
                'c' => result.confirm = true,
                'p' => result.print = true,
                '&' => result.use_previous = true,
                _ => {}
            }
        }
        result
    }
}

/// Result of a substitution operation.
#[derive(Debug, Clone, Default)]
pub struct SubstituteResult {
    /// Lines changed.
    pub lines_changed: usize,
    /// Total substitutions.
    pub substitutions: usize,
    /// Changed line numbers and new content.
    pub changes: HashMap<usize, String>,
}

impl SubstituteResult {
    /// Returns whether any substitutions were made.
    pub fn has_changes(&self) -> bool {
        self.substitutions > 0
    }

    /// Returns a summary message.
    pub fn message(&self) -> String {
        if self.substitutions == 0 {
            "Pattern not found".to_string()
        } else if self.substitutions == 1 {
            "1 substitution on 1 line".to_string()
        } else {
            format!(
                "{} substitutions on {} lines",
                self.substitutions, self.lines_changed
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitute_flags_parse() {
        let flags = SubstituteFlags::parse("gi");
        assert!(flags.global);
        assert!(flags.ignore_case);
    }

    #[test]
    fn test_substitute_result_message() {
        let mut result = SubstituteResult::default();
        assert_eq!(result.message(), "Pattern not found");

        result.substitutions = 1;
        result.lines_changed = 1;
        assert!(result.message().contains("1 substitution"));
    }
}
