//! UI model types and snapshot structures.
//!
//! Per /docs/spec/architecture/render-pipeline.md, the render task receives
//! an `EditorSnapshot` via a watch channel and uses it to build cell grids
//! without touching core state.

mod buffer_snapshot;
mod cmdline;
mod layout;
mod notification;
mod snapshot;
mod terminal_snapshot;
mod theme;

pub use buffer_snapshot::BufferSnapshot;
pub use cmdline::CmdlineState;
pub use layout::{LayoutNode, Rect, WindowLayout};
pub use notification::{Notification, NotificationLevel};
pub use snapshot::EditorSnapshot;
pub use terminal_snapshot::TerminalSnapshot;
pub use theme::{Theme, ThemeColor};
