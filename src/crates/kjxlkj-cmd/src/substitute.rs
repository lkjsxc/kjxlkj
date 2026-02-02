//! Substitute command implementation.
//!
//! Handles :s/pattern/replacement/flags style substitutions.

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

/// A parsed substitute command.
#[derive(Debug, Clone)]
pub struct SubstituteCommand {
    /// Search pattern.
    pub pattern: String,
    /// Replacement string.
    pub replacement: String,
    /// Flags.
    pub flags: SubstituteFlags,
}

impl SubstituteCommand {
    /// Creates a new substitute command.
    pub fn new(pattern: &str, replacement: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            replacement: replacement.to_string(),
            flags: SubstituteFlags::default(),
        }
    }

    /// Sets the flags.
    pub fn with_flags(mut self, flags: SubstituteFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Parses a substitute command string like "s/pattern/replacement/flags".
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();

        // Handle abbreviated forms.
        let input = if input.starts_with("s/") || input.starts_with("s!") {
            &input[1..]
        } else if input.starts_with("substitute") {
            &input[10..].trim_start()
        } else {
            return None;
        };

        // Determine delimiter.
        let delimiter = input.chars().next()?;
        let parts: Vec<&str> = input[1..].split(delimiter).collect();

        let pattern = parts.first()?.to_string();
        let replacement = parts.get(1).unwrap_or(&"").to_string();
        let flags_str = parts.get(2).unwrap_or(&"");
        let flags = SubstituteFlags::parse(flags_str);

        Some(Self {
            pattern,
            replacement,
            flags,
        })
    }

    /// Performs substitution on a single line.
    pub fn substitute_line(&self, line: &str) -> (String, usize) {
        let mut result = line.to_string();
        let mut count = 0;

        if self.pattern.is_empty() {
            return (result, 0);
        }

        if self.flags.global {
            // Replace all occurrences.
            while let Some(pos) = result.find(&self.pattern) {
                result.replace_range(pos..pos + self.pattern.len(), &self.replacement);
                count += 1;
            }
        } else {
            // Replace first occurrence only.
            if let Some(pos) = result.find(&self.pattern) {
                result.replace_range(pos..pos + self.pattern.len(), &self.replacement);
                count = 1;
            }
        }

        (result, count)
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
    fn test_substitute_command_parse() {
        let cmd = SubstituteCommand::parse("s/foo/bar/g").unwrap();
        assert_eq!(cmd.pattern, "foo");
        assert_eq!(cmd.replacement, "bar");
        assert!(cmd.flags.global);
    }

    #[test]
    fn test_substitute_command_parse_different_delimiter() {
        let cmd = SubstituteCommand::parse("s!foo!bar!").unwrap();
        assert_eq!(cmd.pattern, "foo");
        assert_eq!(cmd.replacement, "bar");
    }

    #[test]
    fn test_substitute_line_single() {
        let cmd = SubstituteCommand::new("foo", "bar");
        let (result, count) = cmd.substitute_line("foo foo foo");
        assert_eq!(result, "bar foo foo");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_substitute_line_global() {
        let cmd = SubstituteCommand::new("foo", "bar")
            .with_flags(SubstituteFlags::parse("g"));
        let (result, count) = cmd.substitute_line("foo foo foo");
        assert_eq!(result, "bar bar bar");
        assert_eq!(count, 3);
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
