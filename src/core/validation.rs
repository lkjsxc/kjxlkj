//! Validation logic for records

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use thiserror::Error;

/// Regex for valid record IDs: lowercase kebab-case
static ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

/// Minimum ID length
const ID_MIN_LEN: usize = 3;

/// Maximum ID length
const ID_MAX_LEN: usize = 48;

/// Error type for ID validation
#[derive(Debug, Error, PartialEq)]
pub enum IdError {
    #[error("id must be at least {ID_MIN_LEN} characters")]
    TooShort,
    #[error("id must be at most {ID_MAX_LEN} characters")]
    TooLong,
    #[error("id must be lowercase kebab-case (a-z, 0-9, hyphens)")]
    InvalidFormat,
}

/// Validate a record ID
pub fn validate_id(id: &str) -> Result<(), IdError> {
    if id.len() < ID_MIN_LEN {
        return Err(IdError::TooShort);
    }
    if id.len() > ID_MAX_LEN {
        return Err(IdError::TooLong);
    }
    if !ID_REGEX.is_match(id) {
        return Err(IdError::InvalidFormat);
    }
    Ok(())
}

/// Normalize and deduplicate tags
pub fn validate_tags(tags: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    tags.into_iter()
        .map(|t| t.to_lowercase())
        .filter(|t| !t.is_empty() && seen.insert(t.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_ids() {
        assert!(validate_id("abc").is_ok());
        assert!(validate_id("test-id").is_ok());
        assert!(validate_id("my-long-record-id-123").is_ok());
        assert!(validate_id("a1b2c3").is_ok());
    }

    #[test]
    fn too_short_id() {
        assert_eq!(validate_id("ab"), Err(IdError::TooShort));
        assert_eq!(validate_id("a"), Err(IdError::TooShort));
        assert_eq!(validate_id(""), Err(IdError::TooShort));
    }

    #[test]
    fn too_long_id() {
        let long_id = "a".repeat(49);
        assert_eq!(validate_id(&long_id), Err(IdError::TooLong));
    }

    #[test]
    fn invalid_format() {
        assert_eq!(validate_id("UPPERCASE"), Err(IdError::InvalidFormat));
        assert_eq!(validate_id("has spaces"), Err(IdError::InvalidFormat));
        assert_eq!(validate_id("has_underscore"), Err(IdError::InvalidFormat));
        assert_eq!(validate_id("-starts-hyphen"), Err(IdError::InvalidFormat));
        assert_eq!(validate_id("ends-hyphen-"), Err(IdError::InvalidFormat));
    }

    #[test]
    fn tag_normalization() {
        let tags = vec!["TAG".to_string(), "tag".to_string(), "Other".to_string()];
        let result = validate_tags(tags);
        assert_eq!(result, vec!["tag", "other"]);
    }

    #[test]
    fn empty_tags_filtered() {
        let tags = vec!["".to_string(), "valid".to_string(), "".to_string()];
        let result = validate_tags(tags);
        assert_eq!(result, vec!["valid"]);
    }
}
