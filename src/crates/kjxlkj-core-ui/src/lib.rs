//! UI model types for kjxlkj editor.
//!
//! This crate defines UI snapshot structures for rendering.

mod buffer_view;
mod completion;
mod cursor;
mod dimensions;
mod layout;
mod scroll;
mod snapshot;
mod status;
mod viewport;

pub use buffer_view::BufferView;
pub use completion::{CompletionItem, CompletionKind, CompletionState};
pub use cursor::{CursorAppearance, CursorBlink, CursorShape};
pub use dimensions::Dimensions;
pub use layout::{Layout, LayoutNode, SplitDirection};
pub use scroll::{CursorPosition, ScrollAmount, ScrollDirection, ScrollState};
pub use snapshot::EditorSnapshot;
pub use status::StatusLine;
pub use viewport::Viewport;
