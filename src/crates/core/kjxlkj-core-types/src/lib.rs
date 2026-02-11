//! Core type definitions for kjxlkj.
//!
//! Shared types used across all crates: IDs, modes, actions, keys, etc.

mod action;
mod ids;
mod key;
mod mode;
mod motion_info;
mod snapshot;
mod window;

pub use action::{Action, Direction, Motion, ResizeEdge};
pub use ids::{BufferId, ExplorerStateId, TerminalId, WindowId};
pub use key::{Key, KeyModifiers};
pub use mode::{CommandKind, ForceModifier, Inclusivity, Mode, Operator, RangeType, VisualKind};
pub use motion_info::{motion_inclusivity, motion_range_type};
pub use snapshot::{CmdlineState, EditorSnapshot, Rect, WindowContent};
pub use window::{ContentKind, WindowType};
