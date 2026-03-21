#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::path::Path;

    use crate::core::content::slug::validate_slug;
    use crate::core::content::{
        ensure_unique_slug, path_for_slug, slug_from_stem, ContentValidationError,
    };

    #[test]
    fn slug_validation_rejects_non_kebab_input() {
        let error = validate_slug("Hello_World").unwrap_err();

        assert_eq!(error.code(), "E_CONTENT_SLUG_INVALID");
        assert_eq!(
            error,
            ContentValidationError::InvalidSlug {
                value: "Hello_World".to_owned()
            }
        );
    }

    #[test]
    fn slug_to_path_maps_to_markdown_file() {
        let path = path_for_slug(Path::new("data/content"), "hello-world").unwrap();
        assert_eq!(path, Path::new("data/content/hello-world.md"));
    }

    #[test]
    fn duplicate_slug_is_rejected() {
        let mut seen = HashSet::new();
        ensure_unique_slug("hello-world", &mut seen).unwrap();

        let error = ensure_unique_slug("hello-world", &mut seen).unwrap_err();
        assert_eq!(error.code(), "E_CONTENT_SLUG_DUPLICATE");
    }

    #[test]
    fn slug_from_stem_requires_lowercase_kebab_case() {
        let slug = slug_from_stem("docs-2025").unwrap();
        assert_eq!(slug, "docs-2025");
    }
}
