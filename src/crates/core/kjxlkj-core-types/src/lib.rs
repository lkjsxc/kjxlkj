//! Core type definitions for kjxlkj.
//!
//! Shared types used across all crates: IDs, modes, actions, keys, etc.

mod action;
mod ids;
mod key;
mod mode;
mod snapshot;
mod window;

pub use action::{Action, Direction, Motion};
pub use ids::{BufferId, ExplorerStateId, TerminalId, WindowId};
pub use key::{Key, KeyModifiers};
pub use mode::{CommandKind, Mode, Operator, VisualKind};
pub use snapshot::{CmdlineState, EditorSnapshot, Rect, WindowContent};
pub use window::{ContentKind, WindowType};
