//! Editing primitives: motions, operators, text objects.

mod motion; mod motion_extra; mod operator;
mod text_object; mod text_object_delim;
mod text_objects_ext;

pub use motion::{apply_motion, compute_motion_range};
pub use operator::apply_operator;
pub use text_object::find_text_object;
