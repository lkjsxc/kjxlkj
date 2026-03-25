//! Core domain models and validation

mod validation;

pub use validation::{extract_title, generate_slug, validate_slug, SlugError};
