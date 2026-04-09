//! Core domain models and validation

mod markdown;
#[cfg(test)]
mod markdown_tests;
mod validation;
#[cfg(test)]
mod validation_tests;

pub use markdown::render_markdown;
pub use validation::{
    derive_summary, derive_title, derive_title_with_fallback, extract_title, generate_id,
    looks_like_id, normalize_alias, validate_id, AliasError, IdError,
};
