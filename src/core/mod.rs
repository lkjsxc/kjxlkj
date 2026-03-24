//! Core domain models and validation

mod record;
mod validation;

pub use record::{Record, RecordInput};
pub use validation::{validate_id, validate_tags, IdError};
