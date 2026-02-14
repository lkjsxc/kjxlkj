// Workspace service per /docs/spec/domain/workspaces.md
// Orchestrates workspace, project, and view operations.

/// Workspace slug validation: lowercase alphanumeric + hyphens.
pub fn validate_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.len() <= 64
        && slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !slug.starts_with('-')
        && !slug.ends_with('-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_slug() {
        assert!(validate_slug("my-workspace"));
        assert!(validate_slug("ws1"));
    }

    #[test]
    fn test_invalid_slug() {
        assert!(!validate_slug(""));
        assert!(!validate_slug("-dash-start"));
        assert!(!validate_slug("UPPERCASE"));
    }
}
