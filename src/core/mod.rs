//! Core domain models and validation

mod validation;

pub use validation::{
    derive_summary, derive_title, extract_title, generate_id, validate_id, IdError,
};
