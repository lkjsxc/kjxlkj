/// Shared domain types for kjxlkj editor.
///
/// This crate defines the foundational types used across
/// all crates in the kjxlkj workspace.
mod action;
mod buffer;
mod cursor;
mod key;
mod mode;
mod service;
mod window;

pub use action::Action;
pub use buffer::{BufferId, BufferName, BufferVersion, Encoding, LineEnding};
pub use cursor::CursorPosition;
pub use key::{Key, KeyCode, Modifier};
pub use mode::{CommandKind, Mode, Operator, VisualKind};
pub use service::{ServiceRequest, ServiceResponse};
pub use window::{ContentSource, LayoutNode, TerminalId, WindowId};
