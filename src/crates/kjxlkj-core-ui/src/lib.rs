//! UI model types for kjxlkj editor.
//!
//! This crate defines UI snapshot structures for rendering.

mod buffer_view;
mod dimensions;
mod layout;
mod snapshot;
mod status;
mod viewport;

pub use buffer_view::BufferView;
pub use dimensions::Dimensions;
pub use layout::{Layout, LayoutNode, SplitDirection};
pub use snapshot::EditorSnapshot;
pub use status::StatusLine;
pub use viewport::Viewport;
