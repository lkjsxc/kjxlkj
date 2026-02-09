/// Editing primitives: motions, operators, text objects.
///
/// All edit operations work through `EditOp` which is applied
/// to a buffer's rope by the core-state crate.
mod motion;
mod motion_helpers;
mod operator;
mod register;
mod text_object;

pub use motion::{resolve_motion, Motion, MotionKind};
pub use operator::EditOp;
pub use register::{Register, RegisterFile, RegisterName};
pub use text_object::{TextObject, TextObjectKind};
