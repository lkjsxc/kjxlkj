#[cfg(test)]
mod tests {
    use crate::core::content::{
        parse_markdown_document, private_or_default, serialize_markdown_document,
        ContentValidationError, Frontmatter,
    };

    #[test]
    fn parse_without_frontmatter_defaults_private_to_true() {
        let parsed = parse_markdown_document("# Hello").unwrap();

        assert_eq!(
            parsed.frontmatter,
            Frontmatter {
                title: None,
                private: true
            }
        );
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
    fn serialize_omits_frontmatter_for_private_default_without_title() {
        let frontmatter = Frontmatter {
            title: None,
            private: true,
        };
        let serialized = serialize_markdown_document(&frontmatter, "# Body");

        assert_eq!(serialized, "# Body");
    }

    #[test]
    fn serialize_preserves_public_flag_without_title() {
        let frontmatter = Frontmatter {
            title: None,
            private: false,
        };
        let serialized = serialize_markdown_document(&frontmatter, "# Body");
        let reparsed = parse_markdown_document(&serialized).unwrap();

        assert_eq!(serialized, "---\nprivate: false\n---\n# Body");
        assert!(!reparsed.frontmatter.private);
    }

    #[test]
    fn private_or_default_uses_private_true_for_unspecified_input() {
        assert!(private_or_default(None));
        assert!(private_or_default(Some(true)));
        assert!(!private_or_default(Some(false)));
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
