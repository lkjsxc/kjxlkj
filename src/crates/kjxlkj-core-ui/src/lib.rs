//! UI model types for kjxlkj editor.
//!
//! This crate defines UI snapshot structures for rendering.

mod buffer_view;
mod completion;
mod conceal;
mod cursor;
mod dimensions;
mod float;
mod layout;
mod popup;
mod scroll;
mod sign;
mod snapshot;
mod status;
mod viewport;
mod virtual_text;

pub use buffer_view::BufferView;
pub use completion::{CompletionItem, CompletionKind, CompletionState};
pub use conceal::{ConcealLevel, ConcealRegion, ConcealState, LineConceal};
pub use cursor::{CursorAppearance, CursorBlink, CursorShape};
pub use dimensions::Dimensions;
pub use float::{FloatAnchor, FloatBorder, FloatConfig, FloatRelative, FloatState, FloatWindow};
pub use layout::{Layout, LayoutNode, SplitDirection};
pub use popup::{PopupItem, PopupMenu, PopupState};
pub use scroll::{CursorPosition, ScrollAmount, ScrollDirection, ScrollState};
pub use sign::{Sign, SignColumn, SignDefinition, SignPriority};
pub use snapshot::EditorSnapshot;
pub use status::StatusLine;
pub use viewport::Viewport;
pub use virtual_text::{VirtualText, VirtualTextChunk, VirtualTextPos, VirtualTextState};
