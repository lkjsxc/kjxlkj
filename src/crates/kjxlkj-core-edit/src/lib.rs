//! Editing primitives: motions, operators, text objects.

mod motion; mod motion_extra; mod operator;
mod text_object; mod text_object_delim;
mod text_objects_ext; mod regex_engine; mod editing_features;
mod range_address; mod syntax_command; mod visual_selection;
mod cursor_overlay;
mod keybinding_full;
mod ui_views;
mod ui_components;

pub use motion::{apply_motion, compute_motion_range};
pub use operator::apply_operator;
pub use text_object::find_text_object;
