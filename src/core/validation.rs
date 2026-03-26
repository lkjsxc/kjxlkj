//! Validation logic for note ids and derived fields

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;
use uuid::Uuid;

static ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z0-9_-]{22}$").unwrap());

const ID_LEN: usize = 22;
const SUMMARY_LIMIT: usize = 120;

#[derive(Debug, Error, PartialEq)]
pub enum IdError {
    #[error("id must be exactly {ID_LEN} characters")]
    InvalidLength,
    #[error("id must be Base64URL without padding")]
    InvalidFormat,
}

pub fn validate_id(id: &str) -> Result<(), IdError> {
    if id.len() != ID_LEN {
        return Err(IdError::InvalidLength);
    }
    if !ID_REGEX.is_match(id) {
        return Err(IdError::InvalidFormat);
    }
    let decoded = URL_SAFE_NO_PAD
        .decode(id)
        .map_err(|_| IdError::InvalidFormat)?;
    if decoded.len() != 16 {
        return Err(IdError::InvalidFormat);
    }
    Ok(())
}

pub fn generate_id() -> String {
    URL_SAFE_NO_PAD.encode(Uuid::new_v4().as_bytes())
}

pub fn extract_title(body: &str) -> Option<String> {
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("# ") {
            return Some(title.to_string());
        }
        if trimmed.starts_with('#') || !trimmed.is_empty() {
            break;
        }
    }
    None
}

pub fn derive_title(body: &str) -> String {
    extract_title(body).unwrap_or_else(|| "Untitled note".to_string())
}

pub fn derive_summary(body: &str) -> String {
    body.lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .map(shorten)
        .unwrap_or_else(|| "No summary yet.".to_string())
}

fn shorten(line: &str) -> String {
    if line.chars().count() <= SUMMARY_LIMIT {
        line.to_string()
    } else {
        let prefix: String = line.chars().take(SUMMARY_LIMIT - 1).collect();
        format!("{prefix}…")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_ids() {
        let id = generate_id();
        assert_eq!(id.len(), 22);
        assert!(validate_id(&id).is_ok());
    }

    #[test]
    fn invalid_ids() {
        assert_eq!(validate_id("short"), Err(IdError::InvalidLength));
        assert_eq!(
            validate_id("contains+plus-sign____"),
            Err(IdError::InvalidFormat)
        );
    }

    #[test]
    fn title_and_summary_derivation() {
        assert_eq!(derive_title("# Hello\n\nBody"), "Hello".to_string());
        assert_eq!(derive_title(""), "Untitled note".to_string());
        assert_eq!(derive_summary("# Hello\n\nBody"), "Body".to_string());
        assert_eq!(derive_summary(""), "No summary yet.".to_string());
    }
}
