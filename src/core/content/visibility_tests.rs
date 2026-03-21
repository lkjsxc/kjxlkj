#[cfg(test)]
mod tests {
    use crate::core::content::{is_visible, Frontmatter, VisibilityContext};

    #[test]
    fn public_context_hides_private_content() {
        let frontmatter = Frontmatter {
            title: None,
            private: true,
        };

        assert!(!is_visible(&frontmatter, VisibilityContext::Public));
    }

    #[test]
    fn admin_context_sees_private_content() {
        let frontmatter = Frontmatter {
            title: Some("Draft".to_owned()),
            private: true,
        };

        assert!(is_visible(&frontmatter, VisibilityContext::Admin));
    }
}
