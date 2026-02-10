//! Shared domain types for kjxlkj.
//!
//! This crate defines fundamental types used across all other crates:
//! buffer identifiers, window identifiers, mode enums, actions, keys, etc.

mod action;
mod buffer;
mod key;
mod mode;
mod terminal;
mod window;

pub use action::{Action, KeyAction, MotionAction, ServiceResponse};
pub use buffer::{BufferId, BufferName, BufferVersion, Encoding, LineEnding};
pub use key::{Key, KeyCode, KeyModifiers};
pub use mode::{CommandKind, Mode, Operator, VisualKind};
pub use terminal::TerminalId;
pub use window::WindowId;
