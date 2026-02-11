//! Window tree layout model.
//!
//! See /docs/spec/editor/windows.md for normative layout tree.

mod layout;
mod focus;

pub use layout::{LayoutNode, LayoutTree};
pub use focus::FocusState;
