//! Application driver for the kjxlkj editor.

mod command_mode;
mod insert_mode;
mod normal_mode;
mod operator_pending;
mod render;
mod replace_mode;
mod visual_mode;

mod application;

pub use application::Application;

