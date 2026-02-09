/// Snapshot and UI model types for rendering.
///
/// The render task consumes immutable `EditorSnapshot`s
/// published by the core task via a watch channel.
mod snapshot;
mod theme;

pub use snapshot::{
    BufferSnapshot, CmdlineState, EditorSnapshot, Notification, NotificationLevel, SearchState,
    TabSnapshot, VisualSelection, WindowArea, WindowSnapshot,
};
pub use theme::{Color, Style, Theme};
