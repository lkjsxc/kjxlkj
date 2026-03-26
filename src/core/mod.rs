//! Core domain models and validation

mod editor;
mod markdown;
mod validation;

pub use editor::{editor_document, EditorDocument, RichBlock, RichBlockKind};
pub use markdown::render_markdown;
pub use validation::{
    derive_summary, derive_title, extract_title, generate_id, validate_id, IdError,
};
