//! UI model types for kjxlkj editor.
//!
//! This crate defines UI snapshot structures for rendering.

mod buffer_view;
mod completion;
mod completion_source;
mod conceal;
mod cursor;
mod dimensions;
mod float;
mod fold;
mod highlight;
mod layout;
mod popup;
mod scroll;
mod sign;
mod snapshot;
mod status;
mod terminal;
mod viewport;
mod virtual_text;
mod wrap;

pub use buffer_view::BufferView;
pub use completion::{CompletionItem, CompletionKind, CompletionState};
pub use completion_source::{SourceConfig, SourceKind, SourceManager, SourcePriority};
pub use conceal::{ConcealLevel, ConcealRegion, ConcealState, LineConceal};
pub use cursor::{CursorAppearance, CursorBlink, CursorShape};
pub use dimensions::Dimensions;
pub use float::{FloatAnchor, FloatBorder, FloatConfig, FloatRelative, FloatState, FloatWindow};
pub use fold::{Fold, FoldMethod, FoldState};
pub use highlight::{Color, HlGroup, HlGroups, TextAttr};
pub use layout::{Layout, LayoutNode, SplitDirection};
pub use popup::{PopupItem, PopupMenu, PopupState};
pub use scroll::{CursorPosition, ScrollAmount, ScrollDirection, ScrollState};
pub use sign::{Sign, SignColumn, SignDefinition, SignPriority};
pub use snapshot::EditorSnapshot;
pub use status::StatusLine;
pub use terminal::{TermBuffer, TermCell, TermCursor, TermSize, TermState};
pub use viewport::Viewport;
pub use virtual_text::{VirtualText, VirtualTextChunk, VirtualTextPos, VirtualTextState};
pub use wrap::{WrapMode, WrapSegment, WrapState, WrappedLine};
