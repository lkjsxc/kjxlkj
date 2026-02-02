//! Input validation for user commands and settings.

use std::path::Path;

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

/// Validates a buffer name.
pub fn validate_buffer_name(name: &str) -> ValidationResult {
    if name.is_empty() {
        return ValidationResult::Invalid("Buffer name cannot be empty".into());
    }
    if name.len() > 4096 {
        return ValidationResult::Invalid("Buffer name too long".into());
    }
    ValidationResult::Valid
}

/// Validates a file path.
pub fn validate_file_path(path: &str) -> ValidationResult {
    if path.is_empty() {
        return ValidationResult::Invalid("File path cannot be empty".into());
    }
    if path.contains('\0') {
        return ValidationResult::Invalid("File path contains null byte".into());
    }
    let p = Path::new(path);
    if p.is_absolute() && !p.starts_with("/") && !p.starts_with("~") {
        return ValidationResult::Invalid("Invalid absolute path".into());
    }
    ValidationResult::Valid
}

/// Validates a register name.
pub fn validate_register(name: char) -> ValidationResult {
    match name {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '"' | '*' | '+' | '_' | '/' | '-' | '%' | '#' | '='
        | '.' | ':' => ValidationResult::Valid,
        _ => ValidationResult::Invalid(format!("Invalid register: {}", name)),
    }
}

/// Validates a mark name.
pub fn validate_mark(name: char) -> ValidationResult {
    match name {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '\'' | '`' | '[' | ']' | '<' | '>' | '"' | '^'
        | '.' => ValidationResult::Valid,
        _ => ValidationResult::Invalid(format!("Invalid mark: {}", name)),
    }
}

/// Validates a count value.
pub fn validate_count(count: usize) -> ValidationResult {
    if count == 0 {
        return ValidationResult::Invalid("Count cannot be zero".into());
    }
    if count > 1_000_000 {
        return ValidationResult::Invalid("Count too large".into());
    }
    ValidationResult::Valid
}

/// Validates a line number.
pub fn validate_line_number(line: usize, max_lines: usize) -> ValidationResult {
    if line >= max_lines {
        return ValidationResult::Invalid(format!(
            "Line {} out of range (0..{})",
            line, max_lines
        ));
    }
    ValidationResult::Valid
}

/// Validates a column number.
pub fn validate_column(col: usize, line_length: usize) -> ValidationResult {
    if col > line_length {
        return ValidationResult::Invalid(format!(
            "Column {} out of range (0..={})",
            col, line_length
        ));
    }
    ValidationResult::Valid
}

/// Validates a window split ratio.
pub fn validate_split_ratio(ratio: f64) -> ValidationResult {
    if !(0.1..=0.9).contains(&ratio) {
        return ValidationResult::Invalid("Split ratio must be between 0.1 and 0.9".into());
    }
    ValidationResult::Valid
}

/// Validates a tab width.
pub fn validate_tab_width(width: usize) -> ValidationResult {
    if width == 0 || width > 16 {
        return ValidationResult::Invalid("Tab width must be 1-16".into());
    }
    ValidationResult::Valid
}

/// Validates a regex pattern.
pub fn validate_pattern(pattern: &str) -> ValidationResult {
    if pattern.is_empty() {
        return ValidationResult::Invalid("Pattern cannot be empty".into());
    }
    // Basic syntax check.
    let mut depth = 0i32;
    for ch in pattern.chars() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return ValidationResult::Invalid("Unmatched closing paren".into());
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return ValidationResult::Invalid("Unmatched opening paren".into());
    }
    ValidationResult::Valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_buffer_name_valid() {
        assert!(validate_buffer_name("test.rs").is_valid());
    }

    #[test]
    fn test_validate_buffer_name_empty() {
        assert!(!validate_buffer_name("").is_valid());
    }

    #[test]
    fn test_validate_file_path_valid() {
        assert!(validate_file_path("/home/user/file.txt").is_valid());
    }

    #[test]
    fn test_validate_file_path_null() {
        assert!(!validate_file_path("foo\0bar").is_valid());
    }

    #[test]
    fn test_validate_register_valid() {
        assert!(validate_register('a').is_valid());
        assert!(validate_register('"').is_valid());
        assert!(validate_register('+').is_valid());
    }

    #[test]
    fn test_validate_register_invalid() {
        assert!(!validate_register('!').is_valid());
    }

    #[test]
    fn test_validate_mark_valid() {
        assert!(validate_mark('a').is_valid());
        assert!(validate_mark('A').is_valid());
        assert!(validate_mark('\'').is_valid());
    }

    #[test]
    fn test_validate_count() {
        assert!(validate_count(1).is_valid());
        assert!(!validate_count(0).is_valid());
        assert!(!validate_count(10_000_000).is_valid());
    }

    #[test]
    fn test_validate_pattern() {
        assert!(validate_pattern("foo").is_valid());
        assert!(validate_pattern("(foo)").is_valid());
        assert!(!validate_pattern("(foo").is_valid());
    }

    #[test]
    fn test_validate_tab_width() {
        assert!(validate_tab_width(4).is_valid());
        assert!(!validate_tab_width(0).is_valid());
        assert!(!validate_tab_width(20).is_valid());
    }
}
