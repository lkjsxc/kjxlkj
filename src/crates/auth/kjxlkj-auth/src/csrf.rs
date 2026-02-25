//! CSRF token generation and validation

use uuid::Uuid;

/// Generate a new CSRF token
pub fn generate_csrf_token() -> String {
    Uuid::new_v4().to_string()
}

/// Validate CSRF token
pub fn validate_csrf_token(provided: &str, expected: &str) -> bool {
    provided == expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csrf_token_generation() {
        let token1 = generate_csrf_token();
        let token2 = generate_csrf_token();
        assert_ne!(token1, token2);
        assert_eq!(token1.len(), 36); // UUID length
    }

    #[test]
    fn test_csrf_token_validation() {
        let token = generate_csrf_token();
        assert!(validate_csrf_token(&token, &token));
        assert!(!validate_csrf_token(&token, "invalid"));
    }
}
