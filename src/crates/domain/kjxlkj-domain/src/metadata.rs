use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

/// Per-note key-value metadata per docs/spec/domain/metadata.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataEntry {
    pub note_id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

/// Max key length per specification.
pub const MAX_METADATA_KEY_LEN: usize = 64;

/// Reserved prefix for server-controlled keys.
pub const RESERVED_PREFIX: &str = "system.";

/// Validate metadata key constraints.
pub fn validate_key(key: &str) -> Result<(), String> {
    if key.is_empty() {
        return Err("key must not be empty".into());
    }
    if key.len() > MAX_METADATA_KEY_LEN {
        return Err(format!("key exceeds max length {MAX_METADATA_KEY_LEN}"));
    }
    if key.starts_with(RESERVED_PREFIX) {
        return Err("key uses reserved system. prefix".into());
    }
    // Enforce lowercase slug pattern
    if !key.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.') {
        return Err("key must be lowercase slug".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_key_valid() {
        assert!(validate_key("my_key").is_ok());
        assert!(validate_key("key.subkey").is_ok());
    }

    #[test]
    fn test_validate_key_reserved() {
        assert!(validate_key("system.foo").is_err());
    }

    #[test]
    fn test_validate_key_too_long() {
        let long = "a".repeat(65);
        assert!(validate_key(&long).is_err());
    }
}
