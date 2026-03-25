//! Validation logic for records

use chrono::Utc;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;

/// Regex for valid slugs: lowercase kebab-case
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

/// Minimum slug length
const SLUG_MIN_LEN: usize = 3;

/// Maximum slug length
const SLUG_MAX_LEN: usize = 64;

/// Error type for slug validation
#[derive(Debug, Error, PartialEq)]
pub enum SlugError {
    #[error("slug must be at least {SLUG_MIN_LEN} characters")]
    TooShort,
    #[error("slug must be at most {SLUG_MAX_LEN} characters")]
    TooLong,
    #[error("slug must be lowercase kebab-case (a-z, 0-9, hyphens)")]
    InvalidFormat,
}

/// Validate a record slug
pub fn validate_slug(slug: &str) -> Result<(), SlugError> {
    if slug.len() < SLUG_MIN_LEN {
        return Err(SlugError::TooShort);
    }
    if slug.len() > SLUG_MAX_LEN {
        return Err(SlugError::TooLong);
    }
    if !SLUG_REGEX.is_match(slug) {
        return Err(SlugError::InvalidFormat);
    }
    Ok(())
}

/// Generate a slug from current datetime (format: YYYY-MM-DD-HHmm)
pub fn generate_slug() -> String {
    Utc::now().format("%Y-%m-%d-%H%M").to_string()
}

/// Extract title from body (first # heading line)
pub fn extract_title(body: &str) -> Option<String> {
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("# ") {
            return Some(title.to_string());
        }
        if trimmed.starts_with("# ") || !trimmed.is_empty() {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_slugs() {
        assert!(validate_slug("abc").is_ok());
        assert!(validate_slug("test-slug").is_ok());
        assert!(validate_slug("2026-03-25-0134").is_ok());
        assert!(validate_slug("a1b2c3").is_ok());
    }

    #[test]
    fn too_short_slug() {
        assert_eq!(validate_slug("ab"), Err(SlugError::TooShort));
        assert_eq!(validate_slug("a"), Err(SlugError::TooShort));
        assert_eq!(validate_slug(""), Err(SlugError::TooShort));
    }

    #[test]
    fn too_long_slug() {
        let long_slug = "a".repeat(65);
        assert_eq!(validate_slug(&long_slug), Err(SlugError::TooLong));
    }

    #[test]
    fn invalid_format() {
        assert_eq!(validate_slug("UPPERCASE"), Err(SlugError::InvalidFormat));
        assert_eq!(validate_slug("has spaces"), Err(SlugError::InvalidFormat));
        assert_eq!(
            validate_slug("has_underscore"),
            Err(SlugError::InvalidFormat)
        );
        assert_eq!(
            validate_slug("-starts-hyphen"),
            Err(SlugError::InvalidFormat)
        );
        assert_eq!(validate_slug("ends-hyphen-"), Err(SlugError::InvalidFormat));
    }

    #[test]
    fn extract_title_from_body() {
        assert_eq!(
            extract_title("# Hello World\n\nContent"),
            Some("Hello World".to_string())
        );
        assert_eq!(extract_title("# Title"), Some("Title".to_string()));
        assert_eq!(extract_title("No heading here"), None);
        assert_eq!(extract_title(""), None);
    }

    #[test]
    fn generate_slug_format() {
        let slug = generate_slug();
        assert!(validate_slug(&slug).is_ok());
        assert!(slug.len() >= 15); // YYYY-MM-DD-HHmm
    }
}
