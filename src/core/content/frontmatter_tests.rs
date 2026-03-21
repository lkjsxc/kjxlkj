#[cfg(test)]
mod tests {
    use crate::core::content::{
        parse_markdown_document, serialize_markdown_document, ContentValidationError, Frontmatter,
    };

    #[test]
    fn parse_without_frontmatter_defaults_private_to_false() {
        let parsed = parse_markdown_document("# Hello").unwrap();

        assert_eq!(parsed.frontmatter, Frontmatter::default());
        assert_eq!(parsed.body, "# Hello");
    }

    #[test]
    fn parse_with_supported_frontmatter_keys() {
        let markdown = "---\ntitle: \"Welcome\"\nprivate: true\n---\n# Body";
        let parsed = parse_markdown_document(markdown).unwrap();

        assert_eq!(parsed.frontmatter.title.as_deref(), Some("Welcome"));
        assert!(parsed.frontmatter.private);
        assert_eq!(parsed.body, "# Body");
    }

    #[test]
    fn serialize_outputs_deterministic_key_order() {
        let frontmatter = Frontmatter {
            title: Some("Welcome".to_owned()),
            private: true,
        };

        let serialized = serialize_markdown_document(&frontmatter, "# Body");
        assert_eq!(
            serialized,
            "---\ntitle: \"Welcome\"\nprivate: true\n---\n# Body"
        );
    }

    #[test]
    fn parse_rejects_unknown_key_with_stable_error() {
        let markdown = "---\nunknown: value\n---\nbody";
        let error = parse_markdown_document(markdown).unwrap_err();

        assert_eq!(error.code(), "E_CONTENT_FRONTMATTER_KEY");
        assert_eq!(
            error,
            ContentValidationError::FrontmatterUnknownKey {
                key: "unknown".to_owned()
            }
        );
    }
}
