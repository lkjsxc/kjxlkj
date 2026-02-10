//! Snapshot and UI model types for rendering.

mod snapshot;

pub use snapshot::{
    BufferSnapshot, CmdlineState, Color, EditorSnapshot, LayoutChild, LayoutNode, Notification,
    NotificationLevel, SearchState, TabSnapshot, TerminalCell, TerminalSnapshot, WindowContent,
    WindowSnapshot,
};
