//! Editor state aggregation and snapshot production.

mod editor;
mod registers;
mod marks;

pub use editor::EditorState;
pub use registers::RegisterStore;
pub use marks::MarkStore;
