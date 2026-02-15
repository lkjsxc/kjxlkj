/// Metadata validation per /docs/spec/domain/metadata.md.

/// Maximum key length in characters.
pub const MAX_KEY_LEN: usize = 64;

/// Reserved key prefix (server-controlled).
pub const RESERVED_PREFIX: &str = "system.";

/// Maximum metadata value size in bytes.
pub const MAX_VALUE_SIZE: usize = 65_536;

/// Validate a metadata key.
/// Returns an error message if invalid, None if valid.
pub fn validate_key(key: &str) -> Option<&'static str> {
    if key.is_empty() {
        return Some("metadata key must not be empty");
    }
    if key.len() > MAX_KEY_LEN {
        return Some("metadata key exceeds 64 character limit");
    }
    if key.starts_with(RESERVED_PREFIX) {
        return Some("metadata key uses reserved system. prefix");
    }
    // Must be lowercase slug: [a-z0-9._-]
    if !key
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '_' || c == '-')
    {
        return Some("metadata key must be lowercase slug [a-z0-9._-]");
    }
    None
}

/// Validate metadata value size.
pub fn validate_value(value: &serde_json::Value) -> Option<&'static str> {
    let serialized = serde_json::to_string(value).unwrap_or_default();
    if serialized.len() > MAX_VALUE_SIZE {
        return Some("metadata value exceeds maximum size");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_key() {
        assert!(validate_key("my-key_01.sub").is_none());
    }

    #[test]
    fn test_reserved_prefix() {
        assert!(validate_key("system.internal").is_some());
    }

    #[test]
    fn test_uppercase_rejected() {
        assert!(validate_key("MyKey").is_some());
    }

    #[test]
    fn test_empty_rejected() {
        assert!(validate_key("").is_some());
    }
}
