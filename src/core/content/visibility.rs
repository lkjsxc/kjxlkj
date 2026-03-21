use super::frontmatter::Frontmatter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityContext {
    Public,
    Admin,
}

pub fn is_visible(frontmatter: &Frontmatter, context: VisibilityContext) -> bool {
    match context {
        VisibilityContext::Public => !frontmatter.private,
        VisibilityContext::Admin => true,
    }
}
