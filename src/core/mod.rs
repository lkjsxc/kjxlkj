mod model;
#[cfg(test)]
mod tests;
mod validate;

pub use model::{Record, RecordInput};
pub use validate::{normalize_tags, validate_id, validate_input};
