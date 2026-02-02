//! UI model types for kjxlkj editor.
//!
//! This crate defines UI snapshot structures for rendering.

mod buffer_view;
mod completion;
mod conceal;
mod cursor;
mod dimensions;
mod layout;
mod scroll;
mod sign;
mod snapshot;
mod status;
mod viewport;

pub use buffer_view::BufferView;
pub use completion::{CompletionItem, CompletionKind, CompletionState};
pub use conceal::{ConcealLevel, ConcealRegion, ConcealState, LineConceal};
pub use cursor::{CursorAppearance, CursorBlink, CursorShape};
pub use dimensions::Dimensions;
pub use layout::{Layout, LayoutNode, SplitDirection};
pub use scroll::{CursorPosition, ScrollAmount, ScrollDirection, ScrollState};
pub use sign::{Sign, SignColumn, SignDefinition, SignPriority};
pub use snapshot::EditorSnapshot;
pub use status::StatusLine;
pub use viewport::Viewport;
