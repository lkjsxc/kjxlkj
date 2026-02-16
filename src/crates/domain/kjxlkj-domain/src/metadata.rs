/// Typed metadata records per /docs/spec/domain/metadata.md
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Per-note metadata entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataEntry {
    pub note_id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
    pub updated_at: NaiveDateTime,
}

/// Input for upserting metadata
#[derive(Debug, Clone, Deserialize)]
pub struct UpsertMetadataInput {
    pub key: String,
    pub value: serde_json::Value,
}

/// Validation: key must be lowercase slug, max 64 chars
pub fn validate_metadata_key(key: &str) -> Result<(), String> {
    if key.is_empty() || key.len() > 64 {
        return Err("key must be 1-64 characters".into());
    }
    if !key
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
    {
        return Err("key must be lowercase slug".into());
    }
    Ok(())
}

/// Reserved prefixes are server-controlled
pub fn is_reserved_prefix(key: &str) -> bool {
    key.starts_with("system.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_key_validation() {
        assert!(validate_metadata_key("valid_key").is_ok());
        assert!(validate_metadata_key("system.created").is_ok());
        assert!(validate_metadata_key("").is_err());
        assert!(validate_metadata_key("UPPER").is_err());
        let long = "a".repeat(65);
        assert!(validate_metadata_key(&long).is_err());
    }

    #[test]
    fn test_reserved_prefix() {
        assert!(is_reserved_prefix("system.tag"));
        assert!(!is_reserved_prefix("user.tag"));
    }
}
