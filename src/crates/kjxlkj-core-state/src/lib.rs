//! Editor state aggregation and snapshot production.

mod buffer_state;
mod editor;
mod marks;
mod registers;

pub use buffer_state::BufferState;
pub use editor::EditorState;
pub use marks::MarkStore;
pub use registers::RegisterStore;
