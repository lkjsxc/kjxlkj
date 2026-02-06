//! Editing primitives: motions, operators, text objects.

mod motion;
mod operator;
mod text_object;

pub use motion::{apply_motion, compute_motion_range};
pub use operator::apply_operator;
pub use text_object::find_text_object;
