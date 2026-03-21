use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentValidationError {
    FrontmatterUnclosed,
    FrontmatterInvalidLine { line: usize },
    FrontmatterUnknownKey { key: String },
    FrontmatterDuplicateKey { key: String },
    FrontmatterEmptyTitle,
    FrontmatterInvalidPrivate { value: String },
    InvalidSlug { value: String },
    DuplicateSlug { slug: String },
}

impl ContentValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::FrontmatterUnclosed => "E_CONTENT_FRONTMATTER_UNCLOSED",
            Self::FrontmatterInvalidLine { .. } => "E_CONTENT_FRONTMATTER_LINE",
            Self::FrontmatterUnknownKey { .. } => "E_CONTENT_FRONTMATTER_KEY",
            Self::FrontmatterDuplicateKey { .. } => "E_CONTENT_FRONTMATTER_DUPLICATE_KEY",
            Self::FrontmatterEmptyTitle => "E_CONTENT_FRONTMATTER_EMPTY_TITLE",
            Self::FrontmatterInvalidPrivate { .. } => "E_CONTENT_FRONTMATTER_PRIVATE",
            Self::InvalidSlug { .. } => "E_CONTENT_SLUG_INVALID",
            Self::DuplicateSlug { .. } => "E_CONTENT_SLUG_DUPLICATE",
        }
    }
}

impl Display for ContentValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FrontmatterUnclosed => {
                write!(f, "frontmatter block is missing closing delimiter")
            }
            Self::FrontmatterInvalidLine { line } => {
                write!(f, "frontmatter line {line} must contain key: value")
            }
            Self::FrontmatterUnknownKey { key } => {
                write!(f, "unsupported frontmatter key: {key}")
            }
            Self::FrontmatterDuplicateKey { key } => {
                write!(f, "frontmatter key appears multiple times: {key}")
            }
            Self::FrontmatterEmptyTitle => write!(f, "frontmatter title must not be empty"),
            Self::FrontmatterInvalidPrivate { value } => {
                write!(f, "frontmatter private must be true or false, got: {value}")
            }
            Self::InvalidSlug { value } => {
                write!(f, "slug must be lowercase kebab-case, got: {value}")
            }
            Self::DuplicateSlug { slug } => write!(f, "duplicate slug encountered: {slug}"),
        }
    }
}

impl std::error::Error for ContentValidationError {}
