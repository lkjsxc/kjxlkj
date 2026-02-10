//! Editor state and action dispatch.
//!
//! This crate owns the mutable EditorState and provides the
//! core loop's state mutation surface.

mod buffer_list;
mod editor;
pub mod explorer;
pub mod session;
mod tab_page;
mod window_tree;

mod ops;
#[cfg(test)]
mod tests;

pub use editor::EditorState;
pub use explorer::ExplorerState;
pub use session::SessionData;
