//! Window tree layout model.
//!
//! See /docs/spec/editor/windows.md for normative layout tree.

mod layout;
mod layout_ops;
mod layout_resize;
mod focus;
pub mod theme;
pub mod statusline;
pub mod messages;
pub mod viewport;
pub mod float_win;
pub mod statusline_dsl;
pub mod tabs;
pub mod zoom;
pub mod mode_config;
pub mod completion;

#[cfg(test)]
mod completion_tests;

pub use layout::{LayoutNode, LayoutTree};
pub use focus::FocusState;
