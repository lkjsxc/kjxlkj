//! Core type definitions for kjxlkj editor.
//!
//! This crate contains fundamental types used across all kjxlkj crates.

pub mod buffer;
pub mod cursor;
pub mod ids;
pub mod intent;
pub mod intent_ui;
pub mod mode;
pub mod motion;
pub mod operator;
pub mod position;
pub mod register;
pub mod snapshot;
pub mod text_object;

// Re-export commonly used types
pub use buffer::{BufferFlags, BufferInfo, BufferName};
pub use cursor::{Cursor, Selection};
pub use ids::{BufferId, BufferVersion, MarkId, RegisterId, WindowId};
pub use mode::{CommandKind, Mode, VisualMode};
pub use motion::{Direction, Motion, MotionResult};
pub use operator::{Operator, OperatorResult, PendingOperator};
pub use position::{ByteOffset, CharOffset, LineIdx, Position};
pub use register::{Register, RegisterContent, RegisterType};
pub use snapshot::{EditorSnapshot, WindowDimensions, WindowSnapshot};
pub use text_object::{TextObject, TextObjectScope, TextRange};
